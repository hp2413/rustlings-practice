# 🦀 Rust Flash-Cards — Build Plan

A browser-based flash-card app for learning Rust, generated from this repo's
139 exercises. **Logic/codebase is Rust** (a generator binary + a web server);
the browser UI is lightweight vanilla HTML/CSS/JS. **All card content lives in
one `data/cards.json`** produced by the Rust generator.

> Decisions locked in with the user:
> - **Architecture:** Rust backend (generator + std web server) + light HTML/JS UI.
> - **Answer field:** real solution code where it exists; otherwise a clearly
>   marked *guidance* answer synthesized from the hint + module concept summary.

---

## 1. What the app does (target UX)

A single card at a time shows:

- **Concept / module** badge (e.g. *Move Semantics*) + a short concept summary.
- **Question / task** — what this exercise teaches and what you must do.
- A **text area** where the learner writes their own answer/attempt.
- **`Hint` button** → reveals the exercise's hint.
- **`Show Answer` button** → reveals the solution code, or a marked *guidance*
  answer when no solution code exists.
- **Prev / Next** navigation + a **progress** indicator (`23 / 139`).
- **Mode toggle** with two ways to walk the deck:
  1. **Ordered** — the exact pedagogical order from `info.toml` (matches the
     README learning order / how exercises appear).
  2. **Shuffle** — a randomized order of the same cards.

---

## 2. Data sources (already in this repo)

| Source | Used for |
|--------|----------|
| `../info.toml` (139 `[[exercises]]` blocks) | **Canonical order**, `name`, `dir` (module), `hint` |
| `../exercises/<dir>/<name>.rs` | The **question/task**: leading teaching comment + `// TODO`s + the broken code |
| `../solutions/<dir>/<name>.rs` | The **answer** — 72/139 have real code; 67 are empty stubs |
| `../exercises/<dir>/README.md` | **Concept summary** + doc links per module |

**Fact established up front:** 72 solutions contain real code, 67 are empty
stubs → the generator must fall back to *guidance* for those 67 (`answer_kind`).

---

## 3. Target folder layout

```
flash-card/
├── PLAN.md              # this file
├── README.md           # run instructions (Phase 5)
├── Cargo.toml          # independent cargo project (does NOT touch rustlings' Cargo.toml)
├── src/
│   ├── lib.rs          # Card struct + shared types
│   ├── bin/generate.rs # Rust: parse repo -> data/cards.json
│   └── main.rs         # Rust: static web server (std-only)
├── data/
│   └── cards.json      # generated; the single source the UI renders
└── web/
    ├── index.html
    ├── styles.css
    └── app.js          # render, flip, hint, shuffle/ordered, nav, progress
```

> `flash-card/` is its **own** Cargo project. The huge rustlings `Cargo.toml` at
> the repo root is left untouched.

---

## 4. Card JSON schema (`data/cards.json`)

```jsonc
{
  "generated_at": "2026-06-16T...",
  "count": 139,
  "cards": [
    {
      "id": "move_semantics2",     // exercise name (stable key)
      "order": 23,                  // 0-based canonical index from info.toml
      "module": "06_move_semantics",
      "module_title": "Move Semantics",   // derived from dir
      "concept_summary": "Ownership & moves ...",  // from module README (1st para)
      "doc_links": ["https://doc.rust-lang.org/book/..."],
      "question": "Make both vectors accessible at the same time ...", // header + TODOs
      "exercise_code": "fn fill_vec(...) { ... }",  // the broken/incomplete .rs
      "hint": "...",                // from info.toml
      "answer": "fn fill_vec(...) { ... }",         // solution code OR guidance text
      "answer_kind": "solution",    // "solution" | "guidance"
      "is_quiz": false,
      "has_tests": true
    }
  ]
}
```

---

## 5. Dependencies (minimal, offline once fetched)

- **Generator** (`generate.rs`): `serde`, `serde_json`, `toml`, `chrono` (timestamp; optional).
- **Server** (`main.rs`): **std only** — `std::net::TcpListener` static file server,
  no web framework. Serves `web/` + `data/cards.json` on `http://127.0.0.1:8080`.
- **Frontend**: zero build step — plain HTML/CSS/JS, no npm.

---

## 6. Phases & how to trigger each

Each phase is independently buildable and testable. To run a phase, just tell me
the trigger phrase (left column). You can also say **"run all phases"** to go
straight through 1→5.

| Phase | Trigger phrase | What gets built | Done when… |
|-------|----------------|-----------------|------------|
| **1. Scaffold & schema** | `implement phase 1` | `Cargo.toml`, `src/lib.rs` (the `Card` struct + serde), folder skeleton, placeholder `data/cards.json` | `cargo build` succeeds in `flash-card/` |
| **2. Card generator** | `implement phase 2` | `src/bin/generate.rs` — parses `../info.toml`, exercises, solutions, module READMEs → writes complete `data/cards.json` (139 cards, canonical order, solution/guidance fallback) | `cargo run --bin generate` prints `139 cards (72 solution / 67 guidance)` and writes valid JSON |
| **3. Web server** | `implement phase 3` | `src/main.rs` — std-only static server serving `web/` + `data/cards.json` | `cargo run` serves `http://127.0.0.1:8080`; `curl` returns `cards.json` |
| **4. Frontend UI** | `implement phase 4` | `web/index.html`, `styles.css`, `app.js` — card render, answer box, Hint + Show Answer buttons, Prev/Next, progress, **Ordered/Shuffle** toggle | Open in browser: flip a card, see hint + answer, switch ordered↔shuffle |
| **5. Polish & docs** | `implement phase 5` | Module filter, keyboard shortcuts (`Space` flip, `h` hint, `←/→` nav), `localStorage` for progress+mode, self-rating (got it / review), responsive CSS, `flash-card/README.md` | App feels complete; README documents `generate` + `run` |

### Acceptance criteria per phase

- **Phase 1** — Project compiles; `Card` (de)serializes to/from JSON in a unit test.
- **Phase 2** — JSON validates; exactly 139 cards; `order` is contiguous 0..138
  matching `info.toml`; every card has non-empty `question` and `hint`;
  `answer_kind` is `solution` for the 72 filled and `guidance` for the 67 stubs.
- **Phase 3** — `GET /` returns `index.html`; `GET /data/cards.json` returns the
  JSON with correct content-type; unknown path → 404.
- **Phase 4** — All buttons work; Ordered mode matches `order`; Shuffle reorders
  without losing/duplicating cards; progress counter is correct.
- **Phase 5** — Reload preserves position + mode; keyboard shortcuts work;
  README's commands reproduce a working app from scratch.

---

## 7. Standard run commands (after Phase 4)

```bash
cd flash-card
cargo run --bin generate   # (re)build data/cards.json from the repo
cargo run                  # start the server
# open http://127.0.0.1:8080
```

---

## 8. Open / deferred ideas (not in scope unless you ask)

- Pull richer per-card "Topics" text from the README "Exercise map" table.
- Spaced-repetition scheduling (Leitner boxes) instead of plain shuffle.
- Export/import progress as a file.
- Fill the 67 empty solution stubs so every answer is real code (separate task).
```