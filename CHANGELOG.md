# Changelog

## [0.2.0] - 2026-07-07

### Changed
- `add_tag` now rejects an empty fingerprint code with a missing fingerprint error instead of adding a tag with an unusable code (#12).
- `add_regex` now accepts built-in token categories without a fingerprint code, so custom punctuation and symbol rules can tag matches instead of returning a missing fingerprint error (#13).
- Lowercase `couldn't` now splits into `could` and `n't`, so token values and token counts change for that input (#14).
- Long teen ordinals such as `111th`, `211th`, and `1011th` now stay one `ordinal` token instead of splitting apart (#15).

### Fixed
- Empty-rule tokenization keeps the same alien token values after whitespace splitting (#16).

## [0.2.0] - 2026-07-07

### Changed
- `add_tag` now rejects an empty fingerprint code with a missing fingerprint error instead of adding a tag with an unusable code (#12).
- `add_regex` now accepts built-in token categories without a fingerprint code, so custom punctuation and symbol rules can tag matches instead of returning a missing fingerprint error (#13).
- Lowercase `couldn't` now splits into `could` and `n't`, so token values and token counts change for that input (#14).
- Long teen ordinals such as `111th`, `211th`, and `1011th` now stay one `ordinal` token instead of splitting apart (#15).

### Fixed
- Empty-rule tokenization keeps the same alien token values after whitespace splitting (#16).
