---
title: "Blind 75 in Rust: Valid Anagram"
date: "2026-06-24"
slug: "blind75-valid-anagram"
excerpt: "Second problem in the Blind 75 series: Valid Anagram. One HashMap, a single pass each way, and a few words on why iterating over UTF-8 isn't quite ASCII."
tags: ["rust", "leetcode", "blind75", "algorithms"]
read_time: 3
---

Second problem in the [Blind 75](https://www.techinterviewhandbook.org/best-practice-questions/) series: **[Valid Anagram](https://leetcode.com/problems/valid-anagram/)**. Previous post: **[Contains Duplicate](/posts/blind75-contains-duplicate)**.

## The problem

> Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, else `false`.

```
s = "anagram", t = "nagaram"  -> true   (same letters, same counts)
s = "rat",     t = "car"      -> false  (different letters)
```

## Naive solution

Sort both strings and compare. If they're anagrams, the sorted forms are identical.

```rust
pub fn is_anagram(s: String, t: String) -> bool {
    let mut a: Vec<char> = s.chars().collect();
    let mut b: Vec<char> = t.chars().collect();
    a.sort_unstable();
    b.sort_unstable();
    a == b
}
```

Correct, but sorting costs O(n log n) time and allocates two vectors. We can do better.

## The solution

`t` is an anagram of `s` when both have the same length and contain the same letters with the same counts. First instinct: build two maps and compare them. But one map is enough. Iterate over `s` and count up; iterate over `t` and count down. If `t` is an anagram, every count lands back at zero.

```rust
/// Return true if `t` is an anagram of `s`.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn is_anagram(s: String, t: String) -> bool {
    let mut counts = std::collections::HashMap::new();

    s.chars().for_each(|c| *counts.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *counts.entry(c).or_insert(0) -= 1);

    counts.values().all(|&v| v == 0)
}
```

One pass to add, one pass to subtract, one pass to check. O(n) time, O(k) space where `k` is the number of distinct chars.

## The Rust bits that matter

* **`entry(c).or_insert(0)` returns `&mut i32`.** So `*counts.entry(c).or_insert(0) += 1` derefs that mutable reference and bumps the count in place: insert-or-update in one expression, no double lookup.
* **`chars()` yields `char`, not bytes.** Each `char` is a full Unicode scalar value, so `"é"` counts as one key, not its two UTF-8 bytes. Right for correctness, but it costs a hash per `char`.
* **No early length check needed.** Different lengths can't sum to all-zeros, so the count comparison catches it. Adding `if s.len() != t.len() { return false; }` up front is a cheap bail-out worth keeping in an interview.

If the input is known ASCII-only lowercase, swap the map for a fixed `[i32; 26]` array, O(1) space, no hashing:

```rust
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts = [0i32; 26];
    for b in s.bytes() {
        counts[(b - b'a') as usize] += 1;
    }
    for b in t.bytes() {
        counts[(b - b'a') as usize] -= 1;
    }
    counts.iter().all(|&v| v == 0)
}
```

Faster and allocation-free, but it breaks the moment a non-`a..z` byte shows up. The HashMap version handles any Unicode without special-casing.

## Testing

Table driven, same pattern as the last post:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        let cases = vec![
            ("anagram", "nagaram", true),
            ("rat", "car", false),
            ("", "", true),
            ("a", "ab", false),
        ];

        for (s, t, expected) in cases {
            assert_eq!(is_anagram(s.to_string(), t.to_string()), expected);
        }
    }
}
```

Source: [`src/array/valid_anagram.rs`](https://github.com/skharchikov/leetcode-rust/blob/master/src/array/valid_anagram.rs).

Next: **[Two Sum](/posts/blind75-two-sum)**.
