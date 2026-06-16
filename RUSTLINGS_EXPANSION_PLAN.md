# Rustlings Expansion Plan

> A detailed, phased implementation plan to extend this Rustlings project with new
> modules and exercises covering concepts found in the three reference PDFs but
> under-covered (or missing) in the current exercise set. Written so each phase can
> be handed to Claude Code independently and implemented end-to-end.

---

## 0. Source material reviewed

| Ref tag | File | What it is | How it's cited below |
|--------|------|------------|----------------------|
| **NOTES** | `myRustNotes.pdf` (72 pp) | User's own 100xdevs bootcamp notes. Strong on: integers/print/const, memory (stack/heap), references & borrowing rules, structs (tuple/unit/impl), enums + pattern matching, Result/Option, collections (array/vec/HashMap), **iterators + adapter tables (very detailed)**, strings vs slices, generics, traits (default impl, trait-as-param, trait bounds), lifetimes (fn + struct), **multithreading (spawn/join, move closures, mpsc channels, drop(tx))**, external crates (`rand`, `chrono`). | `NOTES: <topic>` |
| **RBE** | `Rust by examples.pdf` (37 pp) | Rust by Example. Chapters: Hello World, Primitives, Custom Types, Variable Bindings, Types, Conversion, Expressions, Flow of Control, Functions (closures/HOF), Modules, Crates, Cargo, Attributes, Generics, Scoping rules, Traits, Macros, Error handling, Std library types, Std misc (files/threads), Testing, Unsafe, Compatibility, Meta (docs/bench). | `RBE: <chapter>` |
| **TRPL** | `The Rust Programming Language.pdf` (158 pp) | The official Rust Book. Standard chapter numbering used in citations (ch1 install → ch20 web-server capstone). | `TRPL ch<n>` |

### Current project inventory (baseline — do NOT duplicate)

```
00_intro(2) 01_variables(6) 02_functions(5) 03_if(3) 04_primitive_types(6)
05_vecs(2) 06_move_semantics(5) 07_structs(3) 08_enums(3) 09_strings(4)
10_modules(3) 11_hashmaps(3) 12_options(3) 13_error_handling(6) 14_generics(2)
15_traits(5) 16_lifetimes(3) 17_tests(3) 18_iterators(5) 19_smart_pointers(4: arc1,box1,cow1,rc1)
20_threads(3) 21_macros(4) 22_clippy(3) 23_conversions(5) quizzes(3)
```

### Gap analysis (what the PDFs teach that Rustlings lacks)

