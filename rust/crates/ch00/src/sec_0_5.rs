// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.5: Collections and iterators.

use std::collections::HashMap;

/// The length of each word in `text`, split on whitespace, in order. The
/// `split_whitespace` and `map` adapters build nothing until `collect`
/// runs them: they describe the walk before it happens.
#[must_use]
pub fn word_lengths(text: &str) -> Vec<usize> {
    text.split_whitespace().map(str::len).collect()
}

/// How many times each word in `words` occurs.
#[must_use]
pub fn word_counts<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut counts = HashMap::new();
    for &word in words {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}
