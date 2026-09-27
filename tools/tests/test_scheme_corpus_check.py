# SPDX-License-Identifier: MIT
import shutil
from pathlib import Path

import pytest

import scheme_corpus_check
from scheme_corpus_check import main

HEADER = ";; SPDX-License-Identifier: GPL-3.0-only\n;; Adapted from SICP sections 4.1 to 5.2\n"
RUNNER = Path(__file__).resolve().parents[2] / "spec/scheme-subset/runner.scm"
guile_required = pytest.mark.skipif(shutil.which("guile") is None, reason="guile is absent")


@pytest.fixture
def one_program_inventory(monkeypatch: pytest.MonkeyPatch) -> None:
    """Shrink the required corpus so a fixture can be one program wide.

    The real inventory is the 39-program contract; these cases exercise the
    output comparison, not the inventory, which has its own tests below.
    """
    monkeypatch.setattr(scheme_corpus_check, "REQUIRED_PROGRAMS", {"core": 1})
    monkeypatch.setattr(scheme_corpus_check, "REQUIRED_NAMES", frozenset())


def corpus(tmp_path: Path, **files: str) -> Path:
    """Lay out a one-program corpus with its manifest."""
    root = tmp_path / "spec"
    (root / "programs/core").mkdir(parents=True)
    (root / "expected/core").mkdir(parents=True)
    shutil.copy(RUNNER, root / "runner.scm")
    (root / "programs/core/01-square.scm").write_text(files.get("program", "(+ 1 1)\n"))
    (root / "expected/core/01-square.txt").write_text(HEADER + files.get("expected", "2\n"))
    default = "core\tcore/01-square.scm\tcore/01-square.txt\tvalue\tall"
    (root / "manifest.txt").write_text(
        ";; SPDX-License-Identifier: GPL-3.0-only\n"
        "# capability\tprogram\texpected\tmode\tevaluators\n" + files.get("row", default) + "\n"
    )
    return root


@guile_required
@pytest.mark.usefixtures("one_program_inventory")
def test_matching_output_passes(tmp_path: Path) -> None:
    root = corpus(tmp_path, program="(define (square x) (* x x))\n(square 21)\n", expected="441\n")
    assert main(["--root", str(root)]) == 0


@guile_required
@pytest.mark.usefixtures("one_program_inventory")
def test_bare_try_again_dispatches_to_program_driver(tmp_path: Path) -> None:
    # printer.md fixes a bare `try-again` symbol as a driver request; the
    # runner evaluates it as a call to the program-defined procedure, so amb
    # programs keep their driver state and the runner never inspects it.
    program_lines = [
        "(define counter 0)",
        "(define (try-again) (set! counter (+ counter 1)) counter)",
        "counter",
        "try-again",
        "try-again",
    ]
    root = corpus(tmp_path, program="\n".join(program_lines) + "\n", expected="0\n1\n2\n")
    assert main(["--root", str(root)]) == 0


@guile_required
@pytest.mark.usefixtures("one_program_inventory")
def test_stale_expected_file_fails(tmp_path: Path) -> None:
    # The expected file says what the program used to print.
    root = corpus(tmp_path, program="(define (square x) (* x x))\n(square 21)\n", expected="42\n")
    assert main(["--root", str(root)]) == 1


@guile_required
@pytest.mark.usefixtures("one_program_inventory")
def test_program_missing_from_manifest_fails(tmp_path: Path) -> None:
    root = corpus(tmp_path)
    (root / "programs/core/02-extra.scm").write_text("(+ 2 2)\n")
    assert main(["--root", str(root)]) == 1


@guile_required
@pytest.mark.usefixtures("one_program_inventory")
def test_manifest_row_without_program_fails(tmp_path: Path) -> None:
    root = corpus(
        tmp_path,
        row=(
            "core\tcore/01-square.scm\tcore/01-square.txt\tvalue\tall\n"
            "core\tcore/99-absent.scm\tcore/99-absent.txt\tvalue\tall"
        ),
    )
    assert main(["--root", str(root)]) == 1


@guile_required
def test_short_corpus_fails_against_the_real_inventory(tmp_path: Path) -> None:
    # A manifest covering one capability partially must not read as green.
    root = corpus(tmp_path)
    assert main(["--root", str(root)]) == 1


@guile_required
def test_capability_scope_ignores_other_capabilities(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    # A run scoped to one capability must not fail on the ones it skipped.
    monkeypatch.setattr(scheme_corpus_check, "REQUIRED_PROGRAMS", {"core": 1, "lazy": 4})
    monkeypatch.setattr(scheme_corpus_check, "REQUIRED_NAMES", frozenset())
    root = corpus(tmp_path)
    assert main(["--root", str(root), "--capability", "core"]) == 0
    assert main(["--root", str(root), "--capability", "lazy"]) == 1
    assert main(["--root", str(root)]) == 1


def test_inventory_totals_thirty_nine_programs() -> None:
    assert sum(scheme_corpus_check.REQUIRED_PROGRAMS.values()) == 39
    assert "core/metacircular.scm" in scheme_corpus_check.REQUIRED_NAMES


def test_malformed_manifest_row_fails(tmp_path: Path) -> None:
    root = corpus(tmp_path, row="core\tonly/two.scm")
    assert main(["--root", str(root)]) == 1


def test_missing_manifest_and_usage(tmp_path: Path) -> None:
    assert main(["--root", str(tmp_path / "absent")]) == 1
    with pytest.raises(SystemExit) as raised:
        main(["--bogus"])
    assert raised.value.code == 2
