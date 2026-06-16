// Quiz 6 — difficulty: HARD 🔴  (solution)
//
// Sections: Implementing Iterator + Closures + Pattern matching + Derive.

#[derive(Debug, Clone, PartialEq)]
struct Stepper {
    current: i64,
    step: i64,
    remaining: usize,
}

impl Iterator for Stepper {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.remaining == 0 {
            return None;
        }
        let value = self.current;
        self.current += self.step;
        self.remaining -= 1;
        Some(value)
    }
}

#[derive(Debug, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

fn classify(n: i64) -> Size {
    match n {
        0..=9 => Size::Small,
        10..=99 => Size::Medium,
        _ => Size::Large,
    }
}

fn collect_sizes<I, F>(iter: I, keep: F) -> Vec<Size>
where
    I: Iterator<Item = i64>,
    F: Fn(i64) -> bool,
{
    iter.filter(|&n| keep(n)).map(classify).collect()
}

fn main() {
    // You can optionally experiment here.
}

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
