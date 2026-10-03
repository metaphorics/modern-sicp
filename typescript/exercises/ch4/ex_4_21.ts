// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.21: recursion without local declarations. Part (a) checks that the
 * self-application expression below computes factorials and asks for an
 * analogous Fibonacci expression. Part (b) asks for the missing operands
 * that complete mutually recursive even/odd functions without internal
 * declarations or recursive bindings.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.21 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's expression: 10 factorial by passing the procedure to itself. */
export const factorialSource = `interface Step {
  run: (self: Step, k: number) => number;
}
const factorialStep: Step = {
  run: (self: Step, k: number): number => k <= 1 ? 1 : k * self.run(self, k - 1),
};
const applyStep = (step: Step, n: number): number => step.run(step, n);
applyStep(factorialStep, 10);`;

/** Complete each recursive call with the two procedure records and the next input. */
export const evenOddSkeleton = `interface OddEven {
  run: (self: OddEven, other: OddEven, n: number) => boolean;
}
const f = (x: number): boolean =>
  ((even: OddEven, odd: OddEven): boolean => even.run(even, odd, x))(
    { run: (self, other, n) => n === 0 ? true : other.run(missingEven, missingOdd, missingN) },
    { run: (self, other, n) => n === 0 ? false : self.run(missingEven, missingOdd, missingN) },
  );`;

export function ex_4_21(): string {
  throw new PendingSolution();
}
