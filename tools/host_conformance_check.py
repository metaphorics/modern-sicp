# SPDX-License-Identifier: MIT
"""Compare preserved teaching cases with native hosts or independent models."""

import argparse
import hashlib
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field, replace
from pathlib import Path
from tempfile import TemporaryDirectory

type Json = bool | int | float | str | list[Json] | dict[str, Json] | None
type Command = tuple[str, ...]

EDITIONS = ("rust", "ocaml", "typescript", "kotlin")
CORE_ENGINES = frozenset({"native", "direct", "analyzed", "eceval", "compiled"})
ENGINES = {
    "core": CORE_ENGINES,
    "compiler": CORE_ENGINES,
    "lazy": frozenset({"reference", "lazy"}),
    "amb": frozenset({"reference", "search"}),
    "query": frozenset({"reference", "query"}),
    "machine": frozenset({"reference", "machine"}),
}
MANIFEST_FIELDS = 5
BUILD_TIMEOUT = 600
RUN_TIMEOUT = 120


@dataclass(frozen=True, slots=True)
class Case:
    """A preserved lesson and its concrete source artifact."""

    name: str
    source: Path
    engines: tuple[str, ...]
    oracle: str
    termination: str


@dataclass(frozen=True, slots=True)
class Driver:
    """Edition-specific argv, executed without a shell."""

    build: tuple[Command, ...]
    run: Command
    native_compile: Command
    native_run: Command


@dataclass(frozen=True, slots=True)
class Observation:
    """Observable termination and the ordered output transcript."""

    termination: str
    stdout: str


@dataclass(frozen=True, slots=True)
class Execution:
    """One subprocess result, separate from its interpreted observation."""

    phase: str
    argv: Command
    returncode: int | None
    stdout: str
    stderr: str
    timed_out: bool = False


@dataclass(frozen=True, slots=True)
class CaseReport:
    """Source provenance and execution evidence for one attempted case."""

    name: str
    source: str
    sha256: str
    executions: list[Execution] = field(default_factory=list[Execution])
    observations: dict[str, Observation] = field(default_factory=dict[str, Observation])
    defect: str | None = None


@dataclass(frozen=True, slots=True)
class EditionReport:
    """Build and case results, including failures before a case can run."""

    build: list[Execution] = field(default_factory=list[Execution])
    cases: list[CaseReport] = field(default_factory=list[CaseReport])
    defects: list[str] = field(default_factory=list[str])


def unique_members(pairs: list[tuple[str, Json]]) -> dict[str, Json]:
    """Reject contradictory JSON members instead of keeping the last value."""
    members: dict[str, Json] = {}
    for key, value in pairs:
        if key in members:
            raise ValueError(f"duplicate JSON member: {key}")
        members[key] = value
    return members


def read_object(path: Path) -> dict[str, Json]:
    """Read a JSON object; reject a different root type."""
    value: Json = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=unique_members)
    if not isinstance(value, dict):
        raise TypeError(f"{path}: expected a JSON object")
    return value


def read_inventory(path: Path) -> dict[str, str]:
    """Read preserved case identities without accepting duplicates or an empty set."""
    values = read_object(path).get("cases")
    if not isinstance(values, list) or not values:
        raise ValueError(f"{path}: missing nonempty cases array")
    cases: dict[str, str] = {}
    for value in values:
        if not isinstance(value, dict):
            raise TypeError(f"{path}: invalid case entry")
        name, capability, termination = (
            value.get("id"),
            value.get("capability"),
            value.get("observation"),
        )
        if not isinstance(name, str) or not isinstance(capability, str):
            raise TypeError(f"{path}: case id and capability must be strings")
        if capability not in ENGINES or not name.startswith(f"{capability}/"):
            raise ValueError(f"{path}: invalid capability for {name}")
        if termination not in ("value", "error"):
            raise ValueError(f"{path}: invalid observation for {name}")
        if name in cases:
            raise ValueError(f"{path}: duplicate case {name}")
        cases[name] = termination
    return cases


