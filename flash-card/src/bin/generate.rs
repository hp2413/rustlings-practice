//! Card generator: parse the rustlings repo into `data/cards.json`.
//!
//! Reads `../info.toml` for the canonical exercise order, `name`, `dir`, and
//! `hint`; pulls the question from each `../exercises/<dir>/<name>.rs`, the
//! answer from `../solutions/<dir>/<name>.rs` (falling back to synthesized
//! guidance when the solution is still a stub), and the per-module concept
//! summary + doc links from `../exercises/<dir>/README.md`.
//!
//! Run from the `flash-card/` directory:
//!
//! ```text
//! cargo run --bin generate
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use flash_card::{AnswerKind, Card, Deck};
use serde::Deserialize;

/// Marker present in unsolved solution stubs shipped by rustlings.
const STUB_MARKER: &str = "DON'T EDIT THIS SOLUTION FILE";

/// One `[[exercises]]` block in `info.toml` (only the fields we need).
#[derive(Debug, Deserialize)]
struct ExerciseInfo {
    name: String,
    dir: String,
    /// rustlings runs `cargo test` for the exercise unless this is `false`.
    #[serde(default = "default_true")]
    test: bool,
    hint: String,
}

#[derive(Debug, Deserialize)]
struct InfoToml {
    exercises: Vec<ExerciseInfo>,
}

fn default_true() -> bool {
    true
}

/// Parsed, cached metadata for one module README.
#[derive(Debug, Clone, Default)]
struct ModuleMeta {
    title: String,
    concept_summary: String,
    doc_links: Vec<String>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    // Anchor every path to this crate's directory so the command works no
    // matter what the current working directory is.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .ok_or("flash-card/ has no parent directory")?
        .to_path_buf();
    let out_path = manifest_dir.join("data/cards.json");

    let info_path = repo_root.join("info.toml");
    let info_src = read(&info_path)?;
    let info: InfoToml =
        toml::from_str(&info_src).map_err(|e| format!("parsing {}: {e}", info_path.display()))?;

    let mut module_cache: HashMap<String, ModuleMeta> = HashMap::new();
    let mut cards: Vec<Card> = Vec::with_capacity(info.exercises.len());
    let (mut solution_count, mut guidance_count) = (0usize, 0usize);

    for (order, ex) in info.exercises.iter().enumerate() {
        // Module metadata (parsed once per dir, then reused).
        let meta = match module_cache.get(&ex.dir) {
            Some(m) => m.clone(),
            None => {
                let readme_path = repo_root.join("exercises").join(&ex.dir).join("README.md");
                let meta = match read(&readme_path) {
                    Ok(src) => parse_module_meta(&src, &ex.dir),
                    Err(_) => ModuleMeta {
                        title: title_from_dir(&ex.dir),
                        ..Default::default()
                    },
                };
                module_cache.insert(ex.dir.clone(), meta.clone());
                meta
            }
        };

        // Question comes from the exercise source. rustlings exercises are
        // solved in-place, so once a module has been worked through, the live
        // file under `../exercises/` holds the learner's answer rather than the
        // starter code. A pristine snapshot vendored under `pristine/<dir>/`
        // takes precedence so the card always shows the original starter.
        let pristine_path = manifest_dir
            .join("pristine")
            .join(&ex.dir)
            .join(format!("{}.rs", ex.name));
        let exercise_path = repo_root
            .join("exercises")
            .join(&ex.dir)
            .join(format!("{}.rs", ex.name));
        let exercise_src_path = if pristine_path.exists() {
            &pristine_path
        } else {
            &exercise_path
        };
        let exercise_code = read(exercise_src_path)?.trim_end().to_string();
        let mut question = extract_question(&exercise_code);
        if question.is_empty() {
            question = format!(
                "Fix `{}` so it compiles and its tests pass.",
                ex.name
            );
        }

        // Answer: real solution code, or synthesized guidance for stubs.
        let solution_path = repo_root
            .join("solutions")
            .join(&ex.dir)
            .join(format!("{}.rs", ex.name));
        let solution_code = read(&solution_path)?;
        let (answer, answer_kind) = if solution_code.contains(STUB_MARKER) {
            guidance_count += 1;
            (
                guidance_answer(&meta.title, &meta.concept_summary, &ex.hint),
                AnswerKind::Guidance,
            )
        } else {
            solution_count += 1;
            (solution_code.trim_end().to_string(), AnswerKind::Solution)
        };

        cards.push(Card {
            id: ex.name.clone(),
            order,
            module: ex.dir.clone(),
            module_title: meta.title.clone(),
            concept_summary: meta.concept_summary.clone(),
            doc_links: meta.doc_links.clone(),
            question,
            exercise_code,
            hint: ex.hint.trim().to_string(),
            answer,
            answer_kind,
            is_quiz: ex.dir == "quizzes",
            has_tests: ex.test,
        });
    }

