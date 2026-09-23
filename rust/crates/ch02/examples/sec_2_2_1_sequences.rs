// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.2.1

//! Section 2.2.1: representing sequences as chains of pairs, and the
//! list operations built by cdring down and consing up.

use ch02::sec_2_2::{List, append, length, length_iter, list_ref, map_list, scale_list};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The sequence 1, 2, 3, 4 as a chain of pairs, built the edition's
    // way; the book's `(list 1 2 3 4)` is this constructor call.
    let one_through_four: List<i128> = List::from_iter([1, 2, 3, 4]);
    println!("{one_through_four}");
    // => (1 2 3 4)
    assert_eq!(one_through_four.to_string(), "(1 2 3 4)");

    // `car` selects the first item; `cdr` the sublist of all the rest;
    // nested applications reach further down the chain.
    let first = one_through_four.car().copied().expect("cons cell");
    println!("{first}");
    // => 1
    assert_eq!(first, 1);
    let rest = one_through_four.cdr().expect("cons cell");
    println!("{rest}");
    // => (2 3 4)
    assert_eq!(rest.to_string(), "(2 3 4)");
    let second = rest.car().copied().expect("cons cell");
    println!("{second}");
    // => 2
    assert_eq!(second, 2);

    // `cons` builds a list like the original with one more item at the
    // front, and the original spine is shared, not copied.
    println!("{}", List::cons(10, &one_through_four));
    // => (10 1 2 3 4)
    println!("{}", List::cons(5, &one_through_four));
    // => (5 1 2 3 4)
    assert_eq!(
        List::cons(10, &one_through_four).to_string(),
        "(10 1 2 3 4)"
    );
    assert_eq!(List::cons(5, &one_through_four).to_string(), "(5 1 2 3 4)");

    // `list-ref` cdrs down n times and takes the car.
    let squares: List<i128> = List::from_iter([1, 4, 9, 16, 25]);
    let third = list_ref(&squares, 3)?;
    println!("{third}");
    // => 16
    assert_eq!(third, 16);

    // `length`, in the book's recursive and iterative forms.
    let odds: List<i128> = List::from_iter([1, 3, 5, 7]);
    println!("{}", length(&odds));
    // => 4
    println!("{}", length_iter(&odds));
    // => 4
    assert_eq!(length(&odds), 4);
    assert_eq!(length_iter(&odds), 4);

    // `append` combines two lists, in either order.
    println!("{}", append(&squares, &odds));
    // => (1 4 9 16 25 1 3 5 7)
    println!("{}", append(&odds, &squares));
    // => (1 3 5 7 1 4 9 16 25)
    assert_eq!(append(&squares, &odds).to_string(), "(1 4 9 16 25 1 3 5 7)");
    assert_eq!(append(&odds, &squares).to_string(), "(1 3 5 7 1 4 9 16 25)");

    // `scale-list`, in its final form as a `map` call.
    let items: List<i128> = List::from_iter([1, 2, 3, 4, 5]);
    let scaled = scale_list(&items, 10)?;
    println!("{scaled}");
    // => (10 20 30 40 50)
    assert_eq!(scaled.to_string(), "(10 20 30 40 50)");

    // `map` applies any one-argument procedure elementwise.
    let with_floats: List<f64> = List::from_iter([-10.0, 2.5, -11.6, 17.0]);
    let absolute = map_list(|x| x.abs(), &with_floats);
    println!("{absolute}");
    // => (10 2.5 11.6 17)
    assert_eq!(absolute.to_string(), "(10 2.5 11.6 17)");
    let squares_of_first_four = map_list(|x: &i128| x * x, &List::from_iter([1, 2, 3, 4]));
    println!("{squares_of_first_four}");
    // => (1 4 9 16)
    assert_eq!(squares_of_first_four.to_string(), "(1 4 9 16)");

    Ok(())
}