| # | Concept | PDF coverage | Rustlings today | Action |
|---|---------|--------------|-----------------|--------|
| 1 | **Closures** (capture borrow/mut/move, `Fn`/`FnMut`/`FnOnce`, returning closures, HOF) | TRPL ch13.1; RBE Functions/Closures; NOTES "move closures" | **None** | **New module `24_closures`** (Phase 1) |
| 2 | **Pattern matching depth** (guards, `@`, ranges, destructure, `..`, `if let`/`while let`/`let else`) | TRPL ch18; RBE Flow of Control/match | only match-in-enums | **New module `25_patterns`** (Phase 2) |
| 3 | **Iterator adapters depth** (`fold`/`scan`/`zip`/`chain`/`flat_map`/`partition`/`enumerate`/`peekable`/iter vs iter_mut vs into_iter) | NOTES iterators (very detailed); TRPL ch13.2 | iterators1–5 | **Extend `18_iterators`** (Phase 3) |
| 4 | **Traits depth** (default impls, `dyn`/trait objects, operator overload `Add`, associated types, `where`) | NOTES traits; TRPL ch10/ch19; RBE Traits | traits1–5 | **Extend `15_traits`** (Phase 4) |
| 5 | **Generics depth** (generic struct + impl, multiple params, `where`) | NOTES generics; TRPL ch10.1; RBE Generics | generics1–2 | **Extend `14_generics`** (Phase 4) |
| 6 | **Smart pointers depth** (`RefCell`/interior mutability, `Deref`, `Drop`, `Box<dyn>` recursive cons-list) | TRPL ch15; RBE; NOTES heap | arc1/box1/cow1/rc1 | **Extend `19_smart_pointers`** (Phase 5) |
| 7 | **Channels / message passing** (`mpsc`, multi-producer, `drop(tx)`, move into thread) | NOTES message passing (full worked example); TRPL ch16.2 | threads1–3 (Arc/Mutex) | **Extend `20_threads`** (Phase 6) |
| 8 | **Lifetimes depth** (struct holding refs, elision, multiple lifetimes) | NOTES lifetimes; TRPL ch10.3 | lifetimes1–3 | **Extend `16_lifetimes`** (Phase 7) |
| 9 | **Doc-tests & documentation** (`///`, ```` ``` ```` doctest, `cargo test` of docs) | RBE Meta/Documentation; TRPL ch14.2 | tests1–3 | **Extend `17_tests`** (Phase 7) |
| 10 | **Custom error types** (`enum` error, `Box<dyn Error>`, `From` conversion, `?` chaining) | TRPL ch9; RBE Error handling | errors1–6 (covers `?`) | **Extend `13_error_handling`** (Phase 7) |
| 11 | **Capstone projects** (minigrep-style CLI; thread-pool parallel sum) | TRPL ch12 + ch20; NOTES "sum 1..10^8 with all cores" | none | **New module `26_capstone`** (Phase 8) |
| 12 | **External crates** (`rand`, `chrono`) — *optional, adds network deps* | NOTES "cargo add rand/chrono"; RBE Crates/Cargo | none | **Optional module `27_crates`** (Phase 8, gated) |

---

## 1. How Rustlings exercises are structured (implementation contract)

Every exercise is **four coordinated artifacts**. New exercises MUST follow this exactly.

1. **Exercise file** `exercises/<NN_module>/<name>.rs`
   - Compiles-broken or test-failing on purpose. Contains `// TODO:` markers describing the fix.
   - Teaching happens in **comments above the code** (the user explicitly wants rich concept comments).
   - Ends with `fn main() { // You can optionally experiment here. }` and a `#[cfg(test)] mod tests { ... }` block when `test = true`.
   - Pattern reference: `exercises/08_enums/enums3.rs` (struct + enum + `impl` + `match` + tests).

2. **Solution file** `solutions/<NN_module>/<name>.rs`
   - A complete, compiling, test-passing version. (In the official `init` project these are placeholders; for our *authored* exercises we fill them in — required by `rustlings dev check --require-solutions`.)

3. **`Cargo.toml` `[[bin]]` entries** — two per exercise:
   ```toml
   { name = "<name>",     path = "exercises/<NN_module>/<name>.rs" },
   { name = "<name>_sol", path = "solutions/<NN_module>/<name>.rs" },
   ```
   Do NOT hand-edit when using the dev workflow — `rustlings dev update` regenerates this from `info.toml` (see Phase 0).

4. **`info.toml` `[[exercises]]` entry** — THIS IS THE HINT MECHANISM (`rustlings hint <name>` reads it):
   ```toml
   [[exercises]]
   name = "closures1"
   dir = "24_closures"
   # test = true is the default; set false for compile-only exercises.
   test = false
   # skip_check_unsolved = true only for exercises that legitimately compile while unsolved.
   hint = """
   Multi-line hint text shown when the learner presses `h` / runs `rustlings hint closures1`.
   Explain the concept and nudge toward the fix without giving the full answer."""
   ```
   (Format verified against the embedded `rustlings-macros-6.5.0/info.toml`.)

5. **Module README** `exercises/<NN_module>/README.md` — short concept intro + a `## Further information` list of `doc.rust-lang.org` links. Pattern: `exercises/08_enums/README.md`.

### Exercise file template (use verbatim, adapt comments)

```rust
// <nameN>.rs
//
// <2-4 line concept explanation in plain English. Cite the idea, e.g.
// "A closure can capture its environment by reference, by mutable reference,
//  or by value (move). The compiler picks the least-restrictive option.">
//
// TODO: <one precise instruction describing exactly what the learner must change.>
//
// Run with `rustlings run <nameN>`, get a hint with `rustlings hint <nameN>`.

// ...intentionally-broken or incomplete code with inline `// TODO:` notes...

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn <descriptive_name>() {
        assert_eq!(/* ... */);
    }
}
```

---

## 2. Phase 0 — Adopt the `rustlings dev` workflow (REQUIRED FIRST, one-time)

**Why:** This project was produced by `rustlings init` (official). The official exercise list **and** hints are compiled into the `rustlings` binary (`rustlings-macros/info.toml`); there is no local `info.toml`. That means `rustlings run <new_name>` / `rustlings hint <new_name>` will NOT see anything we add. The supported way to add custom exercises with working hints is the **community-exercise (`dev`) workflow**, which reads a **local root `info.toml`**.

**Deliverables for Phase 0:**

1. Create `info.toml` at the project root with:
   - `format_version = 1`
   - `welcome_message` / `final_message` (copy/adapt from the embedded one).
   - One `[[exercises]]` block for **every existing exercise** (00→23 + quizzes), in order, each with `name`, `dir`, correct `test` flag, and a `hint` (copy the official hints out of the embedded `rustlings-macros-6.5.0/info.toml` at
     `~/.cargo/registry/src/index.crates.io-*/rustlings-macros-6.5.0/info.toml`).
   - *Then* append the new exercises from later phases.
2. Run `rustlings dev update` → regenerates `Cargo.toml` `[[bin]]` list from `info.toml`.
3. Run `rustlings dev check --require-solutions` → validates every exercise compiles, every solution passes, every exercise has a hint. Must exit clean.

**Acceptance:** `rustlings dev check` is green; `rustlings hint enums2` prints the migrated hint; `rustlings list` shows all exercises.

> Lower-risk alternative (note in PR if Phase 0 migration is undesirable): scaffold a **separate** project with `rustlings dev new rustlings-extra`, and put ONLY the new modules there. Same per-exercise contract; keeps the official project pristine. The phases below are written to drop into either layout — only the `info.toml`/`Cargo.toml` location differs.

---

## 3. Phase 1 — New module `24_closures`

**Concept sources:** TRPL ch13.1 "Closures: Anonymous Functions that Capture Their Environment"; RBE "Functions → Closures / Higher Order Functions"; NOTES "Using move Closures with Threads".

**README.md** intro: closures are anonymous functions that can capture variables from the scope they're defined in; the three capture modes map to the `Fn`/`FnMut`/`FnOnce` traits. Further-info links: `book/ch13-01-closures.html`, `rust-by-example/fn/closures.html`.

| File | Learning goal | Broken-code design | Test | Hint (info.toml) |
|------|---------------|--------------------|------|------------------|
| `closures1.rs` | Basic closure syntax + type inference | Give a `let add = ???;` the learner must write `|a, b| a + b`; call it in tests. `test=true` | `assert_eq!(add(2,3),5)` | "Closure syntax is `|params| body`. Types are usually inferred from how you call it." |
| `closures2.rs` | Capture by reference vs `move` | A closure reads an outer `Vec`; a second usage forces the learner to choose borrow vs move. | assert closure returns expected sum; outer var still usable when borrowed | "A closure borrows by default. Add `move` to take ownership — but then you can't use the captured value afterwards. NOTES 'move closures'." |
| `closures3.rs` | `FnMut` — mutating captured state | Counter closure must be `let mut counter = ...; |...|` mutating a captured `count`. | call 3×, assert count==3 | "Mutating a captured variable needs `mut` on the closure binding and makes it `FnMut`." |
| `closures4.rs` | Closure as function parameter (HOF) | `fn apply<F: Fn(i32)->i32>(f: F, x: i32) -> i32` with a `// TODO` bound. | `apply(|n| n*2, 5)==10` | "Accept a closure with a trait bound `F: Fn(i32) -> i32`. See TRPL ch13.1." |
| `closures5.rs` | Returning a closure | `fn make_adder(x:i32) -> impl Fn(i32)->i32` (or `Box<dyn Fn>`). | `make_adder(3)(4)==7` | "Return `impl Fn(i32) -> i32`, or `Box<dyn Fn(i32) -> i32>` when the concrete type must be erased." |

