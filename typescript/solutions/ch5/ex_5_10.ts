// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assemble,
  assign,
  branch,
  type ControllerLine,
  c,
  getRegisterContents,
  jump,
  type Machine,
  makeNewMachine,
  mark,
  op,
  reg,
  setRegisterContents,
  test,
  type Value,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk } from "./ex_5_07.js";

/** The new surface: the book's instructions wrapped in book lines, plus
 * three new register-to-register forms. */
export type NewStmt =
  | { readonly tag: "cpy"; readonly to: string; readonly from: string }
  | { readonly tag: "inc"; readonly reg: string }
  | { readonly tag: "dec"; readonly reg: string }
  | { readonly tag: "book"; readonly stmt: ControllerLine };

export const book = (stmt: ControllerLine): NewStmt => ({ tag: "book", stmt });
export const cpy = (to: string, from: string): NewStmt => ({ tag: "cpy", to, from });
export const inc = (name: string): NewStmt => ({ tag: "inc", reg: name });
export const dec = (name: string): NewStmt => ({ tag: "dec", reg: name });

/** The isolated syntax procedures: every new form expands to exactly one
 * book instruction, before labels are scanned. */
export const syntaxExpand = (controller: ReadonlyArray<NewStmt>): ControllerLine[] => {
  const expanded: ControllerLine[] = [];
  for (const stmt of controller) {
    switch (stmt.tag) {
      case "cpy":
        expanded.push(assign(stmt.to, reg(stmt.from)));
        break;
      case "inc":
        expanded.push(assign(stmt.reg, op("+", reg(stmt.reg), c(1))));
        break;
      case "dec":
        expanded.push(assign(stmt.reg, op("-", reg(stmt.reg), c(1))));
        break;
      case "book":
        expanded.push(stmt.stmt);
        break;
    }
  }
  return expanded;
};

/** The book's gcd machine written in the new syntax: the two register
 * shuffles become cpy lines. */
const gcdControllerNewSyntax: NewStmt[] = [
  book(mark("test-b")),
  book(test("=", reg("b"), c(0))),
  book(branch("gcd-done")),
  book(assign("t", op("rem", reg("a"), reg("b")))),
  cpy("a", "b"),
  cpy("b", "t"),
  book(jump("test-b")),
  book(mark("gcd-done")),
];

/** A countdown that exercises dec and inc: while count is nonzero,
 * decrement it and increment sum, which starts at zero. */
const countdownController: NewStmt[] = [
  book(assign("count", reg("n"))),
  book(assign("sum", c(0))),
  book(mark("loop")),
  book(test("=", reg("count"), c(0))),
  book(branch("done")),
  dec("count"),
  inc("sum"),
  book(jump("loop")),
  book(mark("done")),
];

const runNewSyntax = (
  controller: ReadonlyArray<NewStmt>,
  registers: string[],
  inputs: Record<string, number>,
  answerReg: string,
): Value => {
  const machine: Machine = makeNewMachine(registers, arithmeticOperations);
  const program = assemble(syntaxExpand(controller), machine);
  if (!program.ok) throw new Error("assembly failed");
  machine.install(program.value);
  for (const [name, value] of Object.entries(inputs)) {
    expectOk(setRegisterContents(machine, name, value));
  }
  expectOk(machine.start());
  return expectOk(getRegisterContents(machine, answerReg));
};

/** Both machines in the new syntax: the gcd answers its usual 2, the
 * countdown sums 1 for each of its three decrements. */
export const newSyntaxRuns = (): string[] => {
  const gcd = runNewSyntax(gcdControllerNewSyntax, ["a", "b", "t"], { a: 206, b: 40 }, "a");
  const countdown = runNewSyntax(countdownController, ["n", "count", "sum"], { n: 3 }, "sum");
  return [`gcd(206, 40) in the new syntax = ${gcd}`, `countdown(3) sum = ${countdown}`];
};
