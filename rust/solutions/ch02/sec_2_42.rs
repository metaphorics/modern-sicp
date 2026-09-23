// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.42, one module and one test.

mod ex_2_42 {
    /// A board position set: one row index per occupied column, in
    /// column order, the edition's representation for the book's set of
    /// row-column positions. A column index carries its own column, so
    /// no pair packing is needed.
    type Positions = Vec<usize>;

    /// `empty-board`: one empty placement, because the sequence of all
    /// ways to place zero queens has exactly one element.
    fn empty_board() -> Vec<Positions> {
        vec![Vec::new()]
    }

    /// `adjoin-position`: adjoins a queen in `new_row` of column `k` to
    /// a position set, by appending the row; the column is the index
    /// the row lands at.
    fn adjoin_position(new_row: usize, positions: &Positions) -> Positions {
        let mut extended = positions.clone();
        extended.push(new_row);
        extended
    }

    /// `safe?`: the queen just added in the last column must not share
    /// a row or a diagonal with any earlier queen. Only the new queen
    /// is checked, as the statement notes; the others are already safe
    /// among themselves.
    fn is_safe(positions: &Positions) -> bool {
        let Some((&new_row, earlier)) = positions.split_last() else {
            return true;
        };
        let new_column = positions.len() - 1;
        earlier.iter().enumerate().all(|(column, &row)| {
            row != new_row && new_column.abs_diff(column) != new_row.abs_diff(row)
        })
    }

    /// The book's `queens`: `queen-cols` returns all ways to place
    /// queens in the first `k` columns; each `k - 1` placement is
    /// extended by every row of column `k`, and `filter` keeps the
    /// safe ones.
    fn queens(board_size: usize) -> Vec<Positions> {
        fn queen_cols(k: usize, board_size: usize) -> Vec<Positions> {
            if k == 0 {
                return empty_board();
            }
            queen_cols(k - 1, board_size)
                .iter()
                .flat_map(|rest_of_queens| {
                    (0..board_size).map(move |new_row| adjoin_position(new_row, rest_of_queens))
                })
                .filter(is_safe)
                .collect()
        }
        queen_cols(board_size, board_size)
    }

    /// Every returned solution must be conflict-free column to column.
    fn all_solutions_are_safe(solutions: &[Positions]) -> bool {
        solutions.iter().all(|positions| {
            positions.iter().enumerate().all(|(i, &row_i)| {
                positions.iter().enumerate().all(|(j, &row_j)| {
                    i == j || (row_i != row_j && i.abs_diff(j) != row_i.abs_diff(row_j))
                })
            })
        })
    }

    /// Exercise 2.42: eight queens
    ///
    /// Returns the number of solutions to the `n = 8` puzzle and a
    /// verdict that every returned solution is conflict-free.
    pub fn ex_2_42() -> (usize, bool) {
        let solutions = queens(8);
        (solutions.len(), all_solutions_are_safe(&solutions))
    }
}

#[test]
fn ex_2_42() {
    assert_eq!(ex_2_42::ex_2_42(), (92, true));
}