---

## 4. Phase 2 — New module `25_patterns`

**Concept sources:** TRPL ch18 "Patterns and Matching"; RBE "Flow of Control" (match, guards, binding, `if let`, `while let`).

**README.md** intro: patterns destructure values; `match` must be exhaustive; guards/bindings/ranges refine arms; `if let`/`while let`/`let else` are ergonomic single-pattern forms.

| File | Learning goal | Design | Hint |
|------|---------------|--------|------|
| `patterns1.rs` | Match guards + ranges | classify an `i32` into Negative/Zero/`1..=9`/Big using `match` with `if` guards and `1..=9` ranges. | "Add a condition to an arm with `if`, and match inclusive ranges with `1..=9`." |
| `patterns2.rs` | Destructuring structs/tuples/nested | destructure a `Point{x,y}` and a nested enum in one `match`. | "Destructure in the pattern: `Point { x, y }`, and `Some((a, b))` for nested values. TRPL ch18.3." |
| `patterns3.rs` | `@` bindings + `_`/`..` ignoring | bind a matched value with `id @ 1..=5` while also ignoring fields with `..`. | "`name @ pattern` binds while testing; `..` ignores remaining fields/elements." |
| `patterns4.rs` | `if let` / `let else` / `while let` | refactor a verbose `match` into `if let Some(x) = ...` and drain a stack with `while let Some(top) = stack.pop()`. | "Use `if let`/`while let` for a single pattern; `let Some(x) = v else { return }` for early exit." |

---

## 5. Phase 3 — Extend `18_iterators` (next: `iterators6`–`iterators8`)

**Concept sources:** NOTES "Iterators" + the adapter tables (consuming vs adapter; `iter`/`iter_mut`/`into_iter`; `map`/`filter`/`fold`/`zip`/`chain`/`flat_map`/`partition`/`scan`/`enumerate`/`peekable`); TRPL ch13.2.

| File | Learning goal | Design | Hint |
|------|---------------|--------|------|
| `iterators6.rs` | `iter` vs `iter_mut` vs `into_iter` ownership | three small functions: sum via `iter()` (borrow), increment-in-place via `iter_mut()`, consume via `into_iter()`. | "`iter()` borrows (`&T`), `iter_mut()` gives `&mut T` (deref with `*`), `into_iter()` takes ownership. NOTES 'Which to choose'." |
| `iterators7.rs` | `fold` / `scan` / `sum`/`product` consuming adapters | reimplement factorial with `fold`/`product`, running totals with `scan`. | "`fold(init, |acc, x| ...)` collapses to one value; `scan` yields each intermediate. NOTES consuming-adapter table." |
| `iterators8.rs` | adapter chains → `collect` into Vec/HashMap | `zip` two slices, `enumerate`, `filter`/`map`, `partition` evens/odds, `collect::<HashMap<_,_>>()`. | "Adapters are lazy until `collect()`. Turbofish the target: `.collect::<HashMap<_, _>>()`. NOTES adapter tables." |

(README: add a `## Further information` link to `book/ch13-02-iterators.html`.)

---

## 6. Phase 4 — Extend `15_traits` (`traits6`–`traits8`) and `14_generics` (`generics3`–`generics4`)

**Concept sources:** NOTES "Traits" (default impl, trait-as-parameter, trait bounds, multiple bounds) + "Generics"; TRPL ch10.1–10.2 & ch19.3 (operator overloading); RBE Traits/Generics.

### `15_traits`
| File | Goal | Design | Hint |
|------|------|--------|------|
| `traits6.rs` | Default method implementation | `trait Summary { fn summarize(&self) -> String { /* default */ } }`; one type overrides, one uses default. | "A trait method can have a default body; `impl Summary for T {}` then inherits it. NOTES 'Trait default implementation'." |
| `traits7.rs` | Trait objects `Box<dyn Trait>` | a `Vec<Box<dyn Summary>>` of mixed concrete types iterated polymorphically. | "Store heterogeneous types behind `Box<dyn Summary>`; call through the trait at runtime (dynamic dispatch). TRPL ch17.2." |
| `traits8.rs` | Operator overloading (`std::ops::Add`) | implement `Add` for a `Point` so `p1 + p2` works. | "Implement `impl Add for Point { type Output = Point; fn add(self, other) {...} }`. TRPL ch19.3." |

### `14_generics`
| File | Goal | Design | Hint |
|------|------|--------|------|
| `generics3.rs` | Generic struct + generic `impl` | `struct Wrapper<T> { value: T }` with a generic method; instantiate with two types. | "Declare the parameter on both the struct and the impl: `impl<T> Wrapper<T>`. TRPL ch10.1." |
| `generics4.rs` | Trait-bounded generic fn + `where` | generic `largest<T: PartialOrd + Copy>` (NOTES example) rewritten with a `where` clause. | "Bound the type so the operation is allowed: `where T: PartialOrd + Copy`. NOTES 'largest' example." |

---

## 7. Phase 5 — Extend `19_smart_pointers` (`refcell1`, `deref1`, `drop1`, `cons_list1`)

**Concept sources:** TRPL ch15 (Box recursive types, `Deref`, `Drop`, `Rc`+`RefCell`); RBE Std misc (RAII/Drop); NOTES heap/Box.

