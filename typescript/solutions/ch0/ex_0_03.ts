// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.3: the recursion floor of the machine.
 *
 * `sumToRecursive` adds after the call returns, so every step holds a
 * frame. `sumToIterative` carries the state in a loop. `recursionDepth`
 * descends until the engine raises `RangeError` and reports the last
 * completed depth; the catch discriminates the one error it means to
 * handle and rethrows anything else.
 */
export function sumToRecursive(n: number): number {
  if (n === 0) {
    return 0;
  }
  return n + sumToRecursive(n - 1);
}

export function sumToIterative(n: number): number {
  let total = 0;
  for (let k = 1; k <= n; k += 1) {
    total += k;
  }
  return total;
}

export function recursionDepth(): number {
  const descend = (d: number): number => {
    try {
      return descend(d + 1);
    } catch (error) {
      if (error instanceof RangeError) {
        return d;
      }
      throw error;
    }
  };
  return descend(1);
}
