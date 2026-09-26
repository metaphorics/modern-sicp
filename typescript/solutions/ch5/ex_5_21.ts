// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assign,
  branch,
  type ControllerLine,
  c,
  getRegisterContents,
  jump,
  jumpReg,
  lbl,
  type Machine,
  makeMachine,
  mark,
  op,
  reg,
  restore,
  save,
  setRegisterContents,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  cons,
  emptyList,
  listOperations,
  type Memory,
  makeMemory,
  renderWord,
  type Value,
} from "../../packages/ch5/src/03-storage.js";
import { expectOk } from "./ex_5_07.js";

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
export const countLeavesRecursive: ControllerLine[] = [
  assign("continue", lbl("count-done")),
  mark("count-loop"),
  test("null?", reg("tree")),
  branch("null-answer"),
  test("pair?", reg("tree")),
  branch("tree-case"),
  assign("val", c(1)),
  jumpReg("continue"),
  mark("null-answer"),
  assign("val", c(0)),
  jumpReg("continue"),
  mark("tree-case"),
  save("continue"),
  save("tree"),
  assign("continue", lbl("after-car")),
  assign("tree", op("car", reg("tree"))),
  jump("count-loop"),
  mark("after-car"),
  restore("tree"),
  assign("tree", op("cdr", reg("tree"))),
  save("val"),
  assign("continue", lbl("after-cdr")),
  jump("count-loop"),
  mark("after-cdr"),
  restore("temp"),
  assign("val", op("+", reg("val"), reg("temp"))),
  restore("continue"),
  jumpReg("continue"),
  mark("count-done"),
];

/** The machine of exercise 5.21b: the explicit counter. n accumulates
 * through both subcalls, so only continue and tree take stack room. */
export const countLeavesIterative: ControllerLine[] = [
  assign("n", c(0)),
  assign("continue", lbl("count-done")),
  mark("count-loop"),
  test("null?", reg("tree")),
  branch("null-case"),
  test("pair?", reg("tree")),
  branch("tree-case"),
  assign("n", op("+", reg("n"), c(1))),
  jumpReg("continue"),
  mark("null-case"),
  jumpReg("continue"),
  mark("tree-case"),
  save("continue"),
  save("tree"),
  assign("continue", lbl("after-car")),
  assign("tree", op("car", reg("tree"))),
  jump("count-loop"),
  mark("after-car"),
  restore("tree"),
  assign("tree", op("cdr", reg("tree"))),
  assign("continue", lbl("after-cdr")),
  jump("count-loop"),
  mark("after-cdr"),
  restore("continue"),
  jumpReg("continue"),
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
  controller: ReadonlyArray<ControllerLine>,
  registers: ReadonlyArray<string>,
  answerReg: string,
  tree: Tree,
): CountRun => {
  const memory = makeMemory(64);
  const machine: Machine = expectOk(
    makeMachine(registers, { ...arithmeticOperations, ...listOperations(memory) }, controller),
  );
  expectOk(setRegisterContents(machine, "tree", plantTree(memory, tree)));
  expectOk(machine.start());
  const answer = expectOk(getRegisterContents(machine, answerReg));
  return { answer: renderWord(answer), statistics: machine.stack.statisticsLine() };
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
