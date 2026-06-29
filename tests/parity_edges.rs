//! Edge cases that guard behavior the main golden set does not reach.
//!
//! These lock down corners where a regex engine could drift: time alternation,
//! standalone number formats, possessive casing, contraction fallthrough,
//! complex emoji clusters, and the empty value alien token from leading
//! whitespace. Expected values are derived by hand from the tokenizer rules.

use english_tokenizer::Tokenizer;

fn pairs(tk: &mut Tokenizer, input: &str) -> Vec<(String, String)> {
    tk.tokenize(input)
        .into_iter()
        .map(|t| (t.value, t.tag))
        .collect()
}

fn tok(value: &str, tag: &str) -> (String, String) {
    (value.to_string(), tag.to_string())
}

#[test]
fn time_alternation_cases() {
    let cases: &[(&str, &[(&str, &str)])] = &[
        // The hour alternation prefers a one digit hour, leaving 25 and the
        // colon to the number and punctuation rules, then 9pm as a time.
        (
            "25:99pm",
            &[
                ("25", "number"),
                (":", "punctuation"),
                ("9", "number"),
                ("9pm", "time"),
            ],
        ),
        ("16:00 hours", &[("16:00 hours", "time")]),
        ("1730 hrs", &[("1730 hrs", "time")]),
        // No am/pm/hours suffix, so this is not a time at all.
        (
            "23:59",
            &[("23", "number"), (":", "punctuation"), ("59", "number")],
        ),
        ("12:30am", &[("12:30am", "time")]),
        ("9 am", &[("9 am", "time")]),
    ];
    let mut tk = Tokenizer::new();
    for (input, want) in cases {
        let expected: Vec<(String, String)> = want.iter().map(|(v, t)| tok(v, t)).collect();
        assert_eq!(pairs(&mut tk, input), expected, "time case {input}");
    }
}

#[test]
fn number_formats_standalone() {
    let cases: &[(&str, &str)] = &[
        ("1/2", "number"),
        ("192.168.0.1", "number"),
        ("3,12.456-7", "number"),
        ("10000.00", "number"),
    ];
    let mut tk = Tokenizer::new();
    for (input, tag) in cases {
        assert_eq!(
            pairs(&mut tk, input),
            vec![tok(input, tag)],
            "number case {input}"
        );
    }
}

#[test]
fn possessive_uppercase_and_mixed() {
    let mut tk = Tokenizer::new();
    // The possessive rules are case insensitive, so uppercase splits too.
    assert_eq!(
        pairs(&mut tk, "DOG'S"),
        vec![tok("DOG", "word"), tok("'S", "word")]
    );
    assert_eq!(
        pairs(&mut tk, "CATS'"),
        vec![tok("CATS", "word"), tok("'", "word")]
    );
    // A trailing apostrophe on a stem that does not end in s stays whole.
    assert_eq!(pairs(&mut tk, "dog'"), vec![tok("dog'", "word")]);
}

#[test]
fn mixed_case_contraction_falls_through_whole() {
    let mut tk = Tokenizer::new();
    // cAn'T is not a table key, ends in 'T not 's, so neither possessive rule
    // fires. It stays as one word token.
    assert_eq!(pairs(&mut tk, "cAn'T"), vec![tok("cAn'T", "word")]);
}

#[test]
fn complex_emoji_clusters_stay_whole() {
    let cases: &[&str] = &[
        "\u{1F469}\u{200D}\u{1F467}", // woman ZWJ girl
        "\u{1F1FA}\u{1F1F8}",         // US flag, two regional indicators
        "1\u{FE0F}\u{20E3}",          // keycap 1
        "\u{1F44D}\u{1F3FD}",         // thumbs up with skin tone
    ];
    let mut tk = Tokenizer::new();
    for input in cases {
        assert_eq!(
            pairs(&mut tk, input),
            vec![tok(input, "emoji")],
            "emoji cluster {input:?}"
        );
    }
}

#[test]
fn unicode_whitespace_separates_tokens() {
    // Non breaking, thin, and ideographic spaces all act as separators.
    let mut tk = Tokenizer::new();
    for sep in ['\u{00A0}', '\u{2009}', '\u{3000}'] {
        let input = format!("a{sep}b");
        assert_eq!(
            pairs(&mut tk, &input),
            vec![tok("a", "word"), tok("b", "word")],
            "separator {sep:?}"
        );
    }
}

#[test]
fn leading_whitespace_yields_empty_alien_with_empty_config() {
    // With no active rules, the original untrimmed text is split on whitespace.
    // A leading space produces a leading empty value alien token.
    let mut tk = Tokenizer::new();
    tk.define_config(&[]);
    assert_eq!(
        pairs(&mut tk, "  a b"),
        vec![tok("", "alien"), tok("a", "alien"), tok("b", "alien"),]
    );
    // No leading space, so no empty token.
    assert_eq!(
        pairs(&mut tk, "a  b"),
        vec![tok("a", "alien"), tok("b", "alien")]
    );
}

#[test]
fn unterminated_and_empty_quoted_phrase() {
    // Quoted phrase on. An unterminated quote leaves the quote as punctuation
    // and tokenizes the rest. An empty pair of quotes is one quoted_phrase.
    let mut tk = Tokenizer::new();
    tk.define_config(&[("quoted_phrase", true)]);
    assert_eq!(
        pairs(&mut tk, "say \"hello world"),
        vec![
            tok("say", "word"),
            tok("\"", "punctuation"),
            tok("hello", "word"),
            tok("world", "word"),
        ]
    );
    assert_eq!(
        pairs(&mut tk, "a \"\" b"),
        vec![
            tok("a", "word"),
            tok("\"\"", "quoted_phrase"),
            tok("b", "word"),
        ]
    );
}