| File | Goal | Design | Hint |
|------|------|--------|------|
| `cons_list1.rs` | Recursive type with `Box` | implement `enum List { Cons(i32, Box<List>), Nil }` + a `len`/`sum`; matches RBE cons-list example. | "A recursive type needs a known size — wrap the tail in `Box<List>`. TRPL ch15.1 / RBE 'Box, stack and heap'." |
| `deref1.rs` | `Deref` trait | implement `Deref` for a `MyBox<T>` so `*b` works. | "Implement `Deref` with `type Target = T; fn deref(&self) -> &T`. TRPL ch15.2." |
| `drop1.rs` | `Drop` / RAII order | a guard type prints on drop; learner orders/`drop()`s to satisfy an assertion of drop order. | "`Drop::drop` runs at end of scope in reverse declaration order; force-early with `std::mem::drop(x)`. TRPL ch15.3." |
| `refcell1.rs` | Interior mutability `Rc<RefCell<T>>` | shared-mutable counter via `Rc<RefCell<i32>>`; mutate through `.borrow_mut()`. | "`RefCell` enforces borrow rules at RUNTIME; combine with `Rc` for shared ownership: `Rc<RefCell<T>>`. TRPL ch15.5." |

---

## 8. Phase 6 — Extend `20_threads` (`channels1`, `channels2`, `move_threads1`)

**Concept sources:** NOTES "Message passing" (the full mpsc worked example + the `drop(tx)` gotcha + the "sum 1..10^8 across cores" assignment) and "Using move Closures with Threads"; TRPL ch16.2.

| File | Goal | Design | Hint |
|------|------|--------|------|
| `move_threads1.rs` | `move` closure into `thread::spawn` | spawn a thread that uses a captured `Vec`; learner adds `move`; `join().unwrap()` the handle. | "A spawned thread may outlive the borrow — take ownership with `move ||`. After moving, the value is gone from the parent. NOTES 'move closures with threads'." |
| `channels1.rs` | Basic `mpsc` send/recv | `let (tx, rx) = mpsc::channel();` send a `String` from a thread, `recv()` in main, handle the `Result`. | "`mpsc` = multi-producer, single-consumer. `tx.send(v).unwrap()`, `rx.recv()` returns `Result`. TRPL ch16.2." |
| `channels2.rs` | Multiple producers + `drop(tx)` + sum | N threads each send a partial sum; clone `tx` per thread; `drop(tx)` the original; collect via `for r in rx`. Mirrors NOTES "sum 1..10^8" assignment. | "Clone `tx` for each producer and `drop` the original, or the `for r in rx` loop blocks forever waiting on the never-dropped sender. NOTES 'The original tx never drops'." |

---

## 9. Phase 7 — Depth fill-ins (`16_lifetimes`, `17_tests`, `13_error_handling`)

**Concept sources:** NOTES lifetimes (fn + struct) & TRPL ch10.3; RBE Meta/Testing & TRPL ch11/ch14.2; TRPL ch9 error handling.

| File | Module | Goal | Hint |
|------|--------|------|------|
| `lifetimes4.rs` | `16_lifetimes` | Struct holding a reference: `struct Excerpt<'a> { part: &'a str }`. | "A struct that stores a reference needs a lifetime parameter: `struct Excerpt<'a>`. NOTES 'Struct with lifetimes' / TRPL ch10.3." |
| `lifetimes5.rs` | `16_lifetimes` | Multiple lifetimes / elision: `longest<'a>(x:&'a str,y:&'a str)->&'a str`. | "Tie inputs and output to the same lifetime `'a` so the borrow checker knows the result can't outlive either input." |
| `tests4.rs` | `17_tests` | Doc-tests: write a `///` doc comment with a runnable ```` ```rust ```` example that `cargo test` executes. | "Code fences in `///` docs are compiled and run as tests. RBE 'Documentation' / TRPL ch14.2." |
| `errors7.rs` | `13_error_handling` | Custom error `enum` + `From` + `Box<dyn Error>` + `?` chaining. | "Define your own error enum, implement `From<...>` so `?` can convert, or return `Box<dyn std::error::Error>`. TRPL ch9.2." |

---

## 10. Phase 8 — Capstone module `26_capstone` (+ optional `27_crates`)

**Concept sources:** TRPL ch12 (minigrep CLI: read input, search lines, case-insensitive) and ch20 (thread-pool); NOTES "sum 1..10^8 with all cores"; RBE Std misc.

### `26_capstone`
| File | Goal | Design (test-driven, no real CLI I/O) | Hint |
|------|------|----------------------------------------|------|
| `word_count1.rs` | minigrep-style search/count | `fn search<'a>(query:&str, contents:&'a str) -> Vec<&'a str>` returning matching lines; plus a case-insensitive variant. Tested with embedded `contents`. | "Iterate `contents.lines()`, keep lines that `contains(query)`; for case-insensitive, compare `to_lowercase()`. TRPL ch12.3–12.4." |
| `parallel_sum1.rs` | thread-pool / divide-and-conquer | split `1..=N` across threads, each returns a partial via `mpsc` or `JoinHandle`, combine. Assert total. | "Split the range, spawn a thread per chunk with `move`, collect partials and add them. NOTES 'sum 1 - 10^8 using all cores'." |

### `27_crates` (OPTIONAL — pulls network dependencies; gate behind a note in the PR)
- `rand1.rs`: use `rand` to draw a number and assert it falls in range (NOTES "cargo add rand"). Requires adding `rand` to `[dependencies]` in `Cargo.toml`.
- `chrono1.rs`: parse/format a date with `chrono` (NOTES "cargo add chrono").
- **Caveat to implement:** Rustlings' `Cargo.toml` currently has no external deps and CI may run offline. Mark these `skip_check_unsolved` is NOT enough — they need real deps. Recommend implementing this module ONLY if offline/CI constraints allow; otherwise document and skip.

---

## 10A. Full coverage audit (every PDF topic → status)

