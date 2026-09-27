// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.75, one module and one ignored
//! test.

mod ex_2_75 {
    use sicp_runtime::Pending;

    /// Exercise 2.75: `make-from-mag-ang` in message-passing style
    ///
    /// Builds the polar complex number with magnitude 5 and the angle of
    /// `(3 . 4)` as a message-passing object, then asks it for its
    /// magnitude and angle. Returns both, printed.
    pub fn ex_2_75() -> Result<(String, String), Pending> {
        Err(Pending::new("2.75"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_75() {
    assert_eq!(
        ex_2_75::ex_2_75(),
        Ok(("5".to_string(), "0.9272952180016122".to_string()))
    );
}
