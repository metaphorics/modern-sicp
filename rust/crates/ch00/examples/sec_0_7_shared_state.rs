// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.7, listing: a log shared through `Rc<RefCell<_>>`, then a
//! parent-child link that uses `Weak` to avoid a reference cycle.

use ch00::sec_0_7::{Node, SharedLog};

fn main() {
    let log = SharedLog::new();
    let other_handle = log.clone();
    log.push("first");
    other_handle.push("second");
    println!("{:?}", log.entries());
    // => ["first", "second"]
    assert_eq!(log.entries(), vec!["first", "second"]);
    println!("{}", log.handle_count());
    // => 2
    assert_eq!(log.handle_count(), 2);

    let parent = Node::new("root");
    let child = Node::new("leaf");
    Node::adopt(&parent, &child);
    let parent_label = child.parent.borrow().upgrade().map(|p| p.label);
    println!("{parent_label:?}");
    // => Some("root")
    assert_eq!(parent_label, Some("root"));
}
