//! Shared helpers for the golden tests.
//!
//! Loads the recorded case data so several test files can read the same inputs
//! and expected outputs.
//!
//! Each test binary compiles this module on its own, so some items look unused
//! from any single binary. The allow below keeps those warnings quiet.
#![allow(dead_code)]

use serde::Deserialize;

/// One recorded step from the spec sequence.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Step {
    /// A tokenize call with its expected tokens.
    #[serde(rename = "tokenize")]
    Tokenize {
        /// Human readable label.
        label: String,
        /// Input sentence.
        input: String,
        /// Expected value and tag pairs.
        tokens: Vec<(String, String)>,
    },
    /// A fingerprint read with its expected string.
    #[serde(rename = "fp")]
    Fp {
        /// Human readable label.
        label: String,
        /// Expected fingerprint.
        fp: String,
    },
    /// A define_config call with its expected unique category count.
    #[serde(rename = "config")]
    Config {
        /// Human readable label.
        label: String,
        /// Expected count.
        count: usize,
    },
}

impl Step {
    /// The label that identifies this step.
    pub fn label(&self) -> &str {
        match self {
            Step::Tokenize { label, .. } => label,
            Step::Fp { label, .. } => label,
            Step::Config { label, .. } => label,
        }
    }
}

/// Loads the recorded steps from the data file.
pub fn load_steps() -> Vec<Step> {
    let raw = include_str!("../data/golden_cases.json");
    serde_json::from_str(raw).expect("golden_cases.json parses")
}

/// Finds a tokenize step by label and returns its input and expected tokens.
pub fn tokenize_case(label: &str) -> (String, Vec<(String, String)>) {
    for step in load_steps() {
        if let Step::Tokenize {
            label: l,
            input,
            tokens,
        } = step
        {
            if l == label {
                return (input, tokens);
            }
        }
    }
    panic!("no tokenize case labeled {label}");
}
