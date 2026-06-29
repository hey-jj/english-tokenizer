//! Token patterns and their categories.
//!
//! Each pattern recognizes one kind of token. The order in [`master`] is load
//! bearing. Higher entries win, so emoticons are tried before time, time before
//! number, number before word, and so on.
//!
//! Several patterns restrict word characters to ASCII. The source rules use
//! JavaScript `\w` without the Unicode flag, which is `[A-Za-z0-9_]`. The
//! `(?-u:...)` groups below keep that ASCII meaning instead of the Unicode
//! default.

use crate::emoji::EMOJI_PATTERN;
use regex::Regex;

/// Whitespace run. Used to split alien text and to trim fragments.
pub const SPACES: &str = r"\s+";

/// Latin ordinals such as `1st`, `2nd`, `12th`, `33rd`. Digits are ASCII only,
/// so Devanagari digits route through [`NUMBER_DV`] instead.
pub const ORDINAL_L1: &str = r"(?-u:1\dth|[04-9]th|1st|2nd|3rd|[02-9]1st|[02-9]2nd|[02-9]3rd|[02-9][04-9]th|\d+\d[04-9]th|\d+\d1st|\d+\d2nd|\d+\d3rd)";

/// Latin numbers, including dates, ip addresses, and fractions via `. - / ,`.
/// Digits are ASCII only, so Devanagari digits route through [`NUMBER_DV`].
pub const NUMBER_L1: &str = r"(?-u:\d+/\d+|\d(?:[\.,\-/]?\d)*(?:\.\d+)?)";

/// Devanagari numbers using digits U+0966 to U+096F.
pub const NUMBER_DV: &str = r"[\x{0966}-\x{096F}]+/[\x{0966}-\x{096F}]+|[\x{0966}-\x{096F}](?:[\.,\-/]?[\x{0966}-\x{096F}])*(?:\.[\x{0966}-\x{096F}]+)?";

/// At mention. Word characters are ASCII only.
pub const MENTION: &str = r"@(?-u:\w)+";

/// Latin hashtag covering the whole Latin-1 letter range plus underscore.
pub const HASHTAG_L1: &str = r"(?i)#[a-z\x{00C0}-\x{00D6}\x{00D8}-\x{00F6}\x{00F8}-\x{00FF}_][a-z0-9\x{00C0}-\x{00D6}\x{00D8}-\x{00F6}\x{00F8}-\x{00FF}_]*";

/// Devanagari hashtag.
pub const HASHTAG_DV: &str = r"(?i)#[\x{0900}-\x{0963}\x{0970}-\x{097F}_][\x{0900}-\x{0963}\x{0970}-\x{097F}\x{0966}-\x{096F}0-9_]*";

/// Email address. A loose approximation, not an RFC 5322 validator. Local part
/// allows the common special characters plus ASCII word characters. JavaScript
/// `\w` inside a class is `[A-Za-z0-9_]`, spelled out here.
pub const EMAIL: &str = r"(?i)[\-!#$%&'*+/=?^A-Za-z0-9_{|}~](?:\.?[\-!#$%&'*+/=?^A-Za-z0-9_`{|}~])*@[a-z0-9](?:-?\.?[a-z0-9])*(?:\.[a-z](?:-?[a-z0-9])*)+";

/// Currency symbols: bitcoin, ruble, rupees, dollar, pound, yen, euro, won.
pub const CURRENCY: &str = r"[\x{20BF}\x{20BD}\x{20B9}\x{20A8}$\x{00A3}\x{00A5}\x{20AC}\x{20A9}]";

/// Punctuation, Latin-1 and Devanagari danda and double danda.
pub const PUNCTUATION: &str =
    "['\u{2019}\u{2018}\u{2019}`\u{201C}\u{201D}\"\\[\\](){}\u{2026},.!;?\\-:\u{0964}\u{0965}]";

/// A double quoted span kept whole.
pub const QUOTED_PHRASE: &str = r#""[^"]*""#;

/// URL with an explicit http or https scheme. Path characters are ASCII word
/// characters plus a few symbols. JavaScript `\w` inside the class is
/// `[A-Za-z0-9_]`, spelled out here.
pub const URL: &str =
    r"(?i)(?:https?://)(?:[\da-z\.\-]+)\.(?:[a-z\.]{2,6})(?:[/A-Za-z0-9_\.\-?#=]*)*/?";

