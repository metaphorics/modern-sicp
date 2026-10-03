// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.8: lower the named binding form. The group's bound names are
 * the parameters of a local function whose body is the group's body; the
 * function is named within its own body and called immediately on the
 * initializers. Example guest program:
 * `function fib(n: number): number { const iter = (a: number, b: number, count: number): number => count === 0 ? b : iter(a + b, a, count - 1); return iter(1, 0, n); }`.
 * Modify `letToCall` of 4.6 to support this named form.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.8 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_08(): string {
  throw new PendingSolution();
}
