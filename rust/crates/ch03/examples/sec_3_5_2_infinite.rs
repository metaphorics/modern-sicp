// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.5.2

//! Section 3.5.2: Infinite streams -- the integers, `no-sevens`, the
//! Fibonacci stream defined by adding the stream to its own tail, the
//! sieve of Eratosthenes, and the alternative primes built the other way
//! round, with the primality test running over the stream itself.

use ch03::sec_3_5::{
    add_streams, fibgen, fibs, integers, integers_starting_from, is_prime, no_sevens, ones, primes,
    primes_filtered, scale_stream, stream_map, stream_ref,
};

fn main() {
    // (stream-ref no-sevens 100) -- the 100th integer not divisible by 7.
    println!("{}", stream_ref(&no_sevens(), 100));
    // => 117
    assert_eq!(stream_ref(&no_sevens(), 100), 117);

    // The Fibonacci numbers, the stream added to its own tail.
    let fib_heads: Vec<i128> = fibs().iter().take(10).collect();
    println!("{fib_heads:?}");
    // => [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    assert_eq!(fib_heads, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);

    // fibgen is the same stream started from any two elements.
    let from_2_5: Vec<i128> = fibgen(2, 5).iter().take(6).collect();
    println!("{from_2_5:?}");
    // => [2, 5, 7, 12, 19, 31]
    assert_eq!(from_2_5, [2, 5, 7, 12, 19, 31]);

    // The fiftieth prime: the sieve only ever sieves what was asked for.
    println!("{}", stream_ref(&primes(), 50));
    // => 233
    assert_eq!(stream_ref(&primes(), 50), 233);
    let first_primes: Vec<i128> = primes().iter().take(7).collect();
    println!("{first_primes:?}");
    // => [2, 3, 5, 7, 11, 13, 17]
    assert_eq!(first_primes, [2, 3, 5, 7, 11, 13, 17]);

    // The alternative definition: primes filtered by the primality test
    // that reads this same primes stream. The book's subtle point -- the
    // test may stop before the stream has been built that far -- holds
    // because the shared memoized spine builds each tail once.
    let filtered_primes: Vec<i128> = primes_filtered().iter().take(7).collect();
    println!("{filtered_primes:?}");
    // => [2, 3, 5, 7, 11, 13, 17]
    assert_eq!(filtered_primes, [2, 3, 5, 7, 11, 13, 17]);
    for n in [2, 3, 5, 7, 11, 13, 17, 233] {
        assert!(is_prime(n, &primes_filtered()));
    }
    assert!(!is_prime(9, &primes_filtered()));

    // ones, add-streams, and scale-stream, the combinators of the
    // implicit-definition paragraphs.
    let ones_heads: Vec<i128> = ones().iter().take(4).collect();
    println!("{ones_heads:?}");
    // => [1, 1, 1, 1]
    assert_eq!(ones_heads, [1, 1, 1, 1]);
    let integers_heads: Vec<i128> = integers().iter().take(6).collect();
    println!("{integers_heads:?}");
    // => [1, 2, 3, 4, 5, 6]
    assert_eq!(integers_heads, [1, 2, 3, 4, 5, 6]);
    let doubles: Vec<i128> = scale_stream(&integers(), 2).iter().take(5).collect();
    println!("{doubles:?}");
    // => [2, 4, 6, 8, 10]
    assert_eq!(doubles, [2, 4, 6, 8, 10]);
    let sums: Vec<i128> = add_streams(&integers(), &integers())
        .iter()
        .take(5)
        .collect();
    println!("{sums:?}");
    // => [2, 4, 6, 8, 10]
    assert_eq!(sums, [2, 4, 6, 8, 10]);

    // integers-starting-from with an offset.
    let from_79: Vec<i128> = integers_starting_from(79).iter().take(3).collect();
    println!("{from_79:?}");
    // => [79, 80, 81]
    assert_eq!(from_79, [79, 80, 81]);

    // A prefix of the doubled integers, as display-stream would print it.
    let evens: Vec<i128> = stream_map(|x| x * 2, &integers_starting_from(1))
        .iter()
        .take(3)
        .collect();
    println!("{evens:?}");
    // => [2, 4, 6]
    assert_eq!(evens, [2, 4, 6]);
}
