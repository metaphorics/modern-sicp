// SPDX-License-Identifier: GPL-3.0-only
import { readFileSync } from "node:fs";
import {
  compileAndGo,
  defaultConfig,
  makeCompiledEvaluator,
  monitoredEcevalController,
  newState,
} from "../../packages/ch5/src/05-compilation.js";

const FACTORIAL = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

const metacircularSource = (): string =>
  readFileSync(new URL("../../packages/ch5/src/metacircular.scm", import.meta.url), "utf8");

const metacircularSession = (): { transcript: readonly string[]; steps: number } => {
  const evaluator = compileAndGo(
    defaultConfig(),
    newState(),
    metacircularSource(),
    "(m-eval '(factorial 5) the-global-environment)",
    { stepLimit: 50_000_000 },
  );
  evaluator.run();
  return { transcript: evaluator.transcript, steps: evaluator.instructionCount() };
};

const level0Steps = (n: number): number => {
  const evaluator = compileAndGo(defaultConfig(), newState(), FACTORIAL, `(factorial ${n})`);
  evaluator.run();
  return evaluator.instructionCount();
};

const level1Pushes = (n: number): number => {
  const evaluator = makeCompiledEvaluator(
    monitoredEcevalController,
    `${FACTORIAL}\n(factorial ${n})`,
  );
  evaluator.run();
  return [...evaluator.transcript]
    .filter((line) => line.startsWith("(total-pushes = "))
    .map((line) => Number(line.split("total-pushes = ")[1]?.split(" ")[0]))
    .reduce((sum, pushes) => sum + pushes, 0);
};

/** Exercise 5.50: the metacircular evaluator compiled and run on the
 * 5.5.7 machine answers ok, 120, and (tick tick tick); the driver
 * then runs an interpreted factorial through the compiled
 * interpreter. The three measurements price each interpretation
 * level: compiled machine steps, interpreted stack pushes, and the
 * compiled metacircular's machine steps for the same computation. */
export const ex_5_50 = (): readonly string[] => {
  const { transcript, steps } = metacircularSession();
  if (!transcript.includes("120")) throw new Error("the compiled metacircular lost 120");
  if (!transcript.includes("(tick tick tick)")) throw new Error("the tick session is missing");
  const l0 = level0Steps(5);
  const l1 = level1Pushes(5);
  const price = `${steps}/${l0}`;
  return [
    `compiled metacircular session: ${transcript.join(" ")}`,
    `level 0 (compiled factorial), machine steps = ${l0}`,
    `level 1 (interpreted factorial), monitored pushes = ${l1}`,
    `level 2 (compiled metacircular), machine steps = ${steps}`,
    `interpretation price: level 2 over level 0 = ${price} machine steps`,
  ];
};
