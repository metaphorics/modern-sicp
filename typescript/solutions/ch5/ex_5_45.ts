// SPDX-License-Identifier: GPL-3.0-only
import { factorialRecursive, runMachine } from "../../packages/ch5/src/01-register-machines.js";
import {
  compileBlock,
  defaultConfig,
  makeCompiledEvaluator,
  monitoredEcevalController,
  newState,
} from "../../packages/ch5/src/05-compilation.js";

const FACTORIAL = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

interface Counters {
  readonly pushes: number;
  readonly depth: number;
}

const counters = (lines: readonly string[]): Counters => {
  const line = [...lines].reverse().find((text) => text.startsWith("(total-pushes"));
  if (line === undefined) throw new Error("missing stack statistics");
  const pushes = Number(line.split("total-pushes = ")[1]?.split(" ")[0]);
  const depth = Number(line.split("maximum-depth = ")[1]?.replace(")", ""));
  return { pushes, depth };
};

const interpretedAt = (n: number): Counters => {
  const evaluator = makeCompiledEvaluator(
    monitoredEcevalController,
    `${FACTORIAL}\n(factorial ${n})`,
  );
  evaluator.run();
  return counters(evaluator.transcript);
};

const compiledAt = (n: number): Counters => {
  const { entry, lines } = compileBlock(defaultConfig(), newState(), FACTORIAL);
  const evaluator = makeCompiledEvaluator(
    [...monitoredEcevalController, ...lines],
    `(factorial ${n})`,
  );
  evaluator.armEntry(entry);
  evaluator.run();
  return counters(evaluator.transcript);
};

const specialAt = (n: number): Counters => {
  const run = runMachine(factorialRecursive, { n });
  if (!run.ok) throw new Error("the special-purpose machine failed");
  const pushes = run.value.events.filter((event) => event.tag === "save").length;
  return { pushes, depth: run.value.maxDepth };
};

/** Exercise 5.45: the same recursive factorial on three machines. The
 * book's numbers at n = 5 are interpreted 144/28, compiled 31/14, and
 * special-purpose 8/8; the compiled code pays a fixed small frame per
 * call, the interpreted evaluator a much larger one. */
export const ex_5_45 = (): readonly string[] => {
  const rows: string[] = [];
  for (const n of [5, 10]) {
    const interpreted = interpretedAt(n);
    const compiled = compiledAt(n);
    const special = specialAt(n);
    rows.push(
      `n=${n}: interpreted ${interpreted.pushes}/${interpreted.depth}, compiled ${compiled.pushes}/${compiled.depth}, special-purpose ${special.pushes}/${special.depth}`,
    );
  }
  if (!rows[0]?.includes("interpreted 144/28")) throw new Error(rows[0]);
  if (!rows[0]?.includes("compiled 31/14")) throw new Error(rows[0]);
  if (!rows[0]?.includes("special-purpose 8/8")) throw new Error(rows[0]);
  return rows;
};