> Second-pass sweep of **all three PDFs** against (a) the existing exercises *as actually written* and (b) Phases 1–8. Anything that was in neither is captured in **Phase 9 / Phase 10** below. Topics that genuinely cannot be exercised here are listed in Section 10D so nothing is silently dropped.

**Verified facts from reading the real exercise files (not just module names):**
- `20_threads` **already** teaches `mpsc::channel` (threads2) and `Arc` + leads to `Mutex` (threads3). → *Correction:* Phase 6's `channels1` is largely redundant; keep only the **multi-producer + `drop(tx)` gotcha** (`channels2`) and the **`move`-into-thread pitfall** (`move_threads1`), and add `Arc<Mutex>` only if threads3 doesn't already.
- `13_error_handling/errors5` uses `impl fmt::Display` *incidentally* for an error — it does **not** teach formatted printing / `Display` / `Debug` as a topic. → **GAP** (Phase 9).
- There is **no** `loop {}` / `while` / labeled-break / break-with-value teaching anywhere. → **GAP** (Phase 9).
- `04_primitive_types` teaches tuples/arrays but there is **no dedicated slices exercise** (`&v[1..3]`, string slices, `first_word`). → **GAP** (Phase 9).
- **No exercise implements the `Iterator` trait** (`type Item` + `fn next`). → **GAP** (Phase 10).
- **No `Weak<T>` / reference-cycle** exercise (smart_pointers has only arc/box/cow/rc). → **GAP** (Phase 10).
- 18 files *use* `#[derive(...)]` but **none teach the derivable traits** (Clone/Copy/PartialEq/Eq/Hash/Default/Ord). → **GAP** (Phase 10).

| PDF topic | Source | Status |
|-----------|--------|--------|
| Formatted print, `Display`, `Debug`, format specifiers | RBE ch1; TRPL ch5 | **GAP → Phase 9 (`28_formatting`)** |
| `loop`/`while`/`for`, loop labels, `break` with value | RBE ch8; TRPL ch3.5; NOTES "Loops" | **GAP → Phase 9 (`29_flow_control`)** |
| Slices (array & string), slice params | TRPL ch4.3; RBE ch2; NOTES "strings vs slices" | **GAP → Phase 9 (`30_slices`)** |
| Implementing the `Iterator` trait | TRPL ch13.2; RBE ch16 | **GAP → Phase 10 (`iterators9`)** |
| `Weak<T>` & reference cycles | TRPL ch15.6 | **GAP → Phase 10 (`weak1`)** |
| `derive` & derivable traits | RBE ch16; TRPL App. C | **GAP → Phase 10 (`31_derive`)** |
| Associated types / supertraits | TRPL ch19.2; RBE ch14/16 | **GAP (optional) → Phase 10 (`traits9`)** |
| `macro_rules!` repetition/DSL/variadics | RBE ch17.4–17.6 | partial (21_macros has 4) → **optional Phase 10 (`macros5`)** |
| Type aliasing (`type`), newtype pattern | RBE ch5; TRPL ch19.3 | optional → folded into `31_derive`/`traits9` notes |
| Closures, capture, `Fn`/`FnMut`/`FnOnce` | TRPL ch13.1 | covered → Phase 1 |
| Patterns (guards/`@`/ranges/`if let`) | TRPL ch18 | covered → Phase 2 |
| Iterator adapters | NOTES; TRPL ch13.2 | covered → Phase 3 |
| Traits depth, operator overload, `dyn` | TRPL ch10/17/19 | covered → Phase 4 |
| Box/Deref/Drop/RefCell/cons-list | TRPL ch15 | covered → Phase 5 |
| Channels, `move` threads, `Arc<Mutex>` | TRPL ch16; NOTES | covered → existing `20_threads` + Phase 6 |
| Lifetimes (struct/multiple), doc-tests, custom errors | TRPL ch9/10/14 | covered → Phase 7 |
| minigrep CLI, thread-pool/parallel sum | TRPL ch12/ch20; NOTES | covered → Phase 8 |
| `rand` / `chrono` external crates | NOTES; RBE ch11/12 | covered → Phase 8 (`27_crates`, optional) |

---

## 10B. Phase 9 — Foundational gaps (new modules `28_formatting`, `29_flow_control`, `30_slices`)

> These are **beginner** topics. Module dir numbers are high (appended), but **place them early in `info.toml`** so the learning order is right (rustlings sequences by `info.toml` order, not dir number): put `28_formatting` after `04_primitive_types`, `29_flow_control` after `03_if`, `30_slices` after `06_move_semantics`.

### `28_formatting` — formatted printing & `std::fmt`  (RBE ch1; TRPL ch5.2)
| File | Goal | Design | Hint |
|------|------|--------|------|
| `formatting1.rs` | format specifiers | fix `println!`/`format!` calls using positional `{0}`, named `{name}`, width/precision `{:>8}`/`{:.2}`, fill/align, and `{:?}` vs `{:#?}`. | "`{:.2}` sets precision, `{:>8}` right-aligns to width 8, `{name}` uses a named arg, `{:#?}` is pretty Debug. RBE 'Formatted print'." |
| `formatting2.rs` | derive + manual `Debug` | a struct that won't print with `{:?}` until `#[derive(Debug)]` is added; then pretty-print with `{:#?}`. | "`{:?}` needs the `Debug` trait — add `#[derive(Debug)]`. TRPL ch5.2." |
| `formatting3.rs` | implement `fmt::Display` | `impl fmt::Display for Point` so `println!("{}", p)` prints `(x, y)`. | "Implement `fmt::Display` with `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, \"({}, {})\", self.x, self.y) }`. RBE 'Display'." |