    validate(&cards)?;

    let deck = Deck::new(chrono::Utc::now().to_rfc3339(), cards);
    let json = serde_json::to_string_pretty(&deck)
        .map_err(|e| format!("serializing deck: {e}"))?;
    std::fs::write(&out_path, format!("{json}\n"))
        .map_err(|e| format!("writing {}: {e}", out_path.display()))?;

    println!(
        "Wrote {} cards ({} solution / {} guidance) -> {}",
        deck.count,
        solution_count,
        guidance_count,
        out_path.display()
    );
    Ok(())
}

/// Sanity-check the deck before writing it out.
fn validate(cards: &[Card]) -> Result<(), String> {
    for (i, card) in cards.iter().enumerate() {
        if card.order != i {
            return Err(format!(
                "card {} ({}) has order {} but should be {i}",
                i, card.id, card.order
            ));
        }
        if card.question.trim().is_empty() {
            return Err(format!("card {} has an empty question", card.id));
        }
        if card.hint.trim().is_empty() {
            return Err(format!("card {} has an empty hint", card.id));
        }
        if card.answer.trim().is_empty() {
            return Err(format!("card {} has an empty answer", card.id));
        }
    }
    Ok(())
}

// --- Extraction helpers ------------------------------------------------------

/// Pull the teaching question from exercise source: the leading comment block
/// plus every `// TODO` comment block, decommented into plain prose.
fn extract_question(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let n = lines.len();
    let is_comment = |s: &str| s.trim_start().starts_with("//");
    let is_todo = |s: &str| {
        is_comment(s)
            && decomment(s)
                .trim_start()
                .to_ascii_lowercase()
                .starts_with("todo")
    };

    // Comment-line intervals worth keeping: the leading header + each TODO block.
    let mut intervals: Vec<(usize, usize)> = Vec::new();

    if n > 0 && is_comment(lines[0]) {
        let mut j = 0;
        while j < n && is_comment(lines[j]) {
            j += 1;
        }
        intervals.push((0, j));
    }

    let mut i = 0;
    while i < n {
        if is_todo(lines[i]) {
            let start = i;
            i += 1;
            while i < n && is_comment(lines[i]) && !is_todo(lines[i]) {
                i += 1;
            }
            intervals.push((start, i));
        } else {
            i += 1;
        }
    }

    // Merge overlapping/adjacent intervals (the header may itself be a TODO).
    intervals.sort_unstable();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (s, e) in intervals {
        match merged.last_mut() {
            Some(last) if s <= last.1 => last.1 = last.1.max(e),
            _ => merged.push((s, e)),
        }
    }

    merged
        .iter()
        .map(|&(s, e)| {
            let block = lines[s..e]
                .iter()
                .map(|l| decomment(l))
                .collect::<Vec<_>>()
                .join("\n");
            strip_todo_prefix(block.trim())
        })
        .collect::<Vec<_>>()
        .join("\n\n")
        .trim()
        .to_string()
}

/// Drop a leading `TODO`/`TODO:` marker so the question reads as an instruction.
fn strip_todo_prefix(block: &str) -> String {
    let t = block.trim_start();
    if t.len() >= 4 && t[..4].eq_ignore_ascii_case("todo") {
        let rest = t[4..].trim_start();
        let rest = rest.strip_prefix(':').unwrap_or(rest);
        return rest.trim_start().to_string();
    }
    block.to_string()
}

/// Strip a leading `//` (and one following space) from a comment line.
fn decomment(line: &str) -> String {
    let t = line.trim_start();
    let t = t.strip_prefix("//").unwrap_or(t);
    t.strip_prefix(' ').unwrap_or(t).to_string()
}

/// Parse a module README into its title, concept summary, and doc links.
fn parse_module_meta(readme: &str, dir: &str) -> ModuleMeta {
    ModuleMeta {
        title: extract_title(readme).unwrap_or_else(|| title_from_dir(dir)),
        concept_summary: extract_concept_summary(readme),
        doc_links: extract_doc_links(readme),
    }
}

/// First `# ` heading in the README.
fn extract_title(readme: &str) -> Option<String> {
    readme
        .lines()
        .find_map(|l| l.trim().strip_prefix("# ").map(|s| s.trim().to_string()))
}

/// First paragraph following the title heading, with markdown links flattened.
fn extract_concept_summary(readme: &str) -> String {
    let mut seen_title = false;
    let mut para: Vec<String> = Vec::new();
    for line in readme.lines() {
        let t = line.trim();
        if !seen_title {
            if t.starts_with("# ") {
                seen_title = true;
            }
            continue;
        }
        if t.is_empty() || t.starts_with('#') {
            if para.is_empty() {
                continue;
            }
            break;
        }
        para.push(t.to_string());
    }
    strip_md_links(&para.join(" ")).trim().to_string()
}

