// SPDX-License-Identifier: GPL-3.0-only
import {
  assign,
  branch,
  type ControllerLine,
  jump,
  mark,
  type Operation,
  op,
  perform,
  reg,
  save,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  EvaluatorFault,
  evaluatorController,
  type PairWord,
  type Word,
} from "../../packages/ch5/src/04-eceval.js";
import { makeCompiledEvaluator } from "../../packages/ch5/src/05-compilation.js";

/** The 5.4 ev-application with the symbol-operator fast path: a call
 * whose operator is a symbol skips the save of env and unev and the
 * dispatch of the operator expression, looking the name up directly. */
const fastApplication: readonly ControllerLine[] = [
  mark("ev-application"),
  save("continue"),
  assign("unev", op("operands", reg("exp"))),
  test("symbol-operator?", reg("exp")),
  branch("ev-appl-symbol-operator"),
  save("env"),
  save("unev"),
  assign("exp", op("operator", reg("exp"))),
  assign("continue", { tag: "label", name: "ev-appl-did-operator" }),
  jump("eval-dispatch"),
  mark("ev-appl-symbol-operator"),
  assign("exp", op("operator", reg("exp"))),
  assign("val", op("lookup-variable-value", reg("exp"), reg("env"))),
  assign("argl", op("empty-arglist")),
  assign("proc", reg("val")),
  test("no-operands?", reg("unev")),
  branch("apply-dispatch"),
  save("proc"),
  jump("ev-appl-operand-loop"),
];

/** The 5.4.4 monitored driver: the plain driver with the stack
 * printer between interactions. */
const monitoredDriver = (): readonly ControllerLine[] => {
  const start = evaluatorController.findIndex(
    (line) => line.tag === "label" && line.name === "read-eval-print-loop",
  );
  const end = evaluatorController.findIndex(
    (line) => line.tag === "label" && line.name === "eval-dispatch",
  );
  if (start < 0 || end < 0) throw new EvaluatorFault("the evaluator controller drifted");
  const driver = evaluatorController.slice(start, end);
  const printResult = driver.findIndex(
    (line) => line.tag === "label" && line.name === "print-result",
  );
  if (printResult < 0) throw new EvaluatorFault("the driver lacks print-result");
  return [
    ...driver.slice(0, printResult + 1),
    perform("print-stack-statistics"),
    ...driver.slice(printResult + 1),
  ];
};

/** The 5.4 controller with ev-application replaced by the fast path,
 * over the monitored driver. */
const fastController = (): readonly ControllerLine[] => {
  const appStart = evaluatorController.findIndex(
    (line) => line.tag === "label" && line.name === "ev-application",
  );
  const appEnd = evaluatorController.findIndex(
    (line) => line.tag === "label" && line.name === "ev-appl-did-operator",
  );
  const driverEnd = evaluatorController.findIndex(
    (line) => line.tag === "label" && line.name === "eval-dispatch",
  );
  if (appStart < 0 || appEnd < 0 || driverEnd < 0)
    throw new EvaluatorFault("the evaluator controller drifted");
  return [
    ...monitoredDriver(),
    ...evaluatorController.slice(driverEnd, appStart),
    ...fastApplication,
    ...evaluatorController.slice(appEnd),
  ];
};

interface Measurement {
  readonly pushes: number;
  readonly depth: number;
}

/** True when the operator of the combination in exp is a symbol. */
const symbolOperator = (): Record<string, Operation> => ({
  "symbol-operator?": (args) => {
    const exp = args[0] as Word;
    const operator = isPairWord(exp) ? exp.car : undefined;
    return isSymbolName(operator);
  },
});

const isPairWord = (word: Word): word is PairWord =>
  typeof word === "object" && word !== null && "car" in word && "cdr" in word;

const isSymbolName = (word: Word | undefined): boolean =>
  typeof word === "object" &&
  word !== null &&
  "symbol" in word &&
  !("car" in word) &&
  !("wordTag" in word);

/** Runs a source on the monitored driver and reads the last counters. */
const measured = (controller: readonly ControllerLine[], source: string): Measurement => {
  const evaluator = makeCompiledEvaluator(controller, source, {
    operations: symbolOperator(),
  });
  evaluator.run();
  const line = [...evaluator.transcript]
    .reverse()
    .find((text) => text.startsWith("(total-pushes = "));
  if (line === undefined) throw new EvaluatorFault("no stack statistics printed");
  const pushes = Number(line.split("total-pushes = ")[1]?.split(" ")[0]);
  const depth = Number(line.split("maximum-depth = ")[1]?.replace(")", ""));
  return { pushes, depth };
};

const FACTORIAL = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))\n(factorial 5)";

/** The design answers: the fast path only helps the operator
 * evaluation of a symbol call, so the compiler's compile-time analysis
 * is strictly better (it avoids the run-time test per call), and the
 * machine's maximum depth is unchanged because the path saves less but
 * still brackets the operand loop the same way. */
export const ex_5_32 = (): readonly string[] => {
  const base = measured(plainController(), FACTORIAL);
  const fast = measured(fastController(), FACTORIAL);
  const evaluator = makeCompiledEvaluator(
    fastController(),
    "(define (f x) (* x x))\n(f 6)\n((lambda (y) (+ y 1)) 41)",
    { operations: symbolOperator() },
  );
  evaluator.run();
  const answers = evaluator.transcript;
  if (fast.pushes >= base.pushes) throw new Error("the fast path saved nothing");
  if (fast.depth !== base.depth) throw new Error("the fast path changed the maximum depth");
  if (!answers.some((line) => line === "36")) throw new Error("symbol call broken");
  if (!answers.some((line) => line === "42")) throw new Error("compound operator broken");
  return [
    `symbol and compound-operator answers: ${answers.join(" ")}`,
    `base monitored factorial pushes/depth: ${base.pushes}/${base.depth}`,
    `fast-path factorial pushes/depth: ${fast.pushes}/${fast.depth}`,
    "design: symbol calls avoid a run-time operator-evaluation path, but every general call still pays the test; the compiler decides at compile time and needs no test at all",
  ];
};

/** The plain 5.4 controller over the monitored driver. */
const plainController = (): readonly ControllerLine[] => {
  const driverEnd = evaluatorController.findIndex(
    (line) => line.tag === "label" && line.name === "eval-dispatch",
  );
  if (driverEnd < 0) throw new EvaluatorFault("the evaluator controller drifted");
  return [...monitoredDriver(), ...evaluatorController.slice(driverEnd)];
};
