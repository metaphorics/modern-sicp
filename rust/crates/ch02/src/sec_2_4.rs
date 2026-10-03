// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.4

//! Section 2.4: Multiple representations for abstract data.
//!
//! This is where the dynamic [`Value`] runtime enters and stays through
//! chapter 4. A complex number is a tagged datum, [`Value::Tagged`] with
//! the tag `rectangular` or `polar`, and the dispatch is the runtime's
//! [`OpTable`] — a real table keyed by the operation name plus the
//! ordered list of type tags — rather than traits, because the section's
//! point is extension at runtime by installing packages that do not all
//! exist when any one caller compiles. Rust traits close the type set at
//! compile time; the table keeps it open.
//!
//! The section builds the same system twice, and the module keeps the
//! intermediate design under distinct names rather than rewriting it
//! away: the 2.4.2 selectors that dispatch explicitly on the tag are
//! [`real_part_dispatch`] and friends, and the 2.4.3 table-backed
//! selectors take over the book's plain names ([`real_part`], and so on)
//! once `apply_generic` exists, exactly as `deriv_unsimplified` and
//! `deriv` share a section in 2.3.2. What cannot move into the table is
//! named too: predicates and arithmetic on plain numbers dispatch on the
//! datum itself, not on a tag, and they stay `match` arms beside the
//! table. The message-passing alternative closes the section: a
//! [`MessageObject`] is a closure that answers operation names.

use std::rc::Rc;

use sicp_runtime::{Handler, Key, OpTable, SicpError, Symbol, Value, car, cdr, cons_cell};

// ---------------------------------------------------------------------
// 2.4.2 Tagged data (introduced here so the representations can carry it)
// ---------------------------------------------------------------------

/// Builds a tagged datum: the book's `attach-tag`. The runtime's
/// [`Value::tagged`] is the same constructor; this is the named surface
/// the section's listings use.
#[must_use]
pub fn attach_tag(type_tag: &str, contents: Value) -> Value {
    Value::tagged(type_tag, contents)
}

/// Extracts the tag of a tagged datum: the book's `type-tag`.
///
/// # Errors
/// [`SicpError::UserRaised`] with the book's message when `datum` is
/// not tagged.
pub fn type_tag(datum: &Value) -> Result<Symbol, SicpError> {
    match datum {
        Value::Tagged { tag, .. } => Ok(Rc::clone(tag)),
        other => Err(SicpError::UserRaised {
            message: "Bad tagged datum: TYPE-TAG".into(),
            irritants: vec![other.clone()],
        }),
    }
}

/// Extracts the contents of a tagged datum: the book's `contents`.
///
/// # Errors
/// [`SicpError::UserRaised`] with the book's message when `datum` is
/// not tagged.
pub fn contents(datum: &Value) -> Result<Value, SicpError> {
    match datum {
        Value::Tagged { data, .. } => Ok((**data).clone()),
        other => Err(SicpError::UserRaised {
            message: "Bad tagged datum: CONTENTS".into(),
            irritants: vec![other.clone()],
        }),
    }
}

/// Recognizes rectangular numbers: the book's `rectangular?`. A datum
/// with no tag at all is simply neither, not an error.
#[must_use]
pub fn is_rectangular(z: &Value) -> bool {
    type_tag(z).is_ok_and(|tag| tag.as_ref() == "rectangular")
}

/// Recognizes polar numbers: the book's `polar?`.
#[must_use]
pub fn is_polar(z: &Value) -> bool {
    type_tag(z).is_ok_and(|tag| tag.as_ref() == "polar")
}

// ---------------------------------------------------------------------
// 2.4.1 Representations for Complex Numbers
// ---------------------------------------------------------------------
//
// The book presents the arithmetic first, against four selectors and two
// constructors assumed as primitives, and only then lets Ben and Alyssa
// choose representations. This module follows that order in prose; in
// code the generic interface arrives with the table in 2.4.3, and the
// arithmetic below ([`add_complex`] and friends) calls it.

/// The single real component behind a complex-number part. The section's
/// complex components are reals, since magnitude, angle, and the
/// trigonometric conversions produce reals.
fn as_real(v: &Value) -> Result<f64, SicpError> {
    match v {
        Value::Real(x) => Ok(*x),
        other => Err(SicpError::TypeMismatch(format!(
            "complex-number components are reals: {other}"
        ))),
    }
}

