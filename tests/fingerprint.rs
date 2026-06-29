//! Fingerprint behavior.
//!
//! Each token contributes its tag code, or its literal value when the tag has no
//! code. Punctuation and symbol have no code, so they pass through.

use english_tokenizer::Tokenizer;

#[test]
fn empty_before_any_tokenize() {
    let tk = Tokenizer::new();
    assert_eq!(tk.get_tokens_fp(), "");
}

#[test]
fn symbol_and_punctuation_pass_through() {
    let mut tk = Tokenizer::new();
    tk.tokenize("a % b , c");
    // word, symbol value, word, punctuation value, word.
    assert_eq!(tk.get_tokens_fp(), "w%w,w");
}

#[test]
fn slash_symbol_in_fingerprint() {
    let mut tk = Tokenizer::new();
    tk.tokenize("two/three");
    assert_eq!(tk.get_tokens_fp(), "w/w");
}

#[test]
fn simple_word_hashtag() {
    let mut tk = Tokenizer::new();
    tk.tokenize("feeling good #FunTime");
    assert_eq!(tk.get_tokens_fp(), "wwh");
}

#[test]
fn currency_and_number() {
    let mut tk = Tokenizer::new();
    tk.tokenize("$5");
    assert_eq!(tk.get_tokens_fp(), "rn");
}
