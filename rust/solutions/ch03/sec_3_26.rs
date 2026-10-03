// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.26: a table whose records are
//! binary-tree nodes, ordered by key, so lookups and insertions take
//! steps proportional to the tree's height rather than its size, and
//! the whole table can be read back in key order.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use sicp_runtime::{SicpError, Value};

/// One node of the tree table: the key, its current value, and the two
/// subtrees. Values sit behind a `RefCell` so an insertion that finds
/// its key replaces the value in place.
struct TreeNode {
    key: Value,
    value: RefCell<Value>,
    left: RefCell<Tree>,
    right: RefCell<Tree>,
}

/// A subtree of the tree table.
#[derive(Default)]
pub struct Tree {
    root: RefCell<Option<Rc<TreeNode>>>,
}

/// The total order this edition's tree keys use: numbers compare as
/// numbers, symbols and strings by their text. Other key shapes are not
/// orderable, which is the exercise's assumption that keys admit an
/// ordering relation.
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    reason = "table keys are small magnitudes; the i128-to-f64 widening for a mixed exact/inexact compare never loses precision here"
)]
pub fn key_order(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Real(x), Value::Real(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        // Mixed exact/inexact keys: the exact integer joins the real
        // line for the comparison, exactly as the book's numeric tower
        // raises before comparing.
        (Value::Int(x), Value::Real(y)) => (*x as f64).partial_cmp(y).unwrap_or(Ordering::Equal),
        (Value::Real(x), Value::Int(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(Ordering::Equal),
        (Value::Sym(x), Value::Sym(y)) | (Value::Str(x), Value::Str(y)) => x.cmp(y),
        _ => Ordering::Equal,
    }
}

impl Tree {
    /// The book's `make-table` for this representation: an empty tree.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `lookup`: the value stored under `key`, found by one
    /// descent from the root.
    #[must_use]
    #[allow(
        clippy::assigning_clones,
        reason = "node is moved out by the while-let pattern each iteration, so clone_from has no initialized place to reuse"
    )]
    pub fn lookup(&self, key: &Value) -> Option<Value> {
        let mut node = self.root.borrow().clone();
        while let Some(current) = node {
            match key_order(key, &current.key) {
                Ordering::Equal => return Some(current.value.borrow().clone()),
                Ordering::Less => node = current.left.borrow().root.borrow().clone(),
                Ordering::Greater => node = current.right.borrow().root.borrow().clone(),
            }
        }
        None
    }

    /// The book's `insert!`: replaces the value under an existing key
    /// in place, or builds the one new leaf on the descent path.
    #[allow(
        clippy::assigning_clones,
        reason = "node is moved out by the let-else pattern each iteration, so clone_from has no initialized place to reuse"
    )]
    pub fn insert(&self, key: Value, value: Value) {
        let mut node = self.root.borrow().clone();
        loop {
            let Some(current) = node else {
                self.place(key, value);
                return;
            };
            match key_order(&key, &current.key) {
                Ordering::Equal => {
                    *current.value.borrow_mut() = value;
                    return;
                }
                Ordering::Less => node = current.left.borrow().root.borrow().clone(),
                Ordering::Greater => node = current.right.borrow().root.borrow().clone(),
            }
        }
    }

    fn place(&self, key: Value, value: Value) {
        self.attach(TreeNode {
            key,
            value: RefCell::new(value),
            left: RefCell::new(Tree::default()),
            right: RefCell::new(Tree::default()),
        });
    }

    fn attach(&self, leaf: TreeNode) {
        let fresh = Rc::new(leaf);
        let Some(root) = self.root.borrow().clone() else {
            *self.root.borrow_mut() = Some(fresh);
            return;
        };
        attach_under(&root, fresh);
    }

    /// The keys in order, left to right.
    #[must_use]
    pub fn keys_in_order(&self) -> Vec<Value> {
        let mut out = Vec::new();
        walk(self.root.borrow().as_ref(), &mut out);
        out
    }
}

fn attach_under(under: &Rc<TreeNode>, fresh: Rc<TreeNode>) {
    let side = match key_order(&fresh.key, &under.key) {
        Ordering::Less | Ordering::Equal => &under.left,
        Ordering::Greater => &under.right,
    };
    let next = side.borrow().root.borrow().clone();
    match next {
        Some(child) => attach_under(&child, fresh),
        None => {
            *side.borrow_mut() = Tree {
                root: RefCell::new(Some(fresh)),
            };
        }
    }
}

fn walk(node: Option<&Rc<TreeNode>>, out: &mut Vec<Value>) {
    if let Some(current) = node {
        walk(current.left.borrow().root.borrow().as_ref(), out);
        out.push(current.key.clone());
        walk(current.right.borrow().root.borrow().as_ref(), out);
    }
}

mod ex_3_26 {
    use super::{SicpError, Tree, Value};

    /// Exercise 3.26: a table as a binary tree ordered by key
    ///
    /// Stores five out-of-order keys, replaces one, reads them all
    /// back, and reports the keys in order.
    ///
    /// # Errors
    /// Reraises the lookup failure when a stored key goes missing.
    pub fn ex_3_26() -> Result<(Vec<i128>, Vec<i128>), SicpError> {
        let table = Tree::new();
        for (key, value) in [(5, 50), (3, 30), (8, 80), (1, 10), (4, 40)] {
            table.insert(Value::int(key), Value::int(value));
        }
        table.insert(Value::int(3), Value::int(33));

        let read = |key: i128| -> Result<i128, SicpError> {
            match table.lookup(&Value::int(key)) {
                Some(Value::Int(n)) => Ok(n),
                _ => Err(SicpError::TypeMismatch(format!("missing key {key}"))),
            }
        };
        let values = vec![read(1)?, read(3)?, read(4)?, read(5)?, read(8)?];
        let keys = table
            .keys_in_order()
            .into_iter()
            .filter_map(|v| match v {
                Value::Int(n) => Some(n),
                _ => None,
            })
            .collect();
        Ok((values, keys))
    }
}

#[test]
fn ex_3_26() {
    let (values, keys) = ex_3_26::ex_3_26().expect("table holds every key");
    assert_eq!(values, vec![10, 33, 40, 50, 80]);
    assert_eq!(keys, vec![1, 3, 4, 5, 8]);

    // A missing key answers no value, and the in-order read of an empty
    // table is empty.
    let empty = Tree::new();
    assert_eq!(empty.lookup(&Value::int(1)), None);
    assert!(empty.keys_in_order().is_empty());
}
