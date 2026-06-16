// Quiz 4 — difficulty: EASY 🟢  (solution)
//
// Sections: Lifetimes + Iterators.

// The `&'a` is required on the element type because the returned reference
// borrows from one of the `&str`s inside the slice. The outer slice borrow can
// stay anonymous — we don't return it.
fn longest_word<'a>(words: &[&'a str]) -> &'a str {
    words
        .iter()
        .copied()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_longest() {
        let words = ["apple", "banana", "kiwi"];
        assert_eq!(longest_word(&words), "banana");
    }

    #[test]
    fn single_word() {
        let words = ["solo"];
        assert_eq!(longest_word(&words), "solo");
    }

    #[test]
    fn empty_slice_returns_empty() {
        let words: [&str; 0] = [];
        assert_eq!(longest_word(&words), "");
    }
}
