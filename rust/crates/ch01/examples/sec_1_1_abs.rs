// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 1.1, listing 4: case analysis with `match` and `if`, and the
//! logical operators `&&`, `||`, `!`.

fn main() {
    // The book composes `>=` from `or` and, alternatively, from `not` and
    // `<`; both compositions are spelled out here rather than collapsed to
    // Rust's own `>=`, which is the point of the example.
    #[allow(clippy::double_comparisons)]
    fn geq_or(x: i64, y: i64) -> bool {
        x > y || x == y
    }

    #[allow(clippy::nonminimal_bool)]
    fn geq_not(x: i64, y: i64) -> bool {
        !(x < y)
    }

    {
        fn abs(x: i64) -> i64 {
            match x {
                y if y > 0 => y,
                0 => 0,
                _ => -x,
            }
        }

        println!("{}", abs(-5));
        // => 5
        assert_eq!(abs(-5), 5);
        assert_eq!(abs(0), 0);
        assert_eq!(abs(7), 7);
    }

    {
        fn abs(x: i64) -> i64 {
            if x < 0 { -x } else { x }
        }

        println!("{}", abs(-5));
        // => 5
        assert_eq!(abs(-5), 5);
    }

    let x = 7;
    let in_range = x > 5 && x < 10;
    println!("{in_range}");
    // => true
    assert!(in_range);

    println!("{}", geq_or(4, 3));
    // => true
    assert!(geq_or(4, 3));
    assert!(geq_not(4, 3));
    assert!(!geq_or(2, 3));
    assert!(!geq_not(2, 3));
}
