// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.7: `let*` as nested `let`
//! bindings with sequential scope.

/// Shared typed support for this exercise.
pub mod support;

use std::collections::HashMap;

#[derive(Clone)]
enum Expr {
    Int(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Let(Vec<(String, Expr)>, Vec<Expr>),
}

fn derive_let_star(bindings: Vec<(String, Expr)>, body: Vec<Expr>) -> Expr {
    let mut nested = Expr::Let(Vec::new(), body);
    for (name, value) in bindings.into_iter().rev() {
        nested = Expr::Let(vec![(name, value)], vec![nested]);
    }
    nested
}

fn eval(expr: &Expr, env: &HashMap<String, i64>) -> Option<i64> {
    match expr {
        Expr::Int(value) => Some(*value),
        Expr::Var(name) => env.get(name).copied(),
        Expr::Add(left, right) => Some(eval(left, env)? + eval(right, env)?),
        Expr::Let(bindings, body) => {
            let mut scope = env.clone();
            for (name, value) in bindings {
                scope.insert(name.clone(), eval(value, &scope)?);
            }
            eval_all(body, &scope)
        }
    }
}

fn eval_all(items: &[Expr], env: &HashMap<String, i64>) -> Option<i64> {
    let mut result = None;
    for item in items {
        result = Some(eval(item, env)?);
    }
    result
}

#[test]
fn ex_4_07() {
    let derived = derive_let_star(
        vec![
            ("x".to_owned(), Expr::Int(3)),
            ("y".to_owned(), Expr::Int(0)),
            (
                "z".to_owned(),
                Expr::Add(
                    Box::new(Expr::Var("x".to_owned())),
                    Box::new(Expr::Var("y".to_owned())),
                ),
            ),
        ],
        vec![Expr::Var("z".to_owned())],
    );
    assert_eq!(eval(&derived, &HashMap::new()), Some(3));
}