### `29_flow_control` — loops  (RBE ch8; TRPL ch3.5; NOTES "Loops")
| File | Goal | Design | Hint |
|------|------|--------|------|
| `flow1.rs` | `loop` + `break` with value | a `loop` that increments a counter and `break counter * 2;` returns a value into a `let`. | "`loop` can return a value: `let x = loop { ... break v; };`. TRPL ch3.5." |
| `flow2.rs` | `while` / `while let` | drain a stack with `while let Some(x) = stack.pop()` and a `while` countdown. | "`while cond {}` repeats; `while let Some(x) = it.next() {}` loops until the pattern fails. RBE 'while / while let'." |
| `flow3.rs` | `for` ranges + labeled break | nested `for` over `1..=n`; use a `'outer:` label to `break 'outer` from the inner loop. | "Label loops with `'name:` and target them: `break 'outer;`. `for i in 1..=5` iterates inclusive ranges. RBE 'for and range'." |

### `30_slices` — array & string slices  (TRPL ch4.3; NOTES "strings vs slices")
| File | Goal | Design | Hint |
|------|------|--------|------|
| `slices1.rs` | array/vec slices | take `&v[1..4]`, pass `&[i32]` to a function that sums it; show a slice borrows a view, not a copy. | "A slice `&v[1..4]` borrows part of the collection; functions should accept `&[i32]` to work on both arrays and vecs. TRPL ch4.3." |
| `slices2.rs` | string slices + `first_word` | implement the classic `fn first_word(s: &str) -> &str` returning a `&str` slice up to the first space. | "Return a string slice `&s[0..i]`; iterate `s.as_bytes()` to find the space. TRPL ch4.3 'String Slices'." |

---

## 10C. Phase 10 — Intermediate/Advanced gaps

### Extend `18_iterators` → `iterators9` — implement the `Iterator` trait  (TRPL ch13.2; RBE ch16)
- Build a `Counter { count: u32 }`; `impl Iterator for Counter { type Item = u32; fn next(&mut self) -> Option<u32> {...} }` counting 1..=5, then chain adapters (`zip`/`map`/`filter`/`sum`) on it.
- **Hint:** "Implementing `Iterator` means defining `type Item` and `fn next(&mut self) -> Option<Self::Item>`; once you do, all adapter methods come for free. TRPL ch13.2."

### Extend `19_smart_pointers` → `weak1` — `Weak<T>` & reference cycles  (TRPL ch15.6)
- A parent/child tree where children hold `Rc<Node>` to peers and a `Weak<Node>` back-reference to the parent (`Rc::downgrade`), avoiding a leak; upgrade with `.upgrade()`.
- **Hint:** "A child pointing back to its parent with `Rc` creates a cycle that never frees. Use `Weak<T>` (`Rc::downgrade`) for the back-edge and `.upgrade()` to access it. TRPL ch15.6."

### New module `31_derive` — derivable traits  (RBE ch16; TRPL App. C)
| File | Goal | Design | Hint |
|------|------|--------|------|
| `derive1.rs` | Clone/Copy/PartialEq/Debug | a struct used in ways that require `Clone`+`Copy` (assignment after use) and `PartialEq` (`==`) and `Debug` (`{:?}`) — learner adds the right `#[derive(...)]`. | "Add `#[derive(Debug, Clone, Copy, PartialEq)]`. `Copy` lets values be used after assignment; `PartialEq` enables `==`. TRPL App. C." |
| `derive2.rs` | Default + PartialOrd/Ord + Hash | build a value with `Default::default()`, sort a `Vec` (needs `Ord`), and use the type as a `HashMap` key (needs `Eq + Hash`). | "`#[derive(Default)]` gives `::default()`; sorting needs `PartialOrd + Ord`; `HashMap` keys need `Eq + Hash`. RBE 'derive'." |

### OPTIONAL (only if you want full ch19/ch17 coverage)
- `15_traits/traits9.rs` — **associated types & supertraits**: a trait with `type Output;` and a `trait Student: Person {}` supertrait. (TRPL ch19.2)
- `21_macros/macros5.rs` — **repetition / variadic macro**: `macro_rules!` with `$( $x:expr ),*` building a `Vec`/`min!`. (RBE ch17.4–17.6)
- `17_tests/tests5.rs` — note: integration tests need a `tests/` dir, which doesn't fit the single-file model; cover via doc-test (`tests4`) instead.

---

## 10D. Deliberately NOT added (cannot/should-not be exercised here — listed so nothing is silently dropped)

| Topic | Source | Why excluded |
|-------|--------|--------------|
| **Unsafe Rust** (raw pointers, `unsafe fn`, static mut, `union`, inline asm) | TRPL ch19.1; RBE ch22 | Repo `Cargo.toml` sets `unsafe_code = "forbid"`. Adding it would violate the project's own lint and Rustlings' philosophy. |
| **FFI** (calling C) | RBE ch20.10 | Requires `unsafe` + external toolchain. |
| **Cargo operational** (publishing to crates.io, workspaces, release profiles, build scripts, `cargo install`) | TRPL ch14; RBE ch11/12 | Operational/CLI knowledge, not a single-file exercise. Partially touched by optional `27_crates`. |
| **Integration tests** (`tests/` directory) | TRPL ch11.3; RBE ch21 | Multi-file layout incompatible with Rustlings' one-file-per-exercise model. Doc-tests (`tests4`) cover the spirit. |
| **Attributes/cfg, conditional compilation, raw identifiers, editions/compatibility** | RBE ch13/23; TRPL App. E | Niche tooling concerns; low learning value as standalone exercises. Mention in READMEs instead. |
| **`Send`/`Sync`, the never type `!`, DSTs, full OOP state pattern, full multithreaded web server** | TRPL ch16.4/ch19.3/ch17.3/ch20 | Conceptual or large-project material; the capstone (`parallel_sum1`) and `traits7` (`dyn`) give a representative taste. Add later as advanced capstones if desired. |

---

## 11. Cross-cutting acceptance criteria (every phase)

A phase is "done" only when ALL hold:

