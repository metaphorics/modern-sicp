// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.3: Ownership, moves, and borrowing.
//!
//! Five short cases, run as doctests so the verdicts below are the
//! compiler's, not an author's claim. Case A moves a `String` and then
//! reads the moved-from binding:
//!
//! ```compile_fail,E0382
//! let a = String::from("case a");
//! let b = a;
//! println!("{a} {b}");
//! ```
//!
//! Case B takes a shared borrow, prints it, and only then mutates: the
//! borrow's last use is the `println!`, so it has already ended by the
//! time `push` needs a mutable borrow.
//!
//! ```
//! let mut v = vec![1, 2, 3];
//! let first = &v[0];
//! println!("{first}");
//! v.push(4);
//! assert_eq!(v, vec![1, 2, 3, 4]);
//! ```
//!
//! Case C takes the same shared borrow but reads it *after* the mutation,
//! so the borrow would have to span the `push`:
//!
//! ```compile_fail,E0502
//! let mut v = vec![1, 2, 3];
//! let first = &v[0];
//! v.push(4);
//! println!("{first}");
//! ```
//!
//! Case D returns a reference into a `String` owned by the function that
//! is about to drop it; there is no lifetime the return type could name:
//!
//! ```compile_fail,E0106
//! fn shout() -> &str {
//!     let s = String::from("hi");
//!     &s
//! }
//! ```
//!
//! Case E clones instead of moving, so both bindings stay valid:
//!
//! ```
//! let a = String::from("case e");
//! let b = a.clone();
//! assert_eq!(a, b);
//! ```

/// Borrows `words` to total their lengths; the caller keeps ownership of
/// `words` after the call.
#[must_use]
pub fn total_len(words: &[String]) -> usize {
    words.iter().map(String::len).sum()
}

/// Appends `word` to `words` through an exclusive mutable borrow; no
/// other borrow of `words` may be alive at the call site.
pub fn append(words: &mut Vec<String>, word: impl Into<String>) {
    words.push(word.into());
}

/// Takes ownership of `words` and returns how many it held; the caller no
/// longer owns `words` after this call.
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "the point of this function is to demonstrate taking ownership, not to avoid it"
)]
pub fn consume_and_count(words: Vec<String>) -> usize {
    words.len()
}

/// The correct compiles-or-not verdict for cases A through E documented
/// on this module, in order. Exercise 0.3 asks for a prediction before
/// checking it against this.
#[must_use]
pub fn verdicts() -> [bool; 5] {
    [false, true, false, false, true]
}
