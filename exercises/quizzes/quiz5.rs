// Quiz 5 — difficulty: MEDIUM 🟡
//
// This is a quiz for the following sections:
// - Smart pointers (Arc)
// - Threads (spawn + mpsc channels)
// - Conversions (parsing strings with `str::parse` / the `FromStr` trait)
//
// You're given a list of number strings such as ["10", "20", "30"]. Compute
// their total as an `i64` — but do the work across threads:
//   * wrap the input in an `Arc` so it can be shared with each thread,
//   * spawn one thread per element; each thread parses its own number and
//     sends the value back over an mpsc channel,
//   * the main thread sums everything it receives from the channel.
//
// Leading/trailing whitespace should be ignored (use `.trim()`), and you may
// assume every string parses successfully.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

// TODO: Complete the function.
fn parallel_sum(numbers: Vec<String>) -> i64 {
    // TODO:
    // 1. let numbers = Arc::new(numbers);
    // 2. let (tx, rx) = mpsc::channel();
    // 3. for each index `i`, clone the Arc and the sender, then spawn a thread
    //    that parses `numbers[i]` into an i64 and sends it.
    // 4. drop the original `tx` so the channel can close, then sum `rx`.
    ???
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_simple() {
        let input = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        assert_eq!(parallel_sum(input), 6);
    }

    #[test]
    fn ignores_whitespace_and_handles_larger() {
        let input = vec![
            " 10 ".to_string(),
            "20".to_string(),
            "  30".to_string(),
            "40 ".to_string(),
        ];
        assert_eq!(parallel_sum(input), 100);
    }

    #[test]
    fn empty_is_zero() {
        let input: Vec<String> = Vec::new();
        assert_eq!(parallel_sum(input), 0);
    }
}
