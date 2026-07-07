//! Contraction table guards. The table holds a fixed number of keys and splits
//! both two part and three part forms.

use english_tokenizer::{Tokenizer, CONTRACTION_COUNT};

fn pairs(tk: &mut Tokenizer, input: &str) -> Vec<(String, String)> {
    tk.tokenize(input)
        .into_iter()
        .map(|t| (t.value, t.tag))
        .collect()
}

#[test]
fn table_size() {
    assert_eq!(CONTRACTION_COUNT, 297);
}

#[test]
fn two_part_split() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "won't"),
        vec![
            ("wo".to_string(), "word".to_string()),
            ("n't".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn lowercase_couldnt_splits() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "couldn't"),
        vec![
            ("could".to_string(), "word".to_string()),
            ("n't".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn three_part_split() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "I'd've"),
        vec![
            ("I".to_string(), "word".to_string()),
            ("'d".to_string(), "word".to_string()),
            ("'ve".to_string(), "word".to_string()),
        ]
    );
}

#[test]
fn n_t_have_family() {
    let mut tk = Tokenizer::new();
    assert_eq!(
        pairs(&mut tk, "shouldn't've"),
        vec![
            ("should".to_string(), "word".to_string()),
            ("n't".to_string(), "word".to_string()),
            ("'ve".to_string(), "word".to_string()),
        ]
    );
}
