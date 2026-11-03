#!/usr/bin/env python3
"""Fail unless the playground wasm module exports the functions the page calls."""

import sys

REQUIRED = (
    "lingua_alloc",
    "lingua_dealloc",
    "lingua_compile",
    "lingua_compile_in",
    "lingua_free",
    "memory",
)


def uleb(data: bytes, index: int) -> tuple[int, int]:
    value = 0
    shift = 0
    while True:
        byte = data[index]
        index += 1
        value |= (byte & 0x7F) << shift
        if byte < 0x80:
            return value, index
        shift += 7


def exports(data: bytes) -> set[str]:
    if data[:4] != b"\0asm":
        sys.exit("not a wasm module")
    index = 8
    names: set[str] = set()
    while index < len(data):
        section = data[index]
        index += 1
        size, index = uleb(data, index)
        payload = data[index : index + size]
        index += size
        if section != 7:
            continue
        count, cursor = uleb(payload, 0)
        for _ in range(count):
            length, cursor = uleb(payload, cursor)
            names.add(payload[cursor : cursor + length].decode())
            cursor += length + 1
            _, cursor = uleb(payload, cursor)
    return names


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit("usage: check_exports.py <lingua_wasm.wasm>")
    path = sys.argv[1]
    data = open(path, "rb").read()
    found = exports(data)
    missing = [name for name in REQUIRED if name not in found]
    if missing:
        sys.exit(f"{path} is missing {', '.join(missing)}; exports: {', '.join(sorted(found))}")
    print(f"{path}: exports ok ({len(data)} bytes)")


if __name__ == "__main__":
    main()
