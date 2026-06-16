// Quiz 6 — difficulty: HARD 🔴
//
// This is a quiz for the following sections:
// - Implementing the `Iterator` trait
// - Closures (higher-order functions)
// - Pattern matching (`match` with range patterns)
// - Derivable traits (Debug, Clone, PartialEq)
//
// This is the hardest quiz — it pulls four topics together.
//
// 1. `Stepper` is a custom iterator. Calling `.next()` should yield `current`,
//    then advance `current` by `step`, for exactly `remaining` items, after
//    which it returns `None`. Derive Debug, Clone and PartialEq for it, and
//    implement `Iterator` with `Item = i64`.
//
// 2. `classify(n)` returns a `Size` using a `match` with RANGE patterns:
//      0..=9 -> Small, 10..=99 -> Medium, anything else -> Large.
//    Derive Debug and PartialEq for `Size`.
//
// 3. `collect_sizes(iter, keep)` is a higher-order function. It takes any
//    iterator of `i64` and a closure `keep: Fn(i64) -> bool`, keeps only the
//    items for which `keep` returns `true`, classifies each kept item with
//    `classify`, and collects the results into a `Vec<Size>`.

// TODO: derive the required traits for `Stepper`.
struct Stepper {
    current: i64,
    step: i64,
    remaining: usize,
}

// TODO: implement `Iterator` for `Stepper` (Item = i64).

// TODO: derive the required traits for `Size`.
enum Size {
    Small,
    Medium,
    Large,
}

// TODO: implement `classify` using range patterns.
fn classify(n: i64) -> Size {
    ???
}

// TODO: implement the higher-order function `collect_sizes`.
fn collect_sizes(/* iter, keep */) -> Vec<Size> {
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
    fn stepper_yields_count_items() {
        let stepper = Stepper { current: 0, step: 5, remaining: 4 };
        assert_eq!(stepper.collect::<Vec<_>>(), vec![0, 5, 10, 15]);
    }

    #[test]
    fn stepper_is_clonable_and_comparable() {
        let a = Stepper { current: 1, step: 2, remaining: 3 };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn classify_uses_ranges() {
        assert_eq!(classify(3), Size::Small);
        assert_eq!(classify(42), Size::Medium);
        assert_eq!(classify(1000), Size::Large);
    }

    #[test]
    fn collect_sizes_filters_and_classifies() {
        let stepper = Stepper { current: 0, step: 25, remaining: 6 };
        // yields 0, 25, 50, 75, 100, 125; keep multiples of 50 -> 0, 50, 100
        let sizes = collect_sizes(stepper, |n| n % 50 == 0);
        assert_eq!(sizes, vec![Size::Small, Size::Medium, Size::Large]);
    }
}
