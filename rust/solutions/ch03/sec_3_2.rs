// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.2: a monitored procedure
//! counts and resets.

mod ex_3_02 {
    /// A request to a monitored procedure: the book's `how-many-calls?`
    /// and `reset-count` symbols became values, alongside the ordinary
    /// call carrying its argument.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MonitorRequest {
        /// Call the monitored function with this argument.
        Call(f64),
        /// Ask how many calls have been made.
        HowManyCalls,
        /// Set the counter back to zero.
        ResetCount,
    }

    /// A monitored procedure's answer: either the wrapped function's
    /// result or the counter.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MonitorReply {
        /// The result of calling the monitored function.
        Result(f64),
        /// The value of the counter.
        Count(u64),
    }

    /// Exercise 3.2: a monitored procedure counts and resets
    ///
    /// Wraps `f` in a closure that owns the counter, increments it
    /// around every real call, and answers the two bookkeeping requests
    /// without touching `f`.
    pub fn make_monitored(f: impl FnMut(f64) -> f64) -> impl FnMut(MonitorRequest) -> MonitorReply {
        let mut f = f;
        let mut count = 0_u64;
        move |request| match request {
            MonitorRequest::Call(x) => {
                count += 1;
                MonitorReply::Result(f(x))
            }
            MonitorRequest::HowManyCalls => MonitorReply::Count(count),
            MonitorRequest::ResetCount => {
                count = 0;
                MonitorReply::Count(count)
            }
        }
    }

    /// Reads the counter out of a bookkeeping answer.
    #[must_use]
    pub fn count_of(reply: MonitorReply) -> u64 {
        match reply {
            MonitorReply::Count(count) => count,
            MonitorReply::Result(_) => unreachable!("bookkeeping answers with a count"),
        }
    }

    /// Reads the value out of a call answer.
    #[must_use]
    pub fn result_of(reply: MonitorReply) -> f64 {
        match reply {
            MonitorReply::Result(value) => value,
            MonitorReply::Count(_) => unreachable!("a call answers with a result"),
        }
    }

    /// Exercise 3.2: a monitored procedure counts and resets
    ///
    /// Returns the result of the monitored square root at 100, the call
    /// count queried right after, and the count after a reset.
    #[must_use]
    pub fn ex_3_02() -> (f64, u64, u64) {
        let mut s = make_monitored(f64::sqrt);
        let root = result_of(s(MonitorRequest::Call(100.0)));
        let count = count_of(s(MonitorRequest::HowManyCalls));
        let reset = count_of(s(MonitorRequest::ResetCount));
        (root, count, reset)
    }
}

#[test]
fn ex_3_02() {
    use ex_3_02::{MonitorRequest, count_of, make_monitored, result_of};

    let (root, count, after_reset) = ex_3_02::ex_3_02();
    assert!((root - 10.0).abs() < 1e-9);
    assert_eq!((count, after_reset), (1, 0));

    // The count tracks real calls only: bookkeeping requests leave it
    // alone, calls advance it, and the reset restarts it from zero.
    let mut s = make_monitored(f64::sqrt);
    assert!((result_of(s(MonitorRequest::Call(144.0))) - 12.0).abs() < 1e-9);
    assert_eq!(count_of(s(MonitorRequest::HowManyCalls)), 1);
    assert_eq!(count_of(s(MonitorRequest::ResetCount)), 0);
    assert_eq!(count_of(s(MonitorRequest::HowManyCalls)), 0);
}
