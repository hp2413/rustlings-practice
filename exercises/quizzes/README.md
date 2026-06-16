# Quizzes

After every couple of sections there is a quiz in this directory that tests your
knowledge across a bunch of sections at once — no scaffolding hand-holding, just
a problem to solve.

There are **six** quizzes. `quiz1`–`quiz3` come from upstream Rustlings and sit
in the first half of the course. `quiz4`–`quiz6` are part of the extended
edition and are graded **easy → medium → hard**, each combining several of the
newer modules:

| Quiz | Appears after | Difficulty | Sections it combines |
|------|---------------|-----------|----------------------|
| `quiz1` | flow control | — | variables, functions, `if` |
| `quiz2` | hashmaps | — | strings, vecs, move semantics, modules, enums |
| `quiz3` | traits | — | generics, traits |
| `quiz4` | iterators | 🟢 easy | lifetimes + iterators |
| `quiz5` | conversions | 🟡 medium | `Arc` + threads/`mpsc` channels + string parsing |
| `quiz6` | derive (final) | 🔴 hard | implementing `Iterator` + closures + pattern matching + derive |

Run a quiz on its own with `rustlings run quiz4` (or just let the watcher reach
it). Stuck? `rustlings hint quiz4`; the full reference answers live in
`solutions/quizzes/`.
