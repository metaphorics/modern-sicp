// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.9: the processes of two additions in terms of inc and dec.
 *
 * plusRecursive defers an inc per call: the chain grows to length a and
 * then contracts, a linear recursive process. plusIterative keeps the
 * whole state in (x, y) and adds one deferred nothing: a linear
 * iterative process. Node performs no tail-call optimization, so only
 * the loop runs in constant space; the recursive spelling at a = 100000
 * overflows the call stack.
 */
export const inc = (x: number): number => x + 1;

export const dec = (x: number): number => x - 1;

export const plusRecursive = (a: number, b: number): number =>
  a === 0 ? b : inc(plusRecursive(dec(a), b));

export const plusIterative = (a: number, b: number): number => {
  let x = a;
  let y = b;
  while (x > 0) {
    x = dec(x);
    y = inc(y);
  }
  return y;
};

/** Every (a, b) the recursive process passes through, outermost first. */
export function plusRecursiveTrace(
  a: number,
  b: number,
): { value: number; states: Array<[number, number]> } {
  const states: Array<[number, number]> = [];
  function rec(x: number, y: number): number {
    states.push([x, y]);
    return x === 0 ? y : inc(rec(dec(x), y));
  }
  return { value: rec(a, b), states };
}

/** The loop's (x, y) state after every pass, entry state included. */
export function plusIterativeTrace(
  a: number,
  b: number,
): { value: number; states: Array<[number, number]> } {
  const states: Array<[number, number]> = [];
  let x = a;
  let y = b;
  while (x > 0) {
    states.push([x, y]);
    x = dec(x);
    y = inc(y);
  }
  states.push([x, y]);
  return { value: y, states };
}
