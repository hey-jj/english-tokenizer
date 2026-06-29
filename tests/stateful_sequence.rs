//! Stateful sequence test.
//!
//! The spec runs the first block of cases on one shared instance. Config changes
//! and added rules carry into later cases. This single test walks that sequence
//! in order to lock the carry over semantics: define_config resets added rules
//! and fingerprint codes, and the fingerprint reads the last tokenize result.

mod common;

use english_tokenizer::{Token, Tokenizer};
use regex::Regex;

fn pairs(tokens: Vec<Token>) -> Vec<(String, String)> {
    tokens.into_iter().map(|t| (t.value, t.tag)).collect()
}

fn expect(label: &str) -> (String, Vec<(String, String)>) {
    common::tokenize_case(label)
}

#[test]
fn shared_instance_sequence() {
    let mut tk = Tokenizer::new();

    // 1: complex tweet.
    let (input, want) = expect("complex_tweet_shared");
    assert_eq!(pairs(tk.tokenize(&input)), want);

    // 2: fingerprint of the previous tokenize.
    assert_eq!(tk.get_tokens_fp(), "m:wwwwwwe;&wwwwjjjjctcjwwtcch");

    // 3: blank inputs produce no tokens.
    let (input, want) = expect("blank_empty");
    assert_eq!(pairs(tk.tokenize(&input)), want);
    let (input, want) = expect("blank_spaces");
    assert_eq!(pairs(tk.tokenize(&input)), want);

    // 4: simple hashtag.
    let (input, want) = expect("simple_hashtag");
    assert_eq!(pairs(tk.tokenize(&input)), want);

    // 5: fingerprint.
    assert_eq!(tk.get_tokens_fp(), "wwh");

    // 6: hashtag off returns 13, then # falls through to symbol.
    assert_eq!(tk.define_config(&[("hashtag", false)]), 13);
    let (input, want) = expect("hashtag_off");
    assert_eq!(pairs(tk.tokenize(&input)), want);

    // 7: complex sentence with hashtag still off (input has none).
    let (input, want) = expect("complex_full");
    assert_eq!(pairs(tk.tokenize(&input)), want);

    // 8: fingerprint.
    assert_eq!(tk.get_tokens_fp(), "m:wwwwwwe;&wwwwjwwtc");

    // 9: add a custom emoticon regex for an existing tag. It wins over built-ins.
    tk.add_regex(
        Regex::new(r"(?i):\||O\.O|:`\(|\+o\(|\(oo\)|:%|:\$|>\|<|<-").unwrap(),
        "emoticon",
        None,
    )
    .unwrap();
    let (input, want) = expect("custom_emoticon");
    assert_eq!(pairs(tk.tokenize(&input)), want);

    // 10: adding a regex for an unknown tag without a code is an error.
    let err = tk
        .add_regex(Regex::new(r"(?i)\(oo\)").unwrap(), "pig", None)
        .unwrap_err();
    assert_eq!(
        err.to_string(),
        "Tag pig doesn't exist; Provide a 'fingerprintCode' to add it as a tag."
    );

    // 11: adding a tag that already exists is an error.
    let err = tk.add_tag("emoticon", "8").unwrap_err();
    assert_eq!(err.to_string(), "Tag emoticon already exists");

    // 12: add a new tag via add_regex, tokenize, fingerprint, then reset.
    tk.add_regex(Regex::new(r"(?i)superman").unwrap(), "superman", Some("s"))
        .unwrap();
    let (input, want) = expect("superman_tag");
    assert_eq!(pairs(tk.tokenize(&input)), want);
    assert_eq!(tk.get_tokens_fp(), "wwwws'ww?!");

    // define_config resets the added rule and fingerprint codes.
    assert_eq!(tk.define_config(&[("word", true)]), 14);
    let (input, want) = expect("superman_reset");
    assert_eq!(pairs(tk.tokenize(&input)), want);
    assert_eq!(tk.get_tokens_fp(), "wwwwwww?!");

    // 13: empty config returns 0 and tags everything alien.
    assert_eq!(tk.define_config(&[]), 0);
    let (input, want) = expect("empty_config_alien");
    assert_eq!(pairs(tk.tokenize(&input)), want);
}
