// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.8: duplicate labels fail the
//! assembly instead of silently picking one site.

use ch05::sec_5_2::{Fault, lookup_label, make_machine};
use sicp_runtime::Value;

/// The book's ambiguous controller: the label `here` marks two
/// different locations.
const AMBIGUOUS_CONTROLLER: &str = "
  (goto (label here))
here
  (assign a (const 3))
  (goto (label there))
here
  (assign a (const 4))
  (goto (label there))
there";

/// The label table the book's `extract-labels` built before the fix:
/// pairs in scan order, `here` listed twice.
fn ambiguous_table() -> Vec<(String, usize)> {
    vec![
        ("here".to_owned(), 0),
        ("there".to_owned(), 4),
        ("here".to_owned(), 2),
    ]
}

mod ex_5_08 {
    //! Exercise 5.8: with the simulator as written, what does
    //! register `a` hold when control reaches `there`? Then make the
    //! assembler reject the duplicate label.

    use super::*;

    /// With the book's first-occurrence `lookup-label` (an `assoc`
    /// over the scan-order table), the goto lands at the first
    /// `here`, so `a` is set to 3 before control reaches `there`.
    #[test]
    fn ex_5_08_as_written_first_label_wins() {
        assert_eq!(lookup_label(&ambiguous_table(), "here"), Ok(0));
        assert_eq!(lookup_label(&ambiguous_table(), "there"), Ok(4));
    }

    /// The repaired assembler refuses the whole controller: the
    /// typed fault names the repeated label, and no machine is
    /// returned at all.
    #[test]
    fn ex_5_08_duplicate_label_fails_assembly() {
        let fault = make_machine(&["a"], &[], AMBIGUOUS_CONTROLLER).unwrap_err();
        assert_eq!(
            fault,
            Fault::DuplicateLabel {
                label: "here".to_owned()
            }
        );
    }

    /// Deduplicating the controller by keeping only the first `here`
    /// block assembles and runs, and confirms the as-written answer:
    /// `a` holds 3 when control reaches `there`.
    #[test]
    fn ex_5_08_first_block_confirms_the_value() {
        let kept = "
  (goto (label here))
here
  (assign a (const 3))
  (goto (label there))
there";
        let mut machine = make_machine(&["a"], &[], kept).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(3));
    }
}
