// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercises 2.73 and 2.73a.

mod ex_2_73 {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt;
    use std::rc::Rc;

    use sicp_runtime::{Key, SchemeError, Symbol, Value};

    /// The exercise's own expression enum: `Pow` is part (c)'s
    /// contribution, and `Atan` sits dormant here — 2.73a installs its
    /// rule in a separate table, but the enum carries the case so its
    /// `operator`/`operands` mapping is total.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) enum Expr {
        Num(i128),
        Var(Symbol),
        Sum(Box<Expr>, Box<Expr>),
        Product(Box<Expr>, Box<Expr>),
        Pow(Box<Expr>, Box<Expr>),
        Atan(Box<Expr>),
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expr::Num(n) => write!(f, "{n}"),
                Expr::Var(x) => write!(f, "{x}"),
                Expr::Sum(a, b) => write!(f, "(+ {a} {b})"),
                Expr::Product(a, b) => write!(f, "(* {a} {b})"),
                Expr::Pow(base, exp) => write!(f, "(** {base} {exp})"),
                Expr::Atan(u) => write!(f, "(atan {u})"),
            }
        }
    }

    /// A variable expression, for building probes in the tests.
    pub(crate) fn variable(name: &str) -> Expr {
        Expr::Var(Symbol::from(name))
    }

    fn is_number(exp: &Expr, n: i128) -> bool {
        matches!(exp, Expr::Num(x) if *x == n)
    }

    pub(crate) fn make_sum(a1: Expr, a2: Expr) -> Result<Expr, SchemeError> {
        if is_number(&a1, 0) {
            return Ok(a2);
        }
        if is_number(&a2, 0) {
            return Ok(a1);
        }
        if let (Expr::Num(x), Expr::Num(y)) = (&a1, &a2) {
            return Ok(Expr::Num(x.checked_add(*y).ok_or(SchemeError::Overflow)?));
        }
        Ok(Expr::Sum(Box::new(a1), Box::new(a2)))
    }

    pub(crate) fn make_product(m1: Expr, m2: Expr) -> Result<Expr, SchemeError> {
        if is_number(&m1, 0) || is_number(&m2, 0) {
            return Ok(Expr::Num(0));
        }
        if is_number(&m1, 1) {
            return Ok(m2);
        }
        if is_number(&m2, 1) {
            return Ok(m1);
        }
        if let (Expr::Num(x), Expr::Num(y)) = (&m1, &m2) {
            return Ok(Expr::Num(x.checked_mul(*y).ok_or(SchemeError::Overflow)?));
        }
        Ok(Expr::Product(Box::new(m1), Box::new(m2)))
    }

    /// Exercise 2.56's exponentiation constructor: `u^0 = 1`, `u^1 = u`,
    /// otherwise a symbolic power.
    pub(crate) fn make_exponentiation(u: Expr, n: Expr) -> Expr {
        if is_number(&n, 0) {
            return Expr::Num(1);
        }
        if is_number(&n, 1) {
            return u;
        }
        Expr::Pow(Box::new(u), Box::new(n))
    }

    /// The operator symbol of an application variant, the book's
    /// `(operator exp)`. `Num` and `Var` answer `None`: part (a)'s fact —
    /// a number or a variable has no operator to index the table by — is
    /// visible directly in this function's return type, not asserted
    /// after the fact.
    pub(crate) fn operator(exp: &Expr) -> Option<&'static str> {
        match exp {
            Expr::Sum(..) => Some("+"),
            Expr::Product(..) => Some("*"),
            Expr::Pow(..) => Some("**"),
            Expr::Atan(..) => Some("atan"),
            Expr::Num(_) | Expr::Var(_) => None,
        }
    }

    /// The operand expressions of an application variant, the book's
    /// `(operands exp)`: two for the binary operators, one for `atan`.
    pub(crate) fn operands(exp: &Expr) -> Vec<&Expr> {
        match exp {
            Expr::Sum(a, b) | Expr::Product(a, b) | Expr::Pow(a, b) => vec![a, b],
            Expr::Atan(u) => vec![u],
            Expr::Num(_) | Expr::Var(_) => vec![],
        }
    }

    fn unknown_expression(exp: &Expr) -> SchemeError {
        SchemeError::UserRaised {
            message: "unknown expression type: DERIV".into(),
            irritants: vec![Value::string(&exp.to_string())],
        }
    }

    /// A `deriv` rule: given the table (so it can recurse into `deriv`),
    /// the operand expressions, and the differentiation variable,
    /// produces the derivative.
    pub(crate) type DerivHandler =
        Rc<dyn Fn(&DerivTable, &[&Expr], &Symbol) -> Result<Expr, SchemeError>>;

    /// The exercise's own operation table: the same `(op, tag) -> handler`
    /// shape as the section's `OpTable`, keyed here by `("deriv", operator)`
    /// (or, flipped in part (d), by `(operator, "deriv")`), holding the
    /// typed handler this exercise's `Expr` needs rather than the
    /// section's `Value`-based one. A struct, not a bare type alias, since
    /// a handler's own signature names this table.
    pub(crate) struct DerivTable(RefCell<HashMap<(Key, Key), DerivHandler>>);

    /// An empty table, ready for `deriv_put`.
    pub(crate) fn deriv_table() -> DerivTable {
        DerivTable(RefCell::new(HashMap::new()))
    }

    /// The book's `put` for this table: installs `handler` under
    /// `(op, tag)`, overwriting any earlier install of exactly that pair.
    pub(crate) fn deriv_put(table: &DerivTable, op: &str, tag: &str, handler: DerivHandler) {
        table
            .0
            .borrow_mut()
            .insert((Key::sym(op), Key::sym(tag)), handler);
    }

    /// The book's `get`: the handler under `(op, tag)`, or the absent
    /// option on a miss.
    fn deriv_get(table: &DerivTable, op: &str, tag: &str) -> Option<DerivHandler> {
        table
            .0
            .borrow()
            .get(&(Key::sym(op), Key::sym(tag)))
            .cloned()
    }

    /// Part (b): the data-directed `deriv` of the exercise statement.
    /// `Num` and `Var` stay `match` arms — they carry no operator — and
    /// every other variant dispatches through the table indexed
    /// `("deriv", operator)`.
    pub(crate) fn deriv(exp: &Expr, var: &Symbol, table: &DerivTable) -> Result<Expr, SchemeError> {
        match exp {
            Expr::Num(_) => Ok(Expr::Num(0)),
            Expr::Var(x) => Ok(Expr::Num(i128::from(x == var))),
            _ => {
                let Some(op) = operator(exp) else {
                    return Err(unknown_expression(exp));
                };
                let Some(handler) = deriv_get(table, "deriv", op) else {
                    return Err(unknown_expression(exp));
                };
                handler(table, &operands(exp), var)
            }
        }
    }

    /// Part (d): the dispatch indexed the opposite way, `(operator, "deriv")`.
    /// Only the two key arguments to `put` and `get` swap; every rule and
    /// every constructor is unchanged.
    pub(crate) fn deriv_flipped(
        exp: &Expr,
        var: &Symbol,
        table: &DerivTable,
    ) -> Result<Expr, SchemeError> {
        match exp {
            Expr::Num(_) => Ok(Expr::Num(0)),
            Expr::Var(x) => Ok(Expr::Num(i128::from(x == var))),
            _ => {
                let Some(op) = operator(exp) else {
                    return Err(unknown_expression(exp));
                };
                let Some(handler) = deriv_get(table, op, "deriv") else {
                    return Err(unknown_expression(exp));
                };
                handler(table, &operands(exp), var)
            }
        }
    }

    fn sum_handler() -> DerivHandler {
        Rc::new(|table, ops, var| make_sum(deriv(ops[0], var, table)?, deriv(ops[1], var, table)?))
    }

    fn product_handler() -> DerivHandler {
        Rc::new(|table, ops, var| {
            let (m, n) = (ops[0], ops[1]);
            let left = make_product(m.clone(), deriv(n, var, table)?)?;
            let right = make_product(deriv(m, var, table)?, n.clone())?;
            make_sum(left, right)
        })
    }

    /// Part (c): the exponentiation rule of exercise 2.56,
    /// `d(u^n)/dx = n * u^(n-1) * du/dx`, valid for a literal integer `n`.
    fn pow_handler() -> DerivHandler {
        Rc::new(|table, ops, var| {
            let (u, n) = (ops[0], ops[1]);
            let Expr::Num(k) = n else {
                return Err(SchemeError::TypeMismatch(
                    "the exponent must be a literal number".into(),
                ));
            };
            let n_minus_one = k.checked_sub(1).ok_or(SchemeError::Overflow)?;
            let lower = make_exponentiation(u.clone(), Expr::Num(n_minus_one));
            let du = deriv(u, var, table)?;
            make_product(Expr::Num(*k), make_product(lower, du)?)
        })
    }

    /// Installs the sum rule under `("deriv", "+")`.
    pub(crate) fn install_sum_rule(table: &DerivTable) {
        deriv_put(table, "deriv", "+", sum_handler());
    }

    /// Installs the product rule under `("deriv", "*")`.
    pub(crate) fn install_product_rule(table: &DerivTable) {
        deriv_put(table, "deriv", "*", product_handler());
    }

    /// Installs the exponentiation rule under `("deriv", "**")`.
    pub(crate) fn install_pow_rule(table: &DerivTable) {
        deriv_put(table, "deriv", "**", pow_handler());
    }

    /// Installs the same three rules flipped, under `(operator, "deriv")`.
    pub(crate) fn install_flipped_rules(table: &DerivTable) {
        deriv_put(table, "+", "deriv", sum_handler());
        deriv_put(table, "*", "deriv", product_handler());
        deriv_put(table, "**", "deriv", pow_handler());
    }

    /// Installs the atan rule under `("deriv", "atan")`: 2.73a's
    /// contribution, kept here so [`super::ex_2_73a`] can install and use
    /// it without duplicating the table or the constructors.
    pub(crate) fn install_atan_rule(table: &DerivTable) {
        deriv_put(
            table,
            "deriv",
            "atan",
            Rc::new(|table, ops, var| {
                let u = ops[0];
                let du = deriv(u, var, table)?;
                let denominator =
                    make_sum(Expr::Num(1), make_exponentiation(u.clone(), Expr::Num(2)))?;
                let reciprocal = make_exponentiation(denominator, Expr::Num(-1));
                make_product(du, reciprocal)
            }),
        );
    }

    fn probe_table() -> DerivTable {
        let table = deriv_table();
        install_sum_rule(&table);
        install_product_rule(&table);
        install_pow_rule(&table);
        table
    }

    /// Exercise 2.73: the printed derivatives of parts (b) and (c) on the
    /// book's probes, `(+ x 3)`, `(* x y)`, and `(** x 3)`, with respect
    /// to `x`.
    pub fn ex_2_73() -> (String, String, String) {
        let table = probe_table();
        let x = Symbol::from("x");
        let sum = Expr::Sum(Box::new(variable("x")), Box::new(Expr::Num(3)));
        let product = Expr::Product(Box::new(variable("x")), Box::new(variable("y")));
        let power = Expr::Pow(Box::new(variable("x")), Box::new(Expr::Num(3)));
        (
            deriv(&sum, &x, &table)
                .expect("sum rule installed")
                .to_string(),
            deriv(&product, &x, &table)
                .expect("product rule installed")
                .to_string(),
            deriv(&power, &x, &table)
                .expect("pow rule installed")
                .to_string(),
        )
    }

    #[cfg(test)]
    pub(crate) mod support {
        pub(crate) use super::{
            Expr, deriv, deriv_flipped, deriv_table, install_flipped_rules, variable,
        };
    }
}

