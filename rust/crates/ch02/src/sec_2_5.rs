// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.5

//! Section 2.5: Systems with generic operations.
//!
//! The section stacks a third layer on the [`OpTable`] of 2.4: a generic
//! arithmetic package whose operations ([`add`], [`sub`], [`mul`], [`div`])
//! dispatch on the kinds of their arguments, a coercion table that
//! [`apply_generic`] consults when no direct entry matches (2.5.2), and a
//! polynomial package whose coefficient arithmetic itself goes through the
//! table, so a polynomial over polynomials is just dispatch recursing
//! (2.5.3).
//!
//! Two edition decisions live here rather than in the exercises:
//!
//! * The tag functions ([`attach_tag`], [`type_tag`], [`contents`]) already
//!   carry the accommodation of exercise 2.78 — a plain [`Value::Int`] or
//!   [`Value::Real`] *is* an ordinary number, no wrapper pair. The section's
//!   own footnote at 2.5.3 assumes exactly this (bare numeric coefficients),
//!   and the host's typed enum plays the role the book assigns to Scheme's
//!   internal type system.
//! * The tower of Figure 2.25 needs a `real` level between the rationals and
//!   the complex numbers, so a small [`install_real_package`] sits beside the
//!   three packages the book writes out. The `equ?` and `=zero?` predicates
//!   of exercises 2.79 and 2.80 are installed too; the section's own listings
//!   (`adjoin-term`, and 2.85's `drop`) call them.

use std::rc::Rc;

use sicp_runtime::{Handler, Key, OpTable, SchemeError, Symbol, Value, car, cdr, cons_cell};

use crate::sec_2_4::{
    imag_part_dispatch, install_polar_package, install_rectangular_package, real_part_dispatch,
    rect_make_from_real_imag_tagged,
};

// ---------------------------------------------------------------------
// Tags (with the exercise 2.78 accommodation the section's footnote assumes)
// ---------------------------------------------------------------------

/// Builds a tagged datum: the book's `attach-tag`. An ordinary number
/// (plain [`Value::Int`] or [`Value::Real`]) is left bare — exercise
/// 2.78's accommodation, which the section's own footnote at 2.5.3
/// assumes for polynomial coefficients.
#[must_use]
pub fn attach_tag(type_tag: &str, contents: Value) -> Value {
    match (type_tag, contents) {
        ("scheme-number", n @ (Value::Int(_) | Value::Real(_))) => n,
        (tag, data) => Value::tagged(tag, data),
    }
}

/// Extracts the tag of a datum: the book's `type-tag`. A plain integer or
/// real answers `scheme-number` — the host enum is the internal type
/// system the exercise leans on.
///
/// # Errors
/// [`SchemeError::UserRaised`] with the book's message when `datum`
/// carries no tag.
pub fn type_tag(datum: &Value) -> Result<Symbol, SchemeError> {
    match datum {
        Value::Int(_) | Value::Real(_) => Ok(Symbol::from("scheme-number")),
        Value::Tagged { tag, .. } => Ok(Rc::clone(tag)),
        other => Err(SchemeError::UserRaised {
            message: "Bad tagged datum: TYPE-TAG".into(),
            irritants: vec![other.clone()],
        }),
    }
}

/// Extracts the contents of a datum: the book's `contents`. A plain
/// number is its own contents.
///
/// # Errors
/// [`SchemeError::UserRaised`] with the book's message when `datum`
/// carries no tag.
pub fn contents(datum: &Value) -> Result<Value, SchemeError> {
    match datum {
        Value::Int(_) | Value::Real(_) => Ok(datum.clone()),
        Value::Tagged { data, .. } => Ok((**data).clone()),
        other => Err(SchemeError::UserRaised {
            message: "Bad tagged datum: CONTENTS".into(),
            irritants: vec![other.clone()],
        }),
    }
}

/// The single real number behind an arithmetic component: accepts the
/// plain integer and real shapes and nothing else. An exact integer
/// converts losslessly only up to 2^53; past that the inexact arithmetic
/// this feeds rounds, which is the documented, wanted truncation.
fn as_arith_real(v: &Value) -> Result<f64, SchemeError> {
    match v {
        #[expect(
            clippy::cast_precision_loss,
            reason = "promoting an exact integer into the inexact tower level is the operation's point"
        )]
        Value::Int(n) => Ok(*n as f64),
        Value::Real(x) => Ok(*x),
        other => Err(SchemeError::TypeMismatch(format!(
            "expected a number: {other}"
        ))),
    }
}

/// The single exact integer behind a rational component.
fn as_int(v: &Value) -> Result<i128, SchemeError> {
    match v {
        Value::Int(n) => Ok(*n),
        other => Err(SchemeError::TypeMismatch(format!(
            "expected an exact integer: {other}"
        ))),
    }
}

/// Builds a cons pair, the shape every two-slot datum here uses.
fn pair(a: Value, b: Value) -> Value {
    Value::Pair(cons_cell(a, b))
}

/// Folds a slice of keys into the list-shaped key the table stores for an
/// ordered tag list, the book's `'(t1 t2 ...)`.
fn key_list(keys: &[Key]) -> Key {
    let mut out = Key::Nil;
    for k in keys.iter().rev() {
        out = Key::pair(k.clone(), out);
    }
    out
}

/// The tags of all arguments, the book's `(map type-tag args)`.
fn tags_of(args: &[Value]) -> Result<Vec<Symbol>, SchemeError> {
    args.iter().map(type_tag).collect()
}

/// The bare contents of all arguments, the book's `(map contents args)`.
fn contents_of(args: &[Value]) -> Result<Vec<Value>, SchemeError> {
    args.iter().map(contents).collect()
}

/// The book's "No method for these types" error, naming the operation and
/// the tag list.
fn no_method(op: &str, tags: &[Symbol]) -> SchemeError {
    SchemeError::UserRaised {
        message: "No method for these types".into(),
        irritants: vec![
            Value::sym(op),
            Value::list(tags.iter().map(|t| Value::Sym(Rc::clone(t))).collect()),
        ],
    }
}

/// Greatest common divisor of two non-negative magnitudes.
fn gcd_u128(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd_u128(b, a % b) }
}

// ---------------------------------------------------------------------
// 2.5.1 Generic arithmetic operations
// ---------------------------------------------------------------------

/// The generic `add`: the book's `(define (add x y) (apply-generic 'add x y))`.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn add(table: &OpTable, x: &Value, y: &Value) -> Result<Value, SchemeError> {
    apply_generic(table, "add", &[x.clone(), y.clone()])
}

