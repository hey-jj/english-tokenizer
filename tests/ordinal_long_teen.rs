//! Long teen ordinal cases.

use english_tokenizer::Tokenizer;

fn pairs(tk: &mut Tokenizer, input: &str) -> Vec<(String, String)> {
    tk.tokenize(input)
        .into_iter()
        .map(|t| (t.value, t.tag))
        .collect()
}

#[test]
fn long_teen_ordinals_stay_whole() {
    let mut tk = Tokenizer::new();
    for value in ["111th", "112th", "113th", "211th", "911th", "1011th"] {
        assert_eq!(
            pairs(&mut tk, value),
            vec![(value.to_string(), "ordinal".to_string())],
            "{value}"
        );
    }
}
