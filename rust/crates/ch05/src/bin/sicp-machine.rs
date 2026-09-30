// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//! The edition teaching driver for the explicit-control evaluator, the
//! compiler, and the named machine cases. Stdout is exactly one JSON
//! object `{"termination": ..., "stdout": ...}`; diagnostics go to
//! stderr.

#[path = "../../../../../spec/host-subsets/rust/case_artifact.rs"]
mod case_artifact;

use ch05::sec_5_1;
use ch05::sec_5_2::{Machine, MachineValue, assemble, standard_operations};
use ch05::sec_5_4::run_session;
use ch05::sec_5_5::compiled_run;
use std::fmt::Write as _;

fn escape(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if u32::from(control) < 0x20 => {
                // Formatting into `String` cannot fail.
                let _ = write!(&mut out, "\\u{:04x}", u32::from(control));
            }
            other => out.push(other),
        }
    }
    out
}

fn emit(termination: &str, transcript: &str) {
    println!(
        "{{\"termination\":\"{}\",\"stdout\":\"{}\"}}",
        termination,
        escape(transcript)
    );
}

fn outcome_lines(outcome: &sicp_runtime::host::ops::RunOutcome) -> (&'static str, String) {
    if let Some(trap) = &outcome.trap {
        eprintln!("{trap:?}");
        ("error", outcome.stdout.clone())
    } else {
        ("value", outcome.stdout.clone())
    }
}

fn artifact_constructor(source: &str, case: &str) -> String {
    case_artifact::read_constructor(source, case).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2);
    })
}

#[derive(Clone, Copy)]
enum MachineKind {
    Gcd,
    Factorial,
    Fibonacci,
}

fn machine_kind(constructor: &str) -> Option<MachineKind> {
    match constructor {
        "gcd_machine" => Some(MachineKind::Gcd),
        "factorial_recursive" => Some(MachineKind::Factorial),
        "fibonacci_machine" => Some(MachineKind::Fibonacci),
        _ => None,
    }
}

/// The machine-case schedule: constructor, input registers, and the
/// register whose final value the controller reports.
fn machine_setup(
    kind: MachineKind,
) -> (sec_5_1::MachineProgram, Vec<(String, MachineValue)>, String) {
    match kind {
        MachineKind::Gcd => (
            sec_5_1::gcd_machine(),
            vec![("a".to_owned(), 206), ("b".to_owned(), 40)],
            "a".to_owned(),
        ),
        MachineKind::Factorial => (
            sec_5_1::factorial_recursive(),
            vec![("n".to_owned(), 6)],
            "val".to_owned(),
        ),
        MachineKind::Fibonacci => (
            sec_5_1::fibonacci_machine(),
            vec![("n".to_owned(), 6)],
            "val".to_owned(),
        ),
    }
}

/// The independent machine reference: the taught computation derived
/// directly, never through the simulator.
fn machine_reference(kind: MachineKind) -> Vec<String> {
    let mut lines = Vec::new();
    let result = match kind {
        MachineKind::Gcd => {
            let (mut a, mut b) = (206_i64, 40_i64);
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }
        MachineKind::Factorial => {
            let mut product = 1_i64;
            let mut counter = 1_i64;
            while counter <= 6 {
                product *= counter;
                counter += 1;
            }
            product
        }
        MachineKind::Fibonacci => {
            let (mut previous, mut current) = (0_i64, 1_i64);
            let mut counter = 0_i64;
            while counter < 6 {
                let next = previous + current;
                previous = current;
                current = next;
                counter += 1;
            }
            previous
        }
    };
    // Every lesson controller ends with `print` of its result
    // register: the observable effect is the rendered result.
    lines.push(result.to_string());
    lines.push(format!("result={result}"));
    lines
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: sicp-machine <engine> <case> <source>");
        std::process::exit(2);
    }
    let (engine, case, source) = (args[1].as_str(), args[2].as_str(), args[3].as_str());
    match engine {
        "eceval" | "compiled" => {
            let text = match std::fs::read_to_string(source) {
                Ok(text) => text,
                Err(error) => {
                    eprintln!("cannot read {source}: {error}");
                    std::process::exit(2);
                }
            };
            let outcome = match engine {
                "eceval" => run_session(&text),
                _ => match sicp_runtime::host::admit(&text) {
                    Ok(program) => Ok(compiled_run(&program)),
                    Err(diag) => Err(diag),
                },
            };
            match outcome {
                Ok(outcome) => {
                    let (termination, transcript) = outcome_lines(&outcome);
                    emit(termination, &transcript);
                }
                Err(diag) => {
                    eprintln!("{diag:?}");
                    emit("rejected", "");
                }
            }
        }
        "machine" => {
            let constructor = artifact_constructor(source, case);
            let Some(kind) = machine_kind(&constructor) else {
                eprintln!("unknown machine constructor: {constructor}");
                std::process::exit(2);
            };
            let (program, inputs, primary) = machine_setup(kind);
            let assembled = match assemble(&program) {
                Ok(assembled) => assembled,
                Err(fault) => {
                    eprintln!("assemble failed: {fault:?}");
                    std::process::exit(2);
                }
            };
            let mut machine = Machine::new(assembled);
            for (name, op) in standard_operations() {
                machine.install_operation(&name, op);
            }
            for (name, value) in &inputs {
                if let Err(fault) = machine.set_register(name, *value) {
                    eprintln!("set_register {name} failed: {fault:?}");
                    std::process::exit(2);
                }
            }
            match machine.run() {
                Ok(run) => {
                    let mut lines = run.output.clone();
                    let result = run.registers.get(&primary).copied().unwrap_or(0);
                    lines.push(format!("result={result}"));
                    emit("value", &lines.join("\n"));
                }
                Err(fault) => {
                    eprintln!("machine fault: {fault:?}");
                    emit("error", "");
                }
            }
        }
        "reference" => {
            let constructor = artifact_constructor(source, case);
            let Some(kind) = machine_kind(&constructor) else {
                eprintln!("unknown machine constructor: {constructor}");
                std::process::exit(2);
            };
            emit("value", &machine_reference(kind).join("\n"));
        }
        other => {
            eprintln!("unknown engine: {other}");
            std::process::exit(2);
        }
    }
}
