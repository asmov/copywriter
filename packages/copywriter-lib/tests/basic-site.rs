//TODO: This needs to be moved to copywriter bin to avoid circular dependencies
#[macro_use] extern  crate  slugify;
use slugify::slugify;

fn parse_markdown_model(md: pulldown_cmark::Parser)
-> anyhow::Result<(asmov_copywriter_lib::BasicModel, String)> {
    let mut h1 = None; // holds the first h1
    let mut h1_done = false; // TRUE: h1 is_some() and it's completely parsed
    let mut after_h1 = false; // TRUE: the current event is first element after an h1 being parsed
    let mut h1_blockquote = None; // holds the first blockquote after the first h1
    let mut h1_blockquote_done = false; // TRUE: h1_blockquote is_some() and it's completely parsed

    let md = md
        .filter_map(|event| match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading{level: pulldown_cmark::HeadingLevel::H1, .. }) => {
                if !h1_done && h1.is_none() {
                    h1 = Some(String::new());
                    None
                } else if after_h1 {
                    after_h1 = false;
                    Some(event)
                } else {
                    Some(event)
                }
            },
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::BlockQuote(None)) => {
                if after_h1 {
                    if h1_blockquote.is_none() {
                        h1_blockquote = Some(String::new());
                    } else {
                        h1_blockquote = None; // don't allow multiple blockquotes >>
                    }

                    after_h1 = false;
                    None
                } else {
                    Some(event)
                }
            },
            pulldown_cmark::Event::Text(ref text) => {
                if !h1_done {
                    if let Some(ref mut h1) = h1 {
                        h1.push_str(&text);
                        None
                    } else {
                        Some(event)
                    }
                } else if !h1_blockquote_done {
                    if let Some(ref mut blockquote) = h1_blockquote {
                        blockquote.push_str(&text);
                        None
                    } else {
                        Some(event)
                    }
                } else if after_h1 {
                    after_h1 = false;
                    Some(event)
                } else {
                    Some(event)
                }
            },
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Heading(pulldown_cmark::HeadingLevel::H1)) => {
                if !h1_done && h1.is_some() {
                    h1_done = true;
                    after_h1 = true;
                    None
                } else if after_h1 {
                    after_h1 = false;
                    Some(event)
                } else {
                    Some(event)
                }
            },
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::BlockQuote(None)) => {
                if after_h1 {
                    after_h1 = false;
                }

                if !h1_blockquote_done {
                    h1_blockquote_done = true;
                    None
                } else {
                    Some(event)
                }
            }
            _ => {
                if after_h1 {
                    after_h1 = false;
                }

                Some(event)
            }
        });

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, md);

    let name = h1.unwrap_or_default();
    let slug = slugify::slugify!(&name);
    let subline = h1_blockquote.unwrap_or_default();

    let basic_model = asmov_copywriter_lib::BasicModel {
        name,
        slug,
        subline,
    };

    dbg!(&basic_model);

    Ok((basic_model, html))
}

#[cfg(test)]
mod tests {
    use asmov_copywriter_lib::{self as lib, modeling::prelude::*};
    use asmov_copywriter_lib_model as model;
    use asmov_common_testing::{self as testing, prelude::*};

    static TESTING: testing::StaticModule = testing::module(|| {
        testing::integration(module_path!())
            .using_temp_dir()
            .using_fixture_dir()
            .build()
    });

    #[test]
    #[named]
    fn test_basic_site() {
        let test = TESTING.test(function_name!())
            .inherit_fixture_dir()
            .using_temp_dir()
            .build();

        let _config = lib::Config {
            input_dir: test.fixture_dir().to_path_buf(),
            output_dir: test.temp_dir().to_path_buf(),
        };

        let site = lib::Site {
            slug: "basic-test-website".to_string(),
            name: "Basic Test Website".to_string(),
            subline: "This is a basic website".to_string(),
            url: "http://localhost:8080".to_string(),
            owner_name: "The Webmaster".to_string(),
            owner_url: "http://127.0.0.1:8080".to_string(),
            description: "This is a basic website".to_string(),
        };

        let fixture_input_dir = test.fixture_dir().join("input");
        let content_dir = fixture_input_dir.join("content");

        let mut registry = lib::modeling::ModelTypeRegistry::new();
        registry.register(model::Article::model_type());

        // parse articles
        let articles_dir = content_dir.join(model::Article::type_dirname());
        let article_dirs = articles_dir
            .read_dir().unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();

        let mut articles = Vec::new();
        for article_dir in article_dirs {
            let slug = article_dir.file_name().unwrap().to_string_lossy().to_string();

            // markdown
            let md = std::fs::read_to_string(article_dir.join(model::Article::type_markdown_content_filename())).unwrap();
            let md_parser = pulldown_cmark::Parser::new(&md);
            let (md_basic_model, md_content) = crate::parse_markdown_model(md_parser).unwrap();

            assert!(!md_basic_model.name.is_empty());
            assert!(!md_basic_model.slug.is_empty());

            // toml
            let toml_file = article_dir.join(model::Article::type_toml_data_filename());
            let toml = std::fs::read_to_string(toml_file).unwrap();
            let mut article_toml: model::Article = toml::from_str(&toml).unwrap();

            if article_toml.slug.is_empty() {
                article_toml.slug = slug.clone();
            }
            if article_toml.name.is_empty() {
                article_toml.name = md_basic_model.name.clone();
            }
            if article_toml.subline.is_empty() {
                article_toml.subline = md_basic_model.subline.clone();
            }

            assert!(article_toml.validate().is_ok());

            assert_eq!(slug, article_toml.slug);
            dbg!(slug);
            dbg!(&article_toml);
            dbg!(&md_content);

            let model_pack = lib::ModelBundle {
                meta: article_toml,
                content: lib::Content {
                    html: Some(md_content),
                    ..Default::default()
                }
            };

            articles.push(model_pack)
        }

        let src_dir = fixture_input_dir.join("src");
        let hbs_dir = src_dir.join("hbs");

        // create index.html
        let index_index_file = hbs_dir.join("index.hbs");
        let index_hbs = std::fs::read_to_string(index_index_file).unwrap();
        let handlebars = handlebars::Handlebars::new();

        let index_json = serde_json::json!({
            "articles": articles,
            "site": site
        });

        let index_html = handlebars.render_template(&index_hbs, &index_json).unwrap();

        println!("{}", index_json);
        println!("{}", index_html);
        //let tmpfile = std::path::PathBuf::from("/tmp/test.html");
        //std::fs::write(&tmpfile, &index_html).unwrap();

        // create each article html
    }
}
