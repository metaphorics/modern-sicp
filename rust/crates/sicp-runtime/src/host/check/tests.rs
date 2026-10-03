// SPDX-License-Identifier: GPL-3.0-only

//! Soundness of the subset checker against `rustc`, the authority the
//! grammar names (§3, §5): each rejected program below fails the pinned
//! native compiler in the same class, and each admitted program
//! compiles natively. A checker that accepts the first kind, or
//! refuses the second, has drifted from the contract.

use crate::host::admit;
use crate::host::diag::DiagKind;

/// Programs `rustc` rejects (or, for the trait and lifetime rules, that
/// the subset must not stretch): the checker must report the named
/// class before any effect.
const REJECTED: &[(&str, DiagKind, &str)] = &[
    (
        "use_after_move",
        DiagKind::Ownership,
        r#"
fn main() {
    let s = String::from("x");
    let t = s;
    println!("{}", s);
    println!("{}", t);
}
"#,
    ),
    (
        "moved_into_call",
        DiagKind::Ownership,
        r#"
fn take(s: String) -> usize {
    s.len()
}
fn main() {
    let s = String::from("x");
    let n = take(s);
    println!("{} {}", s, n);
}
"#,
    ),
    (
        "moved_in_loop",
        DiagKind::Ownership,
        r#"
fn main() {
    let s = String::from("x");
    let mut i = 0i64;
    while i < 2 {
        let t = s;
        println!("{}", t);
        i += 1;
    }
}
"#,
    ),
    (
        "for_by_value_consumes",
        DiagKind::Ownership,
        r#"
fn main() {
    let v = vec![1i64, 2];
    for x in v {
        println!("{}", x);
    }
    println!("{}", v.len());
}
"#,
    ),
    (
        "move_out_of_index",
        DiagKind::Ownership,
        r#"
fn main() {
    let v = vec![String::from("a")];
    let s = v[0];
    println!("{}", s);
}
"#,
    ),
    (
        "pattern_moves_scrutinee",
        DiagKind::Ownership,
        r#"
fn main() {
    let o = Some(String::from("a"));
    match o {
        Some(s) => println!("{}", s),
        None => println!("none"),
    }
    let p = o;
}
"#,
    ),
    (
        "move_closure_consumes",
        DiagKind::Ownership,
        r#"
fn main() {
    let s = String::from("x");
    let f = move || s.len();
    println!("{}", f());
    println!("{}", s);
}
"#,
    ),
    (
        "main_with_parameter",
        DiagKind::Type,
        r#"
fn main(x: i64) {
    println!("{}", x);
}
"#,
    ),
    (
        "main_returning_value",
        DiagKind::Type,
        r"
fn main() -> i64 {
    1
}
",
    ),
    (
        "reference_field",
        DiagKind::Type,
        r"
struct Bad {
    value: &i64,
}
fn main() {}
",
    ),
    (
        "returned_reference_without_source",
        DiagKind::Type,
        r"
fn make() -> &i64 {
    let x = 1;
    &x
}
fn main() {}
",
    ),
    (
        "returned_reference_ambiguous",
        DiagKind::Type,
        r"
fn first(a: &i64, b: &i64) -> &i64 {
    a
}
fn main() {}
",
    ),
    (
        "boxed_closure_clone",
        DiagKind::Type,
        r#"
fn main() {
    let f: Box<dyn Fn() -> i64 + 'static> = Box::new(|| 1);
    let g = f.clone();
    println!("{}", g());
}
"#,
    ),
    (
        "derive_clone_over_closure",
        DiagKind::Type,
        r"
#[derive(Clone)]
struct S {
    f: Box<dyn Fn() -> i64 + 'static>,
}
fn main() {}
",
    ),
    (
        "nested_pattern_not_exhaustive",
        DiagKind::Type,
        r#"
fn f(o: Option<bool>) -> i64 {
    match o {
        Some(true) => 1,
        None => 0,
    }
}
fn main() {
    println!("{}", f(None));
}
"#,
    ),
    (
        "literal_pattern_not_exhaustive",
        DiagKind::Type,
        r#"
fn f(o: Option<i64>) -> i64 {
    match o {
        Some(0) => 1,
        None => 0,
    }
}
fn main() {
    println!("{}", f(None));
}
"#,
    ),
    (
        "struct_pattern_omits_field",
        DiagKind::Type,
        r#"
struct P {
    x: i64,
    y: i64,
}
fn main() {
    let p = P { x: 1, y: 2 };
    let n: i64 = match p {
        P { x } => x,
    };
    println!("{}", n);
}
"#,
    ),
    (
        "hashmap_new_without_import",
        DiagKind::Type,
        r#"
fn main() {
    let mut m = HashMap::new();
    m.insert(String::from("a"), 1_i64);
}
"#,
    ),
    (
        "hashmap_type_without_import",
        DiagKind::Type,
        r"
fn main() {
    let m: HashMap<String, i64> = Vec::new();
}
",
    ),
    (
        "equality_over_underived_struct",
        DiagKind::Type,
        r#"
struct S {
    f: Box<dyn Fn() -> i64 + 'static>,
}
fn main() {
    let a = Some(S { f: Box::new(|| 1) });
    let b = Some(S { f: Box::new(|| 1) });
    println!("{}", a == b);
}
"#,
    ),
    (
        "bare_variant_pattern",
        DiagKind::Type,
        r#"
enum Shape {
    Circle(i64),
    Square(i64),
}
fn area(s: Shape) -> i64 {
    match s {
        Circle(r) => r,
        Square(w) => w,
    }
}
fn main() {
    println!("{}", area(Shape::Circle(2)));
}
"#,
    ),
    (
        "enum_name_as_tuple_pattern",
        DiagKind::Type,
        r#"
enum W {
    V(i64),
}
fn f(w: W) -> i64 {
    match w {
        W(x) => x,
    }
}
fn main() {
    println!("{}", f(W::V(2)));
}
"#,
    ),
    (
        "pattern_of_another_enum",
        DiagKind::Type,
        r#"
enum Shape {
    Circle(i64),
    Square(i64),
}
enum Other {
    Circle(i64),
}
fn area(s: Shape) -> i64 {
    match s {
        Other::Circle(r) => r,
        Shape::Circle(r) => r,
        Shape::Square(w) => w,
    }
}
fn main() {
    println!("{}", area(Shape::Circle(2)));
}
"#,
    ),
    (
        "suffixed_pattern_wrong_type",
        DiagKind::Type,
        r#"
fn main() {
    let u = 5usize;
    match u {
        5i64 => println!("hit"),
        _ => println!("miss"),
    }
}
"#,
    ),
    (
        "suffixed_usize_pattern_on_i64",
        DiagKind::Type,
        r#"
fn main() {
    let u = 5i64;
    match u {
        5usize => println!("hit"),
        _ => println!("miss"),
    }
}
"#,
    ),
    (
        "as_str_loan_blocks_mutation",
        DiagKind::Ownership,
        r#"
fn main() {
    let mut s = String::from("a");
    let r = s.as_str();
    s.push_str("b");
    println!("{}", r);
}
"#,
    ),
    (
        "box_deref_write_immutable",
        DiagKind::Ownership,
        r#"
fn main() {
    let c = Box::new(0i64);
    *c = 2;
    println!("{}", *c);
}
"#,
    ),
    (
        "box_deref_mut_borrow_immutable",
        DiagKind::Ownership,
        r#"
fn main() {
    let e = Box::new(1i64);
    let re = &mut *e;
    *re += 1;
    println!("{}", re);
}
"#,
    ),
    (
        "box_deref_move_then_use",
        DiagKind::Ownership,
        r#"
fn main() {
    let b = Box::new(String::from("s"));
    let inner = *b;
    println!("{}", *b);
}
"#,
    ),
    (
        "box_deref_move_through_shared_ref",
        DiagKind::Ownership,
        r#"
fn main() {
    let r = &Box::new(String::from("s"));
    let s = **r;
    println!("{}", s);
}
"#,
    ),
];

