// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arrayLit,
  assign,
  bin,
  block,
  call,
  cond,
  exprStmt,
  functionDecl,
  ident,
  lam,
  num,
  type Program,
  param,
  returnStmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.ts";

/**
 * Exercise 4.10: eval need not be married to one syntax. The demand: use
 * data abstraction to parameterize the evaluator over a syntax table (a
 * map from syntax tag to handler), then install a second syntax for
 * the same language, and show one program evaluate to the same value under
 * both. The pending part is the table, the two installations, and the
 * demonstration.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.10 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The same little program in checked TypeScript-subset source. */
export const squareProgram = `function square(x: number): number {
  return x * x;
}
square(7);`;

/** The same program as an arrow-function surface, lowered to the shared
 * syntax representation by the exercise. */
export const alternateSquareProgram = `const square = (x: number): number => x * x;
square(7);`;

/** The shared lowerings for both surfaces. */
export const squareLowering: Program = [
  functionDecl("square", [param("x")], [returnStmt(bin("*", ident("x"), ident("x")))]),
  exprStmt(call(ident("square"), [num(7)])),
];

export const alternateSquareLowering: Program = [
  varDecl("const", "square", lam([param("x")], [returnStmt(bin("*", ident("x"), ident("x")))])),
  exprStmt(call(ident("square"), [num(7)])),
];

export function ex_4_10(): string {
  throw new PendingSolution();
}
