// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  constant,
  type MachineStatement,
  type MachineValue,
  op,
  register,
} from "../../packages/ch5/src/01-register-machines.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, mark } from "./ex_5_07.ts";

/** The new surface of exercise 5.10: register-to-register forms beside
 * the book's instruction shapes. The syntax stays a separate typed
 * vocabulary, so the simulator's data model never grows a second
 * representation. */
export type NewStmt =
  | { readonly tag: "new-label"; readonly name: string }
  | { readonly tag: "new-load"; readonly to: string; readonly value: MachineValue }
  | { readonly tag: "new-move"; readonly to: string; readonly from: string }
  | { readonly tag: "new-add"; readonly to: string; readonly left: string; readonly right: string }
  | { readonly tag: "new-sub"; readonly to: string; readonly left: string; readonly right: string }
  | { readonly tag: "new-rem"; readonly to: string; readonly left: string; readonly right: string }
  | { readonly tag: "new-test-equal"; readonly left: string; readonly right: MachineValue }
  | { readonly tag: "new-jump"; readonly label: string }
  | { readonly tag: "new-branch-if"; readonly label: string };

/** One syntax procedure per form: the recognizer. */
export const isLabel = (line: NewStmt): boolean => line.tag === "new-label";
export const isMove = (line: NewStmt): boolean => line.tag === "new-move";

/** The translator: every new form lowers to the simulator's own
 * statements, and nothing else knows the new spelling. */
export const translate = (line: NewStmt): MachineStatement[] => {
  switch (line.tag) {
    case "new-label":
      return [mark(line.name)];
    case "new-load":
      return [assign(line.to, constant(line.value))];
    case "new-move":
      return [assign(line.to, register(line.from))];
    case "new-add":
      return [assign(line.to, op("+", register(line.left), register(line.right)))];
    case "new-sub":
      return [assign(line.to, op("-", register(line.left), register(line.right)))];
    case "new-rem":
      return [assign(line.to, op("rem", register(line.left), register(line.right)))];
    case "new-test-equal":
      return [
        {
          tag: "test",
          operation: "=",
          args: [register(line.left), constant(line.right)],
        },
      ];
    case "new-jump":
      return [{ tag: "goto-label", label: line.label }];
    case "new-branch-if":
      return [{ tag: "branch", label: line.label }];
  }
};

/** The gcd machine of the book written in the new syntax. */
export const gcdInNewSyntax: readonly NewStmt[] = [
  { tag: "new-label", name: "test-b" },
  { tag: "new-test-equal", left: "b", right: 0 },
  { tag: "new-branch-if", label: "gcd-done" },
  { tag: "new-rem", to: "t", left: "a", right: "b" },
  { tag: "new-move", to: "a", from: "b" },
  { tag: "new-move", to: "b", from: "t" },
  { tag: "new-jump", label: "test-b" },
  { tag: "new-label", name: "gcd-done" },
];

/** Runs the translated machine on one input pair. */
export const ex_5_10 = (a: number, b: number): MachineValue => {
  const controller = gcdInNewSyntax.flatMap(translate);
  const machine = makeMachine({
    registers: ["a", "b", "t"],
    operations: arithmeticOperations,
    controller,
  });
  machine.writeRegister("a", a);
  machine.writeRegister("b", b);
  const run = machine.run();
  expectOk(run);
  const answer = machine.readRegister("a");
  return answer === undefined ? null : answer;
};
