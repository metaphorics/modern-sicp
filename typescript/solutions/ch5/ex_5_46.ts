// SPDX-License-Identifier: GPL-3.0-only
import { fibonacciRecursive, runMachine } from "../../packages/ch5/src/01-register-machines.js";
import {
  compileBlock,
  defaultConfig,
  makeCompiledEvaluator,
  monitoredEcevalController,
  newState,
} from "../../packages/ch5/src/05-compilation.js";

const FIB = "(define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))";

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
  const evaluator = makeCompiledEvaluator(monitoredEcevalController, `${FIB}\n(fib ${n})`);
  evaluator.run();
  return counters(evaluator.transcript);
};

const compiledAt = (n: number): Counters => {
  const { entry, lines } = compileBlock(defaultConfig(), newState(), FIB);
  const evaluator = makeCompiledEvaluator([...monitoredEcevalController, ...lines], `(fib ${n})`);
  evaluator.armEntry(entry);
  evaluator.run();
  return counters(evaluator.transcript);
};

const specialAt = (n: number): Counters => {
  const run = runMachine(fibonacciRecursive, { n });
  if (!run.ok) throw new Error("the special-purpose machine failed");
  const pushes = run.value.events.filter((event) => event.tag === "save").length;
  return { pushes, depth: run.value.maxDepth };
};

/** Exercise 5.46: Fibonacci's call tree grows exponentially, so the
 * total pushes grow with the number of calls in all three machines;
 * the ratio of interpreted to compiled stays around five, the
 * compiler's constant advantage per call. */
export const ex_5_46 = (): readonly string[] => {
  const rows: string[] = [];
  for (const n of [5, 6, 7]) {
    const interpreted = interpretedAt(n);
    const compiled = compiledAt(n);
    const special = specialAt(n);
    rows.push(
      `n=${n}: interpreted ${interpreted.pushes}/${interpreted.depth}, compiled ${compiled.pushes}/${compiled.depth}, special-purpose ${special.pushes}/${special.depth}`,
    );
  }
  if (rows.length !== 3) throw new Error("missing measurement rows");
  if (!rows[2]?.startsWith("n=7")) throw new Error(rows[2]);
  const ratio = (row: string): number => {
    const interpreted = Number(row.split("interpreted ")[1]?.split("/")[0]);
    const compiled = Number(row.split("compiled ")[1]?.split("/")[0]);
    return interpreted / compiled;
  };
  for (const row of rows) {
    const r = ratio(row);
    if (!(r > 3 && r < 10)) throw new Error(`the ratio left the expected band: ${row}`);
  }
  return rows;
};
