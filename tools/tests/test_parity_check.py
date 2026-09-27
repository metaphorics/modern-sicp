# SPDX-License-Identifier: MIT

import pytest

from parity_check import counts
from split_texi import compare


def test_different_anchor_sets_cannot_hide_behind_equal_counts() -> None:
    original = b"@anchor{Exercise 1.1}\n"
    corrupted = b"@anchor{Exercise 1.2}\n"
    assert counts(original.decode()) == counts(corrupted.decode())
    with pytest.raises(ValueError, match="byte offset"):
        compare(original, corrupted)


def test_truncation_is_detected() -> None:
    with pytest.raises(ValueError, match="byte offset 3"):
        compare(b"abcde", b"abc")
