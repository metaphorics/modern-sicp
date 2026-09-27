// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.41: an ordinary program for the multiple-dwelling puzzle.
 * This exercise is an A row: the statement asks for an ordinary program in
 * the reader's working language, which here is TypeScript, so the solver is
 * a plain host function. It enumerates the 120 permutations of the five
 * floors over the five people and filters them with the puzzle's
 * restrictions; no evaluator, no search machinery. The enumeration answers
 * exactly one assignment, the amb evaluator's answer too: the exercise's
 * point that the nondeterministic program buys notation, not power.
 */

/** One assignment: the floors of baker, cooper, fletcher, miller, smith. */
export type Assignment = readonly [
  baker: number,
  cooper: number,
  fletcher: number,
  miller: number,
  smith: number,
];

/** The puzzle's restrictions on a complete assignment. */
export const meetsRestrictions = (a: Assignment): boolean =>
  new Set(a).size === 5 &&
  a[0] !== 5 &&
  a[1] !== 1 &&
  a[2] !== 5 &&
  a[2] !== 1 &&
  a[3] > a[1] &&
  Math.abs(a[4] - a[2]) !== 1 &&
  Math.abs(a[2] - a[1]) !== 1;

const permutations = (items: readonly number[]): ReadonlyArray<readonly number[]> => {
  if (items.length === 0) {
    return [[]];
  }
  const out: number[][] = [];
  for (let i = 0; i < items.length; i += 1) {
    const head = items[i];
    if (head === undefined) {
      continue;
    }
    const rest = [...items.slice(0, i), ...items.slice(i + 1)];
    for (const tail of permutations(rest)) {
      out.push([head, ...tail]);
    }
  }
  return out;
};

const toAssignment = (p: readonly number[]): Assignment | undefined => {
  const [baker, cooper, fletcher, miller, smith] = p;
  return baker !== undefined &&
    cooper !== undefined &&
    fletcher !== undefined &&
    miller !== undefined &&
    smith !== undefined
    ? [baker, cooper, fletcher, miller, smith]
    : undefined;
};

/** Every solution of the puzzle, in the assignment's person order. */
export const solutions = (): ReadonlyArray<Assignment> =>
  permutations([1, 2, 3, 4, 5]).flatMap((p) => {
    const a = toAssignment(p);
    return a !== undefined && meetsRestrictions(a) ? [a] : [];
  });

const names = ["baker", "cooper", "fletcher", "miller", "smith"] as const;

/** Renders an assignment the way the amb evaluator prints its answer. */
export const render = (a: Assignment): string =>
  `(${a.map((floor, i) => `(${names[i]} ${floor})`).join(" ")})`;

export function ex_4_41(): string {
  const found = solutions();
  const first = found[0];
  if (found.length !== 1 || first === undefined) {
    throw new Error(`expected exactly one solution, got ${found.length}`);
  }
  const answer = render(first);
  return (
    "The ordinary program enumerates the 120 permutations of the five floors " +
    "over the five people and filters them with the puzzle's restrictions: no " +
    "evaluator, no search machinery, just enumeration and predicates. It " +
    "answers exactly one assignment, " +
    `${answer}, which is the amb evaluator's answer as well: the ` +
    "nondeterministic program buys notation, not power."
  );
}