mod ex_2_73a {
    use super::ex_2_73::{
        Expr, deriv, deriv_table, install_atan_rule, install_pow_rule, install_product_rule,
        install_sum_rule, variable,
    };
    use sicp_runtime::Symbol;

    /// Exercise 2.73a (this edition's addition): installs the atan rule,
    /// `d(arctan u)/dx = du/dx / (1 + u^2)`, represented without division
    /// as `du * (1 + u^2)^-1`, as a new `Expr` variant with its own table
    /// entry — no existing rule or constructor changes.
    pub fn ex_2_73a() -> String {
        let table = deriv_table();
        install_sum_rule(&table);
        install_product_rule(&table);
        install_pow_rule(&table);
        install_atan_rule(&table);
        let x = Symbol::from("x");
        deriv(&Expr::Atan(Box::new(variable("x"))), &x, &table)
            .expect("atan rule installed")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::ex_2_73::support::*;
    use sicp_runtime::Symbol;

    #[test]
    fn ex_2_73() {
        assert_eq!(
            super::ex_2_73::ex_2_73(),
            (
                "1".to_string(),
                "y".to_string(),
                "(* 3 (** x 2))".to_string()
            )
        );
    }

    #[test]
    fn part_a_residual_arms_handle_numbers_and_variables() {
        let table = deriv_table();
        let x = Symbol::from("x");
        assert_eq!(deriv(&Expr::Num(5), &x, &table), Ok(Expr::Num(0)));
        assert_eq!(deriv(&variable("x"), &x, &table), Ok(Expr::Num(1)));
        assert_eq!(deriv(&variable("y"), &x, &table), Ok(Expr::Num(0)));
        // An application variant with no installed rule still reaches the
        // table lookup, not a missing match arm: the error names the
        // book's message.
        let err = deriv(&Expr::Atan(Box::new(variable("x"))), &x, &table)
            .expect_err("no atan rule installed here");
        assert!(
            err.to_string()
                .starts_with("unknown expression type: DERIV")
        );
    }

    #[test]
    fn part_d_flipped_indexing_gives_the_same_answers() {
        let table = deriv_table();
        install_flipped_rules(&table);
        let x = Symbol::from("x");
        let sum = Expr::Sum(Box::new(variable("x")), Box::new(Expr::Num(3)));
        let product = Expr::Product(Box::new(variable("x")), Box::new(variable("y")));
        let power = Expr::Pow(Box::new(variable("x")), Box::new(Expr::Num(3)));
        assert_eq!(
            deriv_flipped(&sum, &x, &table)
                .expect("flipped sum")
                .to_string(),
            "1"
        );
        assert_eq!(
            deriv_flipped(&product, &x, &table)
                .expect("flipped product")
                .to_string(),
            "y"
        );
        assert_eq!(
            deriv_flipped(&power, &x, &table)
                .expect("flipped pow")
                .to_string(),
            "(* 3 (** x 2))"
        );
        // The un-flipped table has no entry under the flipped keys.
        let err = deriv_flipped(&sum, &x, &deriv_table()).expect_err("empty flipped table");
        assert!(
            err.to_string()
                .starts_with("unknown expression type: DERIV")
        );
    }

    #[test]
    fn ex_2_73a() {
        assert_eq!(super::ex_2_73a::ex_2_73a(), "(** (+ 1 (** x 2)) -1)");
    }

    #[test]
    fn atan_of_a_sum_differentiates_through_the_chain_rule() {
        let table = deriv_table();
        super::ex_2_73::install_sum_rule(&table);
        super::ex_2_73::install_atan_rule(&table);
        let x = Symbol::from("x");
        let u = Expr::Sum(
            Box::new(variable("x")),
            Box::new(super::ex_2_73::Expr::Num(3)),
        );
        let derivative = deriv(&Expr::Atan(Box::new(u)), &x, &table).expect("atan rule installed");
        assert_eq!(derivative.to_string(), "(** (+ 1 (** (+ x 3) 2)) -1)");
    }
}