/// Programs `rustc` accepts that the checker must admit.
const ADMITTED: &[(&str, &str)] = &[
    (
        "container_equality",
        r#"
fn main() {
    let a = Some(1_i64);
    let b = Some(1_i64);
    println!("{}", a == b);
    let v = vec![1_i64];
    let w = vec![1_i64];
    println!("{}", v == w);
    let t = (1_i64, String::from("a"));
    let u = (1_i64, String::from("a"));
    println!("{}", t == u);
}
"#,
    ),
    (
        "derived_equality_in_containers",
        r#"
#[derive(PartialEq)]
struct P {
    x: i64,
}
fn main() {
    let a = Some(P { x: 1 });
    let b = Some(P { x: 1 });
    println!("{}", a == b);
}
"#,
    ),
    (
        "with_capacity",
        r#"
fn main() {
    let mut v: Vec<i64> = Vec::with_capacity(4);
    v.push(1);
    println!("{}", v.len());
}
"#,
    ),
    (
        "hashmap_iter",
        r#"
use std::collections::HashMap;
fn main() {
    let mut m: HashMap<String, i64> = HashMap::new();
    m.insert(String::from("a"), 1);
    for (k, v) in m.iter() {
        println!("{} {}", k, v);
    }
}
"#,
    ),
    (
        "hashmap_import_rename",
        r#"
use std::collections::HashMap as Map;
fn main() {
    let mut m: Map<String, i64> = Map::new();
    m.insert(String::from("a"), 1);
    println!("{}", m.len());
}
"#,
    ),
    (
        "mutable_reference_reborrows",
        r#"
fn inc(x: &mut i64) {
    *x += 1;
}
fn go(v: &mut Vec<i64>, n: i64) {
    if n > 0 {
        v.push(n);
        go(v, n - 1);
        go(v, n - 1);
    }
}
fn main() {
    let mut a = 1i64;
    let r = &mut a;
    inc(r);
    inc(r);
    println!("{}", a);
    let mut v: Vec<i64> = Vec::new();
    go(&mut v, 2);
    println!("{}", v.len());
}
"#,
    ),
    (
        "copy_composites_reused",
        r#"
fn main() {
    let t = (1i64, 2i64);
    let u = t;
    let v = t;
    println!("{} {}", u.0, v.1);
    let o = Some(3i64);
    let p = o;
    let q = o;
    println!("{}", p == q);
}
"#,
    ),
    (
        "move_then_reassign_in_loop",
        r#"
enum List {
    Cons(i64, Box<List>),
    Nil,
}
fn main() {
    let mut list = List::Nil;
    let mut i = 0i64;
    while i < 3 {
        list = List::Cons(i, Box::new(list));
        i += 1;
    }
    match list {
        List::Cons(n, _) => println!("{}", n),
        List::Nil => println!("empty"),
    }
}
"#,
    ),
    (
        "matching_without_moving",
        r#"
fn main() {
    let o = Some(3i64);
    match o {
        Some(k) => println!("{}", k),
        None => println!("none"),
    }
    let p = o;
    println!("{}", p == o);
}
"#,
    ),
    (
        "elided_reference_results",
        r#"
struct S {
    f: Box<dyn Fn(&i64) -> &i64 + 'static>,
}
fn first(a: &i64) -> &i64 {
    a
}
fn apply(f: fn(&i64) -> &i64, x: &i64) -> i64 {
    *f(x)
}
fn main() {
    let x = 1i64;
    println!("{}", *first(&x));
    println!("{}", apply(first, &x));
}
"#,
    ),
    (
        "clone_of_shared_reference",
        r#"
struct Q {
    x: i64,
}
fn main() {
    let a = Q { x: 1 };
    let r = &a;
    let s = r.clone();
    println!("{}", s.x);
}
"#,
    ),
    (
        "nested_exhaustive",
        r#"
fn f(o: Option<Option<bool>>) -> i64 {
    match o {
        Some(Some(true)) => 1,
        Some(Some(false)) => 2,
        Some(None) => 3,
        None => 4,
    }
}
fn main() {
    println!("{}", f(None));
}
"#,
    ),
    (
        "main_signature",
        r#"
fn main() -> () {
    println!("ok");
}
"#,
    ),
    (
        "suffixed_int_patterns",
        r#"
fn main() {
    let u = 5usize;
    match u {
        5usize => println!("hit"),
        _ => println!("miss"),
    }
    let i = 7i64;
    match i {
        7i64 => println!("hit"),
        _ => println!("miss"),
    }
}
"#,
    ),
    (
        "as_str_borrow_releases_after_last_use",
        r#"
fn main() {
    let mut s = String::from("a");
    let r = s.as_str();
    println!("{}", r);
    s.push_str("b");
    println!("{}", s);
}
"#,
    ),
    (
        "box_deref_owned_place",
        r#"
fn give() -> Box<i64> {
    Box::new(9)
}
fn main() {
    let b = Box::new(42i64);
    let h = *b;
    println!("{}", h);
    let mut c = Box::new(0i64);
    *c += 2;
    *c = *c + 1;
    println!("{}", *c);
    let d = Box::new(7i64);
    let rd = &*d;
    println!("{}", rd);
    let mut e = Box::new(1i64);
    let re = &mut *e;
    *re += 10;
    println!("{}", *e);
    println!("{}", *give());
    let g = *give();
    println!("{}", g);
    let bb = Box::new(Box::new(5i64));
    println!("{}", **bb);
    let mb = Box::new(6i64);
    match *mb {
        6 => println!("six"),
        _ => println!("other"),
    }
    let name = Box::new(String::from("n"));
    let inner = *name;
    println!("{}", inner);
}
"#,
    ),
    (
        "box_deref_through_mut_ref",
        r#"
fn main() {
    let mut v = Box::new(1i64);
    let r = &mut v;
    **r = 9;
    println!("{}", **r);
    let mut a = 1i64;
    let x = &mut a;
    *x += 1;
    *x += 1;
    println!("{}", a);
}
"#,
    ),
];

