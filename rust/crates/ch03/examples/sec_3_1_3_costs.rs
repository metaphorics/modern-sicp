// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.1.3

//! Section 3.1.3: the costs of introducing assignment — the broken
//! substitution argument, sameness and change, and the aliasing trap.

use ch03::sec_3_1::{Request, make_account, make_decrementer, make_simplified_withdraw};

fn main() {
    // `make-simplified-withdraw` mutates its captured balance: repeated
    // calls on the same object give different answers.
    let mut w = make_simplified_withdraw(25);
    let answer = w(20);
    println!("{answer}");
    // => 5
    assert_eq!(answer, 5);

    let answer = w(10);
    println!("{answer}");
    // => -5
    assert_eq!(answer, -5);

    // `make-decrementer` is assignment-free: its closure is an `Fn`, so
    // the answers never accumulate.
    let d = make_decrementer(25);
    let answer = d(20);
    println!("{answer}");
    // => 5
    assert_eq!(answer, 5);

    let answer = d(10);
    println!("{answer}");
    // => 15
    assert_eq!(answer, 15);

    // The substitution model still explains the decrementer: evaluating
    // `make_decrementer(25)(20)` by inlining the body, then the
    // argument, lands on the right answer.
    #[allow(clippy::redundant_closure_call)]
    // the point of this spelling is the immediately-invoked closure itself
    let inlined = (|amount: i128| 25 - amount)(20);
    println!("{inlined}");
    // => 5
    assert_eq!(inlined, 5);

    let constant_folded = 25 - 20;
    println!("{constant_folded}");
    // => 5
    assert_eq!(constant_folded, 5);

    // Substitution cannot explain the simplified withdraw: the two
    // occurrences of `balance` in the body are the one captured word,
    // and the assignment changes it before the second occurrence reads
    // it. A second call proves the state survived the first.
    let mut w2 = make_simplified_withdraw(25);
    let answer = w2(20);
    println!("{answer}");
    // => 5
    assert_eq!(answer, 5);

    let answer = w2(20);
    println!("{answer}");
    // => -15
    assert_eq!(answer, -15);

    // Two decrementers built alike are interchangeable; two simplified
    // withdrawers never are.
    let d1 = make_decrementer(25);
    let d2 = make_decrementer(25);
    assert_eq!(d1(20), d2(20));
    assert_eq!(d1(20), d2(20));

    let mut w1 = make_simplified_withdraw(25);
    let mut w3 = make_simplified_withdraw(25);
    assert_eq!(w1(20), 5);
    assert_eq!(w1(20), -15);
    assert_eq!(w3(20), 5);

    // Sameness: two accounts made separately are distinct objects, but
    // a cloned name denotes the same object, so a withdrawal through
    // one name is visible through the other.
    let peter_acc = make_account(100);
    let pauls_own = make_account(100);
    assert!(!peter_acc.same_object_as(&pauls_own));

    let paul_acc = peter_acc.clone();
    assert!(peter_acc.same_object_as(&paul_acc));
    let answer = peter_acc.send(Request::Withdraw(30));
    println!("{answer}");
    // => 70
    assert_eq!(answer, ch03::sec_3_1::Reply::Balance(70));
    let answer = paul_acc.send(Request::Deposit(0));
    println!("{answer}");
    // => 70
    assert_eq!(answer, ch03::sec_3_1::Reply::Balance(70));

    // Imperative factorial: the same result as the recursive version of
    // 1.2.1, reached by assignment, where the order of the two updates
    // is a real decision the compiler does not check.
    let answer = factorial(5);
    println!("{answer}");
    // => 120
    assert_eq!(answer, 120);
    assert_eq!(factorial(10), 3_628_800);
}

/// The book's imperative `factorial`: `product` and `counter` are `mut`
/// bindings updated in a loop. Multiplying before incrementing keeps
/// every factor in the product; the opposite order would drop the
/// final factor and pick up one past `n`.
fn factorial(n: u32) -> u128 {
    let mut product = 1_u128;
    let mut counter = 1_u32;
    while counter <= n {
        product *= u128::from(counter);
        counter += 1;
    }
    product
}
