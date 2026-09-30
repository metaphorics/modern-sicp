# SPDX-License-Identifier: MIT

import json
import sys
from pathlib import Path

import pytest

import host_conformance_check
from host_conformance_check import (
    Case,
    Driver,
    Observation,
    check_edition,
    read_driver,
    read_inventory,
    read_manifest,
    run_case,
)


@pytest.fixture
def process_driver(tmp_path: Path) -> Driver:
    script = tmp_path / "driver.py"
    script.write_text(
        "import json, subprocess, sys\n"
        "source, engine, case = sys.argv[1:]\n"
        "result = subprocess.run([sys.executable, source], capture_output=True, text=True)\n"
        "termination = 'value' if result.returncode == 0 else 'error'\n"
        "print(json.dumps({'termination': termination, 'stdout': result.stdout}))\n",
        encoding="utf-8",
    )
    return Driver(
        (),
        (sys.executable, str(script), "{source}", "{engine}", "{case}"),
        (
            sys.executable,
            "-c",
            (
                "import pathlib, sys; "
                "compile(pathlib.Path(sys.argv[1]).read_text(), 'program', 'exec')"
            ),
            "{source}",
        ),
        (sys.executable, "{source}"),
    )


def test_native_compile_failure_is_not_runtime_error(
    tmp_path: Path,
    process_driver: Driver,
) -> None:
    source = tmp_path / "invalid.py"
    source.write_text("print(\n", encoding="utf-8")
    case = Case("core/20-error-stop", source, ("native", "direct"), "native", "error")
    report = run_case(case, process_driver, tmp_path)
    assert report.defect is not None
    assert "native-compile: exit" in report.defect
    assert report.observations == {}
    assert [execution.phase for execution in report.executions] == ["native-compile"]


def test_runtime_error_preserves_prefix_and_differs_from_rejection(
    tmp_path: Path,
    process_driver: Driver,
) -> None:
    source = tmp_path / "runtime.py"
    source.write_text("print('before', flush=True)\nraise ValueError('stop')\n", encoding="utf-8")
    case = Case("core/20-error-stop", source, ("native", "direct"), "native", "error")
    report = run_case(case, process_driver, tmp_path)
    assert report.defect is None
    assert report.observations == {
        "native": Observation("error", "before\n"),
        "direct": Observation("error", "before\n"),
    }
    assert report.executions[0].returncode == 0
    assert report.executions[1].returncode != 0


def test_output_disagreement_fails_with_both_transcripts(
    tmp_path: Path,
    process_driver: Driver,
) -> None:
    source = tmp_path / "source.py"
    source.write_text("print(6 * 7)\n", encoding="utf-8")
    wrong = tmp_path / "wrong.py"
    wrong.write_text(
        "import json\nprint(json.dumps({'termination': 'value', 'stdout': '43\\n'}))\n",
        encoding="utf-8",
    )
    driver = Driver(
        (), (sys.executable, str(wrong)), process_driver.native_compile, process_driver.native_run
    )
    report = run_case(
        Case("core/01-square", source, ("native", "direct"), "native", "value"), driver, tmp_path
    )
    assert report.defect == "direct: observation differs from native"
    assert report.observations["native"].stdout == "42\n"
    assert report.observations["direct"].stdout == "43\n"


@pytest.mark.parametrize(
    "output",
    [
        '{"termination":"rejected","stdout":""}',
        '{"termination":"value","stdout":42}',
        '{"termination":"value","stdout":"42\\n","extra":true}',
        'debug\n{"termination":"value","stdout":"42\\n"}',
        '{"termination":"rejected","termination":"value","stdout":"42\\n"}',
    ],
)
def test_rejected_or_malformed_driver_output_fails(
    tmp_path: Path,
    process_driver: Driver,
    output: str,
) -> None:
    source = tmp_path / "source.py"
    source.write_text("print(42)\n", encoding="utf-8")
    reporter = tmp_path / "reporter.py"
    reporter.write_text(f"print({output!r})\n", encoding="utf-8")
    driver = Driver(
        (),
        (sys.executable, str(reporter)),
        process_driver.native_compile,
        process_driver.native_run,
    )
    report = run_case(
        Case("core/01-square", source, ("native", "direct"), "native", "value"), driver, tmp_path
    )
    assert report.defect is not None


