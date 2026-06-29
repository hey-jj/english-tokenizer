//! Regex-driven tokenizer that tags each token with its type.
//!
//! Give it a sentence and it returns an ordered list of tokens. Each token has a
//! `value` and a `tag`. Tags name the token type: `word`, `number`, `ordinal`,
//! `email`, `url`, `mention`, `hashtag`, `emoji`, `emoticon`, `time`, `currency`,
//! `quoted_phrase`, `punctuation`, `symbol`, plus `alien` for text that no active
//! rule matched.
//!
//! The tokenizer covers Latin-1 and Devanagari scripts. It splits common English
//! contractions and possessives into separate word tokens. For example `I'll`
//! becomes `I` and `'ll`, and `dog's` becomes `dog` and `'s`.
//!
//! # Example
//!
//! ```
//! use english_tokenizer::Tokenizer;
//!
//! let mut tk = Tokenizer::new();
//! let tokens = tk.tokenize("feeling good #FunTime");
//! assert_eq!(tokens.len(), 3);
//! assert_eq!(tokens[2].value, "#FunTime");
//! assert_eq!(tokens[2].tag, "hashtag");
//! ```
//!
//! # Configuration
//!
//! [`Tokenizer::new`] enables every token type except `quoted_phrase`. Use
//! [`Tokenizer::define_config`] to turn types on or off. An empty config splits
//! on spaces and tags everything `alien`.
//!
//! [`Tokenizer::add_regex`] injects a custom rule that wins over the built-ins.
//! [`Tokenizer::add_tag`] registers a new tag with a fingerprint code.
//! [`Tokenizer::get_tokens_fp`] returns a one character per token fingerprint of
//! the last tokenize call.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod contractions;
mod emoji;
mod patterns;

use regex::Regex;
use std::collections::HashMap;

use patterns::Rule;

/// A single token: its text and its type tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The matched text, exactly as it appeared in the input.
    pub value: String,
    /// The token type, for example `word`, `number`, or `punctuation`.
    pub tag: String,
}

impl Token {
    fn new(value: impl Into<String>, tag: impl Into<String>) -> Self {
        Token {
            value: value.into(),
            tag: tag.into(),
        }
    }
}

/// Error returned when a tag or regex cannot be added.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddError {
    /// A new tag was needed but no fingerprint code was given.
    ///
    /// The string is the full message, matching the wording callers expect.
    MissingFingerprint(String),
    /// The tag already exists.
    ///
    /// The string is the full message.
    TagExists(String),
}

