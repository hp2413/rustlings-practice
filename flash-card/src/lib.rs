//! Shared types for the Rust flash-card app.
//!
//! A [`Deck`] is the single artifact the UI renders: it is produced by the
//! generator binary (Phase 2) into `data/cards.json` and served as-is by the
//! web server (Phase 3). Every [`Card`] is built from one rustlings exercise.

use serde::{Deserialize, Serialize};

/// Whether a card's `answer` is real solution code or synthesized guidance.
///
/// 72 of the 139 exercises ship a real solution; the remaining 67 have empty
/// solution stubs and fall back to [`AnswerKind::Guidance`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnswerKind {
    /// `answer` holds real solution code from `solutions/<dir>/<name>.rs`.
    Solution,
    /// `answer` holds guidance synthesized from the hint + concept summary.
    Guidance,
}

/// One flash-card, generated from a single rustlings exercise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    /// Exercise name — the stable key (e.g. `"move_semantics2"`).
    pub id: String,
    /// 0-based canonical index from `info.toml`.
    pub order: usize,
    /// Module directory (e.g. `"06_move_semantics"`).
    pub module: String,
    /// Human-friendly module title derived from `module` (e.g. `"Move Semantics"`).
    pub module_title: String,
    /// Short concept summary, from the module README's first paragraph.
    pub concept_summary: String,
    /// Documentation links pulled from the module README.
    pub doc_links: Vec<String>,
    /// The task to solve: teaching header + `// TODO`s.
    pub question: String,
    /// The broken/incomplete exercise source.
    pub exercise_code: String,
    /// Hint from `info.toml`.
    pub hint: String,
    /// Solution code, or synthesized guidance text (see [`Card::answer_kind`]).
    pub answer: String,
    /// Whether `answer` is real solution code or guidance.
    pub answer_kind: AnswerKind,
    /// True if this exercise is a quiz.
    pub is_quiz: bool,
    /// True if the exercise ships with tests.
    pub has_tests: bool,
}

/// The full generated deck — the single source the UI renders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck {
    /// RFC 3339 timestamp of when the deck was generated.
    pub generated_at: String,
    /// Number of cards; mirrors `cards.len()`.
    pub count: usize,
    /// The cards, in canonical `order`.
    pub cards: Vec<Card>,
}

impl Deck {
    /// Build a deck from cards, setting `count` to match `cards.len()`.
    pub fn new(generated_at: impl Into<String>, cards: Vec<Card>) -> Self {
        Deck {
            generated_at: generated_at.into(),
            count: cards.len(),
            cards,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_card() -> Card {
        Card {
            id: "move_semantics2".into(),
            order: 23,
            module: "06_move_semantics".into(),
            module_title: "Move Semantics".into(),
            concept_summary: "Ownership & moves.".into(),
            doc_links: vec!["https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html".into()],
            question: "Make both vectors accessible at the same time.".into(),
            exercise_code: "fn fill_vec(vec: Vec<i32>) -> Vec<i32> { /* TODO */ }".into(),
            hint: "Try cloning the vector.".into(),
            answer: "fn fill_vec(vec: Vec<i32>) -> Vec<i32> { vec }".into(),
            answer_kind: AnswerKind::Solution,
            is_quiz: false,
            has_tests: true,
        }
    }

    #[test]
    fn card_round_trips_through_json() {
        let card = sample_card();
        let json = serde_json::to_string(&card).unwrap();
        let back: Card = serde_json::from_str(&json).unwrap();
        assert_eq!(card, back);
    }

    #[test]
    fn answer_kind_serializes_lowercase() {
        assert_eq!(
            serde_json::to_string(&AnswerKind::Solution).unwrap(),
            "\"solution\""
        );
        assert_eq!(
            serde_json::to_string(&AnswerKind::Guidance).unwrap(),
            "\"guidance\""
        );
    }

    #[test]
    fn deck_new_sets_count_from_cards() {
        let deck = Deck::new("2026-06-16T00:00:00Z", vec![sample_card()]);
        assert_eq!(deck.count, 1);
    }

    #[test]
    fn deck_round_trips_through_json() {
        let deck = Deck::new("2026-06-16T00:00:00Z", vec![sample_card()]);
        let json = serde_json::to_string_pretty(&deck).unwrap();
        let back: Deck = serde_json::from_str(&json).unwrap();
        assert_eq!(deck, back);
    }
}
