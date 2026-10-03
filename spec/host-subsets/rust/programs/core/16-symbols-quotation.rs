// SPDX-License-Identifier: GPL-3.0-only
// Case: core/16-symbols-quotation. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Symbolic data as explicit syntax data: the quotation lesson re-cut
/// as a closed domain of symbol terms rather than quoted source text.
enum Sym {
    Atom(String),
    Pair(Box<Sym>, Box<Sym>),
    Nil,
}

fn atom(name: &str) -> Sym {
    Sym::Atom(String::from(name))
}

fn symbol_name(term: &Sym) -> String {
    match term {
        Sym::Atom(name) => String::from(name.as_str()),
        Sym::Pair(first, _) => symbol_name(first),
        Sym::Nil => String::from("()"),
    }
}

fn main() {
    let quoted = Sym::Pair(Box::new(atom("a")), Box::new(Sym::Pair(Box::new(atom("b")), Box::new(Sym::Nil))));
    println!("{}", symbol_name(&quoted));
    println!("{}", symbol_name(&atom("hello")));
}
