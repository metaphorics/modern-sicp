// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.7: Shared ownership and interior mutability.
//!
//! `RefCell` enforces the borrow rules at run time instead of compile
//! time: holding two mutable borrows at once panics where a `&mut`
//! reference would have refused to compile.
//!
//! ```should_panic
//! use std::cell::RefCell;
//! let cell = RefCell::new(0);
//! let _first = cell.borrow_mut();
//! let _second = cell.borrow_mut();
//! ```

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A pair of closures sharing one running total (exercise 0.4): call
/// `.0` with an amount to add it and get the new total; call `.1` to
/// reset the total to zero and get the total it held.
pub type Accumulator = (Box<dyn Fn(i128) -> i128>, Box<dyn Fn() -> i128>);

/// A shared, growable log: cloning a [`SharedLog`] shares the same
/// backing storage, so an entry appended through one clone is visible
/// through every other.
#[derive(Clone, Default)]
pub struct SharedLog(Rc<RefCell<Vec<String>>>);

impl SharedLog {
    /// An empty log.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends `entry` to the shared backing storage.
    pub fn push(&self, entry: impl Into<String>) {
        self.0.borrow_mut().push(entry.into());
    }

    /// A snapshot of the entries appended so far, in order.
    #[must_use]
    pub fn entries(&self) -> Vec<String> {
        self.0.borrow().clone()
    }

    /// How many `SharedLog` handles point at this backing storage.
    #[must_use]
    pub fn handle_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

/// A tree node that can point at its parent without keeping it alive: an
/// owning back-edge here would leave parent and child owning each other,
/// so neither is ever freed. [`Weak`] observes the parent without
/// contributing to its strong count.
pub struct Node {
    /// The node's own label.
    pub label: &'static str,
    /// A non-owning link to the parent, if any.
    pub parent: RefCell<Weak<Node>>,
    /// The owned children.
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    /// A leaf node with no parent and no children yet.
    #[must_use]
    pub fn new(label: &'static str) -> Rc<Self> {
        Rc::new(Self {
            label,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    /// Adopts `child` under `parent`, recording the back-edge as `Weak`
    /// so dropping `parent` can still free it even while `child` is held.
    pub fn adopt(parent: &Rc<Node>, child: &Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}
