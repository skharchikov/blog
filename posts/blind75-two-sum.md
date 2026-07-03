---
title: "Blind 75 in Rust: Two Sum"
date: "2026-07-02"
slug: "blind75-two-sum"
excerpt: "Third problem in the Blind 75 series: Two Sum, the classic. One HashMap, a single pass, and why storing the value-to-index mapping beats the brute force nested loop."
tags: ["rust", "leetcode", "blind75", "algorithms"]
read_time: 3
---

Third problem in the [Blind 75](https://www.techinterviewhandbook.org/best-practice-questions/) series: **[Two Sum](https://leetcode.com/problems/two-sum/description/)**. Previous post: **[Valid Anagram](/posts/blind75-valid-anagram)**.

## The problem

> Given an integer array `nums` and an integer `target`, return the indices of the two numbers that add up to `target`.

```
nums = [2, 7, 11, 15], target = 9  -> [0, 1]   (2 + 7)
nums = [3, 2, 4],      target = 6  -> [1, 2]   (2 + 4)
nums = [3, 3],         target = 6  -> [0, 1]
```

Exactly one solution exists, and you can't reuse the same element twice.

## Naive solution

Check every pair. For each `i`, scan the rest for a `j` where `nums[i] + nums[j] == target`.

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}
```

Correct, no extra space, but O(n²) time. With `nums.length` up to 10^4 that's 100M pairs worst case. We can drop it to one pass.

## The solution

For each number `n`, the number that completes the pair is `target - n`. Instead of scanning ahead for it, remember every value already seen and the index it sat at. Then each step is a single lookup: have I already seen my complement?

```rust
use std::collections::HashMap;

/// Return the indices of the two numbers that add up to `target`.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index, &n) in nums.iter().enumerate() {
        if let Some(idx) = map.get(&(target - n)) {
            return vec![*idx as i32, index as i32];
        }
        map.insert(n, index);
    }

    vec![]
}
```

Walk once. Check the map for the complement first, *then* insert `n`. Checking before inserting is what stops an element from pairing with itself.

## The Rust bits that matter

* **`iter().enumerate()` yields `(usize, &i32)`.** The `&n` in the pattern destructures the reference, so `n` is a plain `i32` (it's `Copy`). Clean arithmetic like `target - n` with no derefs sprinkled around.
* **The map is `HashMap<i32, usize>`, value → index.** The key is the number so `get(&(target - n))` is an O(1) lookup by value; the stored `usize` is the index we need to return.
* **`get` returns `Option<&usize>`.** `if let Some(idx)` binds `idx: &usize`, so `*idx as i32` derefs then casts to match the `Vec<i32>` return type.
* **Order matters: lookup before insert.** Insert first and `[3, 3]` with `target = 6` would find its own index and return `[0, 0]`. Checking the earlier entries before adding the current one guarantees two distinct indices.

## Testing

Table driven, same pattern as the last two posts:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let cases = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for (nums, target, expected) in cases {
            assert_eq!(two_sum(nums, target), expected);
        }
    }
}
```

Source: [`src/array/two_sum.rs`](https://github.com/skharchikov/leetcode-rust/blob/master/src/array/two_sum.rs).

Next: **Best Time to Buy and Sell Stock**.
