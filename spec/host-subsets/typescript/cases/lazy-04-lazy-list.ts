// SPDX-License-Identifier: GPL-3.0-only
// case lazy/04-lazy-list: SICP 4.2.3 lazy lists: delayed tails built on demand and memoized, so infinite streams are finite work.
let built = 0;
type Stream = { readonly head: number; readonly tail: Stream };
function integersFrom(k: number): Stream {
  built = built + 1;
  return { head: k, tail: delay(integersFrom(k + 1)) };
}
function streamRef(s: Stream, k: number): number {
  return k === 0 ? s.head : streamRef(force(s.tail), k - 1);
}
function scale(s: Stream, factor: number): Stream {
  return { head: s.head * factor, tail: delay(scale(force(s.tail), factor)) };
}
const naturals = integersFrom(0);
console.log(streamRef(naturals, 5));
console.log(streamRef(naturals, 5));
console.log(built);
console.log(streamRef(scale(integersFrom(1), 3), 4));