/// The generic `sub`.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn sub(table: &OpTable, x: &Value, y: &Value) -> Result<Value, SchemeError> {
    apply_generic(table, "sub", &[x.clone(), y.clone()])
}

/// The generic `mul`.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn mul(table: &OpTable, x: &Value, y: &Value) -> Result<Value, SchemeError> {
    apply_generic(table, "mul", &[x.clone(), y.clone()])
}

/// The generic `div`.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn div(table: &OpTable, x: &Value, y: &Value) -> Result<Value, SchemeError> {
    apply_generic(table, "div", &[x.clone(), y.clone()])
}

/// Exact integer division with Scheme's shape: an exact result stays an
/// integer, an inexact one becomes a real — the "limited-precision
/// division" the book's exercise 2.95 footnote blames for broken divisors.
fn int_div(x: i128, y: i128) -> Result<Value, SchemeError> {
    if y == 0 {
        return Err(SchemeError::DivisionByZero);
    }
    if x % y == 0 {
        return Ok(Value::Int(x / y));
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "the inexact quotient is the point: exercise 2.95 feeds on these"
    )]
    let q = x as f64 / y as f64;
    Ok(Value::real(q))
}

/// One arithmetic step over two numeric contents. Exact integer shapes
/// stay exact through `int_op`; anything mixed or inexact goes through
/// the `f64` operation.
fn arith2(
    a: &Value,
    b: &Value,
    int_op: fn(i128, i128) -> Result<i128, SchemeError>,
    real_op: fn(f64, f64) -> f64,
) -> Result<Value, SchemeError> {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => int_op(*x, *y).map(Value::Int),
        _ => Ok(Value::real(real_op(as_arith_real(a)?, as_arith_real(b)?))),
    }
}

/// The division handler: exact integer division stays exact, everything
/// else divides in `f64`.
fn arith_div(a: &Value, b: &Value) -> Result<Value, SchemeError> {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => int_div(*x, *y),
        _ => Ok(Value::real(as_arith_real(a)? / as_arith_real(b)?)),
    }
}

fn checked_add(x: i128, y: i128) -> Result<i128, SchemeError> {
    x.checked_add(y).ok_or(SchemeError::Overflow)
}

fn checked_sub(x: i128, y: i128) -> Result<i128, SchemeError> {
    x.checked_sub(y).ok_or(SchemeError::Overflow)
}

fn checked_mul(x: i128, y: i128) -> Result<i128, SchemeError> {
    x.checked_mul(y).ok_or(SchemeError::Overflow)
}

/// The two-argument handler shape the ordinary-number package installs:
/// integer contents through the checked operation, everything else
/// through the real one.
fn ordinary2(
    int_op: fn(i128, i128) -> Result<i128, SchemeError>,
    real_op: fn(f64, f64) -> f64,
) -> impl Fn(&[Value]) -> Result<Value, SchemeError> {
    move |args: &[Value]| arith2(&args[0], &args[1], int_op, real_op)
}

fn f64_add(a: f64, b: f64) -> f64 {
    a + b
}

fn f64_sub(a: f64, b: f64) -> f64 {
    a - b
}

fn f64_mul(a: f64, b: f64) -> f64 {
    a * b
}

fn f64_div(a: f64, b: f64) -> f64 {
    a / b
}

fn table_err(package: &str) -> SchemeError {
    SchemeError::UserRaised {
        message: format!("{package} package is not installed"),
        irritants: vec![],
    }
}

/// Fetches a one-slot constructor from the table.
fn get_make(table: &OpTable, package: &str) -> Result<Handler, SchemeError> {
    table
        .get(&Key::sym("make"), &Key::sym(package))
        .ok_or_else(|| table_err(package))
}

/// Installs the package for ordinary numbers: the book's
/// `install-scheme-number-package`, keyed by
/// `(scheme-number scheme-number)`. With the 2.78 accommodation the
/// package's `tag` is the identity on plain numbers.
pub fn install_scheme_number_package(table: &OpTable) {
    let sn2 = key_list(&[Key::sym("scheme-number"), Key::sym("scheme-number")]);
    table.put(
        Key::sym("add"),
        sn2.clone(),
        Rc::new(ordinary2(checked_add, f64_add)),
    );
    table.put(
        Key::sym("sub"),
        sn2.clone(),
        Rc::new(ordinary2(checked_sub, f64_sub)),
    );
    table.put(
        Key::sym("mul"),
        sn2.clone(),
        Rc::new(ordinary2(checked_mul, f64_mul)),
    );
    table.put(
        Key::sym("div"),
        sn2,
        Rc::new(|args: &[Value]| arith_div(&args[0], &args[1])),
    );
    table.put(
        Key::sym("make"),
        Key::sym("scheme-number"),
        Rc::new(|args: &[Value]| Ok(args[0].clone())),
    );
}

/// Builds a tagged ordinary number: the book's `make-scheme-number`.
///
/// # Errors
/// When the scheme-number package is not installed.
pub fn make_scheme_number(table: &OpTable, n: i128) -> Result<Value, SchemeError> {
    let make = get_make(table, "scheme-number")?;
    make(&[Value::Int(n)])
}

/// Reduces a numerator/denominator pair by their gcd, sign on the
/// numerator, denominator positive. Magnitudes stay in `u128` so
/// `i128::MIN` divides without wrapping; the reduced parts convert back
/// with a real overflow check.
fn norm_ratio(n: i128, d: i128) -> Result<(i128, i128), SchemeError> {
    if d == 0 {
        return Err(SchemeError::DivisionByZero);
    }
    let (sn, sd) = (n.is_negative(), d.is_negative());
    let (an, ad) = (n.unsigned_abs(), d.unsigned_abs());
    let g = gcd_u128(an, ad);
    let (an, ad) = (an / g, ad / g);
    let numer = i128::try_from(an).map_err(|_| SchemeError::Overflow)?;
    let denom = i128::try_from(ad).map_err(|_| SchemeError::Overflow)?;
    if sn ^ sd {
        Ok((-numer, denom))
    } else {
        Ok((numer, denom))
    }
}

