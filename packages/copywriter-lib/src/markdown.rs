use slugify::slugify;
use pulldown_cmark as md;
use crate::*;

/// Parses a markdown document into a BasicModel and an HTML string.
/// The first H1 is removed from the HTML and stored in [BasicModel::name].
/// The first blockquote after the first H1 is removed from the HTML and stored in [BasicModel::subline].
/// All other headers are bumped up one level. Eg. H2 -> H1
/// The [BasicModel::slug] is generated from [BasicModel::name].
pub fn parse_markdown_model(markdown: md::Parser) -> anyhow::Result<(BasicModel, String)> {
    let mut h1 = None; // holds the first h1
    let mut h1_done = false; // TRUE: h1 is_some() and it's completely parsed
    let mut after_h1 = false; // TRUE: the current event is first element after an h1 being parsed
    let mut h1_blockquote = None; // holds the first blockquote after the first h1
    let mut h1_blockquote_done = false; // TRUE: h1_blockquote is_some() and it's completely parsed

    let markdown = markdown
        .filter_map(|event| match event {
            md::Event::Start(md::Tag::Heading{level: md::HeadingLevel::H1, .. }) => {
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
            md::Event::End(md::TagEnd::Heading(md::HeadingLevel::H1)) => {
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

    let basic_model = BasicModel {
        name,
        slug,
        subline,
    };

    dbg!(&basic_model);

    Ok((basic_model, html))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_markdown_model() {
        let markdown = md::Parser::new(
r"# Hello World
> This is a subline

## Section 1

### Subsection 1.1

This *is* some [text](#Section-1).

### Subsection 1.2

This is also **some** text.");

        let (basic_model, html) = parse_markdown_model(markdown).unwrap();

        dbg!(html);

        assert_eq!(basic_model.name, "Hello World");
        assert_eq!(basic_model.slug, "hello-world");
        assert_eq!(basic_model.subline, "This is a subline");

    }
}
