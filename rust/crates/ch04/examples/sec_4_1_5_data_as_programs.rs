// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.5: data as programs. The evaluator is fed the
//! definition of `factorial` as checked source and emulates the
//! factorial machine; the same evaluator runs any other machine
//! description we feed it, which is what makes it a universal machine.

use ch04::sec_4_1::run_program;
use sicp_runtime::host::ast::{ExprKind, Item, Stmt};
use sicp_runtime::host::parse_program;

const FACTORIAL: &str = "\
fn factorial(n: i64) -> i64 {
    if n == 1 {
        1
    } else {
        factorial(n - 1) * n
    }
}

fn main() {
    println!(\"{}\", factorial(6));
    println!(\"{}\", factorial(10));
}
";

const FIB: &str = "\
fn fib(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    println!(\"{}\", fib(12));
}
";

fn main() {
    // The factorial program as the description of a machine: feed it
    // to the evaluator and the evaluator computes factorials.
    let output = run_program(FACTORIAL);
    println!("{output}");
    // => 720
    // => 3628800
    assert_eq!(output, "720\n3628800\n");

    // The same evaluator emulates a different machine from data: the
    // Fibonacci machine of section 1.2.2.
    let output = run_program(FIB);
    println!("{output}");
    // => 144
    assert_eq!(output, "144\n");

    // And the program's own syntax is host data the front end can take
    // apart: the printed call's head is its function name and its
    // argument is the operand.
    let program = parse_program(FACTORIAL).expect("parses");
    let entry = program
        .items
        .iter()
        .find_map(|item| match item {
            Item::Fn(function) if function.name.name == "main" => Some(function),
            _ => None,
        })
        .expect("a main item");
    let printed = entry
        .body
        .stmts
        .iter()
        .find_map(|stmt| match stmt {
            Stmt::Expr { expr, .. } => match &expr.kind {
                ExprKind::Format { args, .. } => args.first(),
                _ => None,
            },
            Stmt::Let(_) => None,
        })
        .expect("one printed call");
    let ExprKind::Call {
        callee,
        args: operands,
    } = &printed.kind
    else {
        panic!("a factorial call");
    };
    let ExprKind::Path(path) = &callee.kind else {
        panic!("a named callee");
    };
    let name = path.first().expect("one name").name.as_str();
    println!("head={} operands={}", name, operands.len());
    // => head=factorial operands=1
    assert_eq!((name, operands.len()), ("factorial", 1));
}