/// Installs the rational package: the book's `install-rational-package`,
/// with the rational-number code of 2.1.1 as the unmodified internal
/// procedures.
pub fn install_rational_package(table: &OpTable) {
    fn numer(x: &Value) -> Result<i128, SchemeError> {
        as_int(&car(x)?)
    }
    fn denom(x: &Value) -> Result<i128, SchemeError> {
        as_int(&cdr(x)?)
    }
    fn make_rat(n: i128, d: i128) -> Result<Value, SchemeError> {
        let (n, d) = norm_ratio(n, d)?;
        Ok(pair(Value::Int(n), Value::Int(d)))
    }
    fn cross(
        n1: i128,
        d1: i128,
        n2: i128,
        d2: i128,
        op: fn(i128, i128) -> Result<i128, SchemeError>,
    ) -> Result<(i128, i128), SchemeError> {
        Ok((
            op(checked_mul(n1, d2)?, checked_mul(n2, d1)?)?,
            checked_mul(d1, d2)?,
        ))
    }

    let rat_key = key_list(&[Key::sym("rational"), Key::sym("rational")]);
    let binary = |f: fn(&Value, &Value) -> Result<Value, SchemeError>| {
        move |args: &[Value]| f(&args[0], &args[1]).map(|v| attach_tag("rational", v))
    };
    let add_rat = |x: &Value, y: &Value| {
        let (n, d) = cross(numer(x)?, denom(x)?, numer(y)?, denom(y)?, checked_add)?;
        make_rat(n, d)
    };
    let sub_rat = |x: &Value, y: &Value| {
        let (n, d) = cross(numer(x)?, denom(x)?, numer(y)?, denom(y)?, checked_sub)?;
        make_rat(n, d)
    };
    let mul_rat = |x: &Value, y: &Value| {
        let n = checked_mul(numer(x)?, numer(y)?)?;
        let d = checked_mul(denom(x)?, denom(y)?)?;
        make_rat(n, d)
    };
    let div_rat = |x: &Value, y: &Value| {
        let n = checked_mul(numer(x)?, denom(y)?)?;
        let d = checked_mul(denom(x)?, numer(y)?)?;
        make_rat(n, d)
    };
    table.put(Key::sym("add"), rat_key.clone(), Rc::new(binary(add_rat)));
    table.put(Key::sym("sub"), rat_key.clone(), Rc::new(binary(sub_rat)));
    table.put(Key::sym("mul"), rat_key.clone(), Rc::new(binary(mul_rat)));
    table.put(Key::sym("div"), rat_key, Rc::new(binary(div_rat)));
    table.put(
        Key::sym("make"),
        Key::sym("rational"),
        Rc::new(|args: &[Value]| {
            make_rat(as_int(&args[0])?, as_int(&args[1])?).map(|v| attach_tag("rational", v))
        }),
    );
}

/// Builds a tagged rational number: the book's `make-rational`.
///
/// # Errors
/// [`SchemeError::DivisionByZero`] for a zero denominator; whatever the
/// table's `make` entry raises.
pub fn make_rational(table: &OpTable, n: i128, d: i128) -> Result<Value, SchemeError> {
    let make = get_make(table, "rational")?;
    make(&[Value::Int(n), Value::Int(d)])
}

/// The numerator of a rational datum: the book's `numer`. Operates on
/// the bare numerator/denominator pair, the level the package's
/// handlers work at.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `x` is not a pair of integers.
pub fn numer(x: &Value) -> Result<i128, SchemeError> {
    as_int(&car(x)?)
}

/// The denominator of a rational datum: the book's `denom`, on the bare
/// pair like [`numer`].
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `x` is not a pair of integers.
pub fn denom(x: &Value) -> Result<i128, SchemeError> {
    as_int(&cdr(x)?)
}

/// Installs the package for the tower's `real` level (Figure 2.25): the
/// book's system needs this level for the raise and project operations,
/// and it is one more instance of the same package shape.
pub fn install_real_package(table: &OpTable) {
    let real_key = key_list(&[Key::sym("real"), Key::sym("real")]);
    let real2 = |f: fn(f64, f64) -> f64| {
        move |args: &[Value]| {
            let x = f(as_arith_real(&args[0])?, as_arith_real(&args[1])?);
            Ok(attach_tag("real", Value::real(x)))
        }
    };
    table.put(Key::sym("add"), real_key.clone(), Rc::new(real2(f64_add)));
    table.put(Key::sym("sub"), real_key.clone(), Rc::new(real2(f64_sub)));
    table.put(Key::sym("mul"), real_key.clone(), Rc::new(real2(f64_mul)));
    table.put(Key::sym("div"), real_key, Rc::new(real2(f64_div)));
    table.put(
        Key::sym("make"),
        Key::sym("real"),
        Rc::new(|args: &[Value]| Ok(attach_tag("real", Value::real(as_arith_real(&args[0])?)))),
    );
}

/// Builds a tagged real: the book's system reaches this level through
/// `raise`; the constructor goes through the table like the others.
///
/// # Errors
/// When the real package is not installed.
pub fn make_real(table: &OpTable, x: f64) -> Result<Value, SchemeError> {
    let make = get_make(table, "real")?;
    make(&[Value::real(x)])
}

/// The dispatch selectors of 2.4.2 read as `f64` parts: the internal
/// arithmetic of the complex package works on the bare rectangular or
/// polar contents.
mod complex_parts {
    use super::as_arith_real;
    use crate::sec_2_4::{
        angle_dispatch, imag_part_dispatch, magnitude_dispatch, real_part_dispatch,
    };
    use sicp_runtime::{SchemeError, Value};

    pub(super) fn real_of(z: &Value) -> Result<f64, SchemeError> {
        as_arith_real(&real_part_dispatch(z)?)
    }

    pub(super) fn imag_of(z: &Value) -> Result<f64, SchemeError> {
        as_arith_real(&imag_part_dispatch(z)?)
    }

    pub(super) fn mag_of(z: &Value) -> Result<f64, SchemeError> {
        as_arith_real(&magnitude_dispatch(z)?)
    }

    pub(super) fn ang_of(z: &Value) -> Result<f64, SchemeError> {
        as_arith_real(&angle_dispatch(z)?)
    }
}

