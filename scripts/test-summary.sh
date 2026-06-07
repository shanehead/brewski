#!/usr/bin/env bash

RUST_TMP=$(mktemp)
JS_TMP=$(mktemp)
RUST_EXIT=0
JS_EXIT=0

echo "── Rust tests ────────────────────────────────────────────────────────────────"
(cd src-tauri && cargo test --color always -- --color always 2>&1) | tee "$RUST_TMP"
RUST_EXIT=${PIPESTATUS[0]}

echo ""
echo "── Frontend tests ────────────────────────────────────────────────────────────"
bun run test 2>&1 | tee "$JS_TMP"
JS_EXIT=${PIPESTATUS[0]}

strip_ansi() { perl -pe 's/\x1b\[[0-9;]*[mGKH]//g'; }

RUST_PASSED=0
RUST_FAILED=0
while IFS= read -r line; do
    if [[ "$line" =~ ^"test result:" ]]; then
        [[ "$line" =~ [[:space:]]([0-9]+)" passed" ]] && RUST_PASSED=$((RUST_PASSED + ${BASH_REMATCH[1]}))
        [[ "$line" =~ [[:space:]]([0-9]+)" failed" ]] && RUST_FAILED=$((RUST_FAILED + ${BASH_REMATCH[1]}))
    fi
done < <(strip_ansi < "$RUST_TMP")

JS_PASSED=0
JS_FAILED=0
while IFS= read -r line; do
    if [[ "$line" =~ ^[[:space:]]*"Tests"[[:space:]] ]]; then
        [[ "$line" =~ ([0-9]+)" passed" ]] && JS_PASSED=${BASH_REMATCH[1]}
        [[ "$line" =~ ([0-9]+)" failed" ]] && JS_FAILED=${BASH_REMATCH[1]}
    fi
done < <(strip_ansi < "$JS_TMP")

rm -f "$RUST_TMP" "$JS_TMP"

GREEN=$'\033[32m'
RED=$'\033[31m'
BOLD=$'\033[1m'
RESET=$'\033[0m'

TOTAL_PASSED=$((RUST_PASSED + JS_PASSED))
TOTAL_FAILED=$((RUST_FAILED + JS_FAILED))

# Status cells: exactly 10 visible chars to match the ──────────── column width
[ "$RUST_EXIT" -eq 0 ] && RUST_ST="${GREEN}   pass   ${RESET}" || RUST_ST="${RED}   FAIL   ${RESET}"
[ "$JS_EXIT"   -eq 0 ] && JS_ST="${GREEN}   pass   ${RESET}"   || JS_ST="${RED}   FAIL   ${RESET}"
[ "$RUST_EXIT" -eq 0 ] && [ "$JS_EXIT" -eq 0 ] && TOTAL_ST="${GREEN}   pass   ${RESET}" || TOTAL_ST="${RED}   FAIL   ${RESET}"

# Returns a right-aligned 6-char colored number
col_passed() { printf "${GREEN}%6d${RESET}" "$1"; }
col_failed() { [ "$1" -gt 0 ] && printf "${RED}%6d${RESET}" "$1" || printf "%6d" "$1"; }

echo ""
printf "┌──────────────┬────────┬────────┬──────────┐\n"
printf "│ %-12s │ %6s │ %6s │  Status  │\n" "Suite" "Passed" "Failed"
printf "├──────────────┼────────┼────────┼──────────┤\n"
printf "│ %-12s │ %s │ %s │%s│\n" "Rust" "$(col_passed "$RUST_PASSED")" "$(col_failed "$RUST_FAILED")" "$RUST_ST"
printf "│ %-12s │ %s │ %s │%s│\n" "JS"   "$(col_passed "$JS_PASSED")"   "$(col_failed "$JS_FAILED")"   "$JS_ST"
printf "├──────────────┼────────┼────────┼──────────┤\n"
printf "│ ${BOLD}%-12s${RESET} │ %s │ %s │%s│\n" "Total" "$(col_passed "$TOTAL_PASSED")" "$(col_failed "$TOTAL_FAILED")" "$TOTAL_ST"
printf "└──────────────┴────────┴────────┴──────────┘\n"

[ "$RUST_EXIT" -ne 0 ] || [ "$JS_EXIT" -ne 0 ] && exit 1
exit 0
