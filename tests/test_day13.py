import functools
from collections.abc import Iterator
from textwrap import dedent
from tqdm import tqdm

from more_itertools import windowed

"""
..##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
"""

part1_text_input = """
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
""".strip().split(
    "\n\n"
)


assert len(part1_text_input) == 2


@functools.cache
def find_horizontal_reflection_lines(lines: tuple[int]) -> list[int]:
    def possible_reflection_lines(_lines: tuple[int]) -> Iterator[int]:
        row_pairs = windowed(_lines, 2)
        for i, (a, b) in enumerate(row_pairs, start=1):
            if a == b:
                yield i

    return list(possible_reflection_lines(lines))


@functools.cache
def find_horizontal_reflection_line(lines: tuple[int]) -> int:
    for candidate in find_horizontal_reflection_lines(lines):
        shorter_slice_len = min(len(lines[:candidate]), len(lines[candidate:]))
        # front = slice(candidate - shorter_slice_len, candidate)
        # back = slice(candidate, candidate + shorter_slice_len, -1)
        front = lines[candidate - shorter_slice_len : candidate]
        back = lines[candidate : candidate + shorter_slice_len][::-1]
        if front == back:
            return candidate
    return 0


@functools.cache
def transpose(grid: str) -> str:
    rows = grid.split("\n")
    col_size = len(rows[0]) if rows else 0
    return "\n".join("".join(row[i] for row in rows) for i in range(col_size))


@functools.cache
def compact(grid: str) -> tuple[int]:
    return tuple(hash(line) for line in grid.split("\n"))


@functools.cache
def find_vertical_reflection_line(grid: str) -> int:
    transposed = transpose(grid)
    return find_horizontal_reflection_line(compact(transposed))


def part1(inp: list[str]) -> int:
    result = 0
    for grid in inp:
        h, v = find_reflection(grid)
        result += 100 * h + v
    return result


def flip_chars(grid: str) -> Iterator[str]:
    for i, char in enumerate(grid):
        if char == "#":
            yield grid[:i] + "." + grid[i + 1 :]
        elif char == ".":
            yield grid[:i] + "#" + grid[i + 1 :]


def test_flip_chars():
    grid = ".#\n#."
    assert list(flip_chars(grid)) == ["##\n#.", "..\n#.", ".#\n..", ".#\n##"]


def part2(inp: list[str]) -> int:
    prev_hs = {}
    prev_vs = {}
    for grid in inp:
        grid_id = id(grid)
        h, v = find_reflection(grid)
        prev_hs[grid_id] = h
        prev_vs[grid_id] = v

    hs = {}
    vs = {}

    assert all([h == 0 or v == 0 for h, v in zip(prev_hs.values(), prev_vs.values())])

    for grid in inp:
        grid_id = id(grid)
        for mutation in flip_chars(grid):
            h, v = find_reflection(mutation)
            if h + v == 0:
                continue
            if h != 0 and h == prev_hs[grid_id] and v == 0:
                continue
            if v != 0 and v == prev_vs[grid_id] and h == 0:
                continue

            hs[grid_id] = h
            vs[grid_id] = v
            break
    print(hs)
    print(vs)

    new_hs = [h for grid_id, h in hs.items() if h != prev_hs[grid_id]]
    new_vs = [v for grid_id, v in vs.items() if v != prev_vs[grid_id]]

    return sum([100 * h for h in new_hs]) + sum(new_vs)


@functools.cache
def find_reflection(grid):
    h = find_horizontal_reflection_line(compact(grid))
    v = find_vertical_reflection_line(grid)
    return h, v


def test_transpose():
    grid = dedent(
        """
    123
    456
    """
    ).strip()
    assert (
        transpose(grid)
        == dedent(
            """
        14
        25
        36
    """
        ).strip()
    )


def test_part1():
    assert part1(part1_text_input) == 405


def test_part2():
    print()
    assert part2(part1_text_input) == 400


if __name__ == "__main__":
    with open("inputs/d13.txt") as f:
        inp = f.read().strip().split("\n\n")
    print(part1(inp))
    print(part2(inp))