/// Common emoticons. The character class after `;` matches the source exactly,
/// including the duplicate slash.
pub const EMOTICON: &str = r"(?i):-?[dps*/\[\]{}()]|;-?[/()d]|<3";

/// Times such as `4pm`, `3pm`, `16:00 hours`. The hour and minute digits are
/// ASCII only, so a Devanagari digit does not start a time match.
pub const TIME: &str =
    r"(?i)(?:(?-u:\d)|[01](?-u:\d)|2[0-3]):?(?:[0-5][0-9])?\s?(?:[ap]\.?m\.?|hours|hrs)";

/// Latin word block. Allows an internal or trailing ASCII apostrophe so
/// contractions stay whole for later splitting.
pub const WORD_L1: &str = r"(?i)[a-z\x{00C0}-\x{00D6}\x{00D8}-\x{00F6}\x{00F8}-\x{00FF}][a-z\x{00C0}-\x{00D6}\x{00D8}-\x{00F6}\x{00F8}-\x{00FF}']*";

/// Devanagari word block, including vedic accent marks so accented words stay
/// whole. Excludes Om at U+0950 and the digit range.
pub const WORD_DV: &str = r"(?i)[\x{0900}-\x{094F}\x{0951}-\x{0963}\x{0970}-\x{097F}]+";

/// Symbols, including Om at U+0950.
pub const SYMBOL: &str = r"[\x{0950}~@#%\^\+=\*\|/<>&]";

/// Singular possessive, for example `dog's` to `dog` and `'s`. End anchored
/// only. The match floats to the rightmost ASCII letter run before `'s`.
pub const POS_SINGULAR: &str = r"(?i)([a-z]+)('s)$";

/// Plural possessive, for example `cats'` to `cats` and `'`. End anchored only.
pub const POS_PLURAL: &str = r"(?i)([a-z]+s)(')$";

/// The token category names. These are the exact tag strings.
pub const QUOTED_PHRASE_CAT: &str = "quoted_phrase";
/// URL category.
pub const URL_CAT: &str = "url";
/// Email category.
pub const EMAIL_CAT: &str = "email";
/// Mention category.
pub const MENTION_CAT: &str = "mention";
/// Hashtag category.
pub const HASHTAG_CAT: &str = "hashtag";
/// Emoji category.
pub const EMOJI_CAT: &str = "emoji";
/// Emoticon category.
pub const EMOTICON_CAT: &str = "emoticon";
/// Time category.
pub const TIME_CAT: &str = "time";
/// Ordinal category.
pub const ORDINAL_CAT: &str = "ordinal";
/// Number category.
pub const NUMBER_CAT: &str = "number";
/// Currency category.
pub const CURRENCY_CAT: &str = "currency";
/// Word category.
pub const WORD_CAT: &str = "word";
/// Punctuation category.
pub const PUNCTUATION_CAT: &str = "punctuation";
/// Symbol category.
pub const SYMBOL_CAT: &str = "symbol";

/// One compiled pattern paired with the category it tags.
#[derive(Clone)]
pub struct Rule {
    /// Compiled matcher for this token kind.
    pub regex: Regex,
    /// Tag applied to matches.
    pub category: String,
}

/// Builds the ordered list of built-in rules. The order matches the source and
/// is essential for correct tokenization.
pub fn master() -> Vec<Rule> {
    let specs: [(&str, &str); 17] = [
        (QUOTED_PHRASE, QUOTED_PHRASE_CAT),
        (URL, URL_CAT),
        (EMAIL, EMAIL_CAT),
        (MENTION, MENTION_CAT),
        (HASHTAG_L1, HASHTAG_CAT),
        (HASHTAG_DV, HASHTAG_CAT),
        (EMOJI_PATTERN, EMOJI_CAT),
        (EMOTICON, EMOTICON_CAT),
        (TIME, TIME_CAT),
        (ORDINAL_L1, ORDINAL_CAT),
        (NUMBER_L1, NUMBER_CAT),
        (NUMBER_DV, NUMBER_CAT),
        (CURRENCY, CURRENCY_CAT),
        (WORD_L1, WORD_CAT),
        (WORD_DV, WORD_CAT),
        (PUNCTUATION, PUNCTUATION_CAT),
        (SYMBOL, SYMBOL_CAT),
    ];
    specs
        .iter()
        .map(|(pat, cat)| Rule {
            regex: Regex::new(pat).expect("built-in pattern compiles"),
            category: (*cat).to_string(),
        })
        .collect()
}
