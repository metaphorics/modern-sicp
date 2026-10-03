// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  constant,
  expectOk,
  gotoLabel,
  gotoRegister,
  labelRef,
  type Machine,
  type MachineStatement,
  makeMachine,
  mark,
  op,
  register,
  restore,
  save,
  test,
} from "./ex_5_07.ts";
import {
  cons,
  emptyList,
  listOperations,
  type Memory,
  makeMemory,
  renderWord,
  type Value,
} from "./exercise-memory.ts";

/** The object-language tree the machines count: a leaf is a number, a
 * node a list of subtrees, e0-terminated, planted as the book's pointers. */
export type Tree =
  | { readonly tag: "leaf"; readonly n: number }
  | { readonly tag: "node"; readonly subtrees: ReadonlyArray<Tree> };

/** Plants the tree's cells through the same allocation path the
 * machine's `cons` uses, and answers the pointer to the root. */
export const plantTree = (memory: Memory, tree: Tree): Value => {
  if (tree.tag === "leaf") return tree.n;
  let acc: Value = emptyList;
  for (const sub of [...tree.subtrees].reverse()) acc = cons(memory, plantTree(memory, sub), acc);
  return acc;
};

/** The machine of exercise 5.21a: pure recursion. The car answer waits
 * in val while the cdr is counted, and the two answers combine with +. */
export const countLeavesRecursive: MachineStatement<Value>[] = [
  assign<Value>("continue", labelRef<Value>("count-done")),
  mark("count-loop"),
  test<Value>("null?", register<Value>("tree")),
  branch<Value>("null-answer"),
  test<Value>("pair?", register<Value>("tree")),
  branch<Value>("tree-case"),
  assign<Value>("val", constant<Value>(1)),
  gotoRegister<Value>("continue"),
  mark("null-answer"),
  assign<Value>("val", constant<Value>(0)),
  gotoRegister<Value>("continue"),
  mark("tree-case"),
  save<Value>("continue"),
  save<Value>("tree"),
  assign<Value>("continue", labelRef<Value>("after-car")),
  assign<Value>("tree", op<Value>("car", register<Value>("tree"))),
  gotoLabel<Value>("count-loop"),
  mark("after-car"),
  restore<Value>("tree"),
  assign<Value>("tree", op<Value>("cdr", register<Value>("tree"))),
  save<Value>("val"),
  assign<Value>("continue", labelRef<Value>("after-cdr")),
  gotoLabel<Value>("count-loop"),
  mark("after-cdr"),
  assign<Value>("temp", register<Value>("val")),
  restore<Value>("val"),
  assign<Value>("val", op<Value>("+", register<Value>("val"), register<Value>("temp"))),
  restore<Value>("continue"),
  gotoRegister<Value>("continue"),
  mark("count-done"),
];

/** The machine of exercise 5.21b: the explicit counter. n accumulates
 * through both subcalls, so only continue and tree take stack room. */
export const countLeavesIterative: MachineStatement<Value>[] = [
  assign<Value>("n", constant<Value>(0)),
  assign<Value>("continue", labelRef<Value>("count-done")),
  mark("count-loop"),
  test<Value>("null?", register<Value>("tree")),
  branch<Value>("null-case"),
  test<Value>("pair?", register<Value>("tree")),
  branch<Value>("tree-case"),
  assign<Value>("n", op<Value>("+", register<Value>("n"), constant<Value>(1))),
  gotoRegister<Value>("continue"),
  mark("null-case"),
  gotoRegister<Value>("continue"),
  mark("tree-case"),
  save<Value>("continue"),
  save<Value>("tree"),
  assign<Value>("continue", labelRef<Value>("after-car")),
  assign<Value>("tree", op<Value>("car", register<Value>("tree"))),
  gotoLabel<Value>("count-loop"),
  mark("after-car"),
  restore<Value>("tree"),
  assign<Value>("tree", op<Value>("cdr", register<Value>("tree"))),
  assign<Value>("continue", labelRef<Value>("after-cdr")),
  gotoLabel<Value>("count-loop"),
  mark("after-cdr"),
  restore<Value>("continue"),
  gotoRegister<Value>("continue"),
  mark("count-done"),
];

/** The host oracle: the statement's definition, read directly. */
export const countLeavesHost = (tree: Tree): number =>
  tree.tag === "leaf" ? 1 : tree.subtrees.reduce((sum, sub) => sum + countLeavesHost(sub), 0);

/** One planted run: the answer word rendered in the book's pointer
 * notation, and the monitored stack statistics of the run. */
interface CountRun {
  readonly answer: string;
  readonly statistics: string;
}

const runCountLeaves = (
  controller: ReadonlyArray<MachineStatement<Value>>,
  registers: ReadonlyArray<string>,
  answerReg: string,
  tree: Tree,
): CountRun => {
  const memory = makeMemory(64);
  const machine: Machine<Value> = makeMachine<Value>({
    registers,
    operations: listOperations(memory),
    controller,
  });
  machine.writeRegister("tree", plantTree(memory, tree));
  const run = machine.run();
  expectOk(run);
  const answer = machine.readRegister(answerReg);
  if (answer === undefined) throw new Error("the count-leaves machine left no answer");
  return {
    answer: renderWord(answer),
    statistics: `(total-pushes = ${run.stackStats.pushes} maximum-depth = ${run.stackStats.maxDepth})`,
  };
};

/** The statement's trees, drawn as the book writes them. */
const treeName = (tree: Tree): string => {
  if (tree.tag === "leaf") return String(tree.n);
  return `(${tree.subtrees.map(treeName).join(" ")})`;
};

/** Both machines over three planted trees, each line reading the two
 * answers beside the oracle's, with each machine's stack use. */
export const countLeavesRuns = (): string[] => {
  const trees: ReadonlyArray<Tree> = [
    {
      tag: "node",
      subtrees: [
        { tag: "leaf", n: 1 },
        { tag: "leaf", n: 2 },
        {
          tag: "node",
          subtrees: [
            { tag: "leaf", n: 3 },
            {
              tag: "node",
              subtrees: [
                { tag: "leaf", n: 4 },
                { tag: "leaf", n: 5 },
              ],
            },
          ],
        },
      ],
    },
    { tag: "node", subtrees: [{ tag: "node", subtrees: [{ tag: "leaf", n: 7 }] }] },
    { tag: "node", subtrees: [] },
  ];
  return trees.map((tree) => {
    const recursive = runCountLeaves(
      countLeavesRecursive,
      ["tree", "val", "temp", "continue"],
      "val",
      tree,
    );
    const iterative = runCountLeaves(countLeavesIterative, ["tree", "n", "continue"], "n", tree);
    const oracle = countLeavesHost(tree);
    return (
      `${treeName(tree)}: recursive ${recursive.answer}, iterative ${iterative.answer}, oracle ${oracle}; ` +
      `recursive stack ${recursive.statistics}, iterative stack ${iterative.statistics}`
    );
  });
};
