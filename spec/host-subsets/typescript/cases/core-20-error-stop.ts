// SPDX-License-Identifier: GPL-3.0-only
// case core/20-error-stop: a printed prefix, then an uncaught error stops the run.
let depth = 0;
function descend(n: number): number {
  depth = n;
  if (n <= 0) {
    throw new Error("bottom");
  }
  return descend(n - 1) + 1;
}
console.log("prefix");
descend(300);
console.log("unreachable");
