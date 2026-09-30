// SPDX-License-Identifier: GPL-3.0-only
// case amb/03-multiple-dwelling: SICP 4.3.2 multiple-dwelling puzzle; an ordinary `=` write made on each path is trailed and undone on backtracking.
let tried = 0;
function distinct(a: number, b: number, c: number, d: number, e: number): boolean {
  return a !== b && a !== c && a !== d && a !== e && b !== c && b !== d && b !== e && c !== d && c !== e && d !== e;
}
const baker = choose(1, 2, 3, 4, 5);
const cooper = choose(1, 2, 3, 4, 5);
const fletcher = choose(1, 2, 3, 4, 5);
const miller = choose(1, 2, 3, 4, 5);
const smith = choose(1, 2, 3, 4, 5);
tried = tried + 1;
require(distinct(baker, cooper, fletcher, miller, smith));
require(baker !== 5);
require(cooper !== 1);
require(fletcher !== 5);
require(fletcher !== 1);
require(miller > cooper);
require(Math.abs(smith - fletcher) !== 1);
require(Math.abs(fletcher - cooper) !== 1);
console.log(`baker ${baker} cooper ${cooper} fletcher ${fletcher} miller ${miller} smith ${smith}`);
console.log(`tried ${tried}`);
