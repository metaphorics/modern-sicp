// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.5

/** A counter over a captured `let`: one mutable cell per call. */
export const makeCounter = (start: number): (() => number) => {
  let n = start;
  return () => {
    n += 1;
    return n;
  };
};

/** A withdrawer over a captured balance: section 3.1's account, in miniature. */
export const makeWithdrawer = (balance: number): ((amount: number) => number) => {
  let b = balance;
  return (amount) => {
    b -= amount;
    return b;
  };
};

/** A mutable object: its fields may be assigned unless declared readonly. */
export interface Cell {
  value: number;
}

/** Mutates one cell and reads it back through a second name. */
export const bumpedThroughAlias = (): number => {
  const cell: Cell = { value: 1 };
  const alias = cell;
  cell.value = 9;
  return alias.value;
};
