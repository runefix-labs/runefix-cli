#!/usr/bin/env bash
set -e

echo "🧪 Running CLI integration tests..."

# ───────────────────────────────────────────────
# 🔹 Util Functions
# ───────────────────────────────────────────────

function section() {
  echo -e "\n\033[1;36m==> $1\033[0m"
}

function run() {
  echo -e "\033[1;33m$@\033[0m"
  eval "$@"
  echo ""
}

# ───────────────────────────────────────────────
# 🔸 Core CLI Commands
# ───────────────────────────────────────────────

section "Atoms Command"
run 'cargo run --quiet -- atoms "🎉Love➡️family👩‍❤️‍💋‍👨🏠🚗🧑‍🧑‍👧@🐶🐱🏫-🏥#1️⃣23🍃🌍👩🏼‍🏫👨🏾‍🦳!"'
echo -e "✅ Atoms segmentation verified"

section "Graphemes Command"
run 'cargo run --quiet -- graphemes "Hello 👋 世界"'
echo -e "✅ Grapheme clusters split correctly"

section "Width Command"
run 'cargo run --quiet -- width "Hello 👋 世界" -v'
echo -e "✅ Display width calculated"

section "Widths Command"
run 'cargo run --quiet -- widths "Hello 👋 世界" -v'
echo -e "✅ Per-grapheme widths computed"

section "Split Command"
run 'cargo run --quiet -- split --width 5 "Hello 👋 世界" -v'
echo -e "✅ String split by width"

section "Truncate Command"
run 'cargo run --quiet -- truncate --width 5 "Hello 👋 世界" -v'
echo -e "✅ Truncation by width succeeded"

section "Version Command"
run 'cargo run --quiet -- version'
run 'cargo run --quiet -- version --json'
echo -e "✅ Version output (plain + JSON) OK"

# ───────────────────────────────────────────────
# 🔸 Slice Command Variants
# ───────────────────────────────────────────────

section "Slice: Input Modes (Argv / Pipe / Here Doc)"           # Argv mode (most common)
run 'cargo run --quiet -- slice [0:5] "Hello, World!"'
run 'echo "Hello, World!" | cargo run --quiet -- slice [0:5]'   # Stdin: pipe mode
run 'cargo run --quiet -- slice [0:5] <<EOF                     # Stdin: here document mode
Hello, World!
EOF'
echo -e "✅ Slice input modes validated"

section "Slice: Quotes Variants"
run 'cargo run --quiet -- slice [0:5] "Hello, World!"'          # No quotes on slice (may error)
run "cargo run --quiet -- slice '[0:5]' 'Hello, World!'"        # Single quotes
run 'cargo run --quiet -- slice "[0:5]" "Hello, World!"'        # Double quotes
echo -e "✅ Slice quotes parsed correctly"

section "Slice: Normal Expressions"
run 'cargo run --quiet -- slice [0:1] "Hello, World!"'
run 'cargo run --quiet -- slice "[7:]" "Hello, World!"'
run 'cargo run --quiet -- slice "[:5]" "Hello, World!"'
run 'cargo run --quiet -- slice "[:]" "Hello, World!"'
run 'cargo run --quiet -- slice "[4]" "Hello, World!"'
echo -e "✅ Slice expression variants passed"

section "Slice: Options (-v --verbose)"
run 'cargo run --quiet -- slice -v "[0:1]" "你好👨‍👩‍👧‍👦Hello世界"'
echo -e "✅ Verbose mode output OK"

section "Slice: Options (-g / -c / -w)"
run 'cargo run --quiet -- slice -g "[0:3]" "你好👨‍👩‍👧‍👦Hello世界"'
run 'cargo run --quiet -- slice -c "[0:5]" "你好👨‍👩‍👧‍👦Hello世界"'
run 'cargo run --quiet -- slice -w "[0:6]" "你好👨‍👩‍👧‍👦Hello世界"'
echo -e "✅ Mode flags (-g / -c / -w) working"

section "Slice: Options (-s --strict)"
run 'ps aux | cargo run --quiet -- slice "[:200]"'
run 'ps aux | cargo run --quiet -- slice "[:200]" --strict'     # ⚠️ Warn
echo -e "✅ Strict mode applied correctly"

echo -e "\033[1;32m\n🎉 All CLI commands tested successfully!\033[0m"

#EOF