# SPDX-License-Identifier: MIT

import json
import os
import shutil
import subprocess
from pathlib import Path

import pytest

from book_structure_check import check_references, identity_defects, main, read_inventory


@pytest.fixture
def book(tmp_path: Path) -> Path:
    chapter = tmp_path / "ch1"
    chapter.mkdir()
    (chapter / "1.1.texi").write_text(
        "@node 1.1\n@section First\n@anchor{Exercise 1.1}\n@anchor{Figure 1.1}\n"
    )
    return tmp_path


def expected_inventory(_book: Path) -> dict[str, set[str]]:
    """Preserve each field the structural checker compares."""
    return {
        "parts": {"ch1/1.1.texi"},
        "sections": {"1.1"},
        "exercises": {"1.1"},
        "figures": {"1.1"},
        "references": set(),
        "images": set(),
    }


@pytest.mark.parametrize(
    ("original", "replacement", "kind"),
    [
        ("@node 1.1", "@node 1.2", "sections"),
        ("@anchor{Exercise 1.1}", "@anchor{Exercise 1.2}", "exercises"),
        ("@anchor{Figure 1.1}", "@anchor{Figure 1.2}", "figures"),
    ],
)
def test_equal_counts_cannot_hide_replaced_identities(
    book: Path, original: str, replacement: str, kind: str
) -> None:
    expected = expected_inventory(book)
    assert identity_defects(book, expected) == []
    part = book / "ch1/1.1.texi"
    part.write_text(part.read_text().replace(original, replacement))
    assert identity_defects(book, expected) == [f"{kind}: missing 1.1", f"{kind}: unexpected 1.2"]


def test_missing_material_is_not_recovered_from_generated_indexes(book: Path) -> None:
    expected = expected_inventory(book)
    (book / "ch1/1.1.texi").write_text("@node 1.1\n@section First\n")
    indexes = book / "back"
    indexes.mkdir()
    (indexes / "exercises.texi").write_text("@anchor{Exercise 1.1}\n")
    (indexes / "figures.texi").write_text("@anchor{Figure 1.1}\n")
    assert identity_defects(book, expected) == ["exercises: missing 1.1", "figures: missing 1.1"]


def test_source_parts_references_and_images_are_preserved(book: Path) -> None:
    expected = expected_inventory(book)
    expected["references"] = {"Reference 1"}
    expected["images"] = {"figures/chap1/Fig1.1"}

    part = book / "ch1/1.1.texi"
    part.write_text(
        "@node 1.1\n@section First\n@anchor{Exercise 1.1}\n@anchor{Figure 1.1}\n"
        "@anchor{Reference 1}\n@ref{Reference\n1}\n@image{figures/chap1/Fig1.1,,,.svg}\n"
    )
    assert identity_defects(book, expected) == []

    part.write_text(
        "@node 1.1\n@section First\n@anchor{Exercise 1.1}\n@anchor{Figure 1.1}\n"
        "@anchor{Reference 1}\n@image{figures/chap1/Fig1.1,,,.svg}\n"
    )
    assert identity_defects(book, expected) == []

    part.write_text("@node 1.1\n@section First\n@anchor{Exercise 1.1}\n@anchor{Figure 1.1}\n")
    assert identity_defects(book, expected) == [
        "references: missing Reference 1",
        "images: missing figures/chap1/Fig1.1",
    ]

    part.rename(book / "ch1/renamed.texi")
    assert identity_defects(book, expected) == [
        "parts: missing ch1/1.1.texi",
        "references: missing Reference 1",
        "images: missing figures/chap1/Fig1.1",
        "parts: unexpected ch1/renamed.texi",
    ]


@pytest.mark.parametrize("entries", [[], ["1.1", "1.1"], [1], ["1.1", None], ["invalid"]])
def test_corrupt_inventory_cannot_disable_preservation(
    tmp_path: Path, entries: list[object]
) -> None:
    inventory = tmp_path / "inventory.json"
    inventory.write_text(
        json.dumps(
            {
                "sources": {
                    "rust": {
                        "parts": ["ch1/1.1.texi"],
                        "sections": entries,
                        "exercises": ["1.1"],
                        "figures": ["1.1"],
                        "references": [],
                        "images": [],
                    }
                }
            }
        )
    )
    with pytest.raises(ValueError, match=r"rust.sections"):
        read_inventory(inventory, "rust")


