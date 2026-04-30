# Minigrep Improvement Plan

Generated: 2026-04-30

This document outlines identified issues and proposed improvements for the minigrep codebase, organized by priority.

---

## Quick Wins (High Impact, Low Effort)

### 1. Remove Unused Import
- **File:** `src/main.rs:5`
- **Issue:** `use std::cmp::PartialEq;` — `PartialEq` is in the prelude and does not need importing.
- **Fix:** Remove this line.

### 2. Remove Unused Clap Feature
- **File:** `Cargo.toml:7`
- **Issue:** `clap = { version = "4.5.20", features = ["derive"] }` enables the derive feature, but the code uses only the builder pattern.
- **Fix:** Change to `clap = "4.5.20"` (remove `"derive"` feature). Reduces compile time and binary size.

### 3. Fix README — Claimed Feature Doesn't Exist
- **File:** `README.md:7`
- **Issue:** README lists "Search for standard input" as a feature, but stdin support is not implemented.
- **Fix:** Remove or rewrite that bullet point to reflect actual features (case-insensitive search, line numbers).

### 4. Derive PartialEq Instead of Manual Impl
- **File:** `src/main.rs:38-48`
- **Issue:** Manual `PartialEq` implementation on `SearchResult` is unnecessary.
- **Fix:** Replace with `#[derive(Debug, PartialEq)]`.

---

## Code Quality (Medium Effort)

### 5. Colorization Always Active
- **File:** `src/main.rs:68-82`
- **Issue:** The `search()` function always applies ANSI color codes, even when stdout is piped to a file or another command. This breaks downstream consumers.
- **Fix:** Add a `--color` flag with options `auto`, `always`, `never`. Use `atty::is(atty::Stream::Stdout)` or the `colored` crate's `control()` feature to auto-disable when piped.

### 6. Inefficient String Allocations in `search()`
- **File:** `src/main.rs:61-65`
- **Issue:** When `ignore_case` is true, a new `String` is created for every line via `.to_string()`.
- **Fix:** Use `.eq_ignore_ascii_case()` or the `regex` crate with case-insensitive flag for better performance and to handle Unicode properly.

### 7. Replace `.unwrap()` with `.expect()`
- **File:** `src/args.rs:44-45`
- **Issue:** `.unwrap()` on `get_one()` calls has no context if something goes wrong.
- **Fix:** Use `.expect("pattern argument is required")` for clearer error messages.

---

## Tests (Medium Effort)

### 8. Hardcoded ANSI Escape Codes in Tests
- **File:** `src/main.rs:99-101`
- **Issue:** `strip_color_codes()` hardcodes `\x1b[31m` (red) and `\x1b[0m` (reset). If the `colored` crate changes its output or colors are disabled, tests will silently break.
- **Fix:** Use a library like `strip-ansi-escapes` crate, or test against the actual escape codes returned at runtime.

### 9. Missing Edge Case Tests
- **File:** `src/main.rs` (tests module)
- **Issue:** No tests for common edge cases.
- **Proposed test coverage:**
  - Empty pattern string
  - File with no matching lines
  - Multiple matches on the same line (colorization of each match)
  - Unicode / multibyte characters in search query and file content
  - Very large files (performance)

---

## CI/CD (Medium-High Effort)

### 10. Update Deprecated `actions-rs` in Build Workflow
- **File:** `.github/workflows/build.yml:23,32,48,57,70,81,89,94`
- **Issue:** Uses archived `actions-rs/toolchain@v1` and `actions-rs/cargo@v1`.
- **Fix:** Migrate to:
  - `dtolnay/rust-toolchain@stable` for toolchain installation
  - Replace `uses: actions-rs/cargo@v1` with native `run: cargo ...` steps

### 11. Fix Deprecated `set-output` Syntax in Release Workflow
- **File:** `.github/workflows/release.yml:140,183`
- **Issue:** `::set-output name=...::...` syntax is deprecated and will stop working.
- **Fix:** Replace with `$GITHUB_OUTPUT` env file syntax:
  ```yaml
  echo "val=$name" >> "$GITHUB_OUTPUT"
  ```

### 12. Update `actions/checkout` to v4 in Release Workflow
- **File:** `.github/workflows/release.yml:64,123`
- **Issue:** Uses `actions/checkout@v2`.
- **Fix:** Update to `actions/checkout@v4`.

---

## UX / Convention (Low Effort)

### 13. `-l` Flag Conflicts with grep Convention
- **File:** `src/args.rs:36-40`
- **Issue:** `-l` is used for "line number", but in traditional `grep`:
  - `-n` shows line numbers
  - `-l` lists filenames with matches
- **Fix:** Change to `-n / --line-number`. Update README and any documentation.

---

## Summary Table

| # | Issue | File | Priority | Effort |
|---|-------|------|----------|--------|
| 1 | Unused `PartialEq` import | `src/main.rs:5` | High | Trivial |
| 2 | Unused clap `"derive"` feature | `Cargo.toml:7` | High | Trivial |
| 3 | README claims stdin support | `README.md:7` | High | Trivial |
| 4 | Manual `PartialEq` impl | `src/main.rs:38-48` | High | Trivial |
| 5 | Colors always active (no `--color` flag) | `src/main.rs:68-82` | Medium | Medium |
| 6 | Inefficient string allocs in `search()` | `src/main.rs:61-65` | Medium | Medium |
| 7 | `.unwrap()` without context | `src/args.rs:44-45` | Low | Trivial |
| 8 | Hardcoded ANSI codes in tests | `src/main.rs:99-101` | Medium | Medium |
| 9 | Missing edge case tests | `src/main.rs` (tests) | Medium | Medium |
| 10 | Deprecated `actions-rs` in CI | `.github/workflows/build.yml` | High | Medium-High |
| 11 | Deprecated `set-output` in release CI | `.github/workflows/release.yml:140` | High | Trivial-Medium |
| 12 | Outdated `actions/checkout@v2` in release CI | `.github/workflows/release.yml:64,123` | Medium | Trivial |
| 13 | `-l` flag conflicts with grep convention | `src/args.rs:36-40` | Low | Trivial |

---

## Suggested Implementation Order

1. **Phase 1 — Quick wins:** Items #1, #2, #3, #4, #7, #12
2. **Phase 2 — CI fixes:** Items #10, #11 (prevents future breakage)
3. **Phase 3 — Code quality:** Items #5, #6 (better user experience)
4. **Phase 4 — Tests:** Items #8, #9 (improve reliability)
5. **Phase 5 — UX polish:** Item #13