/// The two real arguments of a constructor handler.
fn as_real_pair(args: &[Value]) -> Result<(f64, f64), SicpError> {
    match args {
        [x, y] => Ok((as_real(x)?, as_real(y)?)),
        _ => Err(SicpError::WrongArity {
            procedure: "operation-table handler".into(),
            expected: "2".into(),
            got: args.len(),
        }),
    }
}

/// The single argument of a selector handler.
fn sole(args: &[Value]) -> Result<&Value, SicpError> {
    match args {
        [one] => Ok(one),
        _ => Err(SicpError::WrongArity {
            procedure: "operation-table handler".into(),
            expected: "1".into(),
            got: args.len(),
        }),
    }
}

// Ben Bitdiddle's rectangular representation: a complex number is the
// pair (real part, imaginary part). These are the procedures he wrote
// working in isolation, named with the `rect_` prefix the book spells as
// a `-rectangular` suffix.

fn rect_real_part(z: &Value) -> Result<Value, SicpError> {
    car(z)
}

fn rect_imag_part(z: &Value) -> Result<Value, SicpError> {
    cdr(z)
}

fn rect_magnitude(z: &Value) -> Result<Value, SicpError> {
    let x = as_real(&car(z)?)?;
    let y = as_real(&cdr(z)?)?;
    Ok(Value::real((x * x + y * y).sqrt()))
}

fn rect_angle(z: &Value) -> Result<Value, SicpError> {
    let x = as_real(&car(z)?)?;
    let y = as_real(&cdr(z)?)?;
    Ok(Value::real(y.atan2(x)))
}

fn rect_make_from_real_imag(x: f64, y: f64) -> Value {
    Value::Pair(cons_cell(Value::real(x), Value::real(y)))
}

fn rect_make_from_mag_ang(r: f64, a: f64) -> Value {
    Value::Pair(cons_cell(
        Value::real(r * a.cos()),
        Value::real(r * a.sin()),
    ))
}

/// Ben's constructor for general use: his pair tagged `rectangular`, the
/// book's `make-from-real-imag-rectangular`.
#[must_use]
pub fn rect_make_from_real_imag_tagged(x: f64, y: f64) -> Value {
    attach_tag("rectangular", rect_make_from_real_imag(x, y))
}

/// Ben's other constructor for general use, the book's
/// `make-from-mag-ang-rectangular`.
#[must_use]
pub fn rect_make_from_mag_ang_tagged(r: f64, a: f64) -> Value {
    attach_tag("rectangular", rect_make_from_mag_ang(r, a))
}

// Alyssa P. Hacker's polar representation: a complex number is the pair
// (magnitude, angle); she pays for the trigonometry in her selectors.

fn polar_real_part(z: &Value) -> Result<Value, SicpError> {
    let r = as_real(&car(z)?)?;
    let a = as_real(&cdr(z)?)?;
    Ok(Value::real(r * a.cos()))
}

fn polar_imag_part(z: &Value) -> Result<Value, SicpError> {
    let r = as_real(&car(z)?)?;
    let a = as_real(&cdr(z)?)?;
    Ok(Value::real(r * a.sin()))
}

fn polar_magnitude(z: &Value) -> Result<Value, SicpError> {
    car(z)
}

fn polar_angle(z: &Value) -> Result<Value, SicpError> {
    cdr(z)
}

fn polar_make_from_real_imag(x: f64, y: f64) -> Value {
    Value::Pair(cons_cell(
        Value::real((x * x + y * y).sqrt()),
        Value::real(y.atan2(x)),
    ))
}

fn polar_make_from_mag_ang(r: f64, a: f64) -> Value {
    Value::Pair(cons_cell(Value::real(r), Value::real(a)))
}

/// Alyssa's constructor for general use, the book's
/// `make-from-real-imag-polar`.
#[must_use]
pub fn polar_make_from_real_imag_tagged(x: f64, y: f64) -> Value {
    attach_tag("polar", polar_make_from_real_imag(x, y))
}

/// Alyssa's other constructor for general use, the book's
/// `make-from-mag-ang-polar`.
#[must_use]
pub fn polar_make_from_mag_ang_tagged(r: f64, a: f64) -> Value {
    attach_tag("polar", polar_make_from_mag_ang(r, a))
}

// ---------------------------------------------------------------------
// 2.4.2 Tagged data: explicit dispatch on the tag
// ---------------------------------------------------------------------

