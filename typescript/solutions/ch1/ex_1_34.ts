// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.34: the type checker rejects self-application.
 *
 * `f` applies its argument to 2. The two working calls of the statement run
 * and land on 4 and 6; the self-application `f(f)` never runs at all,
 * because the type checker refuses it before the program can start. The
 * rejection evidence below was produced by running the project's tsc
 * (TypeScript 7.0.2) on a probe file carrying `f`, the two working calls,
 * and `f(f)`; the two calls compile and only the `f(f)` line is rejected.
 * The probe file was removed after the run - the tree must keep type
 * checking - and the diagnostic's exact words are pinned here instead.
 */
export const f = (g: (n: number) => number): number => g(2);

const square = (x: number): number => x * x;

export function workingCalls(): number[] {
  return [f(square), f((z: number): number => z * (z + 1))];
}

const REJECTION = [
  "error TS2345: Argument of type '(g: (n: number) => number) => number' is not assignable to parameter of type '(n: number) => number'.",
  "  Types of parameters 'g' and 'n' are incompatible.",
  "    Type 'number' is not assignable to type '(n: number) => number'.",
].join("\n");

export function selfApplicationDiagnostic(): string {
  return REJECTION;
}
