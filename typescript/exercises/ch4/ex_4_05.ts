// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.5: the case analysis of 4.1.2 matches its discriminant against
 * literal tests. Add a clause whose match evaluates a recipient and calls it
 * with the matched value, the call's result becoming the analysis value.
 * Example: `const entries = [{ key: "a", value: 1 }, { key: "b", value: 2 }];`
 * and a case analysis of `lookup("b", entries)` with recipient `entryValue`
 * returns 2. Modify `switchToIf` and the 4.1.2 syntax procedures.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.5 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_05(): string {
  throw new PendingSolution();
}
