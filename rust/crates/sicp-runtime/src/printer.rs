// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The corpus printer of `spec/scheme-subset/printer.md`: the one
//! renderer every edition's conformance output is compared against.
//! [`print_value`](crate::printer::print_value) is the value form the
//! top-level prints (`#t`/`#f`, strings double-quoted with `\"` and
//! `\\` escapes only, proper lists `(1 2 3)`, dotted pairs `(a . b)`,
//! the empty list `()`, and procedures as `#[primitive-procedure name]`
//! and `#[compound-procedure name]`); [`display_value`] is the form the
//! object language's `display` writes, the same rules except that a
//! string carries no quotes and no escapes.

use std::fmt::Write as _;

use crate::value::Value;

/// The two string renderings the printer contract fixes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Form {
    /// The value form: strings quoted and escaped.
    Printed,
    /// The `display` form: strings bare.
    Displayed,
}

impl Form {
    /// Renders one string under this form.
    fn string(self, s: &str) -> String {
        match self {
            Self::Printed => {
                let mut out = String::with_capacity(s.len() + 2);
                out.push('"');
                escape_into(&mut out, s);
                out.push('"');
                out
            }
            Self::Displayed => s.to_owned(),
        }
    }

    /// Renders one boolean under this form.
    fn boolean(b: bool) -> String {
        if b { "#t".to_owned() } else { "#f".to_owned() }
    }
}

/// Renders `v` in the printer's value form.
#[must_use]
pub fn print_value(v: &Value) -> String {
    render(v, Form::Printed)
}

/// Renders `v` the way `display` prints it: strings bare.
#[must_use]
pub fn display_value(v: &Value) -> String {
    render(v, Form::Displayed)
}

fn render(v: &Value, form: Form) -> String {
    match v {
        Value::Int(n) => n.to_string(),
        Value::Real(x) => float_string(*x),
        Value::Bool(b) => Form::boolean(*b),
        Value::Sym(s) => s.to_string(),
        Value::Str(s) => form.string(s),
        Value::Nil => "()".to_owned(),
        Value::Pair(_) => render_pair(v, form),
        Value::Tagged { tag, data } => {
            if data.is_nil() {
                format!("({tag})")
            } else {
                format!("({tag} {})", render(data, form))
            }
        }
        Value::Primitive { name, .. } => format!("#[primitive-procedure {name}]"),
        Value::Closure(c) => match &c.name {
            Some(name) => format!("#[compound-procedure {name}]"),
            None => "#[compound-procedure]".to_owned(),
        },
        Value::Thunk(_) => "#[thunk]".to_owned(),
        Value::CompiledProc(p) => format!("#[compiled-procedure {}]", p.entry),
    }
}

/// Renders one pair chain: `(1 2 3)` across a proper list,
/// `(a b . c)` across a dotted tail.
fn render_pair(v: &Value, form: Form) -> String {
    let mut parts = Vec::new();
    let mut cursor = v.clone();
    while let Value::Pair(cell) = cursor {
        parts.push(render(&cell.car.borrow(), form));
        cursor = cell.cdr.borrow().clone();
    }
    let mut out = String::from("(");
    out.push_str(&parts.join(" "));
    if cursor.is_nil() {
        out.push(')');
    } else {
        let _ = write!(out, " . {})", render(&cursor, form));
    }
    out
}

/// Writes `s` with the only two escapes the subset's strings carry: a
/// double quote and a backslash.
fn escape_into(out: &mut String, s: &str) {
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            other => out.push(other),
        }
    }
}

/// Formats one float as `printer.md` fixes it: the shortest decimal
/// string that reads back as the same IEEE 754 double, always carrying
/// a decimal point, fixed notation while the magnitude is in
/// `[1e-6, 1e21)` and mantissa-exponent form outside it.
#[must_use]
pub fn float_string(x: f64) -> String {
    if x.is_nan() {
        return "nan".to_owned();
    }
    if x.is_infinite() {
        return if x.is_sign_negative() { "-inf" } else { "inf" }.to_owned();
    }
    let fixed = x.abs() >= 1e-6 && x.abs() < 1e21;
    if !fixed {
        return with_mantissa_point(&format!("{x:e}"));
    }
    let text = format!("{x}");
    if text.contains('.') {
        text
    } else {
        format!("{text}.0")
    }
}

