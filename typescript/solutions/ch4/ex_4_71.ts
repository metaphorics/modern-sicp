// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  interleaveDelayed,
  Stream,
  singletonStream,
  streamAppendDelayed,
} from "../../packages/ch4/src/04-logic.js";
export function delayedAppend() {
  let n = 0;
  const loop = (): Stream<number> => {
    n++;
    return Stream.cons(2, loop);
  };
  const xs = streamAppendDelayed(singletonStream(1), loop).take(3);
  return [...xs, n];
}
export function delayedInterleave() {
  let n = 0;
  const loop = (): Stream<number> => {
    n++;
    return Stream.cons(2, loop);
  };
  const xs = interleaveDelayed(singletonStream(1), loop).take(4);
  return [...xs, n];
}
export function ex_4_71() {
  return "Delaying the recursive stream lets the first result escape before the recursive alternative is forced.";
}
