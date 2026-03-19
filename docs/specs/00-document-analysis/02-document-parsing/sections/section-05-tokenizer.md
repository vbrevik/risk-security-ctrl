# Section 5: Tokenizer

## Overview

This section implements `backend/src/features/analysis/tokenizer.rs`, a module of pure utility functions for text processing. These functions are consumed by the downstream matching engine (split 03) for FTS5 queries and TF-IDF scoring. The tokenizer has no dependencies on any other section in this split -- it can be implemented in parallel with section 01 (deps and types).

All functions are pure, synchronous, and require no database access or async runtime.

## File to Create

- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/tokenizer.rs`

## Dependencies

- None from this split. The tokenizer is self-contained.
- Uses only the Rust standard library (`std::collections::HashMap`, `std::collections::HashSet`).
- No external crates required.

## Tests First

All tests go in a `#[cfg(test)] mod tests` block at the bottom of `tokenizer.rs`. Run with `cargo test` from `backend/`.

### Test: sentence_split breaks on period + capital

Verify that `sentence_split` splits text at sentence boundaries where a period (or `!` or `?`) is followed by whitespace and a capital letter.

```rust
#[test]
fn sentence_split_on_period_capital() {
    let text = "First sentence. Second sentence. Third one.";
    let result = sentence_split(text);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "First sentence.");
    assert_eq!(result[1], "Second sentence.");
    assert_eq!(result[2], "Third one.");
}
```

### Test: sentence_split handles newlines

Newlines should also act as sentence boundaries.

```rust
#[test]
fn sentence_split_on_newlines() {
    let text = "Line one\nLine two\nLine three";
    let result = sentence_split(text);
    assert_eq!(result.len(), 3);
}
```

### Test: extract_keywords removes stopwords

Common English stopwords must be filtered out.

```rust
#[test]
fn extract_keywords_removes_stopwords() {
    let text = "the risk assessment is a process for identifying threats";
    let keywords = extract_keywords(text);
    assert!(!keywords.contains(&"the".to_string()));
    assert!(!keywords.contains(&"is".to_string()));
    assert!(!keywords.contains(&"a".to_string()));
    assert!(!keywords.contains(&"for".to_string()));
    assert!(keywords.contains(&"risk".to_string()));
    assert!(keywords.contains(&"assessment".to_string()));
    assert!(keywords.contains(&"process".to_string()));
    assert!(keywords.contains(&"identifying".to_string()));
    assert!(keywords.contains(&"threats".to_string()));
}
```

### Test: extract_keywords filters short tokens

Tokens shorter than 3 characters must be removed.

```rust
#[test]
fn extract_keywords_filters_short_tokens() {
    let text = "an IT security do or be";
    let keywords = extract_keywords(text);
    // "an", "IT" (becomes "it", len 2), "do", "or", "be" are all < 3 chars or stopwords
    assert!(!keywords.contains(&"an".to_string()));
    assert!(!keywords.contains(&"it".to_string()));
    assert!(!keywords.contains(&"do".to_string()));
    assert!(keywords.contains(&"security".to_string()));
}
```

### Test: extract_keywords deduplicates

Duplicate words should appear only once, preserving first-occurrence order.

```rust
#[test]
fn extract_keywords_deduplicates() {
    let text = "risk and risk assessment and risk";
    let keywords = extract_keywords(text);
    let risk_count = keywords.iter().filter(|k| k.as_str() == "risk").count();
    assert_eq!(risk_count, 1);
}
```

### Test: generate_ngrams produces correct bigrams

```rust
#[test]
fn generate_ngrams_bigrams() {
    let words: Vec<String> = vec!["multi".into(), "factor".into(), "authentication".into()];
    let bigrams = generate_ngrams(&words, 2);
    assert_eq!(bigrams, vec!["multi factor", "factor authentication"]);
}
```

### Test: generate_ngrams with n=3 produces trigrams

