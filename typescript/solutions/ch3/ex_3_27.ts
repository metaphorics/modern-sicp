// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.27: memoFib. The book's memoizer wraps the naive recursive procedure
 * in a local table: each call first looks its argument up, answers a
 * previously computed value when it is there, and otherwise computes
 * in the ordinary way, stores, and answers. The wrapped body must
 * recurse into the memoized procedure itself (the `memoFib` procedure closes over one table and its body calls `memoFib`),
 * so the nested calls share the one table and each fib value is
 * computed once: the steps grow proportional to n. Whether the
 * plan would work over plain recursion (it would not: the recursive
 * calls would never consult the table) is a question about exactly
 * this sharing, which the trace makes visible.
 */

/** One call the memoizer answered: a `Compute` ran the wrapped
 * lambda for `n` and stored the result in the table, a `Recall`
 * found `n` already there and answered without computing. */
export type MemoFibEvent =
  | { readonly _tag: "Compute"; readonly n: number }
  | { readonly _tag: "Recall"; readonly n: number };

/** Builds the book's memoFib over one local `Map` table, appending
 * one event per call to `log`; the wrapped lambda's body recurses
 * into the memoized procedure, so nested calls fill the same table. */
const buildMemoFib = (log: MemoFibEvent[]): ((n: number) => number) => {
  const table = new Map<number, number>();
  const memoFib = (n: number): number => {
    const prior = table.get(n);
    if (prior !== undefined) {
      log.push({ _tag: "Recall", n });
      return prior;
    }
    log.push({ _tag: "Compute", n });
    const result = n === 0 ? 0 : n === 1 ? 1 : memoFib(n - 1) + memoFib(n - 2);
    table.set(n, result);
    return result;
  };
  return memoFib;
};

/** The book's memoFib: the n-th Fibonacci number, each value
 * computed once and recalled from the table ever after. */
export const memoFib = (n: number): number => buildMemoFib([])(n);

/** Runs the book's memoFib and answers the log of the calls it
 * answered: the environment structure of the computation, as
 * events. */
export const memoFibTrace = (n: number): MemoFibEvent[] => {
  const log: MemoFibEvent[] = [];
  buildMemoFib(log)(n);
  return log;
};
