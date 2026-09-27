// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.80: the series RLC circuit of
//! Figure 3.37, a network of two integrals whose state streams feed
//! each other. The capacitor voltage `v_C` integrates `-i_L/C`; the
//! inductor current `i_L` integrates `v_C/L - (R/L) i_L`. Each
//! integrand reads the other's stream, so both arrive delayed, and the
//! one name that outlives its definition -- `i_L`'s integrand needs
//! `v_C` while `v_C` is still being built -- is routed through an
//! explicit definition slot, Scheme's mutually visible internal
//! `define`s made mechanical.

use std::cell::OnceCell;
use std::rc::Rc;

use ch03::sec_3_5::{Stream, add_streams, integral_delayed, scale_stream, self_stream, stream_ref};

/// The book's `RLC`: answers the procedure from the initial state
/// `(v_C0, i_L0)` to the pair of state streams `(v_C, i_L)` of the
/// circuit with resistance `r`, inductance `l`, and capacitance `c`,
/// integrated at step `dt`. Building the pair constructs `i_L` first
/// (its integrand reads `v_C` out of the slot), then `v_C`, then fills
/// the slot -- before any tail thunk can run, so every integrand reads
/// finished streams only.
///
/// # Panics
/// Panics only on a violation of the construction's own definition
/// discipline: reading the slot before `v_C` exists, or defining
/// `v_C` twice. The constructor fills the slot after `v_C` is built
/// and before the pair is returned, and every integrand touches the
/// slot only from a tail thunk, so neither trip is reachable through
/// the returned closure. The returned procedure answers the pair of
/// streams and never panics: the integrals it builds consume their
/// inputs only through memoized tail forces.
#[must_use]
#[expect(
    clippy::double_must_use,
    reason = "the must_use names the returned procedure's contract, not just the value"
)]
pub fn rlc(r: f64, l: f64, c: f64, dt: f64) -> impl Fn(f64, f64) -> (Stream<f64>, Stream<f64>) {
    move |v_c0: f64, i_l0: f64| {
        let v_c_slot: Rc<OnceCell<Stream<f64>>> = Rc::new(OnceCell::new());
        let i_l = {
            let v_c_slot = Rc::clone(&v_c_slot);
            self_stream(move |i_l_name| {
                // d i_L/dt = (1/L) v_C - (R/L) i_L, the adder of
                // Figure 3.37's lower loop.
                let di_l = {
                    let v_c_slot = Rc::clone(&v_c_slot);
                    move || {
                        let v_c = v_c_slot.get().expect("v_c stream defined").clone();
                        let i_l_stream = i_l_name.stream();
                        add_streams(
                            &scale_stream(&v_c, 1.0 / l),
                            &scale_stream(&i_l_stream, -r / l),
                        )
                    }
                };
                integral_delayed(di_l, i_l0, dt)
            })
        };
        // d v_C/dt = -i_L/C, the scaled feedback of the upper loop.
        let dv_c = {
            let i_l = i_l.clone();
            move || scale_stream(&i_l, -1.0 / c)
        };
        let v_c = integral_delayed(dv_c, v_c0, dt);
        assert!(v_c_slot.set(v_c.clone()).is_ok(), "v_c defined twice");
        (v_c, i_l)
    }
}

mod ex_3_80 {
    use super::{rlc, stream_ref};

    /// Exercise 3.80: RLC coupled streams
    ///
    /// Answers the book's circuit run (`R` = 1 ohm, `L` = 1 henry,
    /// `C` = 0.2 farad, `dt` = 0.1 second, `v_C0` = 10 volts, `i_L0` =
    /// 0 amps): the first six samples each of the `v_C` and `i_L`
    /// streams, in seconds 0.0 through 0.5.
    #[must_use]
    pub fn ex_3_80() -> ([f64; 6], [f64; 6]) {
        let circuit = rlc(1.0, 1.0, 0.2, 0.1);
        let (v_c, i_l) = circuit(10.0, 0.0);
        (
            std::array::from_fn(|i| stream_ref(&v_c, i)),
            std::array::from_fn(|i| stream_ref(&i_l, i)),
        )
    }
}

#[test]
fn ex_3_80() {
    let (v_c, i_l) = ex_3_80::ex_3_80();
    // The circuit opens at the stated physical state: v_C(0) = 10,
    // i_L(0) = 0. From there the Euler steps are hand-checkable:
    // dv_C[0] = -i_L[0]/C = 0 so v_C[1] = 10; di_L[0] = v_C[0]/L = 10
    // so i_L[1] = 1; and so on, the capacitor discharging into the
    // rising, R-damped inductor current.
    for (v, pinned) in v_c.into_iter().zip(PINNED_V_C) {
        assert!((v - pinned).abs() < 1e-12);
    }
    for (i, pinned) in i_l.into_iter().zip(PINNED_I_L) {
        assert!((i - pinned).abs() < 1e-12);
    }
    // The physical check the statement asks to see: the current peaks
    // positive and then the pair relaxes as the resistor dissipates --
    // by 0.5 s the capacitor voltage has fallen 10 -> 5.5955 V while
    // the current has risen 0 -> 3.6461 A, heading for both to 0.
    assert!(i_l[1] > i_l[0]);
    assert!(i_l[5] > i_l[1]);
    assert!(v_c[5] < v_c[0]);
}

/// The measured `v_C` samples 0 through 5 of the book's run.
const PINNED_V_C: [f64; 6] = [10.0, 10.0, 9.5, 8.55, 7.22, 5.595_5];

/// The measured `i_L` samples 0 through 5 of the book's run.
const PINNED_I_L: [f64; 6] = [0.0, 1.0, 1.9, 2.66, 3.249, 3.646_1];
