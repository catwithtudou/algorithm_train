# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Purpose

Algorithm training repository containing LeetCode problem solutions in both Go and Rust. Primarily focused on LeetCode Daily Question practice.

## Language Organization

- **Go**: `go/` directory with `go.mod` (module `catwithtudou/algorithm_train/go`). Solutions in `go/leetcode/<number>.go` (e.g., `go/leetcode/11.go`).
- **Rust**: `rust/` directory with Cargo workspace. Solutions in `rust/src/leetcode/<name>.rs` with module declarations in `rust/src/leetcode.rs`.

## Key Scripts

- `scripts/add_problem.py <number>` - Creates new problem stubs in both languages. Takes problem number (e.g., `3499`) and optional flags:
  - `--tt` - Text for README entry (e.g., `"[hard]3347. 执行操作后元素的最高频率 II"`)
  - `--sl` - Tags for README (e.g., `"「滑动窗口」"`)

The Rust module name is derived from English number words (e.g., problem 11 → `eleven`, 102 → `one_hundred_and_two`).

## Git Workflow

- `push.sh` - Interactive script for committing and pushing to git. Takes optional commit message and branch name.

## Architecture Notes

- Each Go file is a standalone `package leetcode` with function implementations.
- Rust uses a single `leetcode.rs` module that declares all problem modules (`pub mod <name>;`).
- The `leetcode/readme.md` tracks all problems with difficulty, number, title, and tags.