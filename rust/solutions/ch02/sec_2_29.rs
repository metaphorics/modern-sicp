// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.29, one module and one test.

mod ex_2_29 {
    /// A binary mobile: two branches, the book's `make-mobile` shape.
    /// The fields are private, so the four selectors below are the only
    /// access, and the representation is free to change (part d).
    #[derive(Debug, Clone, PartialEq)]
    pub struct Mobile {
        left: Branch,
        right: Branch,
    }

    /// A branch: a rod of a certain length with a structure hanging
    /// from it, the book's `make-branch`.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Branch {
        length: i128,
        structure: BranchStructure,
    }

    /// What hangs from a branch: a simple weight, or another mobile.
    /// The recursive variant carries the submobile behind a `Box`,
    /// which is the one indirection the recursion needs.
    #[derive(Debug, Clone, PartialEq)]
    pub enum BranchStructure {
        /// A simple weight.
        Weight(i128),
        /// Another binary mobile hanging off this branch.
        Submobile(Box<Mobile>),
    }

    /// The book's `(make-mobile left right)`.
    pub fn make_mobile(left: Branch, right: Branch) -> Mobile {
        Mobile { left, right }
    }

    /// The book's `(make-branch length structure)`.
    pub fn make_branch(length: i128, structure: BranchStructure) -> Branch {
        Branch { length, structure }
    }

    impl Mobile {
        /// The book's `left-branch`.
        pub fn left_branch(&self) -> &Branch {
            &self.left
        }

        /// The book's `right-branch`.
        pub fn right_branch(&self) -> &Branch {
            &self.right
        }
    }

    impl Branch {
        /// The book's `branch-length`.
        pub fn branch_length(&self) -> i128 {
            self.length
        }

        /// The book's `branch-structure`.
        pub fn branch_structure(&self) -> &BranchStructure {
            &self.structure
        }
    }

    /// The weight hanging from one branch.
    fn branch_weight(branch: &Branch) -> i128 {
        match branch.branch_structure() {
            BranchStructure::Weight(w) => *w,
            BranchStructure::Submobile(submobile) => total_weight(submobile),
        }
    }

    /// Exercise 2.29b: `total-weight`, defined on the selectors only.
    fn total_weight(mobile: &Mobile) -> i128 {
        branch_weight(mobile.left_branch()) + branch_weight(mobile.right_branch())
    }

    /// Exercise 2.29c: the balance predicate. A mobile is balanced when
    /// the torques of its two top branches agree and every submobile is
    /// itself balanced; the recursion mirrors the structure of the
    /// data.
    fn balanced(mobile: &Mobile) -> bool {
        let left = mobile.left_branch();
        let right = mobile.right_branch();
        let torques_agree = left.branch_length() * branch_weight(left)
            == right.branch_length() * branch_weight(right);
        let submobiles_balanced = match (left.branch_structure(), right.branch_structure()) {
            (BranchStructure::Submobile(a), BranchStructure::Submobile(b)) => {
                balanced(a) && balanced(b)
            }
            (BranchStructure::Submobile(a), _) => balanced(a),
            (_, BranchStructure::Submobile(b)) => balanced(b),
            _ => true,
        };
        torques_agree && submobiles_balanced
    }

    /// A balanced sample: a 20 weight on a rod of length 3 against a
    /// submobile of total weight 10 on a rod of length 6 (the submobile
    /// hangs an 8 weight on a rod of length 2 and a 2 weight on a rod
    /// of length 8, so it balances too). Total weight 30.
    fn balanced_sample() -> Mobile {
        make_mobile(
            make_branch(3, BranchStructure::Weight(20)),
            make_branch(
                6,
                BranchStructure::Submobile(Box::new(make_mobile(
                    make_branch(2, BranchStructure::Weight(8)),
                    make_branch(8, BranchStructure::Weight(2)),
                ))),
            ),
        )
    }

    /// An unbalanced sample: weights 5 and 3 on rods of lengths 2 and
    /// 5. Torques 10 against 15. Total weight 8.
    fn unbalanced_sample() -> Mobile {
        make_mobile(
            make_branch(2, BranchStructure::Weight(5)),
            make_branch(5, BranchStructure::Weight(3)),
        )
    }

    /// Exercise 2.29: binary mobiles
    ///
    /// Returns the total weight and balance verdict of a balanced
    /// sample mobile and of an unbalanced one, in that order.
    pub fn ex_2_29() -> [(i128, bool); 2] {
        let balanced_mobile = balanced_sample();
        let unbalanced_mobile = unbalanced_sample();
        [
            (total_weight(&balanced_mobile), balanced(&balanced_mobile)),
            (
                total_weight(&unbalanced_mobile),
                balanced(&unbalanced_mobile),
            ),
        ]
    }

    /// Exercise 2.29d, on the alternative representation. When the
    /// constructors change to the cons-shaped pairs of the book's
    /// question, everything in this module that speaks only to the
    /// selectors is untouched: [`total_weight`] and [`balanced`]
    /// recompile unchanged against a new `Mobile` whose fields store a
    /// `(Branch, Branch)` tuple. Only the constructor calls change,
    /// because they are the one place allowed to know the
    /// representation.
    #[derive(Debug, Clone, PartialEq)]
    pub struct PairMobile(Branch, Branch);

    fn total_weight_pair(mobile: &PairMobile) -> i128 {
        branch_weight(&mobile.0) + branch_weight(&mobile.1)
    }

    /// Both representations report the same weights, because the weight
    /// logic never saw the representation.
    #[test]
    fn alternative_representation_keeps_the_weight_logic() {
        let original = balanced_sample();
        let paired = PairMobile(
            make_branch(3, BranchStructure::Weight(20)),
            make_branch(
                6,
                BranchStructure::Submobile(Box::new(make_mobile(
                    make_branch(2, BranchStructure::Weight(8)),
                    make_branch(8, BranchStructure::Weight(2)),
                ))),
            ),
        );
        assert_eq!(total_weight(&original), total_weight_pair(&paired));
    }
}

#[test]
fn ex_2_29() {
    assert_eq!(ex_2_29::ex_2_29(), [(30, true), (8, false)]);
}
