// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.73: the RC circuit as a
//! signal-processing system. The injected-current stream is scaled by
//! 1/C and run through the section's implicit integrator, seeded with
//! the initial capacitor voltage v0; a parallel path scales the current
//! by r, and the adder combines both, exactly the Figure 3.33 wiring:
//! v[k] = v0 + (1/C)*dt*sum_{j<k} i[j] + r*i[k].

use ch03::sec_3_5::{Stream, add_streams, cons_stream, integral, scale_stream, self_stream};

/// Builds the RC circuit's voltage processor: `rc(r, c, dt)` answers
/// the function that maps a current stream and the initial capacitor
/// voltage v0 to the voltage stream v.
fn rc(r: f64, c: f64, dt: f64) -> impl Fn(&Stream<f64>, f64) -> Stream<f64> {
    move |current, v0| {
        let integrated = integral(&scale_stream(current, 1.0 / c), v0, dt);
        add_streams(&integrated, &scale_stream(current, r))
    }
}

/// The test's injected current: the constant unit signal, an infinite
/// stream defined in terms of itself like the section's `ones`.
#[must_use]
fn unit_current() -> Stream<f64> {
    self_stream(|ones| cons_stream(1.0, move || ones.stream()))
}

mod ex_3_73 {
    use super::{rc, unit_current};

    /// Exercise 3.73: RC circuit signal processor
    ///
    /// Answers the first six voltages of the unit-current response for
    /// r = 0.5 ohm, c = 1 farad, dt = 0.2 second, v0 = 0.
    #[must_use]
    pub fn ex_3_73() -> [f64; 6] {
        let response = rc(0.5, 1.0, 0.2);
        let voltages = response(&unit_current(), 0.0);
        let mut prefix = [0.0; 6];
        for (slot, value) in prefix.iter_mut().zip(voltages.iter()) {
            *slot = value;
        }
        prefix
    }
}

#[test]
fn ex_3_73() {
    let v = ex_3_73::ex_3_73();
    // With i constant 1, the integrator answers v0 + dt*k = 0.2*k at
    // index k, and the resistor path adds r*i = 0.5 at every index --
    // including index 0, where the initial-value path (v0 = 0 carried
    // inside the integral) meets the resistor term in the adder:
    // v[k] = 0.2*k + 0.5.
    for (got, want) in v.iter().zip([0.5, 0.7, 0.9, 1.1, 1.3, 1.5]) {
        assert!((got - want).abs() < 1e-12, "v: {got} vs {want}");
    }
}
