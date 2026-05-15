use pulldown_cmark::{Options, Parser, html};

/// Convert a Markdown string to HTML. Enables GFM tables, footnotes, strikethrough, task lists.
pub fn to_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(md, options);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    html_buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings_and_paragraphs() {
        let h = to_html("# Title\n\nHello *world*.");
        assert!(h.contains("<h1>"));
        assert!(h.contains("<em>world</em>"));
    }

    #[test]
    fn tables_render() {
        let h = to_html("| a | b |\n|---|---|\n| 1 | 2 |");
        assert!(h.contains("<table>"));
        assert!(h.contains("<th>a</th>"));
    }

    #[test]
    fn links_render() {
        let h = to_html("[hi](https://example.com)");
        assert!(h.contains("href=\"https://example.com\""));
    }
}
