# english-tokenizer

Regex-driven tokenizer that splits a sentence and tags each token with its type.

Give it a string and it returns an ordered list of tokens. Each token has a
`value` and a `tag`. Tags name the type: `word`, `number`, `ordinal`, `email`,
`url`, `mention`, `hashtag`, `emoji`, `emoticon`, `time`, `currency`,
`quoted_phrase`, `punctuation`, `symbol`, plus `alien` for text no rule matched.

The tokenizer handles Latin-1 and Devanagari scripts. It splits common English
contractions and possessives into separate word tokens. `I'll` becomes `I` and
`'ll`. `dog's` becomes `dog` and `'s`.

## Installation

```toml
[dependencies]
english-tokenizer = "0.1"
```

## Usage

```rust
use english_tokenizer::Tokenizer;

let mut tk = Tokenizer::new();
let tokens = tk.tokenize("feeling good #FunTime");
assert_eq!(tokens[2].value, "#FunTime");
assert_eq!(tokens[2].tag, "hashtag");
```

### Configuration

`Tokenizer::new` enables every type except `quoted_phrase`. Turn types on or off
with `define_config`. It returns the count of unique active categories.

```rust
use english_tokenizer::Tokenizer;

let mut tk = Tokenizer::new();
tk.define_config(&[("hashtag", false)]); // returns 13
```

An empty config splits on spaces and tags everything `alien`.

The `tags` module holds the built-in tag names as constants. Use `tags::HASHTAG`
in place of the `"hashtag"` literal so a typo fails to compile.

### Custom rules

`add_regex` injects a rule that wins over the built-ins. `add_tag` registers a
new tag with a fingerprint code. A later `define_config` call removes added
rules.

```rust
use english_tokenizer::Tokenizer;
use regex::Regex;

let mut tk = Tokenizer::new();
tk.add_regex(Regex::new(r"(?i)\(oo\)").unwrap(), "emoticon", None).unwrap();
let tokens = tk.tokenize("(oo) Hi!");
assert_eq!(tokens[0].tag, "emoticon");
```

### Fingerprint

`get_tokens_fp` returns a one character per token fingerprint of the last
tokenize call. Each token contributes its tag code, or its literal value when the
tag has no code. Punctuation and symbol have no code, so they pass through.

```rust
use english_tokenizer::Tokenizer;

let mut tk = Tokenizer::new();
tk.tokenize("feeling good #FunTime");
assert_eq!(tk.get_tokens_fp(), "wwh");
```

## License

Licensed under the [MIT license](LICENSE).
