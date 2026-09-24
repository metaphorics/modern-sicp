// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercises 2.73 and 2.73a, one module and one
//! ignored test each.

mod ex_2_73 {
    use sicp_runtime::Pending;

    /// Exercise 2.73: data-directed differentiation
    ///
    /// Re-expresses the section 2.3.2 `deriv` so the per-operator rules
    /// are found in the operation table instead of `match` arms, with the
    /// rules for sums, products, and exponentiation installed. Returns
    /// the printed derivatives of `(+ x 3)`, `(* x y)`, and `(** x 3)`
    /// with respect to `x`.
    pub fn ex_2_73() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.73"))
    }
}

mod ex_2_73a {
    use sicp_runtime::Pending;

    /// Exercise 2.73a (this edition's addition): a new variant, `atan`,
    /// with its rule installed in the same table. Returns the printed
    /// derivative of `(atan x)` with respect to `x`.
    pub fn ex_2_73a() -> Result<String, Pending> {
        Err(Pending::new("2.73a"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_73() {
    assert_eq!(
        ex_2_73::ex_2_73(),
        Ok((
            "1".to_string(),
            "y".to_string(),
            "(* 3 (** x 2))".to_string()
        ))
    );
}

#[test]
#[ignore = "pending solution"]
fn ex_2_73a() {
    assert_eq!(
        ex_2_73a::ex_2_73a(),
        Ok("(** (+ 1 (** x 2)) -1)".to_string())
    );
}
