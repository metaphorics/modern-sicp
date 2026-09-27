// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.1

//! Section 3.1: Assignment and local state.
//!
//! The book models an object with time-varying state by capturing a
//! variable in a closure and changing it with `set!`. This edition
//! expresses the same idea with interior mutability: the captured state
//! is a [`Cell<i128>`](std::cell::Cell), and the closure returned by
//! [`make_withdraw`] is an `impl FnMut` that reads and writes that one
//! cell on every call. Two calls with the same argument now answer
//! differently, which is the new kind of behavior the section is about;
//! sharing the cell in an [`Rc`](std::rc::Rc) is what makes two names
//! denote the same object.

use std::cell::Cell;
use std::fmt;
use std::rc::Rc;

use sicp_runtime::SchemeError;

/// What a bank-account operation answers with: the book's `withdraw`
/// returns either the new balance or the `Insufficient funds` message,
/// and this edition keeps both kinds of answer in one typed value that
/// displays the way the book's interactions print.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reply {
    /// The new balance after a successful operation.
    Balance(i128),
    /// A message such as `Insufficient funds`, printed double-quoted
    /// like the book's strings.
    Message(&'static str),
}

impl fmt::Display for Reply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reply::Balance(n) => write!(f, "{n}"),
            // The book prints its messages with the quotes showing.
            Reply::Message(m) => write!(f, "\"{m}\""),
        }
    }
}

/// A message to a bank-account object: the book's symbol passed to
/// `dispatch`, made a value so the compiler can prove every request is
/// handled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Request {
    /// Take this amount out, if the balance allows it.
    Withdraw(i128),
    /// Add this amount to the balance.
    Deposit(i128),
}

/// Creates a withdrawal processor holding `balance` in its own
/// environment: the book's `make-withdraw`. The formal parameter is
/// captured by the returned closure, so each call to this function
/// yields an independent object.
pub fn make_withdraw(mut balance: i128) -> impl FnMut(i128) -> Reply {
    move |amount| {
        if balance >= amount {
            balance -= amount;
            Reply::Balance(balance)
        } else {
            Reply::Message("Insufficient funds")
        }
    }
}

/// Creates the book's `new-withdraw`: a `let`-bound balance of 100
/// captured by a `move` closure, so the variable `balance` is
/// encapsulated within the returned object and no other procedure can
/// reach it.
pub fn new_withdraw() -> impl FnMut(i128) -> Reply {
    let balance = Cell::new(100);
    move |amount| {
        if balance.get() >= amount {
            balance.set(balance.get() - amount);
            Reply::Balance(balance.get())
        } else {
            Reply::Message("Insufficient funds")
        }
    }
}

/// A bank-account object: the book's `make-account` value. The balance
/// lives in a reference-counted cell, so cloning an [`Account`] gives a
/// second name for the *same* account, and two accounts built
/// separately never share.
#[derive(Clone, Debug)]
pub struct Account {
    balance: Rc<Cell<i128>>,
}

/// Creates a bank-account object with the given initial balance: the
/// book's `make-account`, whose `dispatch` this edition folds into
/// [`Account::send`].
#[must_use]
pub fn make_account(balance: i128) -> Account {
    Account {
        balance: Rc::new(Cell::new(balance)),
    }
}

impl Account {
    /// Sends the account a message and returns its answer. The book's
    /// `((acc 'withdraw) 50)` becomes one call; the unknown-request
    /// error arm of the book's `dispatch` disappears, because a
    /// [`Request`] can spell nothing else.
    #[must_use]
    pub fn send(&self, request: Request) -> Reply {
        match request {
            Request::Withdraw(amount) => {
                if self.balance.get() >= amount {
                    self.balance.set(self.balance.get() - amount);
                    Reply::Balance(self.balance.get())
                } else {
                    Reply::Message("Insufficient funds")
                }
            }
            Request::Deposit(amount) => {
                self.balance.set(self.balance.get() + amount);
                Reply::Balance(self.balance.get())
            }
        }
    }

    /// Whether two names denote the same account object. The book can
    /// only test this by changing one account and watching the other;
    /// `Rc::ptr_eq` answers it directly.
    #[must_use]
    pub fn same_object_as(&self, other: &Account) -> bool {
        Rc::ptr_eq(&self.balance, &other.balance)
    }
}

/// The book's `random-init`: the fixed word this edition starts `rand`
/// from, so every run of the chapter is reproducible.
pub const RANDOM_INIT: u64 = 1;