/// Gives the mantissa of an exponent form its point: `1e22` is `1.0e22`.
fn with_mantissa_point(text: &str) -> String {
    match text.split_once('e') {
        Some((mantissa, exponent)) if !mantissa.contains('.') => {
            format!("{mantissa}.0e{exponent}")
        }
        _ => text.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{display_value, print_value};
    use std::rc::Rc;

    use crate::env::Env;
    use crate::pair::cons_cell;
    use crate::value::{Closure, Value};

    #[test]
    fn prints_booleans_numbers_and_the_empty_list() {
        assert_eq!(print_value(&Value::boolean(true)), "#t");
        assert_eq!(print_value(&Value::boolean(false)), "#f");
        assert_eq!(print_value(&Value::int(-441)), "-441");
        assert_eq!(print_value(&Value::Nil), "()");
    }

    #[test]
    fn prints_floats_with_a_point_and_exponents_outside_the_fixed_range() {
        assert_eq!(print_value(&Value::real(441.0)), "441.0");
        assert_eq!(print_value(&Value::real(2.5)), "2.5");
        assert_eq!(print_value(&Value::real(0.001)), "0.001");
        assert_eq!(print_value(&Value::real(1e-7)), "1.0e-7");
        assert_eq!(print_value(&Value::real(2.5e-7)), "2.5e-7");
        assert_eq!(print_value(&Value::real(1e22)), "1.0e22");
        assert_eq!(
            print_value(&Value::real(-3.000_091_554_131_38)),
            "-3.00009155413138"
        );
    }

    #[test]
    fn prints_strings_quoted_and_escaped_but_display_bare() {
        let s = Value::string("a \"quote\" and a \\ backslash");
        assert_eq!(print_value(&s), "\"a \\\"quote\\\" and a \\\\ backslash\"");
        assert_eq!(display_value(&s), "a \"quote\" and a \\ backslash");
    }

    #[test]
    fn prints_proper_dotted_and_nested_lists() {
        let pair = |a: Value, b: Value| Value::Pair(cons_cell(a, b));
        assert_eq!(print_value(&pair(Value::int(1), Value::int(2))), "(1 . 2)");
        assert_eq!(
            print_value(&Value::list(vec![
                Value::int(1),
                Value::int(2),
                Value::int(3)
            ])),
            "(1 2 3)"
        );
        let dotted_tail = pair(Value::sym("a"), pair(Value::sym("b"), Value::sym("c")));
        assert_eq!(print_value(&dotted_tail), "(a b . c)");
        let nested = Value::list(vec![pair(Value::int(1), Value::int(2)), Value::sym("d")]);
        assert_eq!(print_value(&nested), "((1 . 2) d)");
    }

    #[test]
    fn prints_procedures_by_name() {
        let primitive = Value::Primitive {
            name: Rc::from("car"),
            f: Rc::new(|_| Ok(Value::Nil)),
        };
        assert_eq!(print_value(&primitive), "#[primitive-procedure car]");
        let anonymous = Value::Closure(Rc::new(Closure {
            name: None,
            params: vec![Rc::from("x")],
            rest: None,
            body: vec![Value::sym("x")],
            env: Env::global(),
        }));
        assert_eq!(print_value(&anonymous), "#[compound-procedure]");
        let named = Value::Closure(Rc::new(Closure {
            name: Some(Rc::from("f")),
            params: vec![Rc::from("x")],
            rest: None,
            body: vec![Value::sym("x")],
            env: Env::global(),
        }));
        assert_eq!(print_value(&named), "#[compound-procedure f]");
    }
}
