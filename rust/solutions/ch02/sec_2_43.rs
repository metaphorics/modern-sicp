// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.43, one module and one test.

mod ex_2_43 {
    /// The board position set of exercise 2.42.
    type Positions = Vec<usize>;

    /// The book's mapping order: `queen_cols(k - 1)` is evaluated once
    /// per level, and every placement is extended by every row.
    fn queen_cols_ordered(k: usize, board_size: usize, calls: &mut usize) -> Vec<Positions> {
        *calls += 1;
        if k == 0 {
            return vec![Vec::new()];
        }
        queen_cols_ordered(k - 1, board_size, calls)
            .into_iter()
            .flat_map(|mut positions| {
                (0..board_size).map(move |new_row| {
                    positions.push(new_row);
                    positions.clone()
                })
            })
            .collect()
    }

    /// Louis's swapped order: the row candidates are mapped on the
    /// outside, so the `k - 1` placements are recomputed for every
    /// row. The recursion tree gains a factor of `board_size` at every
    /// level, turning linear work into `board_size^k`; solving the
    /// puzzle in time `T` makes this version take about
    /// `board_size^board_size` times that, roughly `T * 8^8` for the
    /// eight-queens board.
    fn queen_cols_swapped(k: usize, board_size: usize, calls: &mut usize) -> Vec<Positions> {
        *calls += 1;
        if k == 0 {
            return vec![Vec::new()];
        }
        (0..board_size)
            .flat_map(|new_row| {
                queen_cols_swapped(k - 1, board_size, calls)
                    .into_iter()
                    .map(move |mut positions| {
                        positions.push(new_row);
                        positions
                    })
            })
            .collect()
    }

    /// Runs one variant and reports how many times it recomputed
    /// `queen_cols`, which is the measurable content of the slowdown.
    fn count_calls(
        run: fn(usize, usize, &mut usize) -> Vec<Positions>,
        board_size: usize,
    ) -> usize {
        let mut calls = 0;
        run(board_size, board_size, &mut calls);
        calls
    }

    /// Exercise 2.43: Louis's swapped mapping order
    ///
    /// Returns how many times each version of `queens` for `n = 5`
    /// recomputes `queen_cols`: the correct order and Louis's swapped
    /// order, whose ratio is the promised slowdown.
    pub fn ex_2_43() -> (usize, usize) {
        let ordered = count_calls(queen_cols_ordered, 5);
        let swapped = count_calls(queen_cols_swapped, 5);
        (ordered, swapped)
    }
}

#[test]
fn ex_2_43() {
    assert_eq!(ex_2_43::ex_2_43(), (6, 3906));
}
