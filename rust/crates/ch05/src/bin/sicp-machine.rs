// SPDX-License-Identifier: GPL-3.0-only
//! The edition teaching driver for the explicit-control evaluator, the
//! compiler, and the named machine cases. Stdout is exactly one JSON
//! object `{"termination": ..., "stdout": ...}`; diagnostics go to
//! stderr.

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

/// The machine-case schedule: constructor, input registers, and the
/// register whose final value the controller reports.
fn machine_setup(case: &str) -> (sec_5_1::MachineProgram, Vec<(String, MachineValue)>, String) {
    match case {
        "machine/01-gcd" | "machine/04-gcd-print" => (
            sec_5_1::gcd_machine(),
            vec![("a".to_owned(), 206), ("b".to_owned(), 40)],
            "a".to_owned(),
        ),
        "machine/02-factorial-recursive" => (
            sec_5_1::factorial_recursive(),
            vec![("n".to_owned(), 6)],
            "val".to_owned(),
        ),
        "machine/03-fibonacci" => (
            sec_5_1::fibonacci_machine(),
            vec![("n".to_owned(), 6)],
            "val".to_owned(),
        ),
        other => {
            eprintln!("unknown machine case: {other}");
            std::process::exit(2);
        }
    }
}

/// The independent machine reference: the taught computation derived
/// directly, never through the simulator.
fn machine_reference(case: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let result = match case {
        "machine/01-gcd" | "machine/04-gcd-print" => {
            let (mut a, mut b) = (206_i64, 40_i64);
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }
        "machine/02-factorial-recursive" => {
            let mut product = 1_i64;
            let mut counter = 1_i64;
            while counter <= 6 {
                product *= counter;
                counter += 1;
            }
            product
        }
        "machine/03-fibonacci" => {
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
        other => {
            eprintln!("unknown machine case: {other}");
            std::process::exit(2);
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
            let (program, inputs, primary) = machine_setup(case);
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
        "reference" if case.starts_with("machine/") => {
            emit("value", &machine_reference(case).join("\n"));
        }
        other => {
            eprintln!("unknown engine: {other}");
            std::process::exit(2);
        }
    }
}