#[test]
fn unsound_programs_are_rejected_in_their_class() {
    let mut wrong = Vec::new();
    for (name, kind, source) in REJECTED {
        match admit(source) {
            Ok(_) => wrong.push(format!("{name}: admitted, but the program is invalid")),
            Err(diag) if diag.kind != *kind => wrong.push(format!(
                "{name}: rejected as {:?}, expected {kind:?}: {}",
                diag.kind, diag.message
            )),
            Err(_) => {}
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn valid_programs_are_admitted() {
    let wrong: Vec<String> = ADMITTED
        .iter()
        .filter_map(|(name, source)| {
            admit(source)
                .err()
                .map(|diag| format!("{name}: rejected: {diag:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The `rustc` half of the module contract: every admitted program must
/// compile natively, so a checker that accepts invalid Rust cannot hide.
#[test]
fn admitted_programs_compile_natively() {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let mut wrong = Vec::new();
    for (name, source) in ADMITTED {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("sicp_admitted_{name}.rs"));
        let meta = dir.join(format!("sicp_admitted_{name}.rmeta"));
        std::fs::write(&path, source).expect("write the program source");
        let output = std::process::Command::new(&rustc)
            .args([
                "--edition",
                "2021",
                "--crate-type",
                "lib",
                "--emit",
                "metadata",
                "-o",
            ])
            .arg(&meta)
            .arg(&path)
            .output()
            .expect("rustc must run on the development machine");
        if !output.status.success() {
            wrong.push(format!(
                "{name}: rustc rejects an admitted program:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&meta);
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
