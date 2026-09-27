// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.5.1

//! Section 3.5.1: Streams are delayed lists -- the interval/filter walk
//! that shows a stream computing only what its consumer demands, the
//! selectors and combinators of the stream abstraction, and the
//! `cons-stream`/`delay` discipline that makes it work.

use std::cell::RefCell;
use std::rc::Rc;

use ch03::sec_3_5::{
    delay, display_stream, force, integers_starting_from, is_prime, primes,
    stream_enumerate_interval, stream_filter, stream_for_each, stream_head, stream_map, stream_ref,
    stream_tail, the_empty_stream,
};

/// The book's `prime?` of 1.2.6, trial division over the small primes.
fn prime(n: i128) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn main() {
    // The stream of the interval is one pair now, the rest on demand.
    let interval = stream_enumerate_interval(10_000, 1_000_000);
    println!("{}", *stream_head(&interval));
    // => 10000
    assert_eq!(*stream_head(&interval), 10000);
    let interval_tail = stream_tail(&interval);
    println!("{}", *stream_head(&interval_tail));
    // => 10001
    assert_eq!(*stream_head(&interval_tail), 10001);

    // The second prime after 10,000: the filter and the interval cons
    // just enough pairs to answer, the demand-driven walk of the text.
    let filtered = stream_filter(|x| prime(*x), &interval);
    println!("{}", *stream_head(&filtered));
    // => 10007
    assert_eq!(*stream_head(&filtered), 10007);
    let second = stream_tail(&filtered);
    println!("{}", second.head());
    // => 10009
    assert_eq!(second.head(), &10009);
    let third = stream_tail(&stream_tail(&filtered));
    println!("{}", third.head());
    // => 10037
    assert_eq!(third.head(), &10037);

    // stream-ref and stream-for-each over finite streams.
    let tens = stream_enumerate_interval(10, 20);
    println!("{}", stream_ref(&tens, 3));
    // => 13
    assert_eq!(stream_ref(&tens, 3), 13);
    let mut seen = Vec::new();
    stream_for_each(|x| seen.push(*x), &tens);
    println!("{seen:?}");
    // => [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
    assert_eq!(seen, [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    let lines = display_stream(&stream_enumerate_interval(1, 4));
    println!("{lines:?}");
    // => ["1", "2", "3", "4"]
    assert_eq!(lines, ["1", "2", "3", "4"]);

    // The empty stream is a value like any other, and map and filter of
    // it are empty again.
    let empty: ch03::sec_3_5::Stream<i128> = the_empty_stream();
    assert!(empty.is_empty());
    let mapped_empty = stream_map(|x| x * 2, &the_empty_stream::<i128>());
    println!("{}", mapped_empty.is_empty());
    // => true
    assert!(mapped_empty.is_empty());

    // delay builds the memoized promise; force runs the body once.
    let runs = Rc::new(RefCell::new(0u32));
    let runs_in_body = Rc::clone(&runs);
    let promise = delay(move || {
        *runs_in_body.borrow_mut() += 1;
        40 + 2
    });
    let first = force(&promise);
    let second = force(&promise);
    println!("{first} {second}");
    // => 42 42
    assert_eq!((*first, *second), (42, 42));
    println!("{}", *runs.borrow());
    // => 1
    assert_eq!(*runs.borrow(), 1, "the memoized promise ran its body once");

    // The stream abstraction, used the way Chapter 2's sequences were:
    // the same square-of-the-odd-elements pipeline, demand-driven.
    let stream_of_primes = primes();
    let _ = stream_of_primes.iter().take(5).collect::<Vec<i128>>();
    let odds_squared: Vec<i128> = stream_map(
        |x| x * x,
        &stream_filter(|x| x % 2 == 1, &integers_starting_from(1)),
    )
    .iter()
    .take(4)
    .collect();
    println!("{odds_squared:?}");
    // => [1, 9, 25, 49]
    assert_eq!(odds_squared, [1, 9, 25, 49]);

    // The selectors error on the empty stream, as in the book.
    let empty = the_empty_stream::<i128>();
    let blown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = stream_head(&empty);
    }));
    println!("{}", blown.is_err());
    // => true
    assert!(blown.is_err(), "stream-car of the empty stream is an error");

    // The primes stream and the primality test of the text agree.
    for n in [2, 3, 5, 7, 11, 13] {
        assert!(is_prime(n, &primes()));
    }
    assert!(!is_prime(4, &primes()));
}
