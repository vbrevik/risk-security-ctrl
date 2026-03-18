use std::collections::{HashMap, HashSet};

const STOPWORDS: &[&str] = &[
    "the", "and", "or", "is", "in", "to", "of", "a", "an", "for", "with", "on", "at", "by",
    "from", "as", "it", "that", "this", "are", "was", "be", "has", "have", "had", "not", "but",
    "will", "can", "do", "if", "so", "no", "all", "they", "we", "you", "their", "its", "our",
    "your", "my", "which", "who", "what", "when", "where", "how", "each", "other", "than", "then",
    "also", "been", "would", "could", "should", "may", "must", "shall",
];

/// Split text into sentences on period/!/? + whitespace + capital, and on newlines.
pub fn sentence_split(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    let mut i = 0;
    while i < len {
        let c = chars[i];

        if c == '\n' {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                sentences.push(trimmed);
            }
            current.clear();
            i += 1;
            continue;
        }

        current.push(c);

        // Check for sentence boundary: .!? followed by whitespace + uppercase
        if (c == '.' || c == '!' || c == '?') && i + 2 < len {
            if chars[i + 1].is_whitespace() && chars[i + 2].is_uppercase() {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    sentences.push(trimmed);
                }
                current.clear();
                i += 1; // skip past the punctuation, whitespace will be consumed next iteration
                continue;
            }
        }

        i += 1;
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        sentences.push(trimmed);
    }

    sentences
}

/// Extract significant keywords from text (lowercase, no stopwords, min 3 chars, deduplicated).
pub fn extract_keywords(text: &str) -> Vec<String> {
    let stopwords: HashSet<&str> = STOPWORDS.iter().copied().collect();
    let lower = text.to_lowercase();
    let mut seen = HashSet::new();
    let mut keywords = Vec::new();

    for token in lower.split(|c: char| !c.is_alphanumeric()) {
        let token = token.trim();
        if token.len() < 3 {
            continue;
        }
        if stopwords.contains(token) {
            continue;
        }
        if seen.insert(token.to_string()) {
            keywords.push(token.to_string());
        }
    }

    keywords
}

/// Generate n-grams from a word list.
pub fn generate_ngrams(words: &[String], n: usize) -> Vec<String> {
    if words.len() < n || n == 0 {
        return Vec::new();
    }
    words
        .windows(n)
        .map(|window| window.join(" "))
        .collect()
}

/// Count term frequency (case-insensitive).
pub fn term_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.to_lowercase().split_whitespace() {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentence_split_on_period_capital() {
        let text = "First sentence. Second sentence. Third one.";
        let result = sentence_split(text);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "First sentence.");
        assert_eq!(result[1], "Second sentence.");
        assert_eq!(result[2], "Third one.");
    }

    #[test]
    fn sentence_split_on_newlines() {
        let text = "Line one\nLine two\nLine three";
        let result = sentence_split(text);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn extract_keywords_removes_stopwords() {
        let text = "the risk assessment is a process for identifying threats";
        let keywords = extract_keywords(text);
        assert!(!keywords.contains(&"the".to_string()));
        assert!(!keywords.contains(&"is".to_string()));
        assert!(keywords.contains(&"risk".to_string()));
        assert!(keywords.contains(&"assessment".to_string()));
        assert!(keywords.contains(&"threats".to_string()));
    }

    #[test]
    fn extract_keywords_filters_short_tokens() {
        let text = "an IT security do or be";
        let keywords = extract_keywords(text);
        assert!(!keywords.contains(&"an".to_string()));
        assert!(!keywords.contains(&"it".to_string()));
        assert!(keywords.contains(&"security".to_string()));
    }

    #[test]
    fn extract_keywords_deduplicates() {
        let text = "risk and risk assessment and risk";
        let keywords = extract_keywords(text);
        let risk_count = keywords.iter().filter(|k| k.as_str() == "risk").count();
        assert_eq!(risk_count, 1);
    }

    #[test]
    fn generate_ngrams_bigrams() {
        let words: Vec<String> = vec!["multi".into(), "factor".into(), "authentication".into()];
        let bigrams = generate_ngrams(&words, 2);
        assert_eq!(bigrams, vec!["multi factor", "factor authentication"]);
    }

    #[test]
    fn generate_ngrams_trigrams() {
        let words: Vec<String> = vec![
            "zero".into(),
            "trust".into(),
            "architecture".into(),
            "model".into(),
        ];
        let trigrams = generate_ngrams(&words, 3);
        assert_eq!(
            trigrams,
            vec!["zero trust architecture", "trust architecture model"]
        );
    }

    #[test]
    fn term_frequency_counts() {
        let text = "risk risk risk assessment assessment control";
        let freq = term_frequency(text);
        assert_eq!(freq.get("risk"), Some(&3));
        assert_eq!(freq.get("assessment"), Some(&2));
        assert_eq!(freq.get("control"), Some(&1));
    }

    #[test]
    fn term_frequency_case_insensitive() {
        let text = "Risk RISK risk";
        let freq = term_frequency(text);
        assert_eq!(freq.get("risk"), Some(&3));
    }
}
