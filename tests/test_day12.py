import itertools
import typing
from pprint import pprint

import pytest

# import regex as re
import re

import functools

if typing.TYPE_CHECKING:
    import re

part1_text_input = """
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
""".strip().splitlines()


@functools.cache
def damage_counts(line: str) -> list[int]:
    return [int(n) for n in line.split(" ", maxsplit=1)[1].split(",")]


#
# @functools.cache
# def conditions(line: str) -> list[str]:
#     """Returns the conditions of the springs in the line"""
#     return line.split(" ", maxsplit=1)[0].split(".")


assert damage_counts("???.### 1,1,3") == [1, 1, 3]


@functools.cache
def possible_arrangements(line: str) -> list[str]:
    """Returns the number of possible arrangements of the operational and damaged springs"""
    conditions, counts = line.split(" ", 1)[0], damage_counts(line)
    pattern = r"(?=([.?]+".join([f"[#?]{{{count}}}))" for count in counts])
    # pattern = r"(.*[.?]?.*){}(.*[.?]?.*$)".format(pattern)
    # pattern = r"(?=(^[.?]*{}(?=([.?]*))".format(pattern)
    # junk_pattern = r"(.+){}(.+)".format(pattern)
    print(pattern)
    # non_possesive_pattern = r"([\.?]*?){}([\.?]*)".format(pattern)
    # print(non_possesive_pattern)

    matches = list(re.compile(pattern).finditer(conditions))  # , overlapped=True))
    pprint([m.groups() for m in matches])
    combos = list(re.compile(pattern).findall(conditions))  # , overlapped=True))

    # breakpoint()
    return sorted(set(tuple(g for g in combo if g) for combo in combos))


@functools.cache
def one_pattern(counts: tuple[int]) -> re.Pattern:
    if len(counts) == 1:
        return re.compile(rf"(?=([#?]{{{counts[0]}}}))")
    first, *rest = counts
    tail_pattern = r"[.?]+?".join([rf"[#?]{{{count}}}" for count in rest])
    return re.compile(rf"(?=([#?]{{{first}}}))([.?]+?{tail_pattern})")

    # if first:
    #     return re.compile(rf"(?=([.?]*)[#?]{{{count}}})")
    # else:
    #     return re.compile(rf"(?=([.?]+)[#?]{{{count}}})")


def f(line: str, counts: tuple[int], visited=None) -> int:
    visited = visited or {}
    key = line, counts
    if key in visited:
        return 0
    print(f"Entering {line=}, {counts=}")
    if not counts:
        visited[key] = 1
        return 1
    pattern = one_pattern(counts)
    print("Pattern:", pattern)

    result = 0
    for match in pattern.finditer(line):
        print(f"Match: pos={match.pos} groups={match.groups()}")
        result += f(line[match.pos + counts[0] :], counts[1:], visited)
        print(f"Intermediate {result=}")

    print(f"{line=}, {counts=}, {result=}")
    visited[key] = result
    return result


@functools.cache
def possible_arrangements(line: str) -> list[str]:
    """Returns the number of possible arrangements of the operational and damaged springs"""
    conditions, counts = line.split(" ", 1)[0], damage_counts(line)
    pattern = r"(?=([.?]+".join([f"[#?]{{{count}}}))" for count in counts])
    # pattern = r"(.*[.?]?.*){}(.*[.?]?.*$)".format(pattern)
    # pattern = r"(?=(^[.?]*{}(?=([.?]*))".format(pattern)
    # junk_pattern = r"(.+){}(.+)".format(pattern)
    print(pattern)
    # non_possesive_pattern = r"([\.?]*?){}([\.?]*)".format(pattern)
    # print(non_possesive_pattern)

    matches = list(re.compile(pattern).finditer(conditions))  # , overlapped=True))
    pprint([m.groups() for m in matches])
    combos = list(re.compile(pattern).findall(conditions))  # , overlapped=True))

    # breakpoint()
    return sorted(set(tuple(g for g in combo if g) for combo in combos))


@pytest.mark.parametrize(
    ("record", "expected"),
    [
        (".??..??...?##. 1,1,3", 4),
        ("???.### 1,1,3", 1),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 1),
        ("????.######..#####. 1,6,5", 4),
        ("?###???????? 3,2,1", 10),
    ],
)
def test_possible_arrangements(record, expected):
    conditions, counts = record.split(" ", 1)[0], damage_counts(record)
    assert f(conditions, tuple(counts)) == expected
