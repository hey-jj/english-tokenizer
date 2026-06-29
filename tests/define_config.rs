//! Config behavior: counts, enabling and disabling categories, and the alien
//! fallback when nothing is active.

use english_tokenizer::Tokenizer;

fn pairs(tk: &mut Tokenizer, input: &str) -> Vec<(String, String)> {
    tk.tokenize(input)
        .into_iter()
        .map(|t| (t.value, t.tag))
        .collect()
}

#[test]
fn default_count_is_thirteen() {
    let mut tk = Tokenizer::new();
    // The default instance has quoted_phrase off. Re-applying that gives 13.
    assert_eq!(tk.define_config(&[("quoted_phrase", false)]), 13);
}

#[test]
fn quoted_phrase_on_is_fourteen() {
    let mut tk = Tokenizer::new();
    assert_eq!(tk.define_config(&[("quoted_phrase", true)]), 14);
}

#[test]
fn empty_config_is_zero() {
    let mut tk = Tokenizer::new();
    assert_eq!(tk.define_config(&[]), 0);
}

#[test]
fn hashtag_off_is_thirteen() {
    let mut tk = Tokenizer::new();
    assert_eq!(tk.define_config(&[("hashtag", false)]), 13);
}

#[test]
fn absent_keys_stay_enabled() {
    let mut tk = Tokenizer::new();
    // Only word listed. Every other category is absent, so all 14 survive
    // including quoted_phrase.
    assert_eq!(tk.define_config(&[("word", true)]), 14);
}

#[test]
fn empty_config_tags_everything_alien() {
    let mut tk = Tokenizer::new();
    tk.define_config(&[]);
    let got = pairs(&mut tk, "r2d2@gmail.com; party");
    assert_eq!(
        got,
        vec![
            ("r2d2@gmail.com;".to_string(), "alien".to_string()),
            ("party".to_string(), "alien".to_string()),
        ]
    );
}

#[test]
fn disabling_hashtag_sends_hash_to_symbol() {
    let mut tk = Tokenizer::new();
    tk.define_config(&[("hashtag", false)]);
    let got = pairs(&mut tk, "good #fun");
    assert_eq!(
        got,
        vec![
            ("good".to_string(), "word".to_string()),
            ("#".to_string(), "symbol".to_string()),
            ("fun".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn config_resets_added_rules() {
    use regex::Regex;
    let mut tk = Tokenizer::new();
    tk.add_regex(Regex::new(r"(?i)superman").unwrap(), "superman", Some("s"))
        .unwrap();
    // The custom tag is active.
    let got = pairs(&mut tk, "superman");
    assert_eq!(got, vec![("superman".to_string(), "superman".to_string())]);
    // Reset removes the rule. The same input is now a word.
    tk.define_config(&[("word", true)]);
    let got = pairs(&mut tk, "superman");
    assert_eq!(got, vec![("superman".to_string(), "word".to_string())]);
}
