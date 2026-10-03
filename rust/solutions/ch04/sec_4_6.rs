// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.6: `let` as a derived
//! expression over typed syntax data.

/// Shared typed support for this exercise.
pub mod support;

use std::collections::HashMap;

#[derive(Clone)]
enum Expr {
    Int(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Lambda {
        parameters: Vec<String>,
        body: Box<Expr>,
    },
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
}

fn let_to_combination(name: &str, value: &Expr, body: &Expr) -> Expr {
    Expr::Call {
        function: Box::new(Expr::Lambda {
            parameters: vec![name.to_owned()],
            body: Box::new(body.clone()),
        }),
        arguments: vec![value.clone()],
    }
}

#[derive(Clone, Copy)]
enum LetEvaluation {
    Direct,
    Derived,
}

fn eval(expr: &Expr, env: &HashMap<String, i64>) -> Option<i64> {
    eval_with_let(expr, env, LetEvaluation::Derived)
}

fn eval_direct(expr: &Expr, env: &HashMap<String, i64>) -> Option<i64> {
    eval_with_let(expr, env, LetEvaluation::Direct)
}

fn eval_with_let(
    expr: &Expr,
    env: &HashMap<String, i64>,
    let_evaluation: LetEvaluation,
) -> Option<i64> {
    match expr {
        Expr::Int(value) => Some(*value),
        Expr::Var(name) => env.get(name).copied(),
        Expr::Add(left, right) => Some(
            eval_with_let(left, env, let_evaluation)? + eval_with_let(right, env, let_evaluation)?,
        ),
        Expr::Call {
            function,
            arguments,
        } => {
            let operands: Option<Vec<i64>> = arguments
                .iter()
                .map(|argument| eval_with_let(argument, env, let_evaluation))
                .collect();
            apply_lambda(function, operands?, env, let_evaluation)
        }
        Expr::Let { name, value, body } => match let_evaluation {
            LetEvaluation::Direct => eval_let(name, value, body, env, let_evaluation),
            LetEvaluation::Derived => {
                eval_with_let(&let_to_combination(name, value, body), env, let_evaluation)
            }
        },
        Expr::Lambda { .. } => None,
    }
}

fn eval_let(
    name: &str,
    value: &Expr,
    body: &Expr,
    env: &HashMap<String, i64>,
    let_evaluation: LetEvaluation,
) -> Option<i64> {
    let initial = eval_with_let(value, env, let_evaluation)?;
    let mut scope = env.clone();
    scope.insert(name.to_owned(), initial);
    eval_with_let(body, &scope, let_evaluation)
}

fn apply_lambda(
    callee: &Expr,
    operands: Vec<i64>,
    env: &HashMap<String, i64>,
    let_evaluation: LetEvaluation,
) -> Option<i64> {
    let Expr::Lambda { parameters, body } = callee else {
        return None;
    };
    if parameters.len() != operands.len() {
        return None;
    }
    let mut scope = env.clone();
    for (name, value) in parameters.iter().cloned().zip(operands) {
        scope.insert(name, value);
    }
    eval_with_let(body, &scope, let_evaluation)
}

#[test]
fn ex_4_06() {
    let mut outer = HashMap::new();
    outer.insert("x".to_owned(), 5);
    let source = Expr::Let {
        name: "x".to_owned(),
        value: Box::new(Expr::Add(
            Box::new(Expr::Var("x".to_owned())),
            Box::new(Expr::Int(1)),
        )),
        body: Box::new(Expr::Add(
            Box::new(Expr::Var("x".to_owned())),
            Box::new(Expr::Int(20)),
        )),
    };
    let Expr::Let { name, value, body } = &source else {
        unreachable!();
    };
    let lowered = let_to_combination(name, value, body);

    assert!(matches!(&lowered, Expr::Call { .. }));
    assert_eq!(eval_direct(&source, &outer), Some(26));
    assert_eq!(eval(&source, &outer), Some(26));
    assert_eq!(eval(&lowered, &outer), Some(26));
}