1. Each new exercise has all five artifacts (Section 1): exercise file, solution, `info.toml` entry **with a hint**, `Cargo.toml` bins (via `dev update`), and the module `README.md` updated.
2. The exercise **fails before** the fix and **passes after** (compile error or failing test by design — never green-on-arrival unless `skip_check_unsolved`).
3. Concept comments are present at the top of the exercise file and inline at each `// TODO:` (the user specifically wants rich teaching comments).
4. `rustlings dev check --require-solutions` exits 0.
5. `cargo fmt` clean; the repo lints in `Cargo.toml` (`unsafe_code = "forbid"`, `clippy::todo = "forbid"`, etc.) are respected — so no leftover `todo!()` in solutions.
6. `rustlings run <name>` and `rustlings hint <name>` both behave correctly for each new exercise.
7. Exercises are ordered so prerequisites precede dependents in `info.toml` (closures before iterator-closure exercises, etc.).

---

## 12. How to execute this plan with Claude Code

Work **one phase per Claude Code session** to stay within the context window. Recommended order: **Phase 0 → 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10** (0 is a hard prerequisite; 1 and 2 are prerequisites for the closure/pattern usage in later phases). Phases 9 and 10 are the gap-audit additions — 9 is foundational (do early if you want correct curriculum order in `info.toml`), 10 is intermediate/advanced.

**Per-session prompt template** (paste into Claude Code at the repo root `rustlings/`):

```
Read RUSTLINGS_EXPANSION_PLAN.md and implement <PHASE N> only.
Follow the exercise contract in Section 1 exactly (exercise file + solution +
info.toml hint entry + README update), then run:
  rustlings dev update
  rustlings dev check --require-solutions
  cargo fmt
Fix anything that fails. Show me `rustlings hint <one new exercise>` output and the
diff before finishing. Do NOT touch other phases.
```

**Concrete first two sessions:**

1. *Session A — Phase 0:* "Implement Phase 0 from RUSTLINGS_EXPANSION_PLAN.md: create the root `info.toml` covering all existing exercises (copy official hints from the embedded `rustlings-macros-6.5.0/info.toml`), then `rustlings dev update` and `rustlings dev check`. Confirm `rustlings hint enums2` works."
2. *Session B — Phase 1:* "Implement Phase 1 (`24_closures`) from the plan: 5 exercises closures1–5 with solutions, hints, and README. Validate with `rustlings dev check --require-solutions`."

**Verifying as you go (run yourself in this session with `! <cmd>`):**
```
rustlings dev check --require-solutions   # all green
rustlings run closures1                   # fails before fix
rustlings hint closures1                  # prints the hint from info.toml
rustlings list                            # new exercises appear in order
```

**Commit guidance:** one commit (or PR) per phase, e.g. `feat(exercises): add 24_closures module (Phase 1)`, so each chunk is reviewable independently.

---

## 13. Summary of what gets added

- **7 new modules:** `24_closures` (5), `25_patterns` (4), `26_capstone` (2), optional `27_crates` (2), `28_formatting` (3), `29_flow_control` (3), `30_slices` (2), `31_derive` (2).
- **Extensions to 8 existing modules:** iterators +4 (incl. `iterators9` Iterator-trait impl), traits +3 (+optional `traits9`), generics +2, smart_pointers +5 (incl. `weak1`), threads +3, lifetimes +2, tests +1, error_handling +1, macros +optional `macros5`.
- **~45 new exercises** (Phases 1–10), each with a teaching comment header, `// TODO` scaffolding, a passing solution, an `info.toml` hint, and updated README — every concept traced back to a specific chapter of `myRustNotes.pdf`, `Rust by examples.pdf`, or `The Rust Programming Language.pdf`.
- **Phases 9–10 close the full-audit gaps:** formatted printing/`Display`/`Debug`, loops (`loop`/`while`/`for`/labels), slices, implementing the `Iterator` trait, `Weak`/reference cycles, and derivable traits. Section 10D lists the few topics deliberately excluded (unsafe, FFI, cargo-publishing, integration tests) with reasons.
- **3 new graded checkpoint quizzes** (beyond upstream `quiz1`–`quiz3`): `quiz4` 🟢 easy (lifetimes + iterators, after `18_iterators`), `quiz5` 🟡 medium (`Arc` + threads/`mpsc` + parsing, after `23_conversions`), `quiz6` 🔴 hard (implementing `Iterator` + closures + patterns + derive, after `31_derive`). Each ships the same four pieces as a normal exercise (broken file, solution, `info.toml` hint, README entry).

---

## 14. Copy-paste prompts (one Claude Code session per phase)

### Generic template (change the number + the per-phase line)

```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE <N> only — nothing from any other phase.

Follow the exercise contract in Section 1 exactly for every new exercise:
  1. exercises/<dir>/<name>.rs  — broken-on-purpose, teaching comment header + inline // TODO
  2. solutions/<dir>/<name>.rs  — complete, compiles, passes
  3. info.toml [[exercises]] entry WITH a hint (this is the hint mechanism)
  4. Cargo.toml bins — via `rustlings dev update`, don't hand-edit
  5. exercises/<dir>/README.md — concept intro + Further information links

Then validate:
  rustlings dev update
  rustlings dev check --require-solutions
  cargo fmt

Every exercise must FAIL before the fix and PASS after. Respect the Cargo.toml lints
(no leftover todo!(), unsafe forbidden). When done, show me:
  - `rustlings run <first-new-exercise>`   (proves it fails unsolved)
  - `rustlings hint <first-new-exercise>`  (proves the hint loads)
  - the full diff
Do NOT touch other phases.
```

