// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.66, one module and one test.

mod ex_2_66 {
    use std::cmp::Ordering;

    /// A data-base record: a numerical key plus a payload, this
    /// exercise's minimal stand-in for the section's discussion of
    /// personnel or accounting records.
    struct Record {
        key: i128,
        value: &'static str,
    }

    /// A binary tree of records, ordered by key: the section's `Tree`
    /// generalized from bare numbers to keyed records, kept local
    /// since the section's own `Tree` is fixed to `i128` entries.
    enum RecordTree {
        Empty,
        Node(Record, Box<RecordTree>, Box<RecordTree>),
    }

    fn node(key: i128, value: &'static str, left: RecordTree, right: RecordTree) -> RecordTree {
        RecordTree::Node(Record { key, value }, Box::new(left), Box::new(right))
    }

    /// Looks up a record by key, analogous to `element-of-set?` for
    /// the tree-set representation of 2.3.3: compare against the
    /// current node's key and recurse left or right, halving the
    /// remaining tree at each step on a balanced tree.
    fn lookup(key: i128, tree: &RecordTree) -> Option<&str> {
        match tree {
            RecordTree::Empty => None,
            RecordTree::Node(record, left, right) => match key.cmp(&record.key) {
                Ordering::Equal => Some(record.value),
                Ordering::Less => lookup(key, left),
                Ordering::Greater => lookup(key, right),
            },
        }
    }

    /// Exercise 2.66: `lookup` for a tree of records
    ///
    /// Returns `lookup(5, &tree)` and `lookup(9, &tree)` on a small
    /// tree of records keyed `2`, `5`, and `8`, in that order.
    pub fn ex_2_66() -> (Option<String>, Option<String>) {
        let tree = node(
            5,
            "bob",
            node(2, "alice", RecordTree::Empty, RecordTree::Empty),
            node(8, "carol", RecordTree::Empty, RecordTree::Empty),
        );
        (
            lookup(5, &tree).map(ToString::to_string),
            lookup(9, &tree).map(ToString::to_string),
        )
    }
}

#[test]
fn ex_2_66() {
    assert_eq!(ex_2_66::ex_2_66(), (Some("bob".to_string()), None));
}
