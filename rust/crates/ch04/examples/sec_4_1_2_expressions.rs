// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.2: representing expressions. The front end answers a
//! located syntax tree whose constructors classify each form — the
//! book's syntax predicates become `match` arms over typed data — and
//! a derived surface form is rewritten to a core form before the
//! evaluator sees it, evaluating exactly the same either way.

use ch04::sec_4_1::run_source;
use sicp_runtime::host::ast::{Expr, ExprKind, Item, Stmt};
use sicp_runtime::host::parse_program;

/// Whether one expression is a call form.
fn is_call(expr: &Expr) -> bool {
    matches!(expr.kind, ExprKind::Call { .. })
}

/// Whether one expression is a format-macro form.
fn is_format(expr: &Expr) -> bool {
    matches!(expr.kind, ExprKind::Format { .. })
}

/// One statement's form and the expression shapes it carries.
fn shape(stmt: &Stmt) -> String {
    let expr = match stmt {
        Stmt::Let(binding) => &binding.value,
        Stmt::Expr { expr, .. } => expr,
    };
    format!(
        "{}:call={} format={}",
        match stmt {
            Stmt::Let(_) => "let",
            Stmt::Expr { .. } => "expr",
        },
        is_call(expr),
        is_format(expr)
    )
}

const DERIVED: &str = "\
fn main() {
    let answer = Some(30);
    if let Some(v) = answer {
        println!(\"{}\", v);
    }
}
";

const CORE: &str = "\
fn main() {
    let answer = Some(30);
    match answer {
        Some(v) => {
            println!(\"{}\", v);
        }
        None => {}
    }
}
";

fn main() {
    // The syntax predicates recognize the forms by their constructors.
    let program = parse_program(
        "fn id(x: i64) -> i64 {\n    x\n}\n\nfn main() {\n    let n = 21;\n    let m = id(n);\n    println!(\"{}\", m);\n}\n",
    )
    .expect("parses");
    let Item::Fn(main_fn) = &program.items[1] else {
        panic!("two function items");
    };
    let shapes = main_fn.body.stmts.iter().map(shape).collect::<Vec<_>>();
    println!("{}", shapes.join(" "));
    // => let:call=false format=false let:call=true format=false expr:call=false format=true
    assert_eq!(
        shapes,
        [
            "let:call=false format=false",
            "let:call=true format=false",
            "expr:call=false format=true",
        ]
    );

    // A form is data to take apart: the call's head is its callee name
    // and its arguments are the operands, in order.
    let Some(ExprKind::Call { callee, args }) =
        main_fn.body.stmts.iter().find_map(|stmt| match stmt {
            Stmt::Let(binding) if is_call(&binding.value) => Some(&binding.value.kind),
            Stmt::Expr { expr, .. } if is_call(expr) => Some(&expr.kind),
            _ => None,
        })
    else {
        panic!("one call form");
    };
    let ExprKind::Path(head) = &callee.kind else {
        panic!("a named callee");
    };
    let name = head.first().expect("one name").name.as_str();
    println!("callee={} args={}", name, args.len());
    // => callee=id args=1
    assert_eq!((name, args.len()), ("id", 1));

    // The derived-expression rewrite the section shows: `if let` is a
    // surface convenience that rewrites to the `match` core form, and
    // the rewrite evaluates the same as the original.
    let derived = run_source(DERIVED).expect("admitted");
    let core = run_source(CORE).expect("admitted");
    println!("{}", derived.stdout);
    // => 30
    assert_eq!(derived.stdout, "30\n");
    assert_eq!(derived.stdout, core.stdout);
}