/// Installs the complex package: the book's `install-complex-package`.
/// The constructors `make-from-real-imag` and `make-from-mag-ang` are
/// extracted from the table, where the rectangular and polar packages of
/// 2.4.3 installed them — additivity in action — so this package is
/// written against nothing of theirs.
///
/// # Errors
/// [`SchemeError::UserRaised`] when the rectangular or polar package is
/// missing from the table.
pub fn install_complex_package(table: &OpTable) -> Result<(), SchemeError> {
    use complex_parts::{ang_of, imag_of, mag_of, real_of};

    fn tagged_binary(
        f: impl Fn(&[Value]) -> Result<Value, SchemeError> + 'static,
    ) -> impl Fn(&[Value]) -> Result<Value, SchemeError> {
        move |args: &[Value]| f(args).map(|z| attach_tag("complex", z))
    }

    let make_from_real_imag = table
        .get(&Key::sym("make-from-real-imag"), &Key::sym("rectangular"))
        .ok_or_else(|| table_err("rectangular"))?;
    let make_from_mag_ang = table
        .get(&Key::sym("make-from-mag-ang"), &Key::sym("polar"))
        .ok_or_else(|| table_err("polar"))?;

    let add_complex = {
        let mk = Rc::clone(&make_from_real_imag);
        move |args: &[Value]| {
            let re = real_of(&args[0])? + real_of(&args[1])?;
            let im = imag_of(&args[0])? + imag_of(&args[1])?;
            mk(&[Value::real(re), Value::real(im)])
        }
    };
    let sub_complex = {
        let mk = Rc::clone(&make_from_real_imag);
        move |args: &[Value]| {
            let re = real_of(&args[0])? - real_of(&args[1])?;
            let im = imag_of(&args[0])? - imag_of(&args[1])?;
            mk(&[Value::real(re), Value::real(im)])
        }
    };
    let mul_complex = {
        let mk = Rc::clone(&make_from_mag_ang);
        move |args: &[Value]| {
            let r = mag_of(&args[0])? * mag_of(&args[1])?;
            let a = ang_of(&args[0])? + ang_of(&args[1])?;
            mk(&[Value::real(r), Value::real(a)])
        }
    };
    let div_complex = {
        let mk = Rc::clone(&make_from_mag_ang);
        move |args: &[Value]| {
            let r = mag_of(&args[0])? / mag_of(&args[1])?;
            let a = ang_of(&args[0])? - ang_of(&args[1])?;
            mk(&[Value::real(r), Value::real(a)])
        }
    };

    let complex_key = key_list(&[Key::sym("complex"), Key::sym("complex")]);
    table.put(
        Key::sym("add"),
        complex_key.clone(),
        Rc::new(tagged_binary(add_complex)),
    );
    table.put(
        Key::sym("sub"),
        complex_key.clone(),
        Rc::new(tagged_binary(sub_complex)),
    );
    table.put(
        Key::sym("mul"),
        complex_key.clone(),
        Rc::new(tagged_binary(mul_complex)),
    );
    table.put(
        Key::sym("div"),
        complex_key,
        Rc::new(tagged_binary(div_complex)),
    );
    let mk_ri = Rc::clone(&make_from_real_imag);
    table.put(
        Key::sym("make-from-real-imag"),
        Key::sym("complex"),
        Rc::new(move |args: &[Value]| mk_ri(args).map(|z| attach_tag("complex", z))),
    );
    let mk_ma = Rc::clone(&make_from_mag_ang);
    table.put(
        Key::sym("make-from-mag-ang"),
        Key::sym("complex"),
        Rc::new(move |args: &[Value]| mk_ma(args).map(|z| attach_tag("complex", z))),
    );
    Ok(())
}

/// Builds a complex number from real and imaginary parts, through the
/// complex package's exported constructor: the book's
/// `make-complex-from-real-imag`.
///
/// # Errors
/// When the complex package is not installed.
pub fn make_complex_from_real_imag(table: &OpTable, x: f64, y: f64) -> Result<Value, SchemeError> {
    let make = table
        .get(&Key::sym("make-from-real-imag"), &Key::sym("complex"))
        .ok_or_else(|| table_err("complex"))?;
    make(&[Value::real(x), Value::real(y)])
}

/// Builds a complex number from magnitude and angle: the book's
/// `make-complex-from-mag-ang`.
///
/// # Errors
/// When the complex package is not installed.
pub fn make_complex_from_mag_ang(table: &OpTable, r: f64, a: f64) -> Result<Value, SchemeError> {
    let make = table
        .get(&Key::sym("make-from-mag-ang"), &Key::sym("complex"))
        .ok_or_else(|| table_err("complex"))?;
    make(&[Value::real(r), Value::real(a)])
}

/// Installs the generic equality predicate `equ?` of exercise 2.79
/// across the number packages; the section's `drop` (2.85) leans on it.
pub fn install_equ(table: &OpTable) {
    fn numeric_eq(a: &Value, b: &Value) -> Result<Value, SchemeError> {
        let eq = match (a, b) {
            (Value::Int(x), Value::Int(y)) => x == y,
            _ => as_arith_real(a)? == as_arith_real(b)?,
        };
        Ok(Value::boolean(eq))
    }
    fn rational_eq(a: &Value, b: &Value) -> Result<Value, SchemeError> {
        let left = checked_mul(numer(a)?, denom(b)?)?;
        let right = checked_mul(numer(b)?, denom(a)?)?;
        Ok(Value::boolean(left == right))
    }
    fn real_eq(a: &Value, b: &Value) -> Result<Value, SchemeError> {
        Ok(Value::boolean(as_arith_real(a)? == as_arith_real(b)?))
    }
    fn complex_eq(a: &Value, b: &Value) -> Result<Value, SchemeError> {
        Ok(Value::boolean(
            real_part_dispatch(a)? == real_part_dispatch(b)?
                && imag_part_dispatch(a)? == imag_part_dispatch(b)?,
        ))
    }
    let boolean = |f: fn(&Value, &Value) -> Result<Value, SchemeError>| {
        move |args: &[Value]| f(&args[0], &args[1])
    };
    let sn2 = key_list(&[Key::sym("scheme-number"), Key::sym("scheme-number")]);
    table.put(Key::sym("equ?"), sn2, Rc::new(boolean(numeric_eq)));
    table.put(
        Key::sym("equ?"),
        key_list(&[Key::sym("rational"), Key::sym("rational")]),
        Rc::new(boolean(rational_eq)),
    );
    table.put(
        Key::sym("equ?"),
        key_list(&[Key::sym("real"), Key::sym("real")]),
        Rc::new(boolean(real_eq)),
    );
    table.put(
        Key::sym("equ?"),
        key_list(&[Key::sym("complex"), Key::sym("complex")]),
        Rc::new(boolean(complex_eq)),
    );
}