/// The book's `rand-update`: one step of this edition's pure generator,
/// `xorshift64*` (Vigna 2016) with the multiplied word fed back as the
/// next state. It is a mathematical function of its input: the same word
/// in, the same word out, every time, which is what the stream
/// formulations of 3.5.5 map. The runtime's stateful `Random` keeps the
/// multiplied word out of its state, so the two agree on the first word
/// from a shared seed and differ after; both are deterministic and the
/// first word matches the edition's fixed vector.
#[must_use]
pub fn rand_update(x: u64) -> u64 {
    let x = x ^ (x >> 12);
    let x = x ^ (x << 25);
    let x = x ^ (x >> 27);
    x.wrapping_mul(0x2545_F491_4F6C_DD1D)
}

/// The book's `rand`: a random-number generator whose only state is one
/// hidden word. Each call to [`Rand::generate`] computes
/// [`rand_update`] of the current word, returns it, and stores it as
/// the new word.
#[derive(Clone, Debug)]
pub struct Rand {
    x: u64,
}

impl Rand {
    /// Starts a generator at `seed`, in the role of the book's
    /// `random-init`.
    ///
    /// # Errors
    /// Returns [`SchemeError::ZeroSeed`] when `seed` is zero, because
    /// `xorshift64*` maps zero to zero forever.
    pub fn new(seed: u64) -> Result<Self, SchemeError> {
        if seed == 0 {
            return Err(SchemeError::ZeroSeed);
        }
        Ok(Self { x: seed })
    }

    /// Produces the next random number of the stream.
    pub fn generate(&mut self) -> u64 {
        self.x = rand_update(self.x);
        self.x
    }
}

/// The greatest common divisor by Euclid's algorithm (section 1.2.5):
/// the statistic the Cesaro experiment tests for.
#[must_use]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// The book's `monte-carlo`: runs `experiment` for `trials` tries and
/// returns the fraction of runs in which it returned true. The
/// experiment arrives as a closure, so `monte_carlo` never learns
/// where its randomness comes from.
#[must_use]
pub fn monte_carlo(trials: u32, experiment: &mut dyn FnMut() -> bool) -> f64 {
    let mut passed = 0_u32;
    for _ in 0..trials {
        if experiment() {
            passed += 1;
        }
    }
    f64::from(passed) / f64::from(trials)
}

/// The book's `estimate-pi`: a Cesaro estimate of pi built on
/// [`monte_carlo`]. The generator is captured once, here at the edge,
/// and nothing deeper in the program names it.
#[must_use]
pub fn estimate_pi(trials: u32, rand: &mut Rand) -> f64 {
    let mut cesaro_test = || gcd(rand.generate(), rand.generate()) == 1;
    let coprime_fraction = monte_carlo(trials, &mut cesaro_test);
    (6.0 / coprime_fraction).sqrt()
}

/// The book's `random-gcd-test`: the same experiment with no local
/// state for the generator. The random words are now threaded through
/// the loop by hand, and `x2` must be recycled as the next round's
/// input.
#[must_use]
pub fn random_gcd_test(trials: u32, initial_x: u64) -> f64 {
    let mut passed = 0_u32;
    let mut x = initial_x;
    for _ in 0..trials {
        let x1 = rand_update(x);
        let x2 = rand_update(x1);
        if gcd(x1, x2) == 1 {
            passed += 1;
        }
        x = x2;
    }
    f64::from(passed) / f64::from(trials)
}

/// The book's stateless `estimate-pi`, which must also name the
/// generator's starting point in its own signature.
#[must_use]
pub fn estimate_pi_stateless(trials: u32, initial_x: u64) -> f64 {
    (6.0 / random_gcd_test(trials, initial_x)).sqrt()
}

/// The book's `make-simplified-withdraw`: no balance check, so the
/// state keeps changing and the answers keep shrinking past zero.
pub fn make_simplified_withdraw(mut balance: i128) -> impl FnMut(i128) -> i128 {
    move |amount| {
        balance -= amount;
        balance
    }
}

/// The book's `make-decrementer`: the same shape with no assignment, so
/// the returned closure is an `Fn`, not an `FnMut`, and repeated calls
/// answer identically forever.
pub fn make_decrementer(balance: i128) -> impl Fn(i128) -> i128 {
    move |amount| balance - amount
}
