//! Adding rules and tags, including error paths and precedence.

use english_tokenizer::{AddError, Tokenizer};
use regex::Regex;

fn pairs(tk: &mut Tokenizer, input: &str) -> Vec<(String, String)> {
    tk.tokenize(input)
        .into_iter()
        .map(|t| (t.value, t.tag))
        .collect()
}

#[test]
fn existing_tag_no_code_needed() {
    let mut tk = Tokenizer::new();
    tk.add_regex(Regex::new(r"(?i)\(oo\)").unwrap(), "emoticon", None)
        .unwrap();
    let got = pairs(&mut tk, "(oo) Hi!");
    assert_eq!(
        got,
        vec![
            ("(oo)".to_string(), "emoticon".to_string()),
            ("Hi".to_string(), "word".to_string()),
            ("!".to_string(), "punctuation".to_string()),
        ]
    );
}

#[test]
fn new_tag_requires_code() {
    let mut tk = Tokenizer::new();
    let err = tk
        .add_regex(Regex::new(r"(?i)\(oo\)").unwrap(), "pig", None)
        .unwrap_err();
    assert_eq!(
        err,
        AddError::MissingFingerprint {
            tag: "pig".to_string()
        }
    );
    assert_eq!(
        err.to_string(),
        "Tag pig doesn't exist; Provide a 'fingerprintCode' to add it as a tag."
    );
}

#[test]
fn new_tag_with_code() {
    let mut tk = Tokenizer::new();
    tk.add_regex(Regex::new(r"(?i)hello").unwrap(), "greeting", Some("g"))
        .unwrap();
    let got = pairs(&mut tk, "hello, how are you?");
    assert_eq!(got[0], ("hello".to_string(), "greeting".to_string()));
    // The new code shows in the fingerprint.
    assert_eq!(tk.get_tokens_fp(), "g,www?");
}

#[test]
fn added_rule_wins_over_builtins() {
    let mut tk = Tokenizer::new();
    // superman would otherwise be a word. The custom rule captures it first.
    tk.add_regex(Regex::new(r"(?i)superman").unwrap(), "superman", Some("s"))
        .unwrap();
    let got = pairs(&mut tk, "why superman");
    assert_eq!(
        got,
        vec![
            ("why".to_string(), "word".to_string()),
            ("superman".to_string(), "superman".to_string()),
        ]
    );
}

#[test]
fn add_tag_duplicate_errors() {
    let mut tk = Tokenizer::new();
    let err = tk.add_tag("emoticon", "8").unwrap_err();
    assert_eq!(
        err,
        AddError::TagExists {
            tag: "emoticon".to_string()
        }
    );
    assert_eq!(err.to_string(), "Tag emoticon already exists");
}

#[test]
fn add_tag_empty_code_errors() {
    let mut tk = Tokenizer::new();
    let err = tk.add_tag("blank", "").unwrap_err();
    assert_eq!(
        err,
        AddError::MissingFingerprint {
            tag: "blank".to_string()
        }
    );
}

#[test]
fn add_tag_then_fingerprint() {
    let mut tk = Tokenizer::new();
    tk.add_tag("greeting", "g").unwrap();
    tk.add_regex(Regex::new(r"(?i)hi").unwrap(), "greeting", None)
        .unwrap();
    let got = pairs(&mut tk, "hi there");
    assert_eq!(got[0], ("hi".to_string(), "greeting".to_string()));
    assert_eq!(tk.get_tokens_fp(), "gw");
}

#[test]
fn new_tag_empty_code_errors() {
    let mut tk = Tokenizer::new();
    let err = tk
        .add_regex(Regex::new(r"(?i)zzz").unwrap(), "zzztag", Some(""))
        .unwrap_err();
    assert_eq!(
        err,
        AddError::MissingFingerprint {
            tag: "zzztag".to_string()
        }
    );
}

#[test]
fn built_in_tag_without_code_needs_no_code() {
    let mut tk = Tokenizer::new();
    tk.add_regex(Regex::new(r"--").unwrap(), "punctuation", None)
        .unwrap();
    let got = pairs(&mut tk, "a -- b");
    assert_eq!(
        got,
        vec![
            ("a".to_string(), "word".to_string()),
            ("--".to_string(), "punctuation".to_string()),
            ("b".to_string(), "word".to_string()),
        ]
    );
    assert_eq!(tk.get_tokens_fp(), "w--w");
}

#[test]
fn capturing_group_does_not_inject_token() {
    // A user regex with a capturing group emits only the whole match, not the
    // capture. `3x` becomes one number token, not `3x` plus `3`.
    let mut tk = Tokenizer::new();
    tk.add_regex(Regex::new(r"(?i)(\d)x").unwrap(), "number", None)
        .unwrap();
    let got = pairs(&mut tk, "a 3x b");
    assert_eq!(
        got,
        vec![
            ("a".to_string(), "word".to_string()),
            ("3x".to_string(), "number".to_string()),
            ("b".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn tokenizer_implements_debug() {
    let tk = Tokenizer::new();
    let shown = format!("{tk:?}");
    assert!(shown.contains("Tokenizer"));
}