def read_manifest(root: Path, edition: str, inventory: dict[str, str]) -> list[Case]:
    """Require every preserved case and all of its execution paths."""
    directory = root / "spec" / "host-subsets" / edition
    path = directory / "manifest.tsv"
    cases: dict[str, Case] = {}
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        fields = line.split("\t")
        if len(fields) != MANIFEST_FIELDS:
            raise ValueError(f"{path}:{number}: expected {MANIFEST_FIELDS} fields")
        name, artifact, engine_names, oracle, note = (part.strip() for part in fields)
        if name not in inventory or name in cases:
            raise ValueError(f"{path}:{number}: unknown or duplicate case {name}")
        engines = tuple(engine_names.split(","))
        required = ENGINES[name.split("/", 1)[0]]
        if frozenset(engines) != required or len(engines) != len(required):
            raise ValueError(f"{name}: required engines are {','.join(sorted(required))}")
        expected_oracle = "native" if "native" in required else "reference"
        if oracle != expected_oracle or not note:
            raise ValueError(f"{name}: requires {expected_oracle} provenance and a lesson note")
        source = (directory / artifact).resolve()
        if not artifact or not source.is_relative_to(root) or not source.is_file():
            raise ValueError(f"{name}: artifact is not a repository file: {artifact}")
        cases[name] = Case(name, source, engines, oracle, inventory[name])
    missing = inventory.keys() - cases.keys()
    if missing:
        raise ValueError(f"{path}: missing cases: {', '.join(sorted(missing))}")
    return list(cases.values())


def command(value: Json, label: str) -> Command:
    """Validate one nonempty argv without implicit coercion."""
    if not isinstance(value, list) or not value:
        raise ValueError(f"{label}: expected a nonempty argv array")
    args: list[str] = []
    for arg in value:
        if not isinstance(arg, str) or not arg or "\0" in arg:
            raise ValueError(f"{label}: argv entries must be nonempty strings without NUL")
        args.append(arg)
    return tuple(args)


def read_driver(path: Path) -> Driver:
    """Require separate build, native checking, and execution commands."""
    value = read_object(path)
    if set(value) != {"build", "run", "native_compile", "native_run"}:
        raise ValueError(f"{path}: expected build, run, native_compile, native_run")
    builds = value["build"]
    if not isinstance(builds, list):
        raise TypeError(f"{path}: build must be an array of argv arrays")
    driver = Driver(
        tuple(command(item, "build") for item in builds),
        command(value["run"], "run"),
        command(value["native_compile"], "native_compile"),
        command(value["native_run"], "native_run"),
    )
    for placeholder in ("{case}", "{engine}", "{source}"):
        if not any(placeholder in arg for arg in driver.run):
            raise ValueError(f"{path}: run must use {placeholder}")
    if not any("{source}" in arg for arg in driver.native_compile):
        raise ValueError(f"{path}: native_compile must check {{source}}")
    return driver


def expand(argv: Command, replacements: dict[str, str]) -> Command:
    """Substitute named paths and modes without shell parsing."""
    result: list[str] = []
    for template in argv:
        arg = template
        for name, value in replacements.items():
            arg = arg.replace(f"{{{name}}}", value)
        result.append(arg)
    return tuple(result)


def execute(argv: Command, cwd: Path, phase: str, evidence: list[Execution]) -> Execution:
    """Run one bounded command and retain its full output before classification."""
    try:
        completed = subprocess.run(
            argv,
            cwd=cwd,
            capture_output=True,
            text=True,
            encoding="utf-8",
            check=False,
            timeout=BUILD_TIMEOUT if phase == "build" else RUN_TIMEOUT,
        )
    except subprocess.TimeoutExpired as exc:
        stdout = (exc.stdout or b"").decode("utf-8", errors="replace")
        stderr = (exc.stderr or b"").decode("utf-8", errors="replace")
        evidence.append(Execution(phase, argv, None, stdout, stderr, timed_out=True))
        raise
    result = Execution(phase, argv, completed.returncode, completed.stdout, completed.stderr)
    evidence.append(result)
    if completed.returncode < 0:
        raise ValueError(f"{phase}: terminated by signal {-completed.returncode}")
    return result


def require_success(result: Execution) -> None:
    """Reject compiler and driver failures instead of treating them as guest errors."""
    if result.returncode != 0:
        raise ValueError(f"{result.phase}: exit {result.returncode}: {result.stderr.strip()}")


