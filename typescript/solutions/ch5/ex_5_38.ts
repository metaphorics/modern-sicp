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

/** The session's final answer: the element after the last
 * ";;; EC-Eval value:" marker, so a pin matches the printed value
 * exactly instead of accepting it inside an earlier or longer line. */
const finalValue = (transcript: readonly string[]): string => {
  const marker = transcript.lastIndexOf(";;; EC-Eval value:");
  const value = marker >= 0 ? transcript[marker + 1] : undefined;
  if (value === undefined) throw new Error("the session printed no value");
  return value;
};

/** (a) spreads two operands into arg1 and arg2 and applies the
 * machine's own operation; (b) the open-coded factorial is roughly
 * half the size; (c) and (d) are the measured sessions: the n-ary
 * forms fold through one register, the comparison predicate works,
 * and the reproducers where an operand is itself a call answer
 * correctly only because every operand register is shielded across
 * the later operand code, the accumulator is shielded across call
 * operands, the fold's result reaches the requested target, and env
 * survives compound-call operands. A compound-call operand's compiled
 * body rewrites arg1 and arg2 as its own scratch, so those shields
 * key on the call's declared clobbers. */
export const ex_5_38 = (): readonly string[] => {
  const plainStatements = count(FACTORIAL, defaultConfig());
  const openStatements = count(FACTORIAL, openCfg(false));
  if (openStatements * 2 > plainStatements * 3)
    throw new Error("the open-coded factorial is not much smaller");
  const nary = compileAndGo(openCfg(true), newState(), "(define (go) (+ 1 2 3 4))", "(go)");
  nary.run();
  if (finalValue(nary.transcript) !== "10") throw new Error("n-ary addition broken");
  const less = compileAndGo(openCfg(true), newState(), "(define (go) (< 1 2))", "(go)");
  less.run();
  if (finalValue(less.transcript) !== "#t") throw new Error("the comparison is not open-coded");
  const nested = compileAndGo(
    openCfg(true),
    newState(),
    "(define (go) (+ (* 2 3) (+ 4 5)))",
    "(go)",
  );
  nested.run();
  if (finalValue(nested.transcript) !== "15") throw new Error("nested open-coded operands broken");
  const callOperand = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f) 40)\n(define (go) (+ 1 2 (f)))",
    "(go)",
  );
  callOperand.run();
  if (finalValue(callOperand.transcript) !== "43")
    throw new Error("accumulator clobbered by an operand");
  const inner = compileAndGo(openCfg(true), newState(), "(define (go) (+ (+ 1 2 3) 4))", "(go)");
  inner.run();
  if (finalValue(inner.transcript) !== "10") throw new Error("nested n-ary broken");
  const mixed = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f x) (+ (h x) x 1))\n(define (h y) (* y 10))",
    "(f 4)",
  );
  mixed.run();
  if (finalValue(mixed.transcript) !== "45") throw new Error("env lost across a compound operand");
  const mixed2 = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f x) (+ (h x) x))\n(define (h y) (* y 10))",
    "(f 4)",
  );
  mixed2.run();
  if (finalValue(mixed2.transcript) !== "44") throw new Error("the mixed fold lost its sum");
  // The compound-call operand pins: a call operand's compiled body
  // uses arg1 and arg2 as its own open-coded scratch, and the call
  // sequence declares those writes, so an earlier operand survives
  // only through its shield. With f(y) = (* y 10) and x = 4, the
  // two-operand spread answers 14, the n-ary fold 17.
  const operandCall = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f y) (* y 10))\n(define x 4)\n(define (go) (+ x (f 1)))",
    "(go)",
  );
  operandCall.run();
  if (finalValue(operandCall.transcript) !== "14")
    throw new Error("the call operand destroyed the earlier arg1");
  const operandCallNary = compileAndGo(
    openCfg(true),
    newState(),
    "(define (f y) (* y 10))\n(define x 4)\n(define (go) (+ x (f 1) 3))",
    "(go)",
  );
  operandCallNary.run();
  if (finalValue(operandCallNary.transcript) !== "17")
    throw new Error("the call operand destroyed the fold's first value");
  // Two nesting depths of mixed interpreted calls inside open-coded
  // operands: the inner folds answer 15 and 105 and the outer fold 120.
  const deepSource =
    "(define (f y) (* y 10))\n(define (g y) (+ y 100))" +
    "\n(define (go) (+ (+ (f 1) (+ 2 3)) (+ (* 2 2) (g 1))))";
  const deepNest = compileAndGo(openCfg(true), newState(), deepSource, "(go)");
  deepNest.run();
  if (finalValue(deepNest.transcript) !== "120")
    throw new Error("the nested call operands lost a value");
  // The n-ary second-operand shield pin: the fold's second operand
  // rewrites arg1 while spreading its own operands, so the first
  // operand's result survives only behind the shield. (A two-operand
  // outer combination is shielded by the spread instead, so the pin
  // needs the third operand to hold the fold open.)
  const secondShield = compileAndGo(
    openCfg(true),
    newState(),
    "(define (go) (+ (+ 1 2 3) (+ 4 5) 7))",
    "(go)",
  );
  secondShield.run();
  if (finalValue(secondShield.transcript) !== "22")
    throw new Error("the second operand clobbered arg1");
  return [
    `plain compilation: ${plainStatements} statements; open-coded: ${openStatements} statements`,
    "(+ 1 2 3 4): 10; (< 1 2): #t",
    "(+ (* 2 3) (+ 4 5)): 15; (+ 1 2 (f)) with f answering 40: 43; (+ (+ 1 2 3) 4): 10",
    "with h(y) = (* y 10) and x = 4: (+ (h x) x 1): 45 and (+ (h x) x): 44",
    "with f(y) = (* y 10) and x = 4: (+ x (f 1)): 14; (+ x (f 1) 3): 17",
    "(+ (+ (f 1) (+ 2 3)) (+ (* 2 2) (g 1))) with f(y) = (* y 10) and g(y) = (+ y 100): 120",
    "(+ (+ 1 2 3) (+ 4 5) 7): 22",
    "the open-coding paths shield each operand register across the later operand code, shield the fold's accumulator across call operands, copy the fold's answer into the requested target, and preserve env around compound-call operands",
  ];
};
