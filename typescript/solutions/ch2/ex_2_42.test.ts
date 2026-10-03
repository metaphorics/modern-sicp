// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { length, list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { adjoinPosition, emptyBoard, isSafe, queens } from "./ex_2_42.js";

/** Flattens a list into an array for the independent re-check. */
const toArray = <A>(xs: List<A>): A[] => {
  const out: A[] = [];
  for (let rest = xs; rest._tag === "Cons"; rest = rest.tail) {
    out.push(rest.head);
  }
  return out;
};

/** True when no two of the rows attack each other, whatever the order. */
const noTwoAttack = (rs: number[]): boolean => {
  for (let i = 0; i < rs.length; i += 1) {
    for (let j = i + 1; j < rs.length; j += 1) {
      const a = rs[i];
      const b = rs[j];
      if (a === undefined || b === undefined) {
        continue;
      }
      if (a === b || Math.abs(a - b) === j - i) {
        return false;
      }
    }
  }
  return true;
};

describe("exercise 2.42", () => {
  it("emptyBoard is nil and adjoinPosition conses in front", () => {
    expect(showList(emptyBoard())).toBe("[]");
    expect(showList(adjoinPosition(3, 2, list(1, 2)))).toBe("[3, 1, 2]");
  });

  it("isSafe rejects same-row and diagonal attacks, allows the rest", () => {
    expect(isSafe(2, list(1, 1))).toBe(false);
    expect(isSafe(2, list(2, 1))).toBe(false);
    expect(isSafe(3, list(3, 1))).toBe(true);
    expect(isSafe(1, list(4))).toBe(true);
  });

  it("a 6-by-6 board has 4 solutions", () => {
    expect(length(queens(6))).toBe(4);
  });

  it("an 8-by-8 board has 92 solutions", () => {
    expect(length(queens(8))).toBe(92);
  });

  it("every 6-queens solution re-checks as mutually safe", () => {
    const solutions = toArray(queens(6));
    expect(solutions.length).toBe(4);
    for (const positions of solutions) {
      expect(noTwoAttack(toArray(positions))).toBe(true);
    }
  });

  it("every 8-queens solution re-checks as mutually safe", () => {
    const solutions = toArray(queens(8));
    expect(solutions.length).toBe(92);
    for (const positions of solutions) {
      expect(noTwoAttack(toArray(positions))).toBe(true);
    }
  });
});
