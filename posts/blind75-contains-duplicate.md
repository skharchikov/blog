---
title: "Blind 75 in Rust: Contains Duplicate"
date: "2026-06-23"
slug: "blind75-contains-duplicate"
excerpt: "Starting a Blind 75 series in Rust. First problem: Contains Duplicate, a one liner that doubles as a lesson in HashSet and ownership."
tags: ["rust", "leetcode", "blind75", "algorithms"]
read_time: 3
---

I'm working through the [Blind 75](https://www.techinterviewhandbook.org/best-practice-questions/) in Rust, one problem per post. The borrow checker turns even trivial problems into small ownership lessons. First up: **Contains Duplicate**.

## The problem

> Given an integer array `nums`, return `true` if any value appears at least twice, else `false`.

```
[1, 2, 3, 1]  -> true   (1 repeats)
[1, 2, 3, 4]  -> false  (all distinct)
```

Values span the full `i32` range, so no bitset shortcut. Brute force is O(n²); skip it. The expected answer trades space for time with a hash set: O(n) both ways.

## The solution

```rust
/// Return true if any value appears at least twice.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = std::collections::HashSet::new();

    for i in nums.iter() {
        if seen.contains(i) {
            return true;
        }
        seen.insert(*i);
    }
    false
}
```

Walk once. Check the set, bail on the first repeat, otherwise record and move on.

## The Rust bits that matter

Two things the compiler makes you think about:

* **`iter()` yields `&i32`.** So `i` is a reference. `HashSet::contains` wants `&T`, and `i` already is one, so `seen.contains(i)` needs no extra `&`.
* **`insert` wants the owned value.** The set holds `i32`, so `*i` derefs the borrow to copy the value in. `i32` is `Copy`, so it's free, and `nums` stays intact.

Collapsed to one line, leaning on `insert` returning `false` when the value already exists:

```rust
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = std::collections::HashSet::new();
    !nums.into_iter().all(|n| seen.insert(n))
}
```

Clever, but the explicit loop reads better at 2am.

## Testing

Table driven, the pattern I'll reuse all series:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let cases = vec![
            (vec![1, 2, 3, 1], true),
            (vec![1, 2, 3, 4], false),
            (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ];

        for (nums, expected) in cases {
            assert_eq!(contains_duplicate(nums), expected);
        }
    }
}
```

Source: [`src/array/contains_duplicate.rs`](https://github.com/skharchikov/leetcode-rust/blob/master/src/array/contains_duplicate.rs).

Next: **[Valid Anagram](/posts/blind75-valid-anagram)**.
