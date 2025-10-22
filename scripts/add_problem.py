#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import List


LESS_THAN_20 = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
]

TENS = [
    "",
    "",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
]

SCALE_WORDS = ["", "thousand", "million", "billion"]

WORD_TO_VALUE = {
    "zero": 0,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "ten": 10,
    "eleven": 11,
    "twelve": 12,
    "thirteen": 13,
    "fourteen": 14,
    "fifteen": 15,
    "sixteen": 16,
    "seventeen": 17,
    "eighteen": 18,
    "nineteen": 19,
    "twenty": 20,
    "thirty": 30,
    "forty": 40,
    "fifty": 50,
    "sixty": 60,
    "seventy": 70,
    "eighty": 80,
    "ninety": 90,
}

SCALE_VALUES = {
    "hundred": 100,
    "thousand": 1000,
    "million": 1_000_000,
    "billion": 1_000_000_000,
}


def number_to_words(num: int) -> str:
    if num <= 0:
        raise ValueError("problem number must be a positive integer")

    def chunk_to_words(chunk: int) -> List[str]:
        words: List[str] = []
        if chunk >= 100:
            words.append(LESS_THAN_20[chunk // 100])
            words.append("hundred")
            chunk %= 100
        if chunk >= 20:
            words.append(TENS[chunk // 10])
            chunk %= 10
        if 0 < chunk < 20:
            words.append(LESS_THAN_20[chunk])
        return words

    parts: List[str] = []
    scale_index = 0
    working = num
    while working > 0:
        chunk = working % 1000
        if chunk:
            chunk_words = chunk_to_words(chunk)
            scale_word = SCALE_WORDS[scale_index]
            if scale_word:
                chunk_words.append(scale_word)
            parts.append(chunk_words)
        working //= 1000
        scale_index += 1

    words_flat: List[str] = []
    for chunk_words in reversed(parts):
        words_flat.extend(chunk_words)
    return "_".join(words_flat)


def create_go_file(root: Path, number: int) -> tuple[Path, bool]:
    go_path = root / "go" / "leetcode" / f"{number}.go"
    if go_path.exists():
        return go_path, False
    go_content = (
        "package leetcode\n\n"
        f"// TODO: add solution for problem {number}.\n"
    )
    go_path.write_text(go_content, encoding="utf-8")
    return go_path, True


def create_rust_file(root: Path, number: int, module_name: str) -> tuple[Path, bool]:
    rs_path = root / "rust" / "src" / "leetcode" / f"{module_name}.rs"
    if rs_path.exists():
        return rs_path, False
    rs_content = (
        "pub struct Solution;\n\n"
        f"// TODO: add implementation for problem {number}.\n"
    )
    rs_path.write_text(rs_content, encoding="utf-8")
    return rs_path, True


def words_to_number(words: str) -> int | None:
    tokens = words.split("_")
    total = 0
    current = 0
    used_hundred = False

    def apply_scale(value: int, scale: int) -> int:
        if value == 0:
            value = 1
        return value * scale

    for token in tokens:
        if token == "and":
            continue
        if token in WORD_TO_VALUE:
            current += WORD_TO_VALUE[token]
            continue
        if token == "hundred":
            if used_hundred:
                return None
            current = apply_scale(current, SCALE_VALUES[token])
            used_hundred = True
            continue
        if token in ("thousand", "million", "billion"):
            total += apply_scale(current, SCALE_VALUES[token])
            current = 0
            used_hundred = False
            continue
        return None
    return total + current


def update_rust_mod(
    root: Path, module_name: str, module_number: int
) -> tuple[Path, bool]:
    mod_path = root / "rust" / "src" / "leetcode.rs"
    line = f"pub mod {module_name};"
    original_lines = mod_path.read_text(encoding="utf-8").splitlines()
    lines = list(original_lines)

    original_idx = None
    if line in lines:
        original_idx = lines.index(line)
        lines.pop(original_idx)

    insert_idx = len(lines)
    last_numeric_idx = None
    for idx, existing in enumerate(lines):
        stripped = existing.strip()
        if not stripped.startswith("pub mod ") or not stripped.endswith(";"):
            continue
        name = stripped[len("pub mod ") : -1]
        value = words_to_number(name)
        if value is None:
            continue
        last_numeric_idx = idx
        if value > module_number:
            insert_idx = idx
            break
    else:
        if insert_idx == len(lines) and last_numeric_idx is not None:
            insert_idx = last_numeric_idx + 1

    lines.insert(insert_idx, line)

    if lines == original_lines:
        return mod_path, False

    mod_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return mod_path, True


def extract_number(line: str) -> int | None:
    match = re.search(r"(\d+)\.", line)
    if match:
        return int(match.group(1))
    return None


def update_readme(root: Path, entry: str, extra: str) -> tuple[Path, bool]:
    readme_path = root / "leetcode" / "readme.md"
    target_line = f"- **{entry}**{extra}"
    target_number = extract_number(target_line)
    if target_number is None:
        raise ValueError(
            "could not detect problem number inside readme entry; "
            "please include something like '1234.' in the text"
        )
    original_lines = readme_path.read_text(encoding="utf-8").splitlines()
    lines = list(original_lines)
    replaced = False
    insert_at = len(lines)
    for idx, line in enumerate(lines):
        existing_number = extract_number(line)
        if existing_number == target_number:
            lines[idx] = target_line
            replaced = True
            break
        if existing_number is not None and existing_number > target_number:
            insert_at = idx
            break
    else:
        if not replaced:
            insert_at = len(lines)

    if not replaced:
        lines.insert(insert_at, target_line)

    changed = lines != original_lines
    if changed:
        readme_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return readme_path, changed


def main(argv: List[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Create LeetCode problem stubs in Go and Rust."
    )
    parser.add_argument("number", type=int, help="problem number, e.g. 3499")
    parser.add_argument(
        "--readme-entry",
        help="text to place inside the bold section of leetcode/readme.md "
        "(e.g. \"[hard]3347. 执行操作后元素的最高频率 II\")",
    )
    parser.add_argument(
        "--extra",
        default="",
        help="optional text appended after the bold section, "
        "for example tags like 「滑动窗口」",
    )
    args = parser.parse_args(argv)

    root = Path(__file__).resolve().parent.parent
    module_name = number_to_words(args.number)

    go_path, go_changed = create_go_file(root, args.number)
    rs_path, rs_changed = create_rust_file(root, args.number, module_name)
    mod_path, mod_changed = update_rust_mod(root, module_name, args.number)

    touched = []
    if go_changed:
        touched.append(go_path)
    if rs_changed:
        touched.append(rs_path)
    if mod_changed:
        touched.append(mod_path)

    if args.readme_entry:
        readme_path, readme_changed = update_readme(
            root,
            args.readme_entry.strip(),
            args.extra or "",
        )
        if readme_changed:
            touched.append(readme_path)

    if touched:
        print("Updated files:")
        for path in touched:
            print(f"  - {path.relative_to(root)}")
    else:
        print("No files changed.")
    print(f"Rust module name: {module_name}")
    if not args.readme_entry:
        print("Readme unchanged (no entry provided).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
