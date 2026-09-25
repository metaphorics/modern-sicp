// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.34a: the same self-application with the types erased.
 *
 * The `any` type is the host's escape hatch: it silences the checker of
 * exercise 1.34 and lets the program run. Running it throws the host's own
 * failure - the number 2 ends up in the callee position - and this solution
 * catches the thrown error and reports its kind and message. The evidence
 * comes from actually running the erased program: the host raises
 * `TypeError: g is not a function`, because after the first application
 * `fAny(fAny)` becomes `fAny(2)`, and inside that call the parameter `g`
 * holds the number 2 when the body applies it.
 */
// biome-ignore lint/suspicious/noExplicitAny: the exercise erases the types on purpose, the host's escape hatch
export const fAny = (g: any): any => g(2);

export function runtimeSelfApplicationError(): { kind: string; message: string } {
  try {
    fAny(fAny);
    return { kind: "none", message: "the erased call did not throw" };
  } catch (error) {
    return error instanceof Error
      ? { kind: error.name, message: error.message }
      : { kind: typeof error, message: String(error) };
  }
}
