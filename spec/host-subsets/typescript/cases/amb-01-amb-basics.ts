// SPDX-License-Identifier: GPL-3.0-only
// case amb/01-amb-basics: SICP 4.3.1 amb basics (amb-depth-first-experiment): choices tried left to right, failures backtrack, every answer found in order.
const x = choose(1, 2, 3);
console.log(`try ${x}`);
require(x > 1);
console.log(`found ${x}`);
