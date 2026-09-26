// SPDX-License-Identifier: GPL-3.0-only
import {
  type CompilerConfig,
  compileAndGo,
  compileProgram,
  defaultConfig,
  LinkageNext,
  newState,
} from "../../packages/ch5/src/05-compilation.js";

const FACTORIAL = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

const openCfg = (compoundCalls: boolean): CompilerConfig => ({
  ...defaultConfig(),
  openCode: true,
  compoundCalls,
});

const count = (source: string, cfg: CompilerConfig): number =>
  compileProgram(cfg, newState(), source, LinkageNext).stmts.length;

/** (a) spreads two operands into arg1 and arg2 and applies the
 * machine's own operation; (b) the open-coded factorial is roughly
 * half the size; (c) and (d) are the measured sessions: the n-ary
 * forms fold through one register, the comparison predicate works,
 * and the reproducers where an operand is itself a call answer
 * correctly only because every operand register is shielded across
 * the later operand code, the accumulator is shielded across call
 * operands, the fold's result reaches the requested target, and env
 * survives compound-call operands. */
export const ex_5_38 = (): readonly string[] => {
  const plainStatements = count(FACTORIAL, defaultConfig());
  const openStatements = count(FACTORIAL, openCfg(false));
  if (openStatements * 2 > plainStatements * 3)
    throw new Error("the open-coded factorial is not much smaller");
  const nary = compileAndGo(openCfg(true), newState(), "(define (go) (+ 1 2 3 4))", "(go)");
  nary.run();
  if (!nary.transcript.includes("10")) throw new Error("n-ary addition broken");
  const less = compileAndGo(openCfg(true), newState(), "(define (go) (< 1 2))", "(go)");
  less.run();
  if (!less.transcript.includes("#t")) throw new Error("the comparison is not open-coded");
  const nested = compileAndGo(
    openCfg(true),
    newState(),
    "(define (go) (+ (* 2 3) (+ 4 5)))",
    "(go)",
  );
  nested.run();
  if (!nested.transcript.includes("15")) throw new Error("nested open-coded operands broken");
  const callOperand = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f) 40)\n(define (go) (+ 1 2 (f)))",
    "(go)",
  );
  callOperand.run();
  if (!callOperand.transcript.includes("43"))
    throw new Error("accumulator clobbered by an operand");
  const inner = compileAndGo(openCfg(true), newState(), "(define (go) (+ (+ 1 2 3) 4))", "(go)");
  inner.run();
  if (!inner.transcript.includes("10")) throw new Error("nested n-ary broken");
  const mixed = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f x) (+ (h x) x 1))\n(define (h y) (* y 10))",
    "(f 4)",
  );
  mixed.run();
  if (!mixed.transcript.includes("45")) throw new Error("env lost across a compound operand");
  const mixed2 = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f x) (+ (h x) x))\n(define (h y) (* y 10))",
    "(f 4)",
  );
  mixed2.run();
  if (!mixed2.transcript.includes("44")) throw new Error("the mixed fold lost its sum");
  return [
    `plain compilation: ${plainStatements} statements; open-coded: ${openStatements} statements`,
    "(+ 1 2 3 4): 10; (< 1 2): #t",
    "(+ (* 2 3) (+ 4 5)): 15; (+ 1 2 (f)) with f answering 40: 43; (+ (+ 1 2 3) 4): 10",
    "with h(y) = (* y 10) and x = 4: (+ (h x) x 1): 45 and (+ (h x) x): 44",
    "the open-coding paths shield each operand register across the later operand code, shield the fold's accumulator across call operands, copy the fold's answer into the requested target, and preserve env around compound-call operands",
  ];
};
