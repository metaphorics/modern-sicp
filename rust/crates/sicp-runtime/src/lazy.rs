// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The book's `memo-proc` (3.5): a delayed computation that runs its body
//! at most once and remembers the answer. `Stream` tails and the demand
//! probes of 4.2 both rest on this.

use std::cell::RefCell;
use std::rc::Rc;

/// A memoized thunk: the generator is dropped on first force, the memoized
/// value survives. Not `Sync`: the runtime is single-threaded.
pub struct Lazy<T> {
    f: RefCell<Option<Box<dyn FnOnce() -> T>>>,
    memo: RefCell<Option<Rc<T>>>,
}

impl<T: 'static> Lazy<T> {
    /// Delays `f` until the first [`Lazy::force`].
    #[must_use]
    pub fn new(f: impl FnOnce() -> T + 'static) -> Self {
        Lazy {
            f: RefCell::new(Some(Box::new(f))),
            memo: RefCell::new(None),
        }
    }

    /// Runs the generator once, then serves the memoized value forever.
    /// Sharing is by pointer: two forces hand out the same `Rc`.
    ///
    /// # Panics
    /// Panics when forcing reenters the same `Lazy` before its first force
    /// finished: the book's memoized thunk has no defined value there
    /// either, and silently re-running the body would break the once-only
    /// guarantee 4.2 teaches.
    pub fn force(&self) -> Rc<T> {
        if let Some(v) = &*self.memo.borrow() {
            return Rc::clone(v);
        }
        let f = self.f.borrow_mut().take();
        // Taking the generator before running it is what makes reentrant
        // forcing a loud panic instead of a silent double run.
        let value = Rc::new(f
            .expect("Lazy forced reentrantly before its first force finished")(
        ));
        *self.memo.borrow_mut() = Some(Rc::clone(&value));
        value
    }

    /// True once the generator has run and the memo is set.
    #[must_use]
    pub fn is_forced(&self) -> bool {
        self.memo.borrow().is_some()
    }
}

impl<T: 'static> std::fmt::Debug for Lazy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("forced", &self.is_forced())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::Lazy;

    #[test]
    fn generator_runs_once_and_memoizes() {
        let calls = Rc::new(Cell::new(0u32));
        let calls_in_body = Rc::clone(&calls);
        let l = Lazy::new(move || {
            calls_in_body.set(calls_in_body.get() + 1);
            40 + 2
        });
        assert!(!l.is_forced());
        let first = l.force();
        assert!(l.is_forced());
        let second = l.force();
        assert_eq!((*first, *second), (42, 42));
        assert_eq!(calls.get(), 1, "the body ran exactly once");
    }

    #[test]
    fn two_forces_hand_out_the_same_pointer() {
        let l = Lazy::new(|| vec![1, 2, 3]);
        let a = l.force();
        let b = l.force();
        assert!(Rc::ptr_eq(&a, &b));
    }

    #[test]
    fn forcing_one_lazy_does_not_run_another() {
        let ran = Rc::new(Cell::new(false));
        let ran_in_body = Rc::clone(&ran);
        let l = Lazy::new(move || {
            ran_in_body.set(true);
        });
        assert!(!ran.get());
        l.force();
        assert!(ran.get());
    }

    #[test]
    #[should_panic(expected = "reentrantly")]
    fn reentrant_force_panics_instead_of_rerunning() {
        let slot: Rc<std::cell::RefCell<Option<Rc<Lazy<u8>>>>> =
            Rc::new(std::cell::RefCell::new(None));
        let slot2 = Rc::clone(&slot);
        let l: Rc<Lazy<u8>> = Rc::new(Lazy::new(move || {
            // Forcing the very same lazy from inside its own generator.
            let _ = slot2.borrow().as_ref().expect("seeded").force();
            0
        }));
        *slot.borrow_mut() = Some(Rc::clone(&l));
        let _ = l.force();
    }
}