/// URLs from the README's "Further information" section.
fn extract_doc_links(readme: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut in_section = false;
    for line in readme.lines() {
        let t = line.trim();
        if let Some(heading) = t.strip_prefix("## ") {
            in_section = heading.to_ascii_lowercase().contains("further information");
            continue;
        }
        if t.starts_with("# ") {
            in_section = false;
            continue;
        }
        if in_section {
            for (_text, url) in md_links(line) {
                if url.starts_with("http") {
                    links.push(url);
                }
            }
        }
    }
    links
}

/// Replace every `[text](url)` markdown link with just `text`.
fn strip_md_links(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' {
            if let Some((text, _url, next)) = parse_md_link(&chars, i) {
                out.push_str(&text);
                i = next;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// Collect all `[text](url)` links on a line as `(text, url)` pairs.
fn md_links(s: &str) -> Vec<(String, String)> {
    let chars: Vec<char> = s.chars().collect();
    let mut links = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' {
            if let Some((text, url, next)) = parse_md_link(&chars, i) {
                links.push((text, url));
                i = next;
                continue;
            }
        }
        i += 1;
    }
    links
}

/// Parse a single `[text](url)` starting at `start` (which must point at `[`).
/// Returns `(text, url, index_after_link)`, or `None` if it isn't a link.
fn parse_md_link(chars: &[char], start: usize) -> Option<(String, String, usize)> {
    let mut j = start + 1;
    let mut text = String::new();
    while j < chars.len() && chars[j] != ']' {
        if chars[j] == '[' {
            return None; // nested bracket — not a simple link
        }
        text.push(chars[j]);
        j += 1;
    }
    if chars.get(j) != Some(&']') || chars.get(j + 1) != Some(&'(') {
        return None;
    }
    j += 2;
    let mut url = String::new();
    while j < chars.len() && chars[j] != ')' {
        url.push(chars[j]);
        j += 1;
    }
    if chars.get(j) != Some(&')') {
        return None;
    }
    Some((text, url, j + 1))
}

/// Title-case a module directory name, e.g. `06_move_semantics` -> `Move Semantics`.
fn title_from_dir(dir: &str) -> String {
    let without_prefix = dir
        .split_once('_')
        .filter(|(p, _)| p.chars().all(|c| c.is_ascii_digit()))
        .map(|(_, rest)| rest)
        .unwrap_or(dir);
    without_prefix
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Build a guidance answer for exercises whose solution is still a stub.
fn guidance_answer(module_title: &str, concept_summary: &str, hint: &str) -> String {
    let mut s = String::new();
    s.push_str(
        "No reference solution ships with this exercise yet — here's guidance to work it out yourself.\n\n",
    );
    if !concept_summary.is_empty() {
        s.push_str(&format!("Concept ({module_title}): {concept_summary}\n\n"));
    }
    s.push_str(&format!("How to approach it: {}", hint.trim()));
    s
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_from_inline_todo() {
        let code = "fn main() {\n    // TODO: Add the missing keyword.\n    let x = 5;\n}";
        assert_eq!(extract_question(code), "Add the missing keyword.");
    }

    #[test]
    fn question_merges_header_and_todo_without_duplication() {
        // Leading comment block that is itself a TODO must appear only once,
        // with the TODO marker stripped from the front.
        let code = "// TODO: keep going\n// even after it works.\n\nfn main() {}";
        assert_eq!(extract_question(code), "keep going\neven after it works.");
    }

    #[test]
    fn concept_summary_takes_first_paragraph_and_flattens_links() {
        let readme = "# Move Semantics\n\nAdapted from [Felix](https://example.com) -- thanks!\n\n## Further information\n";
        assert_eq!(
            extract_concept_summary(readme),
            "Adapted from Felix -- thanks!"
        );
    }

    #[test]
    fn doc_links_only_from_further_information() {
        let readme = "# T\n\nIntro [skip](https://nope.com)\n\n## Further information\n\n- [Book](https://doc.rust-lang.org/book/)\n";
        assert_eq!(
            extract_doc_links(readme),
            vec!["https://doc.rust-lang.org/book/".to_string()]
        );
    }

    #[test]
    fn title_from_dir_strips_numeric_prefix() {
        assert_eq!(title_from_dir("06_move_semantics"), "Move Semantics");
        assert_eq!(title_from_dir("quizzes"), "Quizzes");
        assert_eq!(title_from_dir("00_intro"), "Intro");
    }
}