/// The 2.4.2 generic `real-part`: checks the tag and calls the
/// representation's procedure on the bare contents. The table-backed
/// [`real_part`] of 2.4.3 takes over the book's plain name, so this
/// design keeps its dispatch in its name.
///
/// # Errors
/// [`SicpError::UserRaised`] with the book's message when `z` carries
/// neither tag.
pub fn real_part_dispatch(z: &Value) -> Result<Value, SicpError> {
    if is_rectangular(z) {
        rect_real_part(&contents(z)?)
    } else if is_polar(z) {
        polar_real_part(&contents(z)?)
    } else {
        Err(SicpError::UserRaised {
            message: "Unknown type: REAL-PART".into(),
            irritants: vec![z.clone()],
        })
    }
}

/// The 2.4.2 generic `imag-part`, as above.
///
/// # Errors
/// [`SicpError::UserRaised`] when `z` carries neither tag.
pub fn imag_part_dispatch(z: &Value) -> Result<Value, SicpError> {
    if is_rectangular(z) {
        rect_imag_part(&contents(z)?)
    } else if is_polar(z) {
        polar_imag_part(&contents(z)?)
    } else {
        Err(SicpError::UserRaised {
            message: "Unknown type: IMAG-PART".into(),
            irritants: vec![z.clone()],
        })
    }
}

/// The 2.4.2 generic `magnitude`, as above.
///
/// # Errors
/// [`SicpError::UserRaised`] when `z` carries neither tag.
pub fn magnitude_dispatch(z: &Value) -> Result<Value, SicpError> {
    if is_rectangular(z) {
        rect_magnitude(&contents(z)?)
    } else if is_polar(z) {
        polar_magnitude(&contents(z)?)
    } else {
        Err(SicpError::UserRaised {
            message: "Unknown type: MAGNITUDE".into(),
            irritants: vec![z.clone()],
        })
    }
}

/// The 2.4.2 generic `angle`, as above.
///
/// # Errors
/// [`SicpError::UserRaised`] when `z` carries neither tag.
pub fn angle_dispatch(z: &Value) -> Result<Value, SicpError> {
    if is_rectangular(z) {
        rect_angle(&contents(z)?)
    } else if is_polar(z) {
        polar_angle(&contents(z)?)
    } else {
        Err(SicpError::UserRaised {
            message: "Unknown type: ANGLE".into(),
            irritants: vec![z.clone()],
        })
    }
}

// ---------------------------------------------------------------------
// 2.4.3 Data-Directed Programming and Additivity
// ---------------------------------------------------------------------

/// Builds the table key for an ordered tag list: the book installs the
/// selectors under the list `(rectangular)` rather than the bare symbol
/// `rectangular`, so that operations with multiple arguments not all of
/// the same type stay expressible.
#[must_use]
pub fn tag_list_key(tags: &[&str]) -> Key {
    tags.iter()
        .rev()
        .fold(Key::Nil, |tail, tag| Key::pair(Key::sym(tag), tail))
}

/// Builds the table key for an ordered tag list already held as [`Key`]s.
fn key_tag_list(tags: &[Key]) -> Key {
    tags.iter()
        .rev()
        .fold(Key::Nil, |tail, tag| Key::pair(tag.clone(), tail))
}

/// Adapts one of Ben's or Alyssa's selector procedures to the handler
/// shape the table stores: peel the sole argument and hand the procedure
/// the bare contents.
fn selector_handler(f: fn(&Value) -> Result<Value, SicpError>) -> Handler {
    Rc::new(move |args| f(sole(args)?))
}

/// Adapts a tagged constructor `(reals) -> tagged value` to the handler
/// shape the table stores.
fn constructor_handler(f: fn(f64, f64) -> Value) -> Handler {
    Rc::new(move |args| {
        let (x, y) = as_real_pair(args)?;
        Ok(f(x, y))
    })
}

/// Installs Ben's rectangular package: the same procedures he wrote in
/// 2.4.1, interfaced to the rest of the system by `put` entries under the
/// tag `(rectangular)`. The internal names are now safe: they are local
/// to the installation, so there is no conflict with Alyssa's.
pub fn install_rectangular_package(table: &OpTable) {
    let tag = tag_list_key(&["rectangular"]);
    table.put(
        Key::sym("real-part"),
        tag.clone(),
        selector_handler(rect_real_part),
    );
    table.put(
        Key::sym("imag-part"),
        tag.clone(),
        selector_handler(rect_imag_part),
    );
    table.put(
        Key::sym("magnitude"),
        tag.clone(),
        selector_handler(rect_magnitude),
    );
    table.put(Key::sym("angle"), tag, selector_handler(rect_angle));
    table.put(
        Key::sym("make-from-real-imag"),
        Key::sym("rectangular"),
        constructor_handler(rect_make_from_real_imag_tagged),
    );
    table.put(
        Key::sym("make-from-mag-ang"),
        Key::sym("rectangular"),
        constructor_handler(rect_make_from_mag_ang_tagged),
    );
}

/// Installs Alyssa's polar package, analogous to Ben's.
pub fn install_polar_package(table: &OpTable) {
    let tag = tag_list_key(&["polar"]);
    table.put(
        Key::sym("real-part"),
        tag.clone(),
        selector_handler(polar_real_part),
    );
    table.put(
        Key::sym("imag-part"),
        tag.clone(),
        selector_handler(polar_imag_part),
    );
    table.put(
        Key::sym("magnitude"),
        tag.clone(),
        selector_handler(polar_magnitude),
    );
    table.put(Key::sym("angle"), tag, selector_handler(polar_angle));
    table.put(
        Key::sym("make-from-real-imag"),
        Key::sym("polar"),
        constructor_handler(polar_make_from_real_imag_tagged),
    );
    table.put(
        Key::sym("make-from-mag-ang"),
        Key::sym("polar"),
        constructor_handler(polar_make_from_mag_ang_tagged),
    );
}

/// The book's `apply-generic`: looks up the handler under the operation
/// name and the tags of all the arguments, and applies it to the bare
/// contents. A miss raises the edition's typed error naming the operation
/// and the tag list — never a false-ish sentinel, since [`OpTable::get`]
/// returns the absent option.
///
/// # Errors
/// [`SicpError::UserRaised`] with the book's message when no handler is
/// installed for `(op, tags)`; whatever the handler raises otherwise.
pub fn apply_generic(table: &OpTable, op: &str, args: &[Value]) -> Result<Value, SicpError> {
    let mut tag_keys = Vec::with_capacity(args.len());
    let mut tag_values = Vec::with_capacity(args.len());
    for arg in args {
        let tag = type_tag(arg)?;
        tag_values.push(Value::Sym(tag.clone()));
        tag_keys.push(Key::Sym(tag));
    }
    let Some(handler) = table.get(&Key::sym(op), &key_tag_list(&tag_keys)) else {
        return Err(SicpError::UserRaised {
            message: "No method for these types: APPLY-GENERIC".into(),
            irritants: vec![Value::list(vec![Value::sym(op), Value::list(tag_values)])],
        });
    };
    let bare: Vec<Value> = args.iter().map(contents).collect::<Result<_, _>>()?;
    handler(&bare)
}

/// The table-backed generic `real-part`: the book's `real-part`, which
/// does not change at all if a new representation is added.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn real_part(table: &OpTable, z: &Value) -> Result<Value, SicpError> {
    apply_generic(table, "real-part", std::slice::from_ref(z))
}

/// The table-backed generic `imag-part`.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn imag_part(table: &OpTable, z: &Value) -> Result<Value, SicpError> {
    apply_generic(table, "imag-part", std::slice::from_ref(z))
}

/// The table-backed generic `magnitude`.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn magnitude(table: &OpTable, z: &Value) -> Result<Value, SicpError> {
    apply_generic(table, "magnitude", std::slice::from_ref(z))
}

/// The table-backed generic `angle`.
///
/// # Errors
/// Whatever [`apply_generic`] raises.
pub fn angle(table: &OpTable, z: &Value) -> Result<Value, SicpError> {
    apply_generic(table, "angle", std::slice::from_ref(z))
}

/// Builds a complex number from real and imaginary parts, extracting the
/// constructor from the table: rectangular, per the 2.4.3 choice.
///
/// # Errors
/// [`SicpError::UserRaised`] when the rectangular constructor is not
/// installed.
pub fn make_from_real_imag(table: &OpTable, x: f64, y: f64) -> Result<Value, SicpError> {
    let Some(handler) = table.get(&Key::sym("make-from-real-imag"), &Key::sym("rectangular"))
    else {
        return Err(SicpError::UserRaised {
            message: "make-from-real-imag is not installed for rectangular".into(),
            irritants: vec![],
        });
    };
    handler(&[Value::real(x), Value::real(y)])
}

/// Builds a complex number from magnitude and angle, extracting the
/// constructor from the table: polar, per the 2.4.3 choice.
///
/// # Errors
/// [`SicpError::UserRaised`] when the polar constructor is not
/// installed.
pub fn make_from_mag_ang(table: &OpTable, r: f64, a: f64) -> Result<Value, SicpError> {
    let Some(handler) = table.get(&Key::sym("make-from-mag-ang"), &Key::sym("polar")) else {
        return Err(SicpError::UserRaised {
            message: "make-from-mag-ang is not installed for polar".into(),
            irritants: vec![],
        });
    };
    handler(&[Value::real(r), Value::real(a)])
}