```rust
#[test]
fn generate_ngrams_trigrams() {
    let words: Vec<String> = vec!["zero".into(), "trust".into(), "architecture".into(), "model".into()];
    let trigrams = generate_ngrams(&words, 3);
    assert_eq!(trigrams, vec!["zero trust architecture", "trust architecture model"]);
}
```

### Test: term_frequency counts correctly

```rust
#[test]
fn term_frequency_counts() {
    let text = "risk risk risk assessment assessment control";
    let freq = term_frequency(text);
    assert_eq!(freq.get("risk"), Some(&3));
    assert_eq!(freq.get("assessment"), Some(&2));
    assert_eq!(freq.get("control"), Some(&1));
}
```

### Test: term_frequency is case-insensitive

```rust
#[test]
fn term_frequency_case_insensitive() {
    let text = "Risk RISK risk";
    let freq = term_frequency(text);
    assert_eq!(freq.get("risk"), Some(&3));
}
```

## Implementation Details

### Function: `sentence_split`

**Signature:** `pub fn sentence_split(text: &str) -> Vec<String>`

Split text on sentence boundaries. A sentence boundary is defined as:
- A period (`.`), exclamation mark (`!`), or question mark (`?`) followed by one or more whitespace characters and then an uppercase letter
- A newline character (`\n`)

Filter out any empty strings after splitting. Trim each resulting sentence.

This is a simple heuristic approach -- no NLP library or regex crate is required. A manual character-walking approach works fine:
- Iterate through characters, tracking the current sentence
- When you encounter `.`/`!`/`?` followed by whitespace + uppercase letter, end the current sentence and start a new one
- When you encounter `\n`, end the current sentence and start a new one
- At end of input, flush any remaining sentence

### Function: `extract_keywords`

**Signature:** `pub fn extract_keywords(text: &str) -> Vec<String>`

Steps:
1. Lowercase the entire text
2. Split on non-alphanumeric characters (any char where `!c.is_alphanumeric()`)
3. Filter out tokens that appear in the stopword set
4. Filter out tokens shorter than 3 characters
5. Deduplicate while preserving first-occurrence order (use a `HashSet` for seen-tracking and a `Vec` for ordered output)

**Stopword list** (hardcoded as a `const` array or lazy `HashSet`):

```
"the", "and", "or", "is", "in", "to", "of", "a", "an", "for", "with", "on", "at",
"by", "from", "as", "it", "that", "this", "are", "was", "be", "has", "have", "had",
"not", "but", "will", "can", "do", "if", "so", "no", "all", "they", "we", "you",
"their", "its", "our", "your", "my", "which", "who", "what", "when", "where", "how",
"each", "other", "than", "then", "also", "been", "would", "could", "should", "may",
"must", "shall"
```

### Function: `generate_ngrams`

**Signature:** `pub fn generate_ngrams(words: &[String], n: usize) -> Vec<String>`

Generate n-grams from a word list by sliding a window of size `n` across the words and joining each window with a space.

For `n=2` and `["multi", "factor", "auth"]`, the result is `["multi factor", "factor auth"]`.

If `words.len() < n`, return an empty `Vec`.

### Function: `term_frequency`

**Signature:** `pub fn term_frequency(text: &str) -> HashMap<String, usize>`

Count occurrences of each word after lowercasing and splitting on whitespace. Every word is counted (no stopword filtering here -- that is the caller's responsibility if needed).

Steps:
1. Lowercase the text
2. Split on whitespace
3. For each token, increment its count in a `HashMap<String, usize>`
4. Return the map

## Notes for Implementers

- All four functions are `pub` so the matching engine (split 03) can import them.
- The module will be wired into the crate in section 06 by adding `pub mod tokenizer;` to `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/mod.rs`. Until then, tests still run because `#[cfg(test)]` modules can access private items.
- No `use` of types from `parser.rs` is needed -- the tokenizer is fully independent.
- The stopword list is intentionally English-only for MVP. Norwegian stopword support is a future enhancement.