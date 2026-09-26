// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assign,
  branch,
  type ControllerLine,
  getRegisterContents,
  jump,
  jumpReg,
  lbl,
  makeMachine,
  mark,
  op,
  perform,
  reg,
  restore,
  save,
  setRegisterContents,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  cons,
  dump,
  emptyList,
  eqWords,
  listOperations,
  type Memory,
  makeMemory,
  pairPointer,
  renderWord,
  type Value,
  write,
} from "../../packages/ch5/src/03-storage.js";
import { expectOk } from "./ex_5_07.js";

/** Plants a proper list of numbers through the allocation path. */
export const plantList = (memory: Memory, values: ReadonlyArray<number>): Value => {
  let acc: Value = emptyList;
  for (const value of [...values].reverse()) acc = cons(memory, value, acc);
  return acc;
};

/** The append machine: it copies x and shares y; the answer lands in z. */
export const appendController: ControllerLine[] = [
  assign("continue", lbl("append-done")),
  mark("append-loop"),
  test("null?", reg("x")),
  branch("base"),
  assign("temp", op("car", reg("x"))),
  save("temp"),
  save("continue"),
  assign("continue", lbl("after-car")),
  assign("x", op("cdr", reg("x"))),
  jump("append-loop"),
  mark("base"),
  assign("z", reg("y")),
  jumpReg("continue"),
  mark("after-car"),
  restore("continue"),
  restore("temp"),
  assign("z", op("cons", reg("temp"), reg("z"))),
  jumpReg("continue"),
  mark("append-done"),
];

/** The append! machine: walk to the last pair of x and splice y in with
 * set-cdr!; no cell is allocated and there is no z. */
export const appendBangController: ControllerLine[] = [
  assign("temp", reg("x")),
  mark("last-pair"),
  assign("cand", op("cdr", reg("temp"))),
  test("null?", reg("cand")),
  branch("splice"),
  assign("temp", op("cdr", reg("temp"))),
  jump("last-pair"),
  mark("splice"),
  perform("set-cdr!", reg("temp"), reg("y")),
];

/** Both exercise runs: append over planted x and y, then append! over a
 * fresh copy, with the memory drawn before and after the splice. */
export const appendRuns = (): string[] => {
  const memory = makeMemory(16);
  const x = plantList(memory, [1, 2, 3]);
  const y = plantList(memory, [4, 5]);
  const machine = expectOk(
    makeMachine(
      ["x", "y", "z", "temp", "continue"],
      { ...arithmeticOperations, ...listOperations(memory) },
      appendController,
    ),
  );
  expectOk(setRegisterContents(machine, "x", x));
  expectOk(setRegisterContents(machine, "y", y));
  expectOk(machine.start());
  const z = expectOk(getRegisterContents(machine, "z"));
  const appendLines = [
    `append: z = ${renderWord(z)} = ${write(memory, z)}`,
    `append: x is still ${write(memory, x)} (${renderWord(x)}), free moved to ` +
      `${renderWord(pairPointer(memory.free))}, three fresh cells`,
  ];

  const memory2 = makeMemory(8);
  const x2 = plantList(memory2, [1, 2, 3]);
  const y2 = plantList(memory2, [4, 5]);
  const before = dump(memory2);
  const machine2 = expectOk(
    makeMachine(
      ["x", "y", "temp", "cand"],
      { ...arithmeticOperations, ...listOperations(memory2) },
      appendBangController,
    ),
  );
  expectOk(setRegisterContents(machine2, "x", x2));
  expectOk(setRegisterContents(machine2, "y", y2));
  expectOk(machine2.start());
  const xSpliced = expectOk(getRegisterContents(machine2, "x"));
  return [
    ...appendLines,
    "append!: before, the last pair of x points at e0:",
    before,
    "append!: after, it points at y:",
    dump(memory2),
    `append!: the answer is x itself, now ${write(memory2, xSpliced)}, the same pointer ` +
      `${renderWord(xSpliced)} the caller passed, and free is still ` +
      `${renderWord(pairPointer(memory2.free))}`,
  ];
};

/** The two procedures' identity properties: append allocates a distinct
 * result and leaves x unchanged, while append! returns the very pointer x. */
export const appendIdentity = (): {
  readonly appendCopies: boolean;
  readonly appendBangShares: boolean;
} => {
  const memory = makeMemory(16);
  const x = plantList(memory, [1, 2, 3]);
  const y = plantList(memory, [4, 5]);
  const appendMachine = expectOk(
    makeMachine(
      ["x", "y", "z", "temp", "continue"],
      { ...arithmeticOperations, ...listOperations(memory) },
      appendController,
    ),
  );
  expectOk(setRegisterContents(appendMachine, "x", x));
  expectOk(setRegisterContents(appendMachine, "y", y));
  expectOk(appendMachine.start());
  const copied = expectOk(getRegisterContents(appendMachine, "z"));

  const memory2 = makeMemory(8);
  const x2 = plantList(memory2, [1, 2, 3]);
  const y2 = plantList(memory2, [4, 5]);
  const appendBangMachine = expectOk(
    makeMachine(["x", "y", "temp", "cand"], listOperations(memory2), appendBangController),
  );
  expectOk(setRegisterContents(appendBangMachine, "x", x2));
  expectOk(setRegisterContents(appendBangMachine, "y", y2));
  expectOk(appendBangMachine.start());
  const spliced = expectOk(getRegisterContents(appendBangMachine, "x"));
  return {
    appendCopies: !eqWords(copied, x) && write(memory, x) === "(1 2 3)",
    appendBangShares: eqWords(spliced, x2) && write(memory2, x2) === "(1 2 3 4 5)",
  };
};
