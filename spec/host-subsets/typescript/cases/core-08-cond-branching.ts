// SPDX-License-Identifier: GPL-3.0-only
// case core/08-cond-branching: SICP 1.1.6 case analysis: if/else chains and conditional expressions.
function grade(score: number): string {
  if (score >= 90) {
    return "A";
  } else if (score >= 80) {
    return "B";
  } else if (score >= 70) {
    return "C";
  }
  return "F";
}
const abs = (x: number): number => (x < 0 ? -x : x);
console.log(grade(95));
console.log(grade(85));
console.log(grade(60));
console.log(abs(-7));