/// Installs the generic `=zero?` predicate of exercise 2.80; the
/// section's `adjoin-term` calls it to drop zero coefficients.
pub fn install_zero(table: &OpTable) {
    fn ordinary_zero(args: &[Value]) -> Result<Value, SchemeError> {
        let zero = match &args[0] {
            Value::Int(n) => *n == 0,
            Value::Real(x) => *x == 0.0,
            other => {
                return Err(SchemeError::TypeMismatch(format!(
                    "=zero?: not an ordinary number: {other}"
                )));
            }
        };
        Ok(Value::boolean(zero))
    }
    fn rational_zero(args: &[Value]) -> Result<Value, SchemeError> {
        Ok(Value::boolean(numer(&args[0])? == 0))
    }
    fn real_zero(args: &[Value]) -> Result<Value, SchemeError> {
        Ok(Value::boolean(as_arith_real(&args[0])? == 0.0))
    }
    fn complex_zero(args: &[Value]) -> Result<Value, SchemeError> {
        let re = as_arith_real(&real_part_dispatch(&args[0])?)?;
        let im = as_arith_real(&imag_part_dispatch(&args[0])?)?;
        Ok(Value::boolean(re == 0.0 && im == 0.0))
    }
    table.put(
        Key::sym("=zero?"),
        key_list(&[Key::sym("scheme-number")]),
        Rc::new(ordinary_zero),
    );
    table.put(
        Key::sym("=zero?"),
        key_list(&[Key::sym("rational")]),
        Rc::new(rational_zero),
    );
    table.put(
        Key::sym("=zero?"),
        key_list(&[Key::sym("real")]),
        Rc::new(real_zero),
    );
    table.put(
        Key::sym("=zero?"),
        key_list(&[Key::sym("complex")]),
        Rc::new(complex_zero),
    );
}

/// Installs every package of the generic arithmetic system on one table:
/// the rectangular and polar packages of 2.4.3, then the ordinary,
/// rational, real, and complex packages, then the `equ?` and `=zero?`
/// predicates. One call gives a program the whole system the figure
/// shows.
///
/// # Errors
/// [`SchemeError::UserRaised`] when the table lacks the rectangular or
/// polar constructors the complex package imports.
pub fn install_generic_arithmetic(table: &OpTable) -> Result<(), SchemeError> {
    install_rectangular_package(table);
    install_polar_package(table);
    install_scheme_number_package(table);
    install_rational_package(table);
    install_real_package(table);
    install_complex_package(table)?;
    install_equ(table);
    install_zero(table);
    Ok(())
}

// ---------------------------------------------------------------------
// 2.5.2 Coercion
// ---------------------------------------------------------------------

/// Installs a coercion under the pair of type names: the book's
/// `put-coercion` over a second table of the same shape.
pub fn put_coercion(table: &OpTable, from: &str, to: &str, f: Handler) {
    table.put(Key::sym(from), Key::sym(to), f);
}

/// Fetches a coercion: the book's `get-coercion`.
#[must_use]
pub fn get_coercion(table: &OpTable, from: &str, to: &str) -> Option<Handler> {
    table.get(&Key::sym(from), &Key::sym(to))
}

/// The typical coercion procedure: an ordinary number becomes the complex
/// number with that real part and zero imaginary part, the book's
/// `scheme-number->complex`. This mirrors the rectangular constructor the
/// complex package exports.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `n` is not a number.
pub fn scheme_number_to_complex(n: &Value) -> Result<Value, SchemeError> {
    Ok(Value::tagged(
        "complex",
        rect_make_from_real_imag_tagged(as_arith_real(n)?, 0.0),
    ))
}

/// The 2.5.2 `apply-generic`: look the operation up by the arguments'
/// tags; on a miss with two arguments, try coercing the first argument to
/// the second's type, then the second to the first's, then give up.
///
/// # Errors
/// [`SchemeError::UserRaised`] naming the operation and tag list when no
/// method and no coercion applies; whatever the dispatched handler or a
/// coercion raises.
pub fn apply_generic(table: &OpTable, op: &str, args: &[Value]) -> Result<Value, SchemeError> {
    let tags = tags_of(args)?;
    let tag_key = key_list(
        &tags
            .iter()
            .map(|t| Key::Sym(Rc::clone(t)))
            .collect::<Vec<_>>(),
    );
    if let Some(proc) = table.get(&Key::sym(op), &tag_key) {
        return proc(&contents_of(args)?);
    }
    if let ([a1, a2], [t1, t2]) = (args, tags.as_slice()) {
        if let Some(coerce) = get_coercion(table, t1.as_ref(), t2.as_ref()) {
            let a1 = coerce(std::slice::from_ref(a1))?;
            return apply_generic(table, op, &[a1, a2.clone()]);
        }
        if let Some(coerce) = get_coercion(table, t2.as_ref(), t1.as_ref()) {
            let a2 = coerce(std::slice::from_ref(a2))?;
            return apply_generic(table, op, &[a1.clone(), a2]);
        }
    }
    Err(no_method(op, &tags))
}

// ---------------------------------------------------------------------
// 2.5.3 Arithmetic on polynomials (sparse term lists)
// ---------------------------------------------------------------------

/// One term of a polynomial: an order (the power of the indeterminate)
/// and a coefficient, the book's `(list order coeff)` abstraction. The
/// coefficient is a full [`Value`], so it can itself be a tagged
/// polynomial.
#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    /// The power of the indeterminate.
    pub order: u32,
    /// The coefficient, any value the generic arithmetic package knows.
    pub coeff: Value,
}

/// The book's `make-term`.
#[must_use]
pub fn make_term(order: u32, coeff: Value) -> Term {
    Term { order, coeff }
}

/// The book's `order`: the power of the indeterminate of a term.
#[must_use]
pub fn order(term: &Term) -> u32 {
    term.order
}

/// The book's `coeff`: the coefficient of a term.
#[must_use]
pub fn coeff(term: &Term) -> &Value {
    &term.coeff
}

/// The book's `the-empty-termlist`.
#[must_use]
pub fn the_empty_termlist() -> Vec<Term> {
    Vec::new()
}

/// The book's `empty-termlist?`.
#[must_use]
pub fn is_empty_termlist(terms: &[Term]) -> bool {
    terms.is_empty()
}

/// Encodes a term as the book's `(list order coeff)` value.
fn term_to_value(term: &Term) -> Value {
    Value::list(vec![Value::Int(i128::from(term.order)), term.coeff.clone()])
}

/// Decodes a term value back into a [`Term`].
fn term_from_value(v: &Value) -> Result<Term, SchemeError> {
    let items = v.list_items()?;
    let [ord, coeff] = items.as_slice() else {
        return Err(SchemeError::TypeMismatch(format!(
            "not an (order coeff) term: {v}"
        )));
    };
    let raw = as_int(ord)?;
    let order =
        u32::try_from(raw).map_err(|_| SchemeError::TypeMismatch("negative term order".into()))?;
    Ok(Term {
        order,
        coeff: coeff.clone(),
    })
}

/// Encodes a term list as the book's list structure: terms from
/// highest-order to lowest.
#[must_use]
pub fn terms_to_value(terms: &[Term]) -> Value {
    Value::list(terms.iter().map(term_to_value).collect())
}

/// Decodes the book's list structure back into a term list.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the value is not a list of
/// two-element terms.
pub fn value_to_terms(v: &Value) -> Result<Vec<Term>, SchemeError> {
    v.list_items()?.iter().map(term_from_value).collect()
}

/// The book's `first-term`: the highest-order term of a non-empty list.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the list is empty.
pub fn first_term(terms: &[Term]) -> Result<&Term, SchemeError> {
    terms
        .first()
        .ok_or_else(|| SchemeError::TypeMismatch("first-term of an empty term list".into()))
}

/// The book's `rest-terms`: all but the highest-order term.
#[must_use]
pub fn rest_terms(terms: &[Term]) -> &[Term] {
    &terms[1.min(terms.len())..]
}

/// The generic `=zero?` over the table.
///
/// # Errors
/// When no `=zero?` handler is installed for the value's type.
pub fn is_zero(table: &OpTable, v: &Value) -> Result<bool, SchemeError> {
    match apply_generic(table, "=zero?", std::slice::from_ref(v))? {
        Value::Bool(b) => Ok(b),
        other => Err(SchemeError::TypeMismatch(format!(
            "=zero? did not answer a boolean: {other}"
        ))),
    }
}

/// The generic `equ?` over the table.
///
/// # Errors
/// When no `equ?` handler is installed for the pair of types.
pub fn is_equ(table: &OpTable, a: &Value, b: &Value) -> Result<bool, SchemeError> {
    match apply_generic(table, "equ?", &[a.clone(), b.clone()])? {
        Value::Bool(b) => Ok(b),
        other => Err(SchemeError::TypeMismatch(format!(
            "equ? did not answer a boolean: {other}"
        ))),
    }
}

/// The book's `adjoin-term`: conses a higher-order term onto the list,
/// dropping the term entirely when its coefficient is `=zero?` — which
/// goes through the table, so zero polynomials are dropped too (2.87).
///
/// # Errors
/// Whatever the generic `=zero?` raises on the coefficient.
pub fn adjoin_term(table: &OpTable, term: Term, terms: &[Term]) -> Result<Vec<Term>, SchemeError> {
    if is_zero(table, &term.coeff)? {
        return Ok(terms.to_vec());
    }
    let mut out = Vec::with_capacity(terms.len() + 1);
    out.push(term);
    out.extend_from_slice(terms);
    Ok(out)
}

