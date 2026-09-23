// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.2.3

//! Section 2.2.3: sequences as conventional interfaces. The signal
//! -flow stages become range expressions and iterator adapters, and
//! the pipelines read like the diagrams.

use ch02::sec_2_2::{
    List, Record, accumulate, enumerate_interval, enumerate_tree, even_fibs, filter, is_prime,
    leaf, list_fib_squares, permutations, prime_sum_pairs, product_of_squares_of_odd_elements,
    salary_of_highest_paid_programmer, sub, sum_odd_squares,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Filtering a sequence selects the elements satisfying a predicate.
    let odd: Vec<i128> = filter(|x| x % 2 != 0, &[1, 2, 3, 4, 5]);
    println!("{odd:?}");
    // => [1, 3, 5]
    assert_eq!(odd, vec![1, 3, 5]);

    // Accumulations: `+` from 0, `*` from 1, and `cons` from the empty
    // list, which rebuilds the sequence.
    println!(
        "{}",
        accumulate(|x, acc: i128| acc + x, 0, &[1, 2, 3, 4, 5])
    );
    // => 15
    println!(
        "{}",
        accumulate(|x, acc: i128| acc * x, 1, &[1, 2, 3, 4, 5])
    );
    // => 120
    let rebuilt: List<i128> = accumulate(
        |x, acc: List<i128>| List::cons(*x, &acc),
        List::Nil,
        &[1, 2, 3, 4, 5],
    );
    println!("{rebuilt}");
    // => (1 2 3 4 5)
    assert_eq!(rebuilt.to_string(), "(1 2 3 4 5)");

    // Enumerating an interval of integers is a range expression; the
    // edition states the translation outright.
    let interval = enumerate_interval(2, 7);
    println!("{interval:?}");
    // => [2, 3, 4, 5, 6, 7]
    assert_eq!(interval, vec![2, 3, 4, 5, 6, 7]);

    // Enumerating the leaves of a tree flattens it into a sequence.
    let tree = sub(&[leaf(1), sub(&[leaf(2), sub(&[leaf(3), leaf(4)])]), leaf(5)]);
    let leaves = enumerate_tree(&tree);
    println!("{leaves}");
    // => (1 2 3 4 5)
    assert_eq!(leaves.to_string(), "(1 2 3 4 5)");

    // The signal-flow pipelines. `sum-odd-squares`: enumerate the
    // leaves, filter the odd ones, square, accumulate with `+` from 0.
    let odd_square_sum = sum_odd_squares(&tree)?;
    println!("{odd_square_sum}");
    // => 35
    assert_eq!(odd_square_sum, 35);

    // `even-fibs`: the range enumerates, `map` computes, `filter`
    // selects, and collecting into a list is the accumulate with cons.
    let evens = even_fibs(10)?;
    println!("{evens}");
    // => (0 2 8 34)
    assert_eq!(evens.to_string(), "(0 2 8 34)");

    // The stages recombine freely: squares of the first eleven
    // Fibonacci numbers.
    let fib_squares = list_fib_squares(10)?;
    println!("{fib_squares}");
    // => (0 1 1 4 9 25 64 169 441 1156 3025)
    assert_eq!(
        fib_squares.to_string(),
        "(0 1 1 4 9 25 64 169 441 1156 3025)"
    );

    // The product of the squares of the odd elements.
    let product = product_of_squares_of_odd_elements(&[1, 2, 3, 4, 5])?;
    println!("{product}");
    // => 225
    assert_eq!(product, 225);

    // A conventional data-processing application: the highest-paid
    // programmer among personnel records.
    let records = [
        Record {
            programmer: true,
            salary: 85_000,
        },
        Record {
            programmer: false,
            salary: 100_000,
        },
        Record {
            programmer: true,
            salary: 75_000,
        },
    ];
    println!("{}", salary_of_highest_paid_programmer(&records));
    // => 85000
    assert_eq!(salary_of_highest_paid_programmer(&records), 85_000);

    // Nested mappings: prime-sum-pairs over `(enumerate-interval 1 n)`.
    let pairs = prime_sum_pairs(6);
    println!("{pairs:?}");
    // => [(2, 1, 3), (3, 2, 5), (4, 1, 5), (4, 3, 7), (5, 2, 7), (6, 1, 7), (6, 5, 11)]
    assert_eq!(pairs.len(), 7);
    assert!(pairs.iter().all(|(i, j, s)| is_prime(i + j) && *s == i + j));

    // Permutations: for each item, the permutations of the rest with
    // the item adjoined at the front.
    let perms = permutations(&[1, 2, 3]);
    println!("{perms:?}");
    // => [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
    assert_eq!(perms.len(), 6);

    Ok(())
}
