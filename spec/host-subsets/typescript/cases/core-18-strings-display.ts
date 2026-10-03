// SPDX-License-Identifier: GPL-3.0-only
// case core/18-strings-display: SICP 2.3.1 display effects: string literals, concatenation, and template rendering of numbers.
const name = "square";
const value = 2.5;
console.log("a");
console.log(`${42}`);
console.log(name + " of " + `${value}` + " is " + `${value * value}`);
console.log("line".padEnd(6, ".") + "|");
