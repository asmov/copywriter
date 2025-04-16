//TODO: This needs to be moved to copywriter bin to avoid circular dependencies

#[cfg(test)]
mod tests {
    use asmov_copywriter_lib::{self as copywriter_lib, Content};
    use asmov_copywriter_lib_model::{self as model, prelude::*};
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

        let config = copywriter_lib::Config {
            input_dir: test.fixture_dir().to_path_buf(),
            output_dir: test.temp_dir().to_path_buf(),
        };

        let site = copywriter_lib::Site {
            name: "Basic Test Website".to_string(),
            url: "http://localhost:8080".to_string(),
            owner: "The Webmaster".to_string(),
            owner_url: "http://127.0.0.1:8080".to_string(),
            description: "This is a basic website".to_string(),
        };

        let fixture_input_dir = test.fixture_dir().join("input");
        let content_dir = fixture_input_dir.join("content");

        let mut registry = copywriter_lib::modeling::ModelTypeRegistry::new();
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

            //toml
            let toml_file = article_dir.join(model::Article::type_toml_data_filename());
            let toml = std::fs::read_to_string(toml_file).unwrap();
            let article_toml: model::Article = toml::from_str(&toml).unwrap();

            assert!(article_toml.validate().is_ok());

            //markdown
            let md = std::fs::read_to_string(article_dir.join(model::Article::type_markdown_content_filename())).unwrap();
            let md_parser = pulldown_cmark::Parser::new(&md);
            let mut md_content = String::new();
            pulldown_cmark::html::push_html(&mut md_content, md_parser);

            assert_eq!(slug, article_toml.slug);
            dbg!(slug);
            dbg!(&article_toml);
            dbg!(&md_content);

            let model_pack = model::ModelBundle {
                meta: article_toml,
                content: Content {
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
        let mut handlebars = handlebars::Handlebars::new();

        let index_json = serde_json::json!({
            "articles": articles,
            "site": site
        });

        let index_html = handlebars.render_template(&index_hbs, &index_json).unwrap();

        println!("{}", index_json);
        println!("{}", index_html);
        let tmpfile = std::path::PathBuf::from("/tmp/test.html");
        std::fs::write(&tmpfile, &index_html).unwrap();

        // create each article html
    }
}