// The arithmetic of 2.4.1, written once against the generic selectors and
// never touched again by either representation — or by a third.

/// Adds two complex numbers: the book's `add-complex`.
///
/// # Errors
/// Whatever the generic selectors raise.
pub fn add_complex(table: &OpTable, z1: &Value, z2: &Value) -> Result<Value, SicpError> {
    let re = as_real(&real_part(table, z1)?)? + as_real(&real_part(table, z2)?)?;
    let im = as_real(&imag_part(table, z1)?)? + as_real(&imag_part(table, z2)?)?;
    make_from_real_imag(table, re, im)
}

/// Subtracts two complex numbers: the book's `sub-complex`.
///
/// # Errors
/// Whatever the generic selectors raise.
pub fn sub_complex(table: &OpTable, z1: &Value, z2: &Value) -> Result<Value, SicpError> {
    let re = as_real(&real_part(table, z1)?)? - as_real(&real_part(table, z2)?)?;
    let im = as_real(&imag_part(table, z1)?)? - as_real(&imag_part(table, z2)?)?;
    make_from_real_imag(table, re, im)
}

/// Multiplies two complex numbers: the book's `mul-complex`.
///
/// # Errors
/// Whatever the generic selectors raise.
pub fn mul_complex(table: &OpTable, z1: &Value, z2: &Value) -> Result<Value, SicpError> {
    let r = as_real(&magnitude(table, z1)?)? * as_real(&magnitude(table, z2)?)?;
    let a = as_real(&angle(table, z1)?)? + as_real(&angle(table, z2)?)?;
    make_from_mag_ang(table, r, a)
}

/// Divides two complex numbers: the book's `div-complex`.
///
/// # Errors
/// Whatever the generic selectors raise.
pub fn div_complex(table: &OpTable, z1: &Value, z2: &Value) -> Result<Value, SicpError> {
    let r = as_real(&magnitude(table, z1)?)? / as_real(&magnitude(table, z2)?)?;
    let a = as_real(&angle(table, z1)?)? - as_real(&angle(table, z2)?)?;
    make_from_mag_ang(table, r, a)
}

// ---------------------------------------------------------------------
// Message passing
// ---------------------------------------------------------------------

/// A message-passing object: a closure that receives an operation name as
/// a message and performs it. The book's footnote limitation holds — this
/// organization permits only generic procedures of one argument.
pub type MessageObject = Rc<dyn Fn(&str) -> Result<Value, SicpError>>;

/// The book's message-passing `make-from-real-imag`: the returned closure
/// is the `dispatch` procedure, invoked when a generic operation requests
/// an answer.
#[must_use]
pub fn make_from_real_imag_message_passing(x: f64, y: f64) -> MessageObject {
    Rc::new(move |op| match op {
        "real-part" => Ok(Value::real(x)),
        "imag-part" => Ok(Value::real(y)),
        "magnitude" => Ok(Value::real((x * x + y * y).sqrt())),
        "angle" => Ok(Value::real(y.atan2(x))),
        other => Err(SicpError::UserRaised {
            message: "Unknown op: MAKE-FROM-REAL-IMAG".into(),
            irritants: vec![Value::sym(other)],
        }),
    })
}

