// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.21: recursion without define. Everything runs on the
 * engine, which needs no extension: the programs themselves carry the
 * recursion. The self-application trick binds a procedure whose first
 * parameter is the procedure itself, so the body recurses by passing
 * itself along. The typed guest spells that contract with an interface:
 * `Step { run(self, k) }` for the single-recursion programs, and the
 * two-procedure `EvenOdd` template fills the exercise's holes — each
 * recursive call forwards BOTH procedure arguments it received, the
 * pair itself plus the decremented count.
 */
import { runSource } from "../../packages/ch4/src/01-metacircular.js";

/** The book's factorial, typed through the self-application contract. */
export const factorialSource = `
interface Step {
  run: (self: Step, k: number) => number;
}
const fact: Step = {
  run: (self, k) => (k === 0 ? 1 : k * self.run(self, k - 1)),
};
fact.run(fact, 10);
`;

/** The Fibonacci analog written the same way. */
export const fibonacciSource = `
interface Step {
  run: (self: Step, k: number) => number;
}
const fib: Step = {
  run: (self, k) => (k < 2 ? k : self.run(self, k - 1) + self.run(self, k - 2)),
};
fib.run(fib, 10);
`;

/** The filled two-procedure template: every call forwards both
 * procedures — the ⟨??⟩ holes are `peer.check(peer, self, n - 1)`. */
export const evenOddSource = `
interface EvenOdd {
  check: (self: EvenOdd, peer: EvenOdd, n: number) => boolean;
}
const even: EvenOdd = {
  check: (self, peer, n) => (n === 0 ? true : peer.check(peer, self, n - 1)),
};
const odd: EvenOdd = {
  check: (self, peer, n) => (n === 0 ? false : peer.check(peer, self, n - 1)),
};
const f = (n: number): boolean => even.check(even, odd, n);
f(5);
`;

/** The same filled template asked for n = 6. */
export const evenOddSixSource = `${evenOddSource.replace("f(5);", "f(6);")}`;

/** Runs one self-application program through the engine. */
export const runSelfApplication = (text: string): RunResult => runSource(text);

export function ex_4_21(): string {
  return (
    "Recursion without define: the procedure takes itself as its first argument and " +
    "passes itself along. The typed guest spells the contract with an interface, so the " +
    "self-application is type-checked: the book's factorial answers 3628800, the " +
    "Fibonacci analog answers 55, and the filled two-procedure template — each call " +
    "forwarding both procedures plus the decremented count — answers false for 5 and " +
    "true for 6."
  );
}