def read_observation(result: Execution) -> Observation:
    """Parse exactly one driver observation, not a native process result."""
    require_success(result)
    value: Json = json.loads(result.stdout, object_pairs_hook=unique_members)
    if not isinstance(value, dict) or set(value) != {"termination", "stdout"}:
        raise ValueError(f"{result.phase}: expected termination and stdout fields")
    termination, stdout = value["termination"], value["stdout"]
    if not isinstance(termination, str) or termination not in ("value", "error", "rejected"):
        raise ValueError(f"{result.phase}: invalid termination")
    if not isinstance(stdout, str):
        raise TypeError(f"{result.phase}: stdout must be a string")
    return Observation(termination, stdout)


def run_case(case: Case, driver: Driver, cwd: Path) -> CaseReport:
    """Run one source snapshot against its oracle and every required engine."""
    content = case.source.read_bytes()
    report = CaseReport(case.name, str(case.source), hashlib.sha256(content).hexdigest())
    try:
        with TemporaryDirectory(prefix="modern-sicp-conformance-") as directory:
            work = Path(directory)
            source = work / ("program" + case.source.suffix)
            source.write_bytes(content)
            replacements = {
                "source": str(source),
                "work": str(work),
                "case": case.name,
                "engine": "native",
            }
            # A `.mts` artifact is a native TypeScript module of typed host data
            # (query and machine cases). The driver imports it without type
            # checking, so the reference oracle must still compile it natively.
            if case.oracle == "native" or case.source.suffix == ".mts":
                checked = execute(
                    expand(driver.native_compile, replacements),
                    cwd,
                    "native-compile",
                    report.executions,
                )
                require_success(checked)
            if case.oracle == "native":
                native = execute(
                    expand(driver.native_run, replacements),
                    cwd,
                    "native-run",
                    report.executions,
                )
                report.observations["native"] = Observation(
                    "value" if native.returncode == 0 else "error",
                    native.stdout,
                )
            for engine in case.engines:
                if engine == "native":
                    continue
                argv = expand(driver.run, {**replacements, "engine": engine})
                result = execute(argv, cwd, engine, report.executions)
                report.observations[engine] = read_observation(result)
            expected = report.observations[case.oracle]
            if expected.termination != case.termination:
                raise ValueError(
                    f"oracle returned {expected.termination}; expected {case.termination}",
                )
            for engine, observed in report.observations.items():
                if observed != expected:
                    raise ValueError(f"{engine}: observation differs from {case.oracle}")
    except (OSError, TypeError, ValueError, subprocess.SubprocessError) as exc:
        return replace(report, defect=str(exc))
    return report


def check_edition(root: Path, edition: str, selected: str | None) -> EditionReport:
    """Check full inventory even when execution is restricted to one case."""
    report = EditionReport()
    try:
        inventory = read_inventory(root / "spec" / "host-subsets" / "cases.json")
        cases = read_manifest(root, edition, inventory)
        driver = read_driver(root / "spec" / "host-subsets" / edition / "driver.json")
        if selected is not None and selected not in inventory:
            raise ValueError(f"unknown selected case: {selected}")
        cwd = root / edition
        for argv in driver.build:
            require_success(execute(argv, cwd, "build", report.build))
        for case in cases:
            if selected is not None and case.name != selected:
                continue
            result = run_case(case, driver, cwd)
            report.cases.append(result)
            if result.defect is not None:
                report.defects.append(f"{case.name}: {result.defect}")
    except (OSError, TypeError, ValueError, subprocess.SubprocessError) as exc:
        report.defects.append(str(exc))
    return report


def main(argv: list[str] | None = None) -> int:
    """Report actual conformance results and fail on any missing proof."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path())
    parser.add_argument("--edition", choices=EDITIONS)
    parser.add_argument("--case", dest="case_name")
    parser.add_argument("--report", type=Path)
    args = parser.parse_args(argv)
    root: Path = args.root.resolve()
    editions = EDITIONS if args.edition is None else (args.edition,)
    reports = {edition: check_edition(root, edition, args.case_name) for edition in editions}
    defects = 0
    checked = 0
    for edition, report in reports.items():
        checked += sum(case.defect is None for case in report.cases)
        defects += len(report.defects)
        for defect in report.defects:
            print(f"{edition}: {defect}", file=sys.stderr)
    if args.report is not None:
        args.report.write_text(
            json.dumps({name: asdict(report) for name, report in reports.items()}, indent=2) + "\n",
            encoding="utf-8",
        )
    print(f"editions={len(editions)} cases={checked} defects={defects}")
    return 1 if defects else 0


if __name__ == "__main__":
    raise SystemExit(main())
