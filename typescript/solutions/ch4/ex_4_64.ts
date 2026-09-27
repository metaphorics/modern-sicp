// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

export const recursiveFirstOutrankedBy = `
(rule (outranked-by ?staff-person ?boss)
  (or (supervisor ?staff-person ?boss)
      (and (outranked-by ?middle-manager ?boss)
           (supervisor ?staff-person ?middle-manager))))
`;

/** Explain the operational failure without executing the non-terminating query. */
export function loopExplanation(): string {
  return "The recursive clause runs before supervisor constrains ?middle-manager. It asks for outranked-by with a fresh, unconstrained staff person, then repeats the same recursive-first branch indefinitely. The original base-first ordering binds a direct supervisor before recursing upward.";
}

export function ex_4_64(): string {
  return loopExplanation();
}