/// The book's `add-terms`: termwise addition, combining same-order
/// coefficients with the *generic* `add` — the decision that lets
/// coefficients be polynomials themselves.
///
/// # Errors
/// Whatever the coefficient arithmetic raises.
pub fn add_terms(table: &OpTable, l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SchemeError> {
    if is_empty_termlist(l1) {
        return Ok(l2.to_vec());
    }
    if is_empty_termlist(l2) {
        return Ok(l1.to_vec());
    }
    let t1 = first_term(l1)?;
    let t2 = first_term(l2)?;
    match order(t1).cmp(&order(t2)) {
        std::cmp::Ordering::Greater => {
            adjoin_term(table, t1.clone(), &add_terms(table, rest_terms(l1), l2)?)
        }
        std::cmp::Ordering::Less => {
            adjoin_term(table, t2.clone(), &add_terms(table, l1, rest_terms(l2))?)
        }
        std::cmp::Ordering::Equal => {
            let sum = add(table, coeff(t1), coeff(t2))?;
            adjoin_term(
                table,
                make_term(order(t1), sum),
                &add_terms(table, rest_terms(l1), rest_terms(l2))?,
            )
        }
    }
}

/// The book's `mul-term-by-all-terms`.
///
/// # Errors
/// Whatever the coefficient arithmetic raises.
pub fn mul_term_by_all_terms(
    table: &OpTable,
    t1: &Term,
    l: &[Term],
) -> Result<Vec<Term>, SchemeError> {
    let Some(t2) = l.first() else {
        return Ok(the_empty_termlist());
    };
    let product = mul(table, coeff(t1), &t2.coeff)?;
    let rest = mul_term_by_all_terms(table, t1, &l[1..])?;
    adjoin_term(table, make_term(order(t1) + t2.order, product), &rest)
}

/// The book's `mul-terms`.
///
/// # Errors
/// Whatever the coefficient arithmetic raises.
pub fn mul_terms(table: &OpTable, l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SchemeError> {
    let Some(t1) = l1.first() else {
        return Ok(the_empty_termlist());
    };
    let head = mul_term_by_all_terms(table, t1, l2)?;
    add_terms(table, &head, &mul_terms(table, &l1[1..], l2)?)
}

/// The untagged poly datum: a variable paired with a term list.
fn make_poly(var: &str, terms: &[Term]) -> Value {
    pair(Value::sym(var), terms_to_value(terms))
}

/// Splits an untagged poly into its variable and term list.
fn poly_parts(p: &Value) -> Result<(Symbol, Vec<Term>), SchemeError> {
    let Value::Sym(var) = &car(p)? else {
        return Err(SchemeError::TypeMismatch(format!(
            "poly variable is not a symbol: {p}"
        )));
    };
    let terms = value_to_terms(&cdr(p)?)?;
    Ok((Rc::clone(var), terms))
}

/// Compares variables by name: the section's `same-variable?`.
fn same_variable(a: &Symbol, b: &Symbol) -> bool {
    a == b
}

/// A term-list operation the poly-level procedures defer to.
type TermsOp = fn(&OpTable, &[Term], &[Term]) -> Result<Vec<Term>, SchemeError>;

/// Shared shape of `add-poly` and `mul-poly`: check the variables agree,
/// run the term-list operation, and assemble. Works on bare polys — the
/// variable-plus-term-list pairs the package's handlers receive.
fn poly_binary(
    table: &OpTable,
    p1: &Value,
    p2: &Value,
    op: &str,
    terms_op: TermsOp,
) -> Result<Value, SchemeError> {
    let (v1, t1) = poly_parts(p1)?;
    let (v2, t2) = poly_parts(p2)?;
    if !same_variable(&v1, &v2) {
        return Err(SchemeError::UserRaised {
            message: format!("Polys not in same var: {op}"),
            irritants: vec![p1.clone(), p2.clone()],
        });
    }
    Ok(make_poly(v1.as_ref(), &terms_op(table, &t1, &t2)?))
}

/// The book's `add-poly`, on bare poly data (the tagged form is stripped
/// by [`apply_generic`] before the package's handler runs).
///
/// # Errors
/// [`SchemeError::UserRaised`] with the book's message when the polys
/// are not in the same variable; whatever the coefficient arithmetic
/// raises.
pub fn add_poly(table: &OpTable, p1: &Value, p2: &Value) -> Result<Value, SchemeError> {
    poly_binary(table, p1, p2, "ADD-POLY", add_terms)
}

/// The book's `mul-poly`, on bare poly data.
///
/// # Errors
/// [`SchemeError::UserRaised`] with the book's message when the polys
/// are not in the same variable; whatever the coefficient arithmetic
/// raises.
pub fn mul_poly(table: &OpTable, p1: &Value, p2: &Value) -> Result<Value, SchemeError> {
    poly_binary(table, p1, p2, "MUL-POLY", mul_terms)
}

/// Installs the polynomial package: the book's
/// `install-polynomial-package`, putting `add` and `mul` under
/// `(polynomial polynomial)` and a `make` constructor under
/// `polynomial`. The add and mul handlers capture the table by `Rc`:
/// the book's package dispatches its coefficient arithmetic through the
/// same global table it installs into, and the handler type's
/// `Rc<dyn Fn>` bound wants that shared table to be `'static`.
pub fn install_polynomial_package(table: &Rc<OpTable>) {
    let poly_key = key_list(&[Key::sym("polynomial"), Key::sym("polynomial")]);
    let t = Rc::clone(table);
    let poly_add =
        move |args: &[Value]| add_poly(&t, &args[0], &args[1]).map(|p| attach_tag("polynomial", p));
    table.put(Key::sym("add"), poly_key.clone(), Rc::new(poly_add));
    let t = Rc::clone(table);
    let poly_mul =
        move |args: &[Value]| mul_poly(&t, &args[0], &args[1]).map(|p| attach_tag("polynomial", p));
    table.put(Key::sym("mul"), poly_key, Rc::new(poly_mul));
    let make = |args: &[Value]| {
        let Value::Sym(var) = &args[0] else {
            return Err(SchemeError::TypeMismatch(format!(
                "poly variable is not a symbol: {}",
                args[0]
            )));
        };
        let terms = value_to_terms(&args[1])?;
        Ok(attach_tag("polynomial", make_poly(var.as_ref(), &terms)))
    };
    table.put(Key::sym("make"), Key::sym("polynomial"), Rc::new(make));
}

/// Builds a tagged polynomial: the book's `make-polynomial`, through the
/// table.
///
/// # Errors
/// When the polynomial package is not installed.
pub fn make_polynomial(table: &OpTable, var: &str, terms: &[Term]) -> Result<Value, SchemeError> {
    let make = get_make(table, "polynomial")?;
    let term_values: Vec<Value> = terms.iter().map(term_to_value).collect();
    make(&[Value::sym(var), Value::list(term_values)])
}

/// The variable of a bare poly datum: the book's `variable`.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `p` is not a poly.
pub fn poly_variable(p: &Value) -> Result<Symbol, SchemeError> {
    let (var, _) = poly_parts(p)?;
    Ok(var)
}

/// The term list of a bare poly datum: the book's `term-list`.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `p` is not a poly.
pub fn poly_term_list(p: &Value) -> Result<Vec<Term>, SchemeError> {
    let (_, terms) = poly_parts(p)?;
    Ok(terms)
}

/// Installs a `=zero?` handler for polynomials: a polynomial is zero
/// when every coefficient is — exercise 2.87's definition, exposed here
/// because the section's own nested-coefficient listings recurse through
/// `adjoin_term`, which asks `=zero?` of every coefficient it stores.
/// Programs that only handle polynomial coefficients over plain numbers
/// never need it; the generic `=zero?` of the installed number packages
/// answers first.
pub fn install_polynomial_is_zero(table: &Rc<OpTable>) {
    let t = Rc::clone(table);
    table.put(
        Key::sym("=zero?"),
        key_list(&[Key::sym("polynomial")]),
        Rc::new(move |args: &[Value]| {
            let (_, terms) = poly_parts(&args[0])?;
            for term in &terms {
                if !is_zero(&t, &term.coeff)? {
                    return Ok(Value::Bool(false));
                }
            }
            Ok(Value::Bool(true))
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Rc<OpTable> {
        let t = Rc::new(OpTable::new());
        install_generic_arithmetic(&t).expect("packages install");
        install_polynomial_package(&t);
        t
    }

    fn rat(t: &OpTable, n: i128, d: i128) -> Value {
        make_rational(t, n, d).expect("rational")
    }

    #[test]
    fn generic_arithmetic_across_types() {
        let t = table();
        assert_eq!(
            add(&t, &Value::Int(1), &Value::Int(2)).unwrap().to_string(),
            "3"
        );
        assert_eq!(
            add(&t, &rat(&t, 1, 2), &rat(&t, 1, 3)).unwrap().to_string(),
            "(rational (5 . 6))"
        );
        let z = make_complex_from_real_imag(&t, 3.0, 4.0).unwrap();
        assert_eq!(z.to_string(), "(complex (rectangular (3 . 4)))");
        let w = make_complex_from_mag_ang(&t, 5.0, 0.927_295_218_001_612_2).unwrap();
        let sum = add(&t, &z, &w).unwrap();
        let re = complex_parts::real_of(&contents(&sum).unwrap()).unwrap();
        assert!((re - 6.0).abs() < 1e-9);
    }

    #[test]
    fn ordinary_numbers_are_bare_per_exercise_2_78() {
        let t = table();
        assert_eq!(type_tag(&Value::Int(7)).unwrap().as_ref(), "scheme-number");
        assert_eq!(contents(&Value::Int(7)).unwrap(), Value::Int(7));
        assert_eq!(
            mul(&t, &Value::Int(6), &Value::Int(7)).unwrap(),
            Value::Int(42)
        );
    }

    #[test]
    fn coercion_makes_mixed_addition_work() {
        let t = OpTable::new();
        install_generic_arithmetic(&t).expect("packages install");
        put_coercion(
            &t,
            "scheme-number",
            "complex",
            Rc::new(|args: &[Value]| scheme_number_to_complex(&args[0])),
        );
        let z = make_complex_from_real_imag(&t, 1.0, 2.0).unwrap();
        let sum = add(&t, &Value::Int(3), &z).unwrap();
        assert_eq!(sum.to_string(), "(complex (rectangular (4 . 2)))");
    }

    #[test]
    fn apply_generic_names_missing_methods() {
        let t = table();
        let z = make_complex_from_real_imag(&t, 1.0, 1.0).unwrap();
        let err = apply_generic(&t, "add", &[z, Value::Int(1)])
            .expect_err("no (complex scheme-number) entry");
        assert!(
            err.to_string().contains("No method for these types"),
            "{err}"
        );
    }

    #[test]
    fn predicates_across_the_system() {
        let t = table();
        assert!(is_equ(&t, &Value::Int(6), &Value::Int(6)).unwrap());
        assert!(is_equ(&t, &rat(&t, 1, 2), &rat(&t, 2, 4)).unwrap());
        assert!(is_zero(&t, &rat(&t, 0, 5)).unwrap());
        assert!(!is_zero(&t, &Value::Int(2)).unwrap());
        let z = make_complex_from_real_imag(&t, 0.0, 0.0).unwrap();
        assert!(is_zero(&t, &z).unwrap());
    }

    #[test]
    fn rational_arithmetic_reduces_and_keeps_sign() {
        let t = table();
        assert_eq!(rat(&t, 1, 2).to_string(), "(rational (1 . 2))");
        assert_eq!(rat(&t, 2, 4).to_string(), "(rational (1 . 2))");
        assert_eq!(rat(&t, -1, 2).to_string(), "(rational (-1 . 2))");
        assert_eq!(
            sub(&t, &rat(&t, 1, 2), &rat(&t, 1, 2)).unwrap(),
            rat(&t, 0, 1)
        );
        assert_eq!(
            div(&t, &rat(&t, 1, 3), &rat(&t, 1, 2)).unwrap(),
            rat(&t, 2, 3)
        );
    }

    #[test]
    fn polynomial_add_and_mul_on_sparse_term_lists() {
        let t = table();
        let p1 = make_polynomial(
            &t,
            "x",
            &[
                make_term(2, Value::Int(1)),
                make_term(1, Value::Int(3)),
                make_term(0, Value::Int(7)),
            ],
        )
        .unwrap();
        let p2 = make_polynomial(
            &t,
            "x",
            &[make_term(2, Value::Int(5)), make_term(1, Value::Int(3))],
        )
        .unwrap();
        assert_eq!(
            add(&t, &p1, &p2).unwrap().to_string(),
            "(polynomial (x (2 6) (1 6) (0 7)))"
        );
        let a = make_polynomial(
            &t,
            "x",
            &[make_term(1, Value::Int(1)), make_term(0, Value::Int(1))],
        )
        .unwrap();
        let b = make_polynomial(
            &t,
            "x",
            &[make_term(1, Value::Int(1)), make_term(0, Value::Int(-1))],
        )
        .unwrap();
        assert_eq!(
            mul(&t, &a, &b).unwrap().to_string(),
            "(polynomial (x (2 1) (0 -1)))"
        );
    }

    #[test]
    fn polynomial_over_polynomial_recurses_through_the_table() {
        let t = table();
        // Polynomial coefficients need exercise 2.87's =zero? before
        // adjoin_term can drop zero polynomial terms; install a local
        // handler so the recursion is observable here.
        install_polynomial_is_zero(&t);
        let y1 = make_polynomial(
            &t,
            "y",
            &[make_term(1, Value::Int(1)), make_term(0, Value::Int(1))],
        )
        .unwrap();
        let y2 = make_polynomial(&t, "y", &[make_term(1, Value::Int(2))]).unwrap();
        // The constant coefficient is a y-polynomial too: mixing a bare
        // number with polynomial coefficients would need the number ->
        // polynomial coercion the section's footnote waves at, which the
        // exercises do not build.
        let y_const = make_polynomial(&t, "y", &[make_term(0, Value::Int(1))]).unwrap();
        let p = make_polynomial(
            &t,
            "x",
            &[
                make_term(3, y1.clone()),
                make_term(1, y2),
                make_term(0, y_const),
            ],
        )
        .unwrap();
        let sum = add(&t, &p, &p).unwrap();
        assert!(
            sum.to_string().contains("(3 (polynomial (y (1 2) (0 2))))"),
            "{sum}"
        );
        // (y^2+1)x^3 + (2y)x + 1, times itself: the x^3 coefficient is a
        // polynomial product, computed by recursing into the same table.
        let sq = mul(&t, &p, &p).unwrap();
        assert!(
            sq.to_string()
                .contains("(6 (polynomial (y (2 1) (1 2) (0 1))))"),
            "{sq}"
        );
    }

    #[test]
    fn adjoin_drops_zero_coefficients_through_the_table() {
        let t = table();
        let empty = the_empty_termlist();
        assert!(
            adjoin_term(&t, make_term(2, Value::Int(0)), &empty)
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            adjoin_term(&t, make_term(2, Value::Int(5)), &empty)
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn div_of_ordinary_numbers_is_exact_when_possible() {
        let t = table();
        assert_eq!(
            div(&t, &Value::Int(6), &Value::Int(3)).unwrap(),
            Value::Int(2)
        );
        assert_eq!(
            div(&t, &Value::Int(1), &Value::Int(2)).unwrap(),
            Value::Real(0.5)
        );
    }

    #[test]
    fn real_package_sits_in_the_tower() {
        let t = table();
        let x = make_real(&t, 2.5).unwrap();
        assert_eq!(x.to_string(), "(real 2.5)");
        let y = make_real(&t, 0.5).unwrap();
        assert_eq!(add(&t, &x, &y).unwrap(), make_real(&t, 3.0).unwrap());
    }

    #[test]
    fn two_level_tags_stay_visible_for_the_complex_package() {
        let t = table();
        let z = make_complex_from_real_imag(&t, 3.0, 4.0).unwrap();
        assert_eq!(type_tag(&z).unwrap().as_ref(), "complex");
        let inner = contents(&z).unwrap();
        assert_eq!(type_tag(&inner).unwrap().as_ref(), "rectangular");
    }
}
