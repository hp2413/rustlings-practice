# 🦀 Rust Flash-Cards

A browser-based flash-card app for learning Rust, generated from this repo's
**139 rustlings exercises**. The logic is Rust (a generator binary + a std-only
web server); the UI is dependency-free vanilla HTML/CSS/JS. Every card's content
lives in one generated `data/cards.json`.

> This is its own Cargo project. It does **not** touch the rustlings
> `Cargo.toml` at the repo root.

---

## Quick start

```bash
cd flash-card
cargo run --bin generate   # build data/cards.json from ../info.toml + exercises
cargo run                  # start the web server
# open http://127.0.0.1:7697
```

`cargo run` (with no `--bin`) starts the **web server**; `cargo run --bin
generate` (re)builds the deck. Run `generate` once before the first `cargo run`,
and again whenever you solve more exercises so their real solutions show up.

### Port

The server listens on **`127.0.0.1:7697`** by default. Override it if that port
is taken:

```bash
cargo run -- 8090                 # bare port -> 127.0.0.1:8090
cargo run -- 0.0.0.0:9000         # full address
FLASH_CARD_ADDR=127.0.0.1:9001 cargo run
```

On bind failure the server prints a hint with these options.

---

## What you can do

- **One card at a time:** module badge + concept summary, the task/question,
  collapsible starter code, a text area for your own attempt, and doc links.
- **Show hint** / **Show answer** — the answer is real solution code where it
  exists, or a clearly-labelled **Guidance** answer (synthesized from the hint +
  module concept) for exercises you haven't solved yet.
- **Ordered / Shuffle** — walk the deck in the canonical `info.toml` order, or a
  randomized order (no card lost or duplicated).
- **Module filter** — focus on a single section (e.g. *Move Semantics*).
- **Rate yourself** — mark each card *Got it* or *Review*.
- **Pick up where you left off** — your position, mode, module, attempts, and
  ratings are saved in `localStorage`.

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `Space` | flip the answer |
| `H` | toggle the hint |
| `←` / `→` | previous / next card |

(Shortcuts are ignored while you're typing in the answer box.)

---

## How the deck is generated

`cargo run --bin generate` reads, for each of the 139 exercises:

| Source | Used for |
|--------|----------|
| `../info.toml` | canonical order, `name`, `dir` (module), `hint`, `test` |
| `pristine/<dir>/<name>.rs` *(if present)*, else `../exercises/<dir>/<name>.rs` | the **question** (leading comment + `// TODO`s) and starter code |
| `../solutions/<dir>/<name>.rs` | the **answer** — real code, or a stub → *guidance* fallback |
| `../exercises/<dir>/README.md` | module title, concept summary, doc links |

A solution counts as a stub (→ guidance) when it still contains the rustlings
marker `DON'T EDIT THIS SOLUTION FILE`. The generator prints the live split, e.g.
`Wrote 139 cards (73 solution / 66 guidance)` — solve more exercises and that
ratio climbs automatically.

### Pristine starter snapshots (`pristine/`)

rustlings exercises are solved **in place**, so once you've worked through a
module the file under `../exercises/` holds *your answer*, not the starter code —
and a regenerated card would show the solution as its "starter". To keep cards
honest, the generator first looks for a pristine snapshot at
`pristine/<dir>/<name>.rs` and only falls back to the live exercise when none
exists. The snapshots for the already-solved modules (00–08) are vendored here,
captured from the original exercise sources.

When you finish a new module and want its cards to still show the starter code,
snapshot the originals **before** solving (or pull them from git), e.g.:

```bash
# from the repo root, for a module you haven't solved yet:
mkdir -p flash-card/pristine/09_strings
cp exercises/09_strings/*.rs flash-card/pristine/09_strings/
```

---

## Project layout

```
flash-card/
├── Cargo.toml          # standalone crate (default-run = the server)
├── src/
│   ├── lib.rs          # Card / Deck / AnswerKind types (serde)
│   ├── bin/generate.rs # repo -> data/cards.json
│   └── main.rs         # std-only static web server
├── data/cards.json     # generated deck (the single source the UI renders)
├── pristine/           # vendored original starter sources for solved modules
└── web/                # index.html, styles.css, app.js (zero build step)
```

---

## Development

```bash
cargo test     # lib + generator unit tests
cargo clippy   # lints
```
