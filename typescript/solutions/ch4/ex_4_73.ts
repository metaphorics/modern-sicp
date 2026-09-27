// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { flattenStream, Stream } from "../../packages/ch4/src/04-logic.js";

const nums = (n: number): Stream<number> => Stream.cons(n, () => nums(n + 1));
const nested = (): Stream<Stream<number>> => Stream.cons(nums(1), nested);
export function flattenPrefix() {
  return flattenStream(nested()).take(6);
}
export function ex_4_73() {
  return "The recursive flatten is delayed so an infinite outer stream yields before its tail is traversed.";
}
