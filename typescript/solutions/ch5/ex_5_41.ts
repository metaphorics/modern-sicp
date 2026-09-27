// SPDX-License-Identifier: GPL-3.0-only
import {
  type Cenv,
  findVariable,
  type LexicalAddress,
} from "../../packages/ch5/src/05-compilation.js";

const CENV: Cenv = [
  ["y", "z"],
  ["a", "b", "c", "d", "e"],
  ["x", "y"],
];

const render = (address: LexicalAddress): string =>
  address.found ? `(${address.frame} ${address.displacement})` : "not-found";

/** Exercise 5.41: find-variable answers the frame number and
 * displacement of a name in the compile-time environment. */
export const ex_5_41 = (): readonly string[] => {
  const answers = [
    `c: ${render(findVariable("c", CENV))}`,
    `x: ${render(findVariable("x", CENV))}`,
    `w: ${render(findVariable("w", CENV))}`,
  ];
  if (answers[0] !== "c: (1 2)") throw new Error(answers[0]);
  if (answers[1] !== "x: (2 0)") throw new Error(answers[1]);
  if (answers[2] !== "w: not-found") throw new Error(answers[2]);
  return answers;
};
