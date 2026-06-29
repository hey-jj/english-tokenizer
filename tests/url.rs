//! URL detection. URLs need an explicit http or https scheme. Bare domains are
//! not URLs.

use english_tokenizer::Tokenizer;

fn pairs(tk: &mut Tokenizer, input: &str) -> Vec<(String, String)> {
    tk.tokenize(input)
        .into_iter()
        .map(|t| (t.value, t.tag))
        .collect()
}

#[test]
fn url_with_path() {
    let mut tk = Tokenizer::new();
    let got = pairs(&mut tk, "check http://example.org/my-regression-tree/ now");
    assert_eq!(
        got,
        vec![
            ("check".to_string(), "word".to_string()),
            (
                "http://example.org/my-regression-tree/".to_string(),
                "url".to_string()
            ),
            ("now".to_string(), "word".to_string()),
        ]
    );
    assert_eq!(tk.get_tokens_fp(), "wuw");
}

#[test]
fn url_next_to_punctuation() {
    let mut tk = Tokenizer::new();
    let got = pairs(&mut tk, "(see https://github.com).");
    assert_eq!(
        got,
        vec![
            ("(".to_string(), "punctuation".to_string()),
            ("see".to_string(), "word".to_string()),
            ("https://github.com".to_string(), "url".to_string()),
            (")".to_string(), "punctuation".to_string()),
            (".".to_string(), "punctuation".to_string()),
        ]
    );
}

#[test]
fn bare_domain_is_not_a_url() {
    let mut tk = Tokenizer::new();
    let got = pairs(&mut tk, "example.org without scheme");
    assert_eq!(
        got,
        vec![
            ("example".to_string(), "word".to_string()),
            (".".to_string(), "punctuation".to_string()),
            ("org".to_string(), "word".to_string()),
            ("without".to_string(), "word".to_string()),
            ("scheme".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn email_and_url_keep_their_tags() {
    let mut tk = Tokenizer::new();
    let got = pairs(&mut tk, "a@b.com and http://b.com");
    assert_eq!(
        got,
        vec![
            ("a@b.com".to_string(), "email".to_string()),
            ("and".to_string(), "word".to_string()),
            ("http://b.com".to_string(), "url".to_string()),
        ]
    );
}
