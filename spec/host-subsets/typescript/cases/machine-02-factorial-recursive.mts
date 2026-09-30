// SPDX-License-Identifier: GPL-3.0-only
// case machine/02-factorial-recursive: SICP 5.1.4 recursive factorial machine: continue register and save/restore stack discipline.
type Word = number | boolean | undefined;
type Source =
  | { readonly tag: "reg"; readonly name: string }
  | { readonly tag: "const"; readonly value: number }
  | { readonly tag: "label"; readonly name: string }
  | { readonly tag: "op"; readonly operation: string; readonly args: ReadonlyArray<Source> };
type Statement =
  | { readonly tag: "label"; readonly name: string }
  | { readonly tag: "assign"; readonly register: string; readonly source: Source }
  | { readonly tag: "test"; readonly operation: string; readonly args: ReadonlyArray<Source> }
  | { readonly tag: "branch"; readonly label: string }
  | { readonly tag: "goto-label"; readonly label: string }
  | { readonly tag: "goto-register"; readonly register: string }
  | { readonly tag: "save"; readonly register: string }
  | { readonly tag: "restore"; readonly register: string }
  | { readonly tag: "perform"; readonly operation: string; readonly args: ReadonlyArray<Source> };
type Operation = (args: ReadonlyArray<Word>) => Word;
interface MachineProgram {
  readonly registers: ReadonlyArray<string>;
  readonly operations: Readonly<Record<string, Operation>>;
  readonly controller: ReadonlyArray<Statement>;
  readonly inputs: ReadonlyArray<readonly [string, number]>;
  readonly outputs: ReadonlyArray<string>;
}
const reg = (name: string): Source => ({ tag: "reg", name });
const num = (value: number): Source => ({ tag: "const", value });
const labelRef = (name: string): Source => ({ tag: "label", name });
const op = (operation: string, ...args: Source[]): Source => ({ tag: "op", operation, args });
const label = (name: string): Statement => ({ tag: "label", name });
const assign = (register: string, source: Source): Statement => ({ tag: "assign", register, source });
const test = (operation: string, ...args: Source[]): Statement => ({ tag: "test", operation, args });
const branch = (target: string): Statement => ({ tag: "branch", label: target });
const goto = (target: string): Statement => ({ tag: "goto-label", label: target });
const gotoReg = (register: string): Statement => ({ tag: "goto-register", register });
const save = (register: string): Statement => ({ tag: "save", register });
const restore = (register: string): Statement => ({ tag: "restore", register });
const perform = (operation: string, ...args: Source[]): Statement => ({ tag: "perform", operation, args });
const n = (value: Word): number => (typeof value === "number" ? value : Number.NaN);
const arithmetic: Readonly<Record<string, Operation>> = {
  add: (args) => n(args[0]) + n(args[1]),
  sub: (args) => n(args[0]) - n(args[1]),
  mul: (args) => n(args[0]) * n(args[1]),
  rem: (args) => n(args[0]) % n(args[1]),
  eq: (args) => n(args[0]) === n(args[1]),
  lt: (args) => n(args[0]) < n(args[1]),
};
export const machine = (emit: (line: string) => void): MachineProgram => ({
  registers: ["n", "val", "continue"],
  operations: arithmetic,
  controller: [
    assign("continue", labelRef("fact-done")),
    label("fact-loop"),
    test("eq", reg("n"), num(1)),
    branch("base-case"),
    save("continue"),
    save("n"),
    assign("n", op("sub", reg("n"), num(1))),
    assign("continue", labelRef("after-fact")),
    goto("fact-loop"),
    label("after-fact"),
    restore("n"),
    restore("continue"),
    assign("val", op("mul", reg("n"), reg("val"))),
    gotoReg("continue"),
    label("base-case"),
    assign("val", num(1)),
    gotoReg("continue"),
    label("fact-done"),
  ],
  inputs: [["n", 5]],
  outputs: ["val"],
});
