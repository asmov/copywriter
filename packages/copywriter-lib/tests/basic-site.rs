//TODO: This needs to be moved to copywriter bin to avoid circular dependencies

#[cfg(test)]
mod tests {
    use asmov_copywriter_lib as copywriter_lib;
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
            name: "Basic Site".to_string(),
            input_dir: test.fixture_dir().to_path_buf(),
            output_dir: test.temp_dir().to_path_buf(),
            url: "http://localhost:8080".to_string(),
        };

        let publish_dir = test.fixture_dir().join("input").join("content").join("publish");
        let articles_dir = publish_dir.join(model::Article::type_dirname());
        let article_dirs = articles_dir
            .read_dir().unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();

        for article_dir in article_dirs {
            let slug = article_dir.file_name().unwrap().to_string_lossy().to_string();
            let json_file = article_dir.join(model::Article::type_json_filename());
            let article: model::Article = serde_json::from_str(
                &std::fs::read_to_string(json_file).unwrap()
            ).unwrap();

            assert_eq!(slug, article.slug);
            dbg!("slug: {}", slug);
            dbg!("article: {:?}", article);
        }
    }
}
