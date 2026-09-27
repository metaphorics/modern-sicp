// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.77: the direct integral, the
//! `integers-starting-from` style instead of the implicit feedback
//! style, rewritten so the integrand arrives delayed. The answer is the
//! initial value followed by a running total; each step adds dt times
//! the current integrand element, and a finite integrand answers a
//! finite stream -- the tail after the last partial sum is
//! the-empty-stream. Because the integrand is forced only when the
//! answer's first tail is demanded, the direct form can serve the
//! solve loop, whose integrand needs the answer it feeds.

use ch03::sec_3_5::{
    Stream, cons_stream, self_stream, solve, stream_enumerate_interval, stream_map, stream_ref,
};

/// The exercise's direct integral with a delayed integrand:
/// `initial_value` followed by the running total, one element per
/// integrand element and none after. `delayed_integrand` is forced
/// once, inside the answer's first tail thunk, which is what lets the
/// integrand depend on the answer itself.
#[must_use]
fn integral_direct_delayed<F>(delayed_integrand: F, initial_value: f64, dt: f64) -> Stream<f64>
where
    F: FnOnce() -> Stream<f64> + 'static,
{
    cons_stream(initial_value, move || {
        let integrand = delayed_integrand();
        running_total(&integrand, initial_value, dt)
    })
}

/// The tail loop of [`integral_direct_delayed`]: the empty integrand
/// ends the stream; otherwise cons `accumulated + dt*head` onto the
/// delayed rest, whose own cons delay is the recursion's "delay of
/// tail".
#[must_use]
fn running_total(integrand: &Stream<f64>, accumulated: f64, dt: f64) -> Stream<f64> {
    if integrand.is_empty() {
        return Stream::Empty;
    }
    let next = accumulated + dt * integrand.head();
    let source = integrand.clone();
    cons_stream(next, move || running_total(&source.tail(), next, dt))
}

/// The section's `solve`, rebuilt on the direct integral: `dy` maps `f`
/// over the answer stream itself and is supplied as the delayed
/// integrand. `f` is cloned into the map's tail thunks, as in the
/// section's own `solve`.
#[must_use]
fn solve_direct<F>(f: F, y0: f64, dt: f64) -> Stream<f64>
where
    F: Fn(f64) -> f64 + Clone + 'static,
{
    self_stream(move |y| {
        let named = y.clone();
        integral_direct_delayed(
            move || stream_map(move |value| f(*value), &named.stream()),
            y0,
            dt,
        )
    })
}

mod ex_3_77 {
    use super::{
        integral_direct_delayed, solve, solve_direct, stream_enumerate_interval, stream_map,
        stream_ref,
    };

    /// Exercise 3.77: integral with delayed integrand
    ///
    /// Answers the direct integral of the integers 0..=9 at dt = 1 from
    /// initial value 0, the number of elements a full walk of that
    /// finite integral visits, element 1000 of the direct solve of
    /// dy/dt = y with y0 = 1 at dt = 0.001, and the same element from
    /// the section's own solve.
    #[must_use]
    pub fn ex_3_77() -> ([f64; 11], usize, f64, f64) {
        let finite = integral_direct_delayed(
            // 0..=9 fits f64 exactly, so the widening cast is lossless.
            #[expect(
                clippy::cast_precision_loss,
                reason = "the integrand starts as small integers, exact in f64"
            )]
            || stream_map(|n: &i128| *n as f64, &stream_enumerate_interval(0, 9)),
            0.0,
            1.0,
        );
        let mut sums = [0.0; 11];
        for (slot, value) in sums.iter_mut().zip(finite.iter()) {
            *slot = value;
        }
        let walked = finite.iter().count();
        let direct = stream_ref(&solve_direct(|y| y, 1.0, 0.001), 1000);
        let module = stream_ref(&solve(|y| y, 1.0, 0.001), 1000);
        (sums, walked, direct, module)
    }
}

#[test]
fn ex_3_77() {
    let (sums, walked, direct, module) = ex_3_77::ex_3_77();

    // (a) Integrating the integers 0..=9 at dt = 1 from 0 answers the
    // running triangular totals: element k is sum_{j<k} j = k(k-1)/2.
    for (k, got) in sums.iter().enumerate() {
        let k = i128::try_from(k).expect("index fits i128");
        #[expect(
            clippy::cast_precision_loss,
            reason = "the triangular totals stay exact in f64 at this range"
        )]
        let want = (k * (k - 1) / 2) as f64;
        assert!((got - want).abs() < 1e-12, "sum {k}: {got} vs {want}");
    }
    // The stream ends with the integrand: a full walk terminates after
    // 11 elements, the initial value plus one per integrand element.
    assert_eq!(walked, 11);

    // (b) The delayed integrand is what the solve loop needs: the
    // direct form carries dy/dt = y from y0 = 1 at dt = 0.001, and
    // element 1000 is the Euler value (1 + dt)^1000, within 1e-2 of e
    // (the run answers 2.716923932235896; the book pins 2.71692...).
    assert!(
        (direct - std::f64::consts::E).abs() < 1e-2,
        "direct solve at 1000: {direct}"
    );
    // The direct and implicit integrators perform the same additions in
    // commuted order, so the two solves agree to the last bit.
    assert!(
        (direct - module).abs() < 1e-12,
        "direct {direct} vs module {module}"
    );
}
