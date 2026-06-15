# Rustlings — Extended Edition 🦀

A hands-on set of small Rust exercises for learning by doing. It is built on the
official [Rustlings](https://rustlings.rust-lang.org/) project and **extended
with ~42 extra exercises** that fill gaps in the original curriculum (closures,
pattern matching, implementing `Iterator`, smart-pointer depth, channels,
lifetimes depth, doctests, custom errors, a capstone, formatting, flow control,
slices, and derivable traits).

Each exercise is intentionally **broken or incomplete** — you fix it until it
compiles and its tests pass. There are now **136 exercises** in total.

Every exercise ships as four coordinated pieces:

| Piece | Location | Purpose |
|------|----------|---------|
| Exercise | `exercises/<module>/<name>.rs` | The broken file **you edit** (teaching comments + `// TODO`s) |
| Solution | `solutions/<module>/<name>.rs` | A complete, passing reference answer |
| Hint | `info.toml` (`[[exercises]]`) | Shown by `rustlings hint <name>` |
| Module intro | `exercises/<module>/README.md` | Concept summary + doc links |

---

## Prerequisites

1. **Rust toolchain** (via [rustup](https://rustup.rs/)):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **The Rustlings CLI** (one-time install):
   ```bash
   cargo install rustlings
   ```

> ⚠️ Do **not** run `rustlings init` for this project. `init` downloads the
> *official* exercises into a fresh folder. This repo is a self-contained
> community project — you run `rustlings` *inside the cloned folder* and it picks
> up the exercises from the local `info.toml`.

---

## Quick start (clone & practice)

```bash
# 1. Clone this repository
git clone https://github.com/hp2413/Rustling.git
cd Rustling          # the cloned folder (this repo)

# 2. Make sure every exercise is in its clean, UNSOLVED state
./reset.sh

# 3. Start the interactive watcher
rustlings
```

`rustlings` opens a watcher that walks you through the exercises **in order**,
re-running the current one every time you save. Press **`h`** for a hint and
**`n`** to advance when an exercise passes.

---

## 🧼 Getting a clean slate (read this!)

This repo may contain exercises the author **already solved**. To practice them
yourself you need them back in their original broken state. Run:

```bash
./reset.sh
```

`reset.sh` restores every exercise file to its original unsolved version and
clears the personal progress tracker — your own answers are never lost because
the complete solutions always remain in `solutions/`.

Other useful resets:

```bash
git checkout -- exercises/01_variables/variables1.rs   # reset ONE custom exercise you edited
rm -f .rustlings-state.txt                             # forget progress, start at exercise #1
```

> `.rustlings-state.txt` is your *personal* progress file. It is git-ignored, so
> cloning this repo always starts you fresh.

---

## Working through an exercise

```bash
rustlings              # interactive watch mode (recommended)
rustlings run <name>   # run a single exercise once,   e.g. rustlings run iterators9
rustlings hint <name>  # print its hint,               e.g. rustlings hint weak1
rustlings list         # list all exercises and progress
```

Workflow: open `exercises/<module>/<name>.rs`, read the comment header, fix the
`// TODO`s until it compiles and the tests pass, then move on. Stuck? Read the
hint; still stuck? Compare with `solutions/<module>/<name>.rs`.

---

## Exercise map (learning order)

> Order follows `info.toml`, not directory numbers — the expansion modules are
> slotted in where they make sense pedagogically. **Bold** = added/extended in
> this edition.

| # | Module | Topics |
|---|--------|--------|
| 00 | `00_intro` | Getting started |
| 01 | `01_variables` | `let`, `mut`, `const`, shadowing |
| 02 | `02_functions` | Functions, parameters, return values |
| 03 | `03_if` | Conditionals |
| ➕ | **`29_flow_control`** | `loop`/`break`-value, `while`/`while let`, `for` + labeled break |
| 04 | `04_primitive_types` | Tuples, arrays, chars, booleans |
| ➕ | **`28_formatting`** | `{:?}`/`{:#?}`, format specifiers, `impl Display` |
| 05 | `05_vecs` | `Vec<T>` |
| 06 | `06_move_semantics` | Ownership & moves |
| ➕ | **`30_slices`** | Array/`Vec` slices, string slices, `first_word` |
| 07 | `07_structs` | Structs & methods |
| 08 | `08_enums` | Enums & `match` |
| 09 | `09_strings` | `String` vs `&str` |
| 10 | `10_modules` | Modules & visibility |
| 11 | `11_hashmaps` | `HashMap` |
| 12 | `12_options` | `Option<T>` |
| 13 | `13_error_handling` | `Result`, `?`, **custom error + `From` + `Box<dyn Error>` (`errors7`)** |
| 14 | `14_generics` | Generics, **generic struct/impl + `where` (`generics3–4`)** |
| 15 | `15_traits` | Traits, **default methods, `Box<dyn>`, operator overload (`traits6–8`)** |
| 16 | `16_lifetimes` | Lifetimes, **struct-holding-ref + multiple lifetimes (`lifetimes4–5`)** |
| 17 | `17_tests` | Tests, **doctests (`tests4`)** |
| 18 | `18_iterators` | Adapters/consumers, **`iter`/`iter_mut`/`into_iter`, `fold`/`scan`, `collect`, and implementing `Iterator` (`iterators6–9`)** |
| 19 | `19_smart_pointers` | `Box`/`Rc`/`Arc`/`Cow`, **cons-list, `Deref`, `Drop`, `RefCell`, `Weak` (`cons_list1`, `deref1`, `drop1`, `refcell1`, `weak1`)** |
| 20 | `20_threads` | `spawn`/`Arc<Mutex>`, **`move` into thread, mpsc channels, multi-producer sum (`move_threads1`, `channels1–2`)** |
| 21 | `21_macros` | `macro_rules!` |
| 22 | `22_clippy` | Clippy lints |
| 23 | `23_conversions` | `as`, `From`/`Into`, `FromStr`, `TryFrom`, `AsRef` |
| ➕ | **`24_closures`** | Capture modes, `Fn`/`FnMut`/`FnOnce`, HOFs, returning closures |
| ➕ | **`25_patterns`** | Guards, ranges, destructuring, `@` bindings, `if let`/`let else`/`while let` |
| ➕ | **`26_capstone`** | minigrep-style search/count; divide-and-conquer parallel sum |
| ➕ | **`31_derive`** | `Debug`/`Clone`/`Copy`/`PartialEq`; `Default`/`Ord`/`Hash` |

---

## 📚 Reference material

The `references/` folder bundles the source material these exercises are drawn
from. Every concept above can be traced back to one of these.

| File | What it is |
|------|------------|
| [`references/myRustNotes.pdf`](references/myRustNotes.pdf) | **Personal study notes** from the [100xdevs Rust Bootcamp](https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-1). A plain-English walkthrough: install → cargo → integers/types/print/const → ownership & borrowing → structs → enums & pattern matching → `Result`/`Option` → collections → **iterators & adapter tables** → strings vs slices → generics → traits → lifetimes → **multithreading (spawn/join, move closures, mpsc channels)** → external crates (`rand`, `chrono`). Best for quick revision. |
| [`references/Rust by examples.pdf`](references/Rust%20by%20examples.pdf) | **Rust by Example (RBE)** — learn each concept through small runnable snippets: primitives, custom types, conversions, flow control, functions/closures, modules, crates/cargo, generics, traits, **macros**, error handling, std types, threads/files, testing, and docs. |
| [`references/The Rust Programming Language.pdf`](references/The%20Rust%20Programming%20Language.pdf) | **"The Book" (TRPL)** — the official, comprehensive guide (ch1 install → ch20 web-server capstone). The canonical reference for everything here. |
| [`references/The rustdoc book.pdf`](references/The%20rustdoc%20book.pdf) | **The rustdoc book** — generating docs with `rustdoc`/`cargo doc`, writing doc comments (`///` outer, `//!` inner), and running **doctests** (the runnable ```` ```rust ```` examples inside docs — directly behind `17_tests/tests4`). |

Online equivalents: [The Book](https://doc.rust-lang.org/book/) ·
[Rust by Example](https://doc.rust-lang.org/rust-by-example/) ·
[The rustdoc book](https://doc.rust-lang.org/rustdoc/) ·
[std docs](https://doc.rust-lang.org/std/).

> The PDFs add ~28 MB to the repo. If you'd rather keep the repo lean, delete
> `references/` and use the online links above instead.

---

## For maintainers — preparing a clean handoff

If you've solved exercises and want recipients to get a pristine set:

```bash
# 1. Make sure the expansion files are committed (they may be new/untracked)
git add -A

# 2. Reset every exercise to unsolved, then commit that clean state
./reset.sh
git add -A
git commit -m "Reset exercises to clean practice state"
git push
```

Your solved progress is never lost — earlier commits keep your answers, and
`solutions/` always holds a complete reference for every exercise. After this,
anyone who clones gets a clean slate without even needing `./reset.sh`.

---

## Credits & license

- Based on [**Rustlings**](https://github.com/rust-lang/rustlings) by the Rust
  team, used under its MIT license. The expansion exercises follow the same
  structure and license.
- Reference PDFs are the property of their respective authors
  (the Rust project; 100xdevs / hkirat) and are included here for personal study.
