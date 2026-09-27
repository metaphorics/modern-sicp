// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.7: separating syntactic analysis from execution.
//! `analyze` walks an expression once and returns an execution
//! procedure, an environment-to-result closure with every dispatch
//! decision already made; executing an analyzed procedure many times
//! never re-analyzes it.

use ch04::sec_4_1::{Analyzer, AnalyzerBase, eval_program, setup_environment};
use sicp_runtime::{Value, read};

fn main() {
    let env = setup_environment();

    // Analyze the definition and one call into execution procedures.
    let definition =
        read("(define (factorial n) (if (= n 1) 1 (* n (factorial (- n 1)))))").expect("read");
    let call = read("(factorial 6)").expect("read");
    let analyzer = AnalyzerBase::new();
    analyzer.eval_exp(&definition, &env).expect("analyzes");
    let exec_call = analyzer.analyze(&call).expect("analyzes");
    println!("{}", exec_call(&env).expect("runs"));
    // => 720
    assert_eq!(exec_call(&env), Ok(Value::int(720)));

    // ...execute many times: the same execution procedure answers each
    // call with no further syntactic work. The base evaluator compares
    // evenly on value, but re-analyzes on every call.
    let base_values = eval_program(&env, "(factorial 6)").expect("runs");
    println!("{}", base_values[0]);
    // => 720
    assert_eq!(base_values[0], Value::int(720));

    // The analyzed evaluator covers the same subset: assignments,
    // sequences, and derived cond.
    let program = read("(define (count-up n) (begin (set! n (+ n 1)) n))").expect("read");
    analyzer.eval_exp(&program, &env).expect("analyzes");
    let call = read("(count-up 41)").expect("read");
    let exec = analyzer.analyze(&call).expect("analyzes");
    println!("{}", exec(&env).expect("runs"));
    // => 42
    assert_eq!(exec(&env), Ok(Value::int(42)));

    let cond_form = read("(cond ((= 1 1) 'yes) (else 'no))").expect("read");
    let exec = analyzer.analyze(&cond_form).expect("analyzes");
    println!("{}", exec(&env).expect("runs"));
    // => yes
    assert_eq!(exec(&env), Ok(Value::sym("yes")));

    // The analyzed evaluator runs the section's benchmark workload;
    // solutions/ch04/sec_4_1.rs times the two against each other.
}
