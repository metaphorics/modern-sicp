// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.53, one module and one test.

mod ex_2_53 {
    use ch02::sec_2_2::{List, Nest, leaf, sub};
    use ch02::sec_2_3::memq;
    use sicp_runtime::Symbol;

    fn sym(s: &str) -> Symbol {
        Symbol::from(s)
    }

    /// Exercise 2.53 (replacement): predicting printed values
    ///
    /// See the exercise's doc comment for the seven sub-questions, in
    /// order. `car(a, short, list)` is never a pair: a `List<Symbol>`'s
    /// `car` is a `Symbol`, which cannot be a compound value by the
    /// type itself, so the edition answers `false` by construction, not
    /// by a runtime check. `memq('red, '((red shoes) (blue socks)))` is
    /// likewise `false`: the outer list holds sublists (`Nest::Sub`),
    /// never the leaf `red`, so no element is ever `eq?` to it.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's seven-part statement"
    )]
    pub fn ex_2_53() -> (String, String, String, String, String, String, String) {
        let abc: List<Symbol> = List::from_iter([sym("a"), sym("b"), sym("c")]);
        let nested_george = sub(&[sub(&[leaf(sym("george"))])]);

        let pairs: List<Nest<Symbol>> = List::from_iter([
            sub(&[leaf(sym("x1")), leaf(sym("x2"))]),
            sub(&[leaf(sym("y1")), leaf(sym("y2"))]),
        ]);
        let pairs_tail = pairs.cdr().expect("nonempty").to_string();
        let pairs_second = pairs
            .cdr()
            .and_then(List::car)
            .expect("two elements")
            .to_string();

        let short_list: List<Symbol> = List::from_iter([sym("a"), sym("short"), sym("list")]);
        // `short_list.car()` is `Some(&Symbol)`: a `Symbol` can never
        // itself be a compound value, so no runtime `pair?` check is
        // needed to know the answer is `false`.
        let _first_of_short_list = short_list.car().expect("nonempty");
        let car_is_pair = "false".to_string();

        let outer = sub(&[
            sub(&[leaf(sym("red")), leaf(sym("shoes"))]),
            sub(&[leaf(sym("blue")), leaf(sym("socks"))]),
        ]);
        let outer_list = match outer {
            Nest::Sub(items) => (*items).clone(),
            Nest::Leaf(_) => List::Nil,
        };
        let red_among_sublists = memq(&Nest::Leaf(sym("red")), &outer_list)
            .is_some()
            .to_string();

        let flat: List<Symbol> =
            List::from_iter([sym("red"), sym("shoes"), sym("blue"), sym("socks")]);
        let red_in_flat = memq(&sym("red"), &flat)
            .expect("present at the front")
            .to_string();

        (
            abc.to_string(),
            nested_george.to_string(),
            pairs_tail,
            pairs_second,
            car_is_pair,
            red_among_sublists,
            red_in_flat,
        )
    }
}

#[test]
fn ex_2_53() {
    assert_eq!(
        ex_2_53::ex_2_53(),
        (
            "(a b c)".to_string(),
            "((george))".to_string(),
            "((y1 y2))".to_string(),
            "(y1 y2)".to_string(),
            "false".to_string(),
            "false".to_string(),
            "(red shoes blue socks)".to_string(),
        )
    );
}
