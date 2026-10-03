#!/usr/bin/env python3
"""The Rust edition conformance dispatcher.

Runs from the edition root (rust/). Selects the teaching binary for
the requested engine and passes its observation through unchanged:
the binary prints exactly one JSON object on stdout and diagnostics on
stderr. Python standard library only; no shell.
"""

import argparse
import os
import subprocess
import sys


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--engine", required=True)
    parser.add_argument("--case", required=True)
    parser.add_argument("--source", required=True)
    parser.add_argument("--work", required=True)
    arguments = parser.parse_args()

    machine_side = arguments.case.startswith("machine/") or arguments.engine in (
        "eceval",
        "compiled",
        "machine",
    )
    binary = os.path.join("target", "debug", "sicp-machine" if machine_side else "sicp-eval")
    completed = subprocess.run(
        [binary, arguments.engine, arguments.case, arguments.source],
        capture_output=True,
        text=True,
        check=False,
    )
    sys.stdout.write(completed.stdout)
    sys.stderr.write(completed.stderr)
    return completed.returncode


if __name__ == "__main__":
    sys.exit(main())
