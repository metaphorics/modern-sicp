// SPDX-License-Identifier: GPL-3.0-only
import { type ControllerLine, mark } from "../../packages/ch5/src/02-simulator.js";
import { EvaluatorFault } from "../../packages/ch5/src/04-eceval.js";
import {
  bumpEntry,
  type CompilerState,
  compileForms,
  defaultConfig,
  ecevalController,
  LinkageReturn,
  makeCompiledEvaluator,
  newState,
  type RuntimeFn,
} from "../../packages/ch5/src/05-compilation.js";

const DEFINITION = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

interface Block {
  readonly entry: string;
  readonly lines: readonly ControllerLine[];
}

/** The compile-and-run primitive: it compiles its quoted expression
 * into a block and answers ok; the block joins the controller at the
 * next assembly, the two-phase interface this edition pins. */
const compileAndRunPrimitive = (
  state: CompilerState,
  blocks: Block[],
): Record<string, RuntimeFn> => ({
  "compile-and-run": (args) => {
    const expression = args[0];
    if (expression === undefined) throw new EvaluatorFault("compile-and-run needs one expression");
    const seq = compileForms(defaultConfig(), state, [expression], LinkageReturn);
    const entry = `compiled-entry-run-${bumpEntry(state)}`;
    blocks.push({ entry, lines: [mark(entry), ...seq.stmts] });
    return { symbol: "ok" };
  },
});

/** Exercise 5.48: a primitive compiles its quoted form and records a
 * block; the machine that runs the recorded block answers the
 * definition and the interpreted call to it. */
export const ex_5_48 = (): readonly string[] => {
  const state = newState();
  const blocks: Block[] = [];
  const first = makeCompiledEvaluator(undefined, `(compile-and-run '${DEFINITION})`, {
    runtime: compileAndRunPrimitive(state, blocks),
  });
  first.run();
  const firstTranscript = first.transcript;
  if (!firstTranscript.includes("ok")) throw new Error("compile-and-run did not answer ok");
  const block = blocks[0];
  if (block === undefined) throw new Error("compile-and-run recorded no code");
  const second = makeCompiledEvaluator([...ecevalController, ...block.lines], "(factorial 5)");
  second.armEntry(block.entry);
  second.run();
  const secondTranscript = second.transcript;
  if (!secondTranscript.includes("120")) throw new Error("the recorded block never ran");
  return [
    `compile-and-run primitive: ${firstTranscript.join(" ")}`,
    `compiled definition and call: ${secondTranscript.join(" ")}`,
  ];
};
