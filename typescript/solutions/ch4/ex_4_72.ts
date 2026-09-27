// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { interleave, Stream, streamAppend } from "../../packages/ch4/src/04-logic.js";

const nums = (n: number): Stream<number> => Stream.cons(n, () => nums(n + 1));
const finite = (xs: number[], i = 0): Stream<number> => {
  const head = xs[i];
  return head === undefined ? Stream.empty() : Stream.cons(head, () => finite(xs, i + 1));
};
export const appended = () => streamAppend(nums(1), finite([100, 200])).take(6);
export const interleaved = () => interleave(nums(1), finite([100, 200])).take(6);
export function ex_4_72() {
  return "Interleaving prevents an infinite first stream from starving later alternatives.";
}
