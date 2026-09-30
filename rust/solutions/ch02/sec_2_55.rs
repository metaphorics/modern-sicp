// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.55, one module and one test.

mod ex_2_55 {
    use sicp_runtime::Symbol;
    use std::fmt;

    /// A tiny quoted-AST value: exactly the shape quotation denotes. A
    /// symbol, a pair of two quoted values, or the empty list.
    enum Quoted {
        Sym(Symbol),
        Pair(Box<Quoted>, Box<Quoted>),
        Nil,
    }

    impl fmt::Display for Quoted {
        /// Renders as explicit Rust data: the variant constructors with
        /// their fields, so the shape of the value is the printed text.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Quoted::Sym(s) => write!(f, "Sym({s:?})"),
                Quoted::Nil => write!(f, "Nil"),
                Quoted::Pair(car, cdr) => write!(f, "Pair({car}, {cdr})"),
            }
        }
    }

    impl Quoted {
        fn car(&self) -> Option<&Quoted> {
            match self {
                Quoted::Pair(car, _) => Some(car),
                Quoted::Sym(_) | Quoted::Nil => None,
            }
        }
    }

    fn sym(s: &str) -> Quoted {
        Quoted::Sym(Symbol::from(s))
    }

    fn list2(a: Quoted, b: Quoted) -> Quoted {
        Quoted::Pair(
            Box::new(a),
            Box::new(Quoted::Pair(Box::new(b), Box::new(Quoted::Nil))),
        )
    }

    /// Exercise 2.55 (replacement): quote in edition AST terms
    ///
    /// Quotation is explicit symbolic data here: a quoted form is the
    /// two-element `Quoted` list whose first element is the symbol
    /// `quote` and whose second is that form. The doubly quoted word
    /// denotes `list2(sym("quote"), list2(sym("quote"),
    /// sym("abracadabra")))`; the outer `Pair` is the outer quotation,
    /// and the value that quotation yields is the inner two-element
    /// list `list2(sym("quote"), sym("abracadabra"))`. The `car` of
    /// that list is its outer `Pair`'s first element -- the symbol
    /// `quote`, not the symbol `abracadabra` and not a further quote
    /// form.
    ///
    /// Returns the printed value of the yielded list, and the printed
    /// `car` of that value, in that order.
    pub fn ex_2_55() -> (String, String) {
        let inner = list2(sym("quote"), sym("abracadabra"));
        let value = inner.to_string();
        let head = inner.car().expect("nonempty").to_string();
        (value, head)
    }
}

#[test]
fn ex_2_55() {
    assert_eq!(
        ex_2_55::ex_2_55(),
        (
            "Pair(Sym(\"quote\"), Pair(Sym(\"abracadabra\"), Nil))".to_string(),
            "Sym(\"quote\")".to_string()
        )
    );
}
