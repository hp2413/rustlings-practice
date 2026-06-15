#!/usr/bin/env bash
#
# reset.sh — restore every exercise to its original, UNSOLVED state.
#
# Use this to get a clean practice slate: right after cloning (the author may
# have solved some exercises), or any time you want to start over. It only
# rewrites files under exercises/ and clears your personal progress file.
# Nothing is lost — the complete reference answers always live in solutions/.
#
# Usage:   ./reset.sh
#
set -euo pipefail
cd "$(dirname "$0")"

# The `rustlings init` commit holds the pristine, unsolved OFFICIAL exercises
# (modules 00–23 + quizzes). Restoring from it guarantees a clean baseline even
# if some of those exercises were solved and committed.
INIT_COMMIT="543ed8cf6dc8566b103445455a3dddfef043279c"

echo "Resetting all exercises to their original unsolved state..."

# 1) Restore every tracked file under exercises/ to the committed version
#    (undoes any edits you have made to custom/expansion exercises).
git checkout HEAD -- exercises/ 2>/dev/null || true

# 2) Force the official exercises back to their pristine init state, in case the
#    repo was committed with some of them already solved.
git ls-tree -r --name-only "$INIT_COMMIT" \
  | grep -E '^exercises/.*\.rs$' \
  | while IFS= read -r f; do
      git checkout "$INIT_COMMIT" -- "$f"
    done

# 3) Clear personal progress so Rustlings starts from the first exercise.
rm -f .rustlings-state.txt

# 4) Regenerate Cargo.toml's [[bin]] list from info.toml.
if command -v rustlings >/dev/null 2>&1; then
  rustlings dev update >/dev/null
fi

echo "✓ Done. Every exercise is unsolved again."
echo "  Start practising with:  rustlings"
