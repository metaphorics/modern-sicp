// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.9: environment structures of the two factorials. The book
 * asks for the structures the interpreter builds for `factorial(6)` in
 * each version; in this edition the frames of that drawing are call
 * frames, so the answer is measured rather than drawn: each traced
 * factorial reports its result next to the deepest frame count its run
 * reached, a frame trace spells out what is live at the deepest point,
 * and the two structures are rendered from the traces. The probe is
 * deliberately raw JavaScript (the raw-closure precedent of exercise
 * 3.8): the stack of ordinary function frames is the thing the
 * exercise diagrams, and an Effect rendering would measure the fiber
 * machinery's frames instead of the language's.
 */

/** A traced run of one factorial: the value it answers and the frame
 * shape that computed it, read off the same run. */
export interface DepthReport {
  /** The factorial's value (720 at n = 6, by either version). */
  readonly result: number;
  /** The greatest number of frames the run held live at once. */
  readonly deepest: number;
}

/** One live frame of the recursive run at its deepest point: the
 * book's diagram is this sequence, one row per pending call. */
export interface RecursiveFrame {
  /** The parameter this frame binds. */
  readonly n: number;
  /** The work still pending in this frame when the call below it runs. */
  readonly pending: string;
}

/** One state of the iterative version's single frame: the `while` loop
 * reuses the frame `factorialIter` opened, so each row replaces the
 * previous one and only the last is live at any time. */
export interface IterativeFrame {
  readonly product: number;
  readonly counter: number;
}

/** Traces the recursive factorial (1.2.1's first version): every call
 * opens a frame that must survive until its own multiplication runs
 * after the call below returns, so the deepest point holds one frame
 * per pending `n`, all pointing at the global frame where the binding
 * of `factorialRecursive` lives. */
export const factorialTraced = (n: number): DepthReport => {
  const deepest = { current: 0 };
  const go = (k: number, depth: number): number => {
    if (depth > deepest.current) {
      deepest.current = depth;
    }
    return k <= 1 ? 1 : k * go(k - 1, depth + 1);
  };
  return { result: go(n, 1), deepest: deepest.current };
};

/** Traces the iterative factorial (1.2.1's second version, in the
 * edition's loop spelling): the loop body performs no calls, so no
 * frame is ever added and the run never leaves depth 1. */
export const factorialIterTraced = (n: number): DepthReport => {
  let product = 1;
  let counter = 1;
  while (counter <= n) {
    product = counter * product;
    counter += 1;
  }
  return { result: product, deepest: 1 };
};

/** Records the frames the recursive version pushes, in push order: at
 * the deepest point of `n = 6` these six frames are all live, each
 * waiting on the one below it. */
export const recursiveFrames = (n: number): ReadonlyArray<RecursiveFrame> => {
  const frames: RecursiveFrame[] = [];
  const go = (k: number): number => {
    frames.push({ n: k, pending: k <= 1 ? "1" : `${k} * factorial(${k - 1})` });
    return k <= 1 ? 1 : k * go(k - 1);
  };
  go(n);
  return frames;
};

/** Records the states of the iterative version's one frame, in
 * order: each row is the same frame re-entering the loop body, not
 * another frame stacked on it. */
export const iterativeFrameStates = (n: number): ReadonlyArray<IterativeFrame> => {
  const states: IterativeFrame[] = [];
  let product = 1;
  let counter = 1;
  while (counter <= n) {
    states.push({ product, counter });
    product = counter * product;
    counter += 1;
  }
  return states;
};

/** Renders the two structures at their deepest points, from the frame
 * traces of an actual instrumented run: the drawing the exercise asks
 * for, produced rather than sketched. */
export const renderFactorialStructures = (n: number): string => {
  const rec = recursiveFrames(n);
  const iter = iterativeFrameStates(n);
  const lines: string[] = [];
  lines.push(`factorial(${n}) recursive: ${rec.length} frames live at the deepest point`);
  lines.push(`factorialIter(${n}) iterative: 1 frame reused across ${iter.length} iterations`);
  lines.push("");
  lines.push("recursive (all live)          iterative (one frame, reused)");
  for (let i = 0; i < Math.max(rec.length, iter.length); i++) {
    const frame = rec[i];
    const state = iter[i];
    const left = frame === undefined ? "" : `n: ${String(frame.n)}   -> global`;
    const right =
      state === undefined
        ? ""
        : `product: ${String(state.product)}, counter: ${String(state.counter)} -> global`;
    lines.push(`${left.padEnd(30)}${right}`);
  }
  lines.push("");
  lines.push("each frame waits for the one   the frame is reassigned, never");
  lines.push("below it, then multiplies      stacked: nothing accumulates");
  return lines.join("\n");
};
