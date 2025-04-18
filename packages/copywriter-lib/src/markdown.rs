use slugify::slugify;
use pulldown_cmark as md;
use crate::*;

/// Parses a markdown document into a BasicModel and an HTML string.
/// The first H1 is removed from the HTML and stored in [BasicModel::name].
/// The first blockquote after the first H1 is removed from the HTML and stored in [BasicModel::subline].
/// All other headers are bumped up one level. Eg. H2 -> H1
/// The [BasicModel::slug] is generated from [BasicModel::name].
pub fn parse_markdown_model(markdown: md::Parser) -> anyhow::Result<(ModelBase, String)> {
    let mut h1 = None; // holds the first h1
    let mut h1_done = false; // TRUE: h1 is_some() and it's completely parsed
    let mut after_h1 = false; // TRUE: the current event is first element after an h1 being parsed
    let mut h1_blockquote = None; // holds the first blockquote after the first h1
    let mut h1_blockquote_done = false; // TRUE: h1_blockquote is_some() and it's completely parsed

    let markdown = markdown
        .filter_map(|event| match event {
            md::Event::Start(md::Tag::Heading{level: md::HeadingLevel::H1, .. }) => {
                if after_h1 {
                    after_h1 = false;
                }

                if !h1_done && h1.is_none() {
                    h1 = Some(String::new());
                    None
                } else {
                    Some(event)
                }
            },
            md::Event::Start(md::Tag::Heading{level, id, classes, attrs }) => {
                let mut level_ord = level as usize;
                if level_ord > 1 {
                    level_ord -= 1;
                }

                let level = md::HeadingLevel::try_from(level_ord).expect("Invalid heading level");
                Some(md::Event::Start(md::Tag::Heading{level, id, classes, attrs }))
            },
            md::Event::Start(md::Tag::Paragraph) | md::Event::End(md::TagEnd::Paragraph) => {
                if after_h1 {
                    after_h1 = false;
                }

                if !h1_blockquote_done && h1_blockquote.is_some() {
                    None
                } else {
                    Some(event)
                }
            },
            md::Event::Start(md::Tag::BlockQuote(None)) => {
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
            md::Event::Text(ref text) => {
                if after_h1 {
                    after_h1 = false;
                }

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
                } else {
                    Some(event)
                }
            },
            md::Event::End(md::TagEnd::Heading(md::HeadingLevel::H1)) => {
                if !h1_done && h1.is_some() {
                    h1_done = true;
                    after_h1 = true;
                    None
                } else {
                    if after_h1 {
                        after_h1 = false;
                    }

                    Some(event)
                }
            },
            md::Event::End(md::TagEnd::Heading(level)) => {
                let mut level_ord = level as usize;
                if level_ord > 1 {
                    level_ord -= 1;
                }

                let level = md::HeadingLevel::try_from(level_ord).expect("Invalid heading level");
                Some(md::Event::End(md::TagEnd::Heading(level)))
            },
            md::Event::End(md::TagEnd::BlockQuote(None)) => {
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
    md::html::push_html(&mut html, markdown);

    let name = h1.unwrap_or_default();
    let slug = slugify::slugify!(&name);
    let subline = h1_blockquote.unwrap_or_default();

    let basic_model = ModelBase {
        name,
        slug,
        subline,
    };

    Ok((basic_model, html))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_markdown_model() {
        const INPUT: &str =
r"# Hello World?
> This is a subline

## Section 1

### Subsection 1.1

This *is* some [text](#Section-1).

### Subsection 1.2

This is also **some** text.";

        const EXPECTED_HTML_UNALTERED: &str =
r##"<h1>Hello World?</h1>
<blockquote>
<p>This is a subline</p>
</blockquote>
<h2>Section 1</h2>
<h3>Subsection 1.1</h3>
<p>This <em>is</em> some <a href="#Section-1">text</a>.</p>
<h3>Subsection 1.2</h3>
<p>This is also <strong>some</strong> text.</p>
"##;

        const EXPECTED_HTML_PARSED: &str =
r##"<h1>Section 1</h1>
<h2>Subsection 1.1</h2>
<p>This <em>is</em> some <a href="#Section-1">text</a>.</p>
<h2>Subsection 1.2</h2>
<p>This is also <strong>some</strong> text.</p>
"##;

        let expected_model: ModelBase = ModelBase {
            name: "Hello World?".to_string(),
            slug: "hello-world".to_string(),
            subline: "This is a subline".to_string(),
        };

        // sanity check: make sure markdown is being parsed normally as expected
        let markdown = md::Parser::new(INPUT);
        let mut unaltered_html = String::new();
        md::html::push_html(&mut unaltered_html, markdown);
        assert_eq!(EXPECTED_HTML_UNALTERED, unaltered_html, "Unaltered HTML should be parsed");

        let markdown = md::Parser::new(INPUT);
        let (basic_model, html) = parse_markdown_model(markdown).expect("HTML should be parsed from Markdown");
        assert_eq!(EXPECTED_HTML_PARSED, html, "HTML should be parsed from Markdown");

        assert_eq!(expected_model, basic_model, "Basic model should be parsed from Markdown");
    }
}
