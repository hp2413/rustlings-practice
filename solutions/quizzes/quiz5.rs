// Quiz 5 — difficulty: MEDIUM 🟡  (solution)
//
// Sections: Smart pointers (Arc) + Threads (spawn + mpsc) + Conversions (parse).

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

fn parallel_sum(numbers: Vec<String>) -> i64 {
    // `Arc` lets every thread share read-only access to the same Vec.
    let numbers = Arc::new(numbers);
    let (tx, rx) = mpsc::channel();

    for i in 0..numbers.len() {
        let numbers = Arc::clone(&numbers);
        let tx = tx.clone();
        thread::spawn(move || {
            // `parse` is powered by the `FromStr` trait; the `: i64` annotation
            // tells it which type to produce.
            let value: i64 = numbers[i].trim().parse().unwrap();
            tx.send(value).unwrap();
        });
    }
    // Drop the original sender so the channel closes once every clone is gone;
    // otherwise `rx` would block forever waiting for more messages.
    drop(tx);

    rx.iter().sum()
}

fn main() {
    // You can optionally experiment here.
}

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
