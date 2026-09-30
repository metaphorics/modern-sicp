// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.12: the environment interface has three procedures that all
 * walk the frame chain looking for one name. The demand: factor that walk
 * out once, as abstract traversals (one over a frame's bindings, one over
 * the environment chain), and implement lookupVariableValue,
 * setVariableValue, and defineVariableValue in terms of them.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.12 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A three-frame chain exercises every traversal path: inner, middle,
 * outer, and a name that lives in no frame at all. */
export const threeFrameProgram = `const a = 1;
const result = (() => {
  const b = 2;
  return (() => {
    const c = 3;
    return [a, b, c];
  })();
})();
result;`;
export const missingName = "missing";
export type MissingLookup = { readonly name: string; readonly expected: "unbound-name" };
export const missingNameLookup: MissingLookup = { name: missingName, expected: "unbound-name" };

export function ex_4_12(): string {
  throw new PendingSolution();
}
