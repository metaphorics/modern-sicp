// SPDX-License-Identifier: GPL-3.0-only
// case core/19-deep-recursion: SICP 1.2.1 deep non-tail recursion: a deferred-operation chain 5000 frames deep.
function count(n: number): number {
  return n === 0 ? 0 : 1 + count(n - 1);
}
console.log(count(5000));
