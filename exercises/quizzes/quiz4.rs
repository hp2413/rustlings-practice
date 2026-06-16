// Quiz 4 — difficulty: EASY 🟢
//
// This is a quiz for the following sections:
// - Lifetimes
// - Iterators
//
// You're given a slice of words. Write `longest_word`, which returns a
// reference to the longest word in the slice (compared by byte length, i.e.
// `str::len`). Because the returned reference borrows from the input, the
// function needs a lifetime annotation — the compiler can't elide it here
// (there are two input lifetimes: the outer slice borrow AND the `&str`
// elements, so elision doesn't know which one the output comes from).
//
// Build it with iterator methods (`.iter()`, `.max_by_key(...)`), not a manual
// index loop. If the slice is empty, return "".

// TODO: Add the lifetime annotation(s) and complete the body.
fn longest_word(words: &[&str]) -> &str {
    // TODO: iterate over `words`, pick the element with the largest `len()`,
    // and handle the empty case with `.unwrap_or("")`.
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