### Phase 0 — infrastructure (do first)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 0 only.
Create the root info.toml: format_version=1, welcome/final messages, and one
[[exercises]] block for EVERY existing exercise (00→23 + quizzes) in order, copying
the official hints out of the embedded ~/.cargo/.../rustlings-macros-6.5.0/info.toml.
Then run `rustlings dev update` and `rustlings dev check --require-solutions`.
Confirm by showing `rustlings hint enums2` and `rustlings list`. Don't add new exercises yet.
```

### Phase 1 — 24_closures (closures1–5)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 1 only: new module 24_closures,
exercises closures1–closures5 per the Section 3 table (syntax/inference, capture+move,
FnMut, closure-as-param HOF, returning a closure). Each needs exercise file + solution +
info.toml hint + README. Validate: rustlings dev update && rustlings dev check
--require-solutions && cargo fmt. Show `rustlings run closures1`, `rustlings hint
closures1`, and the diff. Don't touch other phases.
```

### Phase 2 — 25_patterns (patterns1–4)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 2 only: new module 25_patterns,
patterns1–patterns4 per Section 4 (match guards+ranges, destructuring, @ bindings + ../_
ignoring, if let/let else/while let). Full contract per Section 1. Validate with rustlings
dev update && rustlings dev check --require-solutions && cargo fmt. Show `rustlings run
patterns1`, `rustlings hint patterns1`, and the diff. Don't touch other phases.
```

### Phase 3 — extend 18_iterators (iterators6–8)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 3 only: add iterators6–iterators8 to
18_iterators per Section 5 (iter vs iter_mut vs into_iter; fold/scan/product; adapter
chains zip/enumerate/filter/partition → collect into Vec/HashMap). Append their info.toml
entries AFTER iterators5. Validate: rustlings dev update && rustlings dev check
--require-solutions && cargo fmt. Show `rustlings run iterators6`, `rustlings hint
iterators6`, and the diff. Don't touch other phases.
```

### Phase 4 — extend 15_traits (traits6–8) + 14_generics (generics3–4)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 4 only: add traits6–traits8 (default
impl, Box<dyn Trait> objects, operator overload Add) and generics3–generics4 (generic
struct+impl, where-bounds) per Section 6. Full contract per Section 1; order them after the
existing exercises in info.toml. Validate: rustlings dev update && rustlings dev check
--require-solutions && cargo fmt. Show `rustlings run traits6`, `rustlings hint traits6`,
and the diff. Don't touch other phases.
```

### Phase 5 — extend 19_smart_pointers (cons_list1, deref1, drop1, refcell1)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 5 only: add cons_list1, deref1, drop1,
refcell1 to 19_smart_pointers per Section 7 (Box recursive type, Deref trait, Drop/RAII
order, Rc<RefCell> interior mutability). Full contract per Section 1. Validate: rustlings
dev update && rustlings dev check --require-solutions && cargo fmt. Show `rustlings run
cons_list1`, `rustlings hint cons_list1`, and the diff. Don't touch other phases.
```

### Phase 6 — extend 20_threads (move_threads1, channels1, channels2)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 6 only: add move_threads1, channels1,
channels2 to 20_threads per Section 8 (move closure into spawn; mpsc send/recv; multiple
producers + drop(tx) + combined sum — the NOTES "sum 1..10^8" pattern). Full contract per
Section 1. Validate: rustlings dev update && rustlings dev check --require-solutions &&
cargo fmt. Show `rustlings run channels1`, `rustlings hint channels2`, and the diff. Don't
touch other phases.
```

### Phase 7 — depth fill-ins (lifetimes4–5, tests4, errors7)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 7 only: add lifetimes4–5 (struct
holding a ref; multiple lifetimes/elision), tests4 (doc-tests in /// comments), and errors7
(custom error enum + From + Box<dyn Error> + ? chaining) per Section 9. Place each after the
existing exercises in their modules. Validate: rustlings dev update && rustlings dev check
--require-solutions && cargo fmt. Show `rustlings run lifetimes4`, `rustlings hint
errors7`, and the diff. Don't touch other phases.
```

### Phase 8 — 26_capstone (+ optional 27_crates)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 8 only: new module 26_capstone with
word_count1 (minigrep-style search/count, test-driven, no real CLI I/O) and parallel_sum1
(divide-and-conquer sum across threads) per Section 10. SKIP the optional 27_crates module
unless I confirm network deps are OK — if you skip it, say so. Full contract per Section 1.
Validate: rustlings dev update && rustlings dev check --require-solutions && cargo fmt.
Show `rustlings run word_count1`, `rustlings hint parallel_sum1`, and the diff.
```

### Phase 9 — foundational gaps (28_formatting, 29_flow_control, 30_slices)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 9 only (Section 10B): three new modules
28_formatting (formatting1-3: format specifiers, derive Debug, impl Display), 29_flow_control
(flow1-3: loop+break-with-value, while/while let, for+labeled break), and 30_slices
(slices1-2: array/vec slices, string slices + first_word). Full contract per Section 1.
IMPORTANT: order them EARLY in info.toml per Section 10B (28 after 04, 29 after 03, 30 after
06) so the curriculum order is correct — dir numbers stay 28/29/30. Validate: rustlings dev
update && rustlings dev check --require-solutions && cargo fmt. Show `rustlings run flow1`,
`rustlings hint formatting3`, and the diff. Don't touch other phases.
```

### Phase 10 — intermediate/advanced gaps (iterators9, weak1, 31_derive)
```
Read RUSTLINGS_EXPANSION_PLAN.md and implement PHASE 10 only (Section 10C): add iterators9
to 18_iterators (implement the Iterator trait: type Item + fn next, then chain adapters), add
weak1 to 19_smart_pointers (Weak<T> + reference cycles, Rc::downgrade/upgrade), and create
new module 31_derive (derive1-2: Clone/Copy/PartialEq/Debug, then Default/Ord/Hash). The
OPTIONAL extras (traits9 associated types/supertraits, macros5 repetition/DSL) — implement
ONLY if I confirm; otherwise skip and say so. Full contract per Section 1. Validate: rustlings
dev update && rustlings dev check --require-solutions && cargo fmt. Show `rustlings run
iterators9`, `rustlings hint weak1`, and the diff. Don't touch other phases.
```