impl std::fmt::Display for AddError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddError::MissingFingerprint(m) | AddError::TagExists(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for AddError {}

/// The default fingerprint codes. `punctuation` and `symbol` have no code on
/// purpose, so their tokens contribute their literal value to a fingerprint.
fn default_fingerprint_codes() -> HashMap<String, String> {
    let pairs = [
        ("emoticon", "c"),
        ("email", "e"),
        ("emoji", "j"),
        ("hashtag", "h"),
        ("mention", "m"),
        ("number", "n"),
        ("ordinal", "o"),
        ("quoted_phrase", "q"),
        ("currency", "r"),
        ("time", "t"),
        ("url", "u"),
        ("word", "w"),
        ("alien", "z"),
    ];
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

/// One token result of [`tokenizeTextUnit`]: either a finished token or a raw
/// fragment that needs more tokenizing.
enum Unit {
    /// A finished token.
    Token(Token),
    /// A text fragment to recurse on.
    Fragment(String),
}

/// A regex tokenizer instance.
///
/// Each instance holds its own rule list, fingerprint codes, and the result of
/// the last tokenize call. Create one with [`Tokenizer::new`].
///
/// The source library keeps fingerprint codes in a process wide variable shared
/// across instances. This type keeps them per instance instead. Most code uses a
/// single instance, so the behavior matches. The difference shows only when two
/// instances run in the same process and one adds a tag the other reads before
/// any reset.
pub struct Tokenizer {
    master: Vec<Rule>,
    rgxs: Vec<Rule>,
    final_tokens: Vec<Token>,
    fingerprint_codes: HashMap<String, String>,
    contraction: Regex,
    pos_singular: Regex,
    pos_plural: Regex,
    spaces: Regex,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    /// Creates a tokenizer with every token type enabled except `quoted_phrase`.
    ///
    /// # Example
    ///
    /// ```
    /// use english_tokenizer::Tokenizer;
    /// let mut tk = Tokenizer::new();
    /// assert!(tk.tokenize("").is_empty());
    /// ```
    pub fn new() -> Self {
        let master = patterns::master();
        let mut tk = Tokenizer {
            rgxs: master.clone(),
            master,
            final_tokens: Vec::new(),
            fingerprint_codes: default_fingerprint_codes(),
            contraction: Regex::new(r"'").expect("contraction pattern compiles"),
            pos_singular: Regex::new(patterns::POS_SINGULAR).expect("possessive pattern compiles"),
            pos_plural: Regex::new(patterns::POS_PLURAL).expect("possessive pattern compiles"),
            spaces: Regex::new(patterns::SPACES).expect("spaces pattern compiles"),
        };
        // Quoted phrase is off by default since it is rarely needed.
        tk.define_config(&[("quoted_phrase", false)]);
        tk
    }

    /// Tokenizes `sentence` and returns the tokens in left to right order.
    ///
    /// The result is also kept for the next [`get_tokens_fp`](Self::get_tokens_fp)
    /// call.
    ///
    /// # Example
    ///
    /// ```
    /// use english_tokenizer::Tokenizer;
    /// let mut tk = Tokenizer::new();
    /// let tokens = tk.tokenize("4pm");
    /// assert_eq!(tokens[0].tag, "time");
    /// ```
    pub fn tokenize(&mut self, sentence: &str) -> Vec<Token> {
        self.final_tokens = Vec::new();
        let rules = self.rgxs.clone();
        self.tokenize_recursive(sentence, &rules);
        self.final_tokens.clone()
    }

    /// Sets which token types are active and returns the count of unique active
    /// categories.
    ///
    /// Each pair is a category name and a flag. A `true` flag enables the
    /// category. A `false` flag disables it. Categories not listed stay enabled.
    /// An empty list disables everything, so tokenize splits on spaces and tags
    /// every piece `alien`.
    ///
    /// This also resets any rules added with [`add_regex`](Self::add_regex) and
    /// resets the fingerprint codes.
    ///
    /// Categories that use more than one rule, like `hashtag` and `number`, count
    /// once. The full default returns 13 because `quoted_phrase` is off. Enabling
    /// `quoted_phrase` returns 14.
    ///
    /// # Example
    ///
    /// ```
    /// use english_tokenizer::Tokenizer;
    /// let mut tk = Tokenizer::new();
    /// assert_eq!(tk.define_config(&[("hashtag", false)]), 13);
    /// assert_eq!(tk.define_config(&[]), 0);
    /// assert_eq!(tk.define_config(&[("quoted_phrase", true)]), 14);
    /// ```
    pub fn define_config(&mut self, config: &[(&str, bool)]) -> usize {
        if config.is_empty() {
            self.rgxs = Vec::new();
        } else {
            let map: HashMap<&str, bool> = config.iter().copied().collect();
            self.rgxs = self
                .master
                .iter()
                .filter(|rule| match map.get(rule.category.as_str()) {
                    // Listed categories follow their flag. Absent ones stay on.
                    Some(flag) => *flag,
                    None => true,
                })
                .cloned()
                .collect();
        }
        let mut unique: Vec<&str> = Vec::new();
        for rule in &self.rgxs {
            if !unique.contains(&rule.category.as_str()) {
                unique.push(rule.category.as_str());
            }
        }
        self.fingerprint_codes = default_fingerprint_codes();
        unique.len()
    }

    /// Returns the fingerprint of the tokens from the last
    /// [`tokenize`](Self::tokenize) call.
    ///
    /// For each token, if its tag has a fingerprint code, that single character
    /// is appended. Otherwise the token's literal value is appended. Tokens
    /// tagged `punctuation` or `symbol` have no code, so their value passes
    /// through. Before any tokenize call this returns an empty string.
    ///
    /// # Example
    ///
    /// ```
    /// use english_tokenizer::Tokenizer;
    /// let mut tk = Tokenizer::new();
    /// tk.tokenize("feeling good #FunTime");
    /// assert_eq!(tk.get_tokens_fp(), "wwh");
    /// ```
    pub fn get_tokens_fp(&self) -> String {
        let mut fp = String::new();
        for token in &self.final_tokens {
            match self.fingerprint_codes.get(&token.tag) {
                Some(code) if !code.is_empty() => fp.push_str(code),
                _ => fp.push_str(&token.value),
            }
        }
        fp
    }

    /// Adds a regex rule that wins over the built-ins.
    ///
    /// The rule is tried before every built-in rule. If `tag` is already known,
    /// `fingerprint_code` is ignored. If `tag` is new, `fingerprint_code` must be
    /// provided so the tag has a fingerprint. A later
    /// [`define_config`](Self::define_config) call removes added rules.
    ///
    /// # Errors
    ///
    /// Returns [`AddError::MissingFingerprint`] when `tag` is new and no
    /// fingerprint code is given.
    ///
    /// # Example
    ///
    /// ```
    /// use english_tokenizer::Tokenizer;
    /// use regex::Regex;
    /// let mut tk = Tokenizer::new();
    /// tk.add_regex(Regex::new(r"(?i)\(oo\)").unwrap(), "emoticon", None).unwrap();
    /// let tokens = tk.tokenize("(oo) Hi!");
    /// assert_eq!(tokens[0].tag, "emoticon");
    /// ```
    pub fn add_regex(
        &mut self,
        regex: Regex,
        tag: &str,
        fingerprint_code: Option<&str>,
    ) -> Result<(), AddError> {
        let known = self.has_code(tag);
        if !known && fingerprint_code.is_none() {
            return Err(AddError::MissingFingerprint(format!(
                "Tag {tag} doesn't exist; Provide a 'fingerprintCode' to add it as a tag."
            )));
        }
        if !known {
            // Safe to unwrap: the guard above ensures a code is present.
            self.add_tag(tag, fingerprint_code.expect("checked above"))?;
        }
        self.rgxs.insert(
            0,
            Rule {
                regex,
                category: tag.to_string(),
            },
        );
        Ok(())
    }

    /// Registers a new tag with a fingerprint code.
    ///
    /// # Errors
    ///
    /// Returns [`AddError::TagExists`] when `name` already has a code.
    pub fn add_tag(&mut self, name: &str, fingerprint_code: &str) -> Result<(), AddError> {
        if self.has_code(name) {
            return Err(AddError::TagExists(format!("Tag {name} already exists")));
        }
        self.fingerprint_codes
            .insert(name.to_string(), fingerprint_code.to_string());
        Ok(())
    }

    /// Whether `tag` has a non-empty fingerprint code. The source uses a truthy
    /// check, so an empty code counts as absent.
    fn has_code(&self, tag: &str) -> bool {
        self.fingerprint_codes
            .get(tag)
            .is_some_and(|c| !c.is_empty())
    }

    /// Recursively tokenizes `text`. Each level applies the first rule, emits its
    /// matches as final tokens, and recurses on the gaps with the rest of the
    /// rules. With no rules left, splits on whitespace and tags every piece
    /// `alien`.
    fn tokenize_recursive(&mut self, text: &str, rules: &[Rule]) {
        if rules.is_empty() {
            for piece in self.spaces.split(text) {
                self.final_tokens
                    .push(Token::new(piece.trim().to_string(), "alien"));
            }
            return;
        }
        let sentence = text.trim();
        let units = self.tokenize_unit(sentence, &rules[0]);
        for unit in units {
            match unit {
                Unit::Fragment(frag) => self.tokenize_recursive(&frag, &rules[1..]),
                Unit::Token(token) => self.final_tokens.push(token),
            }
        }
    }

    /// Applies one rule to `text`. Matches become tokens. The gaps between them
    /// become fragments to recurse on. Word matches that contain an apostrophe go
    /// through contraction handling.
    fn tokenize_unit(&self, text: &str, rule: &Rule) -> Vec<Unit> {
        let mut units = Vec::new();
        let is_word = rule.category == "word";
        let mut last = 0;
        for m in rule.regex.find_iter(text) {
            let gap = &text[last..m.start()];
            let trimmed = gap.trim();
            if !trimmed.is_empty() {
                units.push(Unit::Fragment(trimmed.to_string()));
            }
            let matched = m.as_str();
            if is_word && self.contraction.is_match(matched) {
                self.manage_contraction(matched, &mut units);
            } else {
                units.push(Unit::Token(Token::new(matched, rule.category.clone())));
            }
            last = m.end();
        }
        let tail = text[last..].trim();
        if !tail.is_empty() {
            units.push(Unit::Fragment(tail.to_string()));
        }
        units
    }

    /// Splits a word that contains an apostrophe. Tries the contraction table
    /// first, then singular possessive, then plural possessive. Otherwise keeps
    /// the word whole.
    fn manage_contraction(&self, word: &str, units: &mut Vec<Unit>) {
        if let Some(parts) = contractions::lookup(word) {
            for part in parts {
                units.push(Unit::Token(Token::new(*part, "word")));
            }
            return;
        }
        if let Some(caps) = self.pos_singular.captures(word) {
            units.push(Unit::Token(Token::new(&caps[1], "word")));
            units.push(Unit::Token(Token::new(&caps[2], "word")));
            return;
        }
        if let Some(caps) = self.pos_plural.captures(word) {
            units.push(Unit::Token(Token::new(&caps[1], "word")));
            units.push(Unit::Token(Token::new(&caps[2], "word")));
            return;
        }
        units.push(Unit::Token(Token::new(word, "word")));
    }
}

/// Number of contraction keys in the built-in table.
///
/// Exposed so callers can confirm the table is complete.
pub const CONTRACTION_COUNT: usize = contractions::COUNT;
