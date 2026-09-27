// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.73: why `flatten-stream` delays
//! its recursion. The delayed combinator produces the first inner
//! stream's element without so much as constructing the second; the
//! undelayed twin must construct every inner stream before the first
//! element emerges. A construction that burns a fuel budget makes the
//! difference measurable, and on finite inputs both agree element for
//! element.

use std::cell::Cell;
use std::rc::Rc;

use ch04::sec_4_4::{Frame, flatten_stream, flatten_stream_undelayed, singleton_stream};
use sicp_runtime::Stream;
use sicp_runtime::Value;

mod ex_4_73 {
    //! Exercise 4.73: the delayed versus undelayed flatten.

    use super::*;

    /// An inner stream whose construction burns the `fuel` budget
    /// synchronously before it returns even one element, marking the
    /// flag when the budget is gone -- the measurable stand-in for a
    /// construction that diverges.
    pub fn burning_inner(fuel: usize, flag: &Rc<Cell<bool>>) -> Stream<Frame> {
        let mut burned = 0usize;
        while burned < fuel {
            burned += 1;
        }
        flag.set(true);
        let frame = Frame::new().extend(Value::sym("marker"), Value::sym("built"));
        Stream::cons_stream(frame, || Stream::Empty)
    }

    /// The probe input: one cheap inner stream, then one expensive one.
    pub fn probe_input(fuel: usize, flag: &Rc<Cell<bool>>) -> Stream<Stream<Frame>> {
        let inner1 = singleton_stream(Frame::new());
        let flag2 = Rc::clone(flag);
        Stream::cons_stream(inner1, move || {
            let inner2 = burning_inner(fuel, &flag2);
            Stream::cons_stream(inner2, || Stream::Empty)
        })
    }
}

#[test]
fn ex_4_73() {
    // The delayed flatten yields the first element while the expensive
    // construction has not even started. head()/tail() are used
    // directly: the stream iterator pre-forces one tail per step.
    let flag = Rc::new(Cell::new(false));
    let delayed = flatten_stream(ex_4_73::probe_input(200_000, &flag));
    let first = delayed.head().clone();
    assert!(first.is_empty(), "the first element is the cheap stream's");
    assert!(
        !flag.get(),
        "the delay kept the expensive construction waiting"
    );

    // Forcing the tail constructs the second inner stream, which burns
    // its budget and raises the flag.
    let rest = delayed.tail();
    assert!(!rest.is_empty());
    assert!(flag.get());

    // The undelayed twin on the same input: it must construct the
    // second inner stream before the first element emerges, so the
    // budget is gone before anything arrives.
    let flag2 = Rc::new(Cell::new(false));
    let undelayed = flatten_stream_undelayed(ex_4_73::probe_input(200_000, &flag2));
    assert!(!undelayed.is_empty());
    assert!(flag2.get(), "the construction ran before any element");

    // On finite inputs the two agree element for element.
    let plain = Stream::cons_stream(singleton_stream(Frame::new()), || {
        Stream::cons_stream(singleton_stream(Frame::new()), || Stream::Empty)
    });
    let left: Vec<bool> = flatten_stream(plain.clone())
        .iter()
        .map(|f| f.is_empty())
        .collect();
    let right: Vec<bool> = flatten_stream_undelayed(plain)
        .iter()
        .map(|f| f.is_empty())
        .collect();
    assert_eq!(left, right);
    assert_eq!(left, [true, true]);
}
