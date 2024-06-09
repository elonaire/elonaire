use regex::{Captures, Regex};

pub struct SyntaxHighlighter {
    pub code: String,
}

impl SyntaxHighlighter {
    pub fn new(code: String) -> Self {
        Self { code }
    }

    pub fn highlight_code(mut self) -> String {
        let code_tag_regex =
            Regex::new(r"(?s)<code class=[^a-z]language-(\w+)[^a-z]>(.*?)</code>").unwrap();
        code_tag_regex
            .replace_all(&mut self.code, |caps: &Captures| {
                // Match the language and content inside the <code> tag
                let lang = &caps[1];
                let content = &caps[2];

                // Depending on the language class, call the appropriate highlight function
                let highlighted_content = match lang {
                    "html" => highlight_html(content),
                    // "rust" => highlight_rust(content),
                    // "js" => highlight_js(content),
                    // "ts" => highlight_ts(content),
                    // Other languages or default case if needed
                    _ => content.to_string(),
                };

                // // Rebuild the <code> tag with the highlighted content
                let highlighted = format!(
                    r#"<code class="language-{lang}">{content}</code>"#,
                    lang = lang,
                    content = highlighted_content
                );
                log::info!("highlighted: {:?}", highlighted);
                highlighted
            })
            .to_string()
    }

    
}

pub fn highlight_html(code: &str) -> String {
    // Highlight HTML content
    let tag_re = Regex::new(r"(&lt;/?[a-z]+&gt;)").unwrap();
    let attr_re = Regex::new(r"([a-z\-]+)(=)").unwrap();

    let code = tag_re.replace_all(
        code,
        r#"<span class="html-tag">$1</span>"#,
    );
    // let code = attr_re.replace_all(
    //     &code,
    //     r#"&lt;span class=&quot;html-attr&quot;&gt;$1&lt;/span&gt;$2"#,
    // );
    let code = attr_re.replace_all(
        &code,
        r#"<span class="html-attr">$1</span>$2"#,
    );

    code.to_string()
}