/// The book's one-line `apply-generic` for message passing: feed the
/// operation name to the object and let the object do the work.
///
/// # Errors
/// Whatever the object raises for a message it does not answer.
pub fn apply_generic_message_passing(op: &str, arg: &MessageObject) -> Result<Value, SicpError> {
    arg(op)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    /// The real number a selector result carries.
    fn real_of(v: Result<Value, SicpError>) -> f64 {
        as_real(&v.expect("selector")).expect("real component")
    }

    fn installed() -> OpTable {
        let table = OpTable::new();
        install_rectangular_package(&table);
        install_polar_package(&table);
        table
    }

    #[test]
    fn tagged_datum_round_trip() {
        let z = rect_make_from_real_imag_tagged(3.0, 4.0);
        assert!(is_rectangular(&z));
        assert!(!is_polar(&z));
        assert_eq!(type_tag(&z).expect("tagged").as_ref(), "rectangular");
        assert_eq!(
            contents(&z).expect("tagged").to_string(),
            rect_make_from_real_imag(3.0, 4.0).to_string()
        );
    }

    #[test]
    fn bad_tagged_datum_raises_the_book_message() {
        let err = type_tag(&Value::int(3)).expect_err("untagged");
        assert_eq!(err.to_string(), "Bad tagged datum: TYPE-TAG 3");
        let err = contents(&Value::int(3)).expect_err("untagged");
        assert_eq!(err.to_string(), "Bad tagged datum: CONTENTS 3");
    }

    #[test]
    fn dispatch_selectors_serve_both_representations() {
        let z = rect_make_from_real_imag_tagged(3.0, 4.0);
        let w = polar_make_from_mag_ang_tagged(5.0, 0.927_295_218_001_612_2);
        assert!(close(real_of(real_part_dispatch(&w)), 3.0));
        assert!(close(real_of(imag_part_dispatch(&w)), 4.0));
        assert!(close(real_of(magnitude_dispatch(&z)), 5.0));
        assert!(close(real_of(angle_dispatch(&z)), 0.927_295_218_001_612_2));
        let unknown = Value::Pair(cons_cell(Value::real(3.0), Value::real(4.0)));
        let err = magnitude_dispatch(&unknown).expect_err("untagged pair");
        assert!(err.to_string().starts_with("Unknown type: MAGNITUDE"));
    }

    #[test]
    fn generic_selectors_read_both_representations_through_one_table() {
        let table = installed();
        let z = make_from_real_imag(&table, 3.0, 4.0).expect("rectangular installed");
        let w = make_from_mag_ang(&table, 1.0, FRAC_PI_2).expect("polar installed");
        assert!(is_rectangular(&z));
        assert!(is_polar(&w));
        assert!(close(real_of(magnitude(&table, &z)), 5.0));
        assert!(close(real_of(angle(&table, &z)), 0.927_295_218_001_612_2));
        assert!(close(real_of(real_part(&table, &w)), 0.0));
        assert!(close(real_of(imag_part(&table, &w)), 1.0));
    }

    #[test]
    fn arithmetic_runs_over_mixed_representations() {
        let table = installed();
        let z = make_from_real_imag(&table, 3.0, 4.0).expect("rectangular");
        let w = make_from_mag_ang(&table, 1.0, FRAC_PI_2).expect("polar");
        let sum = add_complex(&table, &z, &w).expect("add");
        assert!(close(real_of(real_part(&table, &sum)), 3.0));
        assert!(close(real_of(imag_part(&table, &sum)), 5.0));
        let product = mul_complex(&table, &z, &w).expect("mul");
        assert!(close(real_of(magnitude(&table, &product)), 5.0));
        assert!(close(
            real_of(angle(&table, &product)),
            0.927_295_218_001_612_2 + FRAC_PI_2
        ));
    }

    #[test]
    fn get_misses_into_the_absent_option_and_put_overwrites() {
        let table = installed();
        assert!(
            table
                .get(&Key::sym("real-part"), &tag_list_key(&["spherical"]))
                .is_none()
        );
        table.put(
            Key::sym("magnitude"),
            tag_list_key(&["rectangular"]),
            Rc::new(|_| Ok(Value::sym("sentinel"))),
        );
        let z = make_from_real_imag(&table, 3.0, 4.0).expect("rectangular");
        assert_eq!(
            apply_generic(&table, "magnitude", std::slice::from_ref(&z))
                .expect("overwriting install wins"),
            Value::sym("sentinel")
        );
    }

    #[test]
    fn missing_handler_raises_a_typed_error_naming_op_and_tags() {
        let table = installed();
        let z = attach_tag(
            "spherical",
            Value::list(vec![Value::real(3.0), Value::real(4.0)]),
        );
        let err = apply_generic(&table, "magnitude", std::slice::from_ref(&z))
            .expect_err("no spherical package");
        assert_eq!(
            err.to_string(),
            "No method for these types: APPLY-GENERIC (magnitude (spherical))"
        );
    }

    #[test]
    fn message_object_answers_messages() {
        let z = make_from_real_imag_message_passing(3.0, 4.0);
        assert_eq!(
            apply_generic_message_passing("real-part", &z).expect("answered"),
            Value::real(3.0)
        );
        assert!(close(
            real_of(apply_generic_message_passing("magnitude", &z)),
            5.0
        ));
        let err = apply_generic_message_passing("rotation", &z).expect_err("unanswered");
        assert_eq!(err.to_string(), "Unknown op: MAKE-FROM-REAL-IMAG rotation");
    }
}
