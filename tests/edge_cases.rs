//! Edge cases: empty input, possessives, contraction case variants, apostrophe
//! names, and the match and split interleaving at string boundaries.

use english_tokenizer::Tokenizer;

fn pairs(tk: &mut Tokenizer, input: &str) -> Vec<(String, String)> {
    tk.tokenize(input)
        .into_iter()
        .map(|t| (t.value, t.tag))
        .collect()
}

#[test]
fn empty_and_whitespace_only() {
    let mut tk = Tokenizer::new();
    assert!(tk.tokenize("").is_empty());
    assert!(tk.tokenize("   ").is_empty());
}

#[test]
fn contraction_case_variants() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "CAN'T"),
        vec![
            ("CA".to_string(), "word".to_string()),
            ("N'T".to_string(), "word".to_string()),
        ]
    );
    assert_eq!(
        pairs(&mut tk, "Can't"),
        vec![
            ("Ca".to_string(), "word".to_string()),
            ("n't".to_string(), "word".to_string()),
        ]
    );
    assert_eq!(
        pairs(&mut tk, "can't"),
        vec![
            ("ca".to_string(), "word".to_string()),
            ("n't".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn let_s_uppercase_splits_to_let() {
    // Uppercase LET'S splits to LET, matching the lower and title case forms.
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "LET'S"),
        vec![
            ("LET".to_string(), "word".to_string()),
            ("'S".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn singular_and_plural_possessive() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "dog's"),
        vec![
            ("dog".to_string(), "word".to_string()),
            ("'s".to_string(), "word".to_string()),
        ]
    );
    assert_eq!(
        pairs(&mut tk, "cats'"),
        vec![
            ("cats".to_string(), "word".to_string()),
            ("'".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn apostrophe_name_stays_whole() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "O'Hara"),
        vec![("O'Hara".to_string(), "word".to_string())]
    );
    assert_eq!(
        pairs(&mut tk, "O'kelly"),
        vec![("O'kelly".to_string(), "word".to_string())]
    );
}

#[test]
fn back_to_back_matches_no_gap() {
    // Currency and number alternate with no spaces. Each is its own token.
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "₽100₹200"),
        vec![
            ("₽".to_string(), "currency".to_string()),
            ("100".to_string(), "number".to_string()),
            ("₹".to_string(), "currency".to_string()),
            ("200".to_string(), "number".to_string()),
        ]
    );
}

#[test]
fn repeated_emoji_run() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "party\u{1F389}\u{1F389}\u{1F389}"),
        vec![
            ("party".to_string(), "word".to_string()),
            ("\u{1F389}".to_string(), "emoji".to_string()),
            ("\u{1F389}".to_string(), "emoji".to_string()),
            ("\u{1F389}".to_string(), "emoji".to_string()),
        ]
    );
}

#[test]
fn emoticon_before_time() {
    // <3 is an emoticon, matched before time. 4pm is left for the time rule.
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "<34pm"),
        vec![
            ("<3".to_string(), "emoticon".to_string()),
            ("4pm".to_string(), "time".to_string()),
        ]
    );
}

#[test]
fn hashtag_with_underscore() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "#happy_path"),
        vec![("#happy_path".to_string(), "hashtag".to_string())]
    );
}

#[test]
fn accented_prefix_with_possessive_tail() {
    // A non-ASCII letter prefix means the possessive split does not fire, so the
    // word stays whole and no leading characters are dropped.
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "ëdog's"),
        vec![("ëdog's".to_string(), "word".to_string())]
    );
    assert_eq!(
        pairs(&mut tk, "aëcats'"),
        vec![("aëcats'".to_string(), "word".to_string())]
    );
}

#[test]
fn devanagari_digits_stay_out_of_latin_rules() {
    // ASCII-only digits in the time and number rules keep Devanagari digits in
    // the Devanagari number rule.
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "१२३ hours"),
        vec![
            ("१२३".to_string(), "number".to_string()),
            ("hours".to_string(), "word".to_string()),
        ]
    );
    assert_eq!(
        pairs(&mut tk, "४pm"),
        vec![
            ("४".to_string(), "number".to_string()),
            ("pm".to_string(), "word".to_string()),
        ]
    );
    assert_eq!(
        pairs(&mut tk, "€1.2१२३"),
        vec![
            ("€".to_string(), "currency".to_string()),
            ("1.2".to_string(), "number".to_string()),
            ("१२३".to_string(), "number".to_string()),
        ]
    );
}

#[test]
fn emoji_off_makes_alien() {
    let mut tk = Tokenizer::new();
    tk.define_config(&[("emoji", false)]);
    assert_eq!(
        pairs(&mut tk, "party\u{1F389}"),
        vec![
            ("party".to_string(), "word".to_string()),
            ("\u{1F389}".to_string(), "alien".to_string()),
        ]
    );
}