def test_driver_crash_is_not_guest_runtime_error(
    tmp_path: Path,
    process_driver: Driver,
) -> None:
    source = tmp_path / "source.py"
    source.write_text("raise ValueError('guest')\n", encoding="utf-8")
    crashing = tmp_path / "crashing.py"
    crashing.write_text("raise RuntimeError('driver crashed')\n", encoding="utf-8")
    driver = Driver(
        (),
        (sys.executable, str(crashing)),
        process_driver.native_compile,
        process_driver.native_run,
    )
    report = run_case(
        Case("core/20-error-stop", source, ("native", "direct"), "native", "error"),
        driver,
        tmp_path,
    )
    assert report.defect is not None
    assert "driver crashed" in report.defect
    assert "direct" not in report.observations


@pytest.fixture
def corpus(tmp_path: Path) -> Path:
    directory = tmp_path / "spec" / "host-subsets" / "rust"
    directory.mkdir(parents=True)
    (tmp_path / "rust").mkdir()
    (directory / "program.rs").write_text("fn main() {}\n", encoding="utf-8")
    inventory = {
        "cases": [
            {"id": name, "capability": "core", "observation": "value"}
            for name in ("core/first", "core/second")
        ],
    }
    (directory.parent / "cases.json").write_text(json.dumps(inventory), encoding="utf-8")
    return tmp_path


def test_scoped_run_still_rejects_missing_inventory(corpus: Path) -> None:
    directory = corpus / "spec" / "host-subsets" / "rust"
    (directory / "manifest.tsv").write_text(
        "core/first\tprogram.rs\tnative,direct,analyzed,eceval,compiled\tnative\tfirst\n",
        encoding="utf-8",
    )
    report = check_edition(corpus, "rust", "core/first")
    assert report.cases == []
    assert any("missing cases: core/second" in defect for defect in report.defects)


@pytest.mark.parametrize(
    "engines",
    [
        "native,direct,analyzed,eceval",
        "native,direct,analyzed,eceval,compiled,compiled",
    ],
)
def test_missing_or_duplicate_engine_cannot_reduce_coverage(corpus: Path, engines: str) -> None:
    directory = corpus / "spec" / "host-subsets" / "rust"
    (directory / "manifest.tsv").write_text(
        f"core/first\tprogram.rs\t{engines}\tnative\tfirst\n",
        encoding="utf-8",
    )
    with pytest.raises(ValueError, match="required engines"):
        read_manifest(corpus, "rust", {"core/first": "value"})


def test_native_checker_must_receive_observed_source(tmp_path: Path) -> None:
    config = {
        "build": [],
        "run": ["driver", "{source}", "{case}", "{engine}"],
        "native_compile": ["compiler", "unrelated.rs"],
        "native_run": ["program"],
    }
    path = tmp_path / "driver.json"
    path.write_text(json.dumps(config), encoding="utf-8")
    with pytest.raises(ValueError, match="native_compile must check"):
        read_driver(path)


def test_duplicate_inventory_case_is_rejected(tmp_path: Path) -> None:
    case = {"id": "core/first", "capability": "core", "observation": "value"}
    path = tmp_path / "cases.json"
    path.write_text(json.dumps({"cases": [case, case]}), encoding="utf-8")
    with pytest.raises(ValueError, match="duplicate case"):
        read_inventory(path)


def test_timeout_retains_partial_process_evidence(
    tmp_path: Path,
    process_driver: Driver,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.setattr(host_conformance_check, "RUN_TIMEOUT", 1)
    source = tmp_path / "slow.py"
    source.write_text(
        "import time\nprint('entered', flush=True)\ntime.sleep(30)\n",
        encoding="utf-8",
    )
    report = run_case(
        Case("core/first", source, ("native", "direct"), "native", "value"),
        process_driver,
        tmp_path,
    )
    assert report.defect is not None
    timed = report.executions[-1]
    assert timed.phase == "native-run"
    assert timed.timed_out
    assert timed.returncode is None
    assert timed.stdout == "entered\n"
    assert timed.argv[0] == sys.executable


def test_native_commands_receive_native_engine(
    tmp_path: Path,
    process_driver: Driver,
) -> None:
    source = tmp_path / "source.py"
    source.write_text("import sys\nprint(sys.argv[1])\n", encoding="utf-8")
    driver = Driver(
        (),
        process_driver.run,
        process_driver.native_compile,
        (sys.executable, "{source}", "{engine}"),
    )
    report = run_case(
        Case("core/first", source, ("native",), "native", "value"),
        driver,
        tmp_path,
    )
    assert report.defect is None
    assert report.observations["native"] == Observation("value", "native\n")
