// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 4.1 given machinery: the guest-source front end and the
//! corpus printer over the section's syntax. The front end is the
//! book's `read`: it turns source text into located syntax data and
//! reports the contract's rejection classes before any effect runs.
//! The printer is the one rendering contract the corpus conformance
//! output is held to.

use sicp_runtime::host::ast::{ExprKind, Item, Stmt};
use sicp_runtime::host::diag::DiagKind;
use sicp_runtime::host::value::{HostValue, render_debug, render_display};
use sicp_runtime::host::{Tok, TokKind, admit, lex, parse_program};

/// One token's class and payload: the front end's atom inventory.
fn atom(tok: &Tok) -> String {
    match &tok.kind {
        TokKind::Int { text, .. } => format!("int:{text}"),
        TokKind::Str(text) => format!("str:{text}"),
        TokKind::Ident(name) => format!("ident:{name}"),
        TokKind::Float => String::from("float"),
        other => format!("{other:?}"),
    }
}

/// One binding initializer's constructor class.
fn shape(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Let(binding) => match &binding.value.kind {
            ExprKind::Tuple(..) => String::from("pair"),
            ExprKind::Array(items) => format!("array[{}]", items.len()),
            other => format!("{other:?}"),
        },
        Stmt::Expr { .. } => String::from("expr"),
    }
}

fn main() {
    // Atoms: an integer, a string, an identifier, and a float each lex
    // to one located token; comments and whitespace are atmosphere.
    let atoms = lex("42 \"word\" value 2.5 // trailing")
        .expect("lexes")
        .iter()
        .map(atom)
        .collect::<Vec<_>>();
    println!("{}", atoms.join(" "));
    // => int:42 str:word ident:value float
    assert_eq!(atoms, ["int:42", "str:word", "ident:value", "float"]);

    // A float is host-valid but excluded from the subset: admission
    // reports the contract's exclusion class before any effect.
    let diag = admit("fn main() { let x = 2.5; }").expect_err("floats are excluded");
    println!("{:?}", diag.kind);
    // => Unsupported
    assert_eq!(diag.kind, DiagKind::Unsupported);

    // A string's debug form is quoted and escaped; its display form is
    // bare.
    let text = HostValue::Text(String::from("a \"quoted\" word"));
    println!("{}", render_debug(&text, &[]));
    // => "a \"quoted\" word"
    assert_eq!(render_debug(&text, &[]), "\"a \\\"quoted\\\" word\"");
    println!("{}", render_display(&text));
    // => a "quoted" word
    assert_eq!(render_display(&text), "a \"quoted\" word");

    // Pairs and sequences round-trip as syntax data: the front end
    // builds the constructors and the program prints the structures
    // they name.
    let shapes = parse_program("fn main() {\n    let pair = (1, 2);\n    let items = [3, 4];\n}\n")
        .expect("parses")
        .items
        .iter()
        .flat_map(|item| match item {
            Item::Fn(function) => function.body.stmts.as_slice(),
            _ => panic!("one function item"),
        })
        .map(shape)
        .collect::<Vec<_>>();
    println!("{}", shapes.join(" "));
    // => pair array[2]
    assert_eq!(shapes, ["pair", "array[2]"]);

    // A program is a sequence of items; each item's name is data the
    // front end already carries in its located syntax tree.
    let program =
        parse_program("// header\nfn first() {}\nfn second() {}\n// trailing\n").expect("parses");
    println!("items={}", program.items.len());
    // => items=2
    assert_eq!(program.items.len(), 2);
    let rendered = program
        .items
        .iter()
        .map(|item| match item {
            Item::Fn(function) => function.name.name.clone(),
            other => panic!("two function items, found {other:?}"),
        })
        .collect::<Vec<_>>()
        .join(", ");
    println!("{rendered}");
    // => first, second
    assert_eq!(rendered, "first, second");
}
