// SPDX-License-Identifier: GPL-3.0-only
import {
  assign,
  type ControllerLine,
  jump,
  mark,
  perform,
} from "../../packages/ch5/src/02-simulator.js";
import {
  compileBlock,
  controllerReplacingDriver,
  defaultConfig,
  guardedDriver,
  makeCompiledEvaluator,
  newState,
} from "../../packages/ch5/src/05-compilation.js";

/** The chained driver of 5.49: after the armed guard, the machine
 * walks one compiled block per form, printing each value; no
 * evaluator dispatch runs the forms. */
const chainDriver = (entries: readonly string[]): readonly ControllerLine[] => {
  const guard = guardedDriver.slice(0, 2);
  const lines: ControllerLine[] = [
    ...guard,
    mark("read-eval-print-loop"),
    assign("env", { tag: "op", op: "get-global-environment", args: [] }),
    jump("chain-start"),
    mark("print-result"),
    jump("read-eval-print-loop"),
    mark("chain-start"),
  ];
  entries.forEach((entry, index) => {
    lines.push(
      perform("prompt-for-input", { tag: "const", value: { symbol: ";;; Compiled input:" } }),
      assign("continue", { tag: "label", name: `print-${index}` }),
      jump(entry),
      mark(`print-${index}`),
      perform("announce-output", { tag: "const", value: { symbol: ";;; Compiled value:" } }),
      perform("user-print", { tag: "reg", name: "val" }),
    );
  });
  lines.push(jump("machine-end"));
  return lines;
};

const FORMS = [
  "(define (square n) (* n n))",
  "(square 12)",
  "(define (twice n) (+ n n))",
  "(twice 441)",
] as const;

/** Exercise 5.49: each form is compiled once and executed by a
 * controller chain; the transcript is the book's prompt and value
 * lines, with the read step replaced by the compiler. */
export const ex_5_49 = (): readonly string[] => {
  const state = newState();
  const blocks = FORMS.map((form) => compileBlock(defaultConfig(), state, form));
  const entries = blocks.map((block) => block.entry);
  // machine-end sits after every block: the goto halts the run loop
  // instead of falling through into the appended code.
  const controller = [
    ...controllerReplacingDriver(chainDriver(entries)),
    ...blocks.flatMap((block) => block.lines),
    mark("machine-end"),
  ];
  const evaluator = makeCompiledEvaluator(controller, "");
  evaluator.run();
  const transcript = evaluator.transcript;
  const values = transcript.filter((line) => !line.startsWith(";;;"));
  if (values.join(",") !== "ok,144,ok,882")
    throw new Error(`the compiled loop produced ${values.join(",")}`);
  return [`compiled loop: ${transcript.join(" ")}`];
};