def test_requested_edition_cannot_fall_back_to_another_inventory(tmp_path: Path) -> None:
    inventory = tmp_path / "inventory.json"
    inventory.write_text(
        json.dumps(
            {
                "sources": {
                    "rust": {
                        "parts": ["ch1/1.1.texi"],
                        "sections": ["1.1"],
                        "exercises": ["1.1"],
                        "figures": ["1.1"],
                        "references": [],
                        "images": [],
                    }
                }
            }
        )
    )
    with pytest.raises(ValueError, match="missing inventory for kotlin"):
        read_inventory(inventory, "kotlin")


def test_missing_tree_cannot_pass_as_an_empty_book(tmp_path: Path) -> None:
    with pytest.raises(ValueError, match="No Texinfo source"):
        identity_defects(
            tmp_path / "absent",
            {
                "parts": {"ch1/1.1.texi"},
                "sections": {"1.1"},
                "exercises": {"1.1"},
                "figures": {"1.1"},
                "references": set(),
                "images": set(),
            },
        )


@pytest.mark.parametrize(
    ("include", "expected_defects"),
    [
        ("@include ch1/1.1.texi\n", []),
        (
            "",
            [
                "sections: not rendered 1.1",
                "exercises: not rendered 1.1",
                "figures: not rendered 1.1",
            ],
        ),
        (
            "@ifset absent\n@include ch1/1.1.texi\n@end ifset\n",
            [
                "sections: not rendered 1.1",
                "exercises: not rendered 1.1",
                "figures: not rendered 1.1",
            ],
        ),
    ],
)
def test_source_inventory_cannot_hide_unpublished_material(
    book: Path, include: str, expected_defects: list[str]
) -> None:
    converter = shutil.which(os.environ.get("TEXI2ANY", "texi2any"))
    if converter is None:
        pytest.skip("Texinfo is required for the rendered-book regression")
    expected = expected_inventory(book)
    (book / "main.texi").write_text(
        "\\input texinfo\n@settitle Check\n@node Top\n@top Check\n" + include + "@bye\n"
    )
    part = book / "ch1/1.1.texi"
    part.write_text(part.read_text().replace("@section", "@chapter"))
    assert identity_defects(book, expected) == []
    assert check_references(book, converter, expected) == expected_defects


@pytest.mark.parametrize("command", ["bin/texi2any", "texi2any"])
def test_relative_converter_keeps_validating_after_directory_change(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch, command: str
) -> None:
    converter = shutil.which(os.environ.get("TEXI2ANY", "texi2any"))
    if converter is None:
        pytest.skip("Texinfo is required for the relative-path regression")
    version = subprocess.run(
        [converter, "--version"], capture_output=True, text=True, check=True, timeout=30
    ).stdout
    if not version.partition("\n")[0].endswith(" 7.3"):
        pytest.skip("Pinned Texinfo 7.3 is required for the CLI regression")
    binaries = tmp_path / "bin"
    binaries.mkdir()
    (binaries / "texi2any").symlink_to(Path(converter).absolute())
    tree = tmp_path / "rust/book"
    (tree / "ch1").mkdir(parents=True)
    (tree / "ch1/1.1.texi").write_text(
        "@node 1.1\n@chapter First\n@anchor{Exercise 1.1}\n@anchor{Figure 1.1}\n"
    )
    introduction = "\\input texinfo\n@settitle Check\n@node Top\n@top Check\n"
    (tree / "main.texi").write_text(introduction + "@include ch1/1.1.texi\n@bye\n")
    inventory = tmp_path / "inventory.json"
    inventory.write_text(
        json.dumps(
            {
                "sources": {
                    "rust": {
                        "parts": ["ch1/1.1.texi"],
                        "sections": ["1.1"],
                        "exercises": ["1.1"],
                        "figures": ["1.1"],
                        "references": [],
                        "images": [],
                    }
                }
            }
        )
    )
    monkeypatch.chdir(tmp_path)
    monkeypatch.setenv("PATH", "bin" + os.pathsep + os.environ["PATH"])
    arguments = [
        "--root",
        str(tmp_path),
        "--edition",
        "rust",
        "--inventory",
        str(inventory),
        "--texi2any",
        command,
    ]
    assert main(arguments) == 0
    (tree / "main.texi").write_text(introduction + "@bye\n")
    assert main(arguments) == 1
