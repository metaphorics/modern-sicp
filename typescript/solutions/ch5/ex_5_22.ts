// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  expectOk,
  gotoLabel,
  gotoRegister,
  labelRef,
  type MachineStatement,
  makeMachine,
  mark,
  op,
  perform,
  register,
  restore,
  save,
  test,
} from "./ex_5_07.ts";
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
} from "./exercise-memory.ts";

/** Plants a proper list of numbers through the allocation path. */
export const plantList = (memory: Memory, values: ReadonlyArray<number>): Value => {
  let acc: Value = emptyList;
  for (const value of [...values].reverse()) acc = cons(memory, value, acc);
  return acc;
};

/** The append machine: it copies x and shares y; the answer lands in z. */
export const appendController: MachineStatement<Value>[] = [
  assign<Value>("continue", labelRef<Value>("append-done")),
  mark("append-loop"),
  test<Value>("null?", register<Value>("x")),
  branch<Value>("base"),
  assign<Value>("temp", op<Value>("car", register<Value>("x"))),
  save<Value>("temp"),
  save<Value>("continue"),
  assign<Value>("continue", labelRef<Value>("after-car")),
  assign<Value>("x", op<Value>("cdr", register<Value>("x"))),
  gotoLabel<Value>("append-loop"),
  mark("base"),
  assign<Value>("z", register<Value>("y")),
  gotoRegister<Value>("continue"),
  mark("after-car"),
  restore<Value>("continue"),
  restore<Value>("temp"),
  assign<Value>("z", op<Value>("cons", register<Value>("temp"), register<Value>("z"))),
  gotoRegister<Value>("continue"),
  mark("append-done"),
];

/** The append! machine: walk to the last pair of x and splice y in with
 * set-cdr!; no cell is allocated and there is no z. */
export const appendBangController: MachineStatement<Value>[] = [
  assign<Value>("temp", register<Value>("x")),
  mark("last-pair"),
  assign<Value>("cand", op<Value>("cdr", register<Value>("temp"))),
  test<Value>("null?", register<Value>("cand")),
  branch<Value>("splice"),
  assign<Value>("temp", op<Value>("cdr", register<Value>("temp"))),
  gotoLabel<Value>("last-pair"),
  mark("splice"),
  perform<Value>("set-cdr!", register<Value>("temp"), register<Value>("y")),
];

/** Both exercise runs: append over planted x and y, then append! over a
 * fresh copy, with the memory drawn before and after the splice. */
export const appendRuns = (): string[] => {
  const memory = makeMemory(16);
  const x = plantList(memory, [1, 2, 3]);
  const y = plantList(memory, [4, 5]);
  const machine = makeMachine<Value>({
    registers: ["x", "y", "z", "temp", "continue"],
    operations: listOperations(memory),
    controller: appendController,
  });
  machine.writeRegister("x", x);
  machine.writeRegister("y", y);
  expectOk(machine.run());
  const z = machine.readRegister("z");
  if (z === undefined) throw new Error("append finished without a result");
  const appendLines = [
    `append: z = ${renderWord(z)} = ${write(memory, z)}`,
    `append: x is still ${write(memory, x)} (${renderWord(x)}), free moved to ` +
      `${renderWord(pairPointer(memory.free))}, three fresh cells`,
  ];

  const memory2 = makeMemory(8);
  const x2 = plantList(memory2, [1, 2, 3]);
  const y2 = plantList(memory2, [4, 5]);
  const before = dump(memory2);
  const machine2 = makeMachine<Value>({
    registers: ["x", "y", "temp", "cand"],
    operations: listOperations(memory2),
    controller: appendBangController,
  });
  machine2.writeRegister("x", x2);
  machine2.writeRegister("y", y2);
  expectOk(machine2.run());
  const xSpliced = machine2.readRegister("x");
  if (xSpliced === undefined) throw new Error("append! finished without its input pair");
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
  const appendMachine = makeMachine<Value>({
    registers: ["x", "y", "z", "temp", "continue"],
    operations: listOperations(memory),
    controller: appendController,
  });
  appendMachine.writeRegister("x", x);
  appendMachine.writeRegister("y", y);
  expectOk(appendMachine.run());
  const copied = appendMachine.readRegister("z");

  const memory2 = makeMemory(8);
  const x2 = plantList(memory2, [1, 2, 3]);
  const y2 = plantList(memory2, [4, 5]);
  const appendBangMachine = makeMachine<Value>({
    registers: ["x", "y", "temp", "cand"],
    operations: listOperations(memory2),
    controller: appendBangController,
  });
  appendBangMachine.writeRegister("x", x2);
  appendBangMachine.writeRegister("y", y2);
  expectOk(appendBangMachine.run());
  const spliced = appendBangMachine.readRegister("x");
  return {
    appendCopies: copied !== undefined && !eqWords(copied, x) && write(memory, x) === "(1 2 3)",
    appendBangShares:
      spliced !== undefined && eqWords(spliced, x2) && write(memory2, x2) === "(1 2 3 4 5)",
  };
};
