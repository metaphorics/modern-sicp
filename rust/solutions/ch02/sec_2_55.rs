// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.55, one module and one test.

mod ex_2_55 {
    use sicp_runtime::Symbol;
    use std::fmt;

    /// A tiny quoted-AST value: exactly enough structure to model what
    /// `'x` and nested quotation build, without a reader. A symbol, or
    /// a pair of two quoted values, or the empty list.
    enum Quoted {
        Sym(Symbol),
        Pair(Box<Quoted>, Box<Quoted>),
        Nil,
    }

    impl fmt::Display for Quoted {
        /// Prints a proper list `(a b c)`; this exercise never builds
        /// a dotted pair, so the improper-list case is unreachable.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Quoted::Sym(s) => write!(f, "{s}"),
                Quoted::Nil => write!(f, "()"),
                Quoted::Pair(car, cdr) => {
                    write!(f, "({car}")?;
                    let mut rest: &Quoted = cdr;
                    while let Quoted::Pair(next, more) = rest {
                        write!(f, " {next}")?;
                        rest = more;
                    }
                    write!(f, ")")
                }
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
    /// `'x` is reader sugar for `(quote x)`, so `''abracadabra` is
    /// `(quote (quote abracadabra))`: the outer `quote` returns its
    /// argument, `(quote abracadabra)`, unevaluated. That argument is
    /// an ordinary two-element list whose first element is the symbol
    /// `quote`, so its `car` is `quote`, not the symbol `abracadabra`
    /// and not another quote form.
    ///
    /// Returns the printed value of `''abracadabra`, and the printed
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
        ("(quote abracadabra)".to_string(), "quote".to_string())
    );
}
