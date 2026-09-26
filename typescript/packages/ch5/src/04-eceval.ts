// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.4

import {
  assemble,
  assign,
  c as baseConst,
  branch,
  type ControllerLine,
  jump,
  jumpReg,
  lbl,
  type Machine,
  makeNewMachine,
  mark,
  type Operation,
  op,
  perform,
  reg,
  restore,
  save,
  test,
  type Value,
} from "./02-simulator.js";

/** A machine word used by the evaluator. Internal words have a reserved tag. */
const c = (value: Value | { readonly label: string }) =>
  typeof value === "object" && value !== null && "label" in value
    ? lbl(value.label)
    : baseConst(value);
export type Word = Value | PairWord | TaggedWord;
export interface PairWord {
  readonly symbol: "pair";
  readonly car: Word;
  readonly cdr: Word;
}
export interface TaggedWord {
  readonly symbol: string;
  readonly wordTag: string;
  readonly payload: unknown;
}
export const nil: Value = { symbol: "nil" };
const symbol = (name: string): Value => ({ symbol: name });
const pair = (car: Word, cdr: Word): PairWord => ({ symbol: "pair", car, cdr });
const isPair = (v: Word): v is PairWord =>
  typeof v === "object" && v !== null && "car" in v && "cdr" in v;
const isNil = (v: Word): boolean =>
  typeof v === "object" && v !== null && "symbol" in v && v.symbol === "nil";
const list = (xs: readonly Word[]): Word =>
  xs.reduceRight((tail, item) => pair(item, tail), nil as Word);
const items = (v: Word): Word[] => {
  const out: Word[] = [];
  let p = v;
  while (isPair(p)) {
    out.push(p.car);
    p = p.cdr;
  }
  if (!isNil(p)) throw new EvaluatorFault("expected a proper list");
  return out;
};
const tagged = (tag: string, payload: unknown): TaggedWord => ({
  symbol: `sicp-word:${tag}`,
  wordTag: tag,
  payload,
});
const isTagged = (v: Word, tag: string): v is TaggedWord =>
  typeof v === "object" && v !== null && "wordTag" in v && v.wordTag === tag;
const render = (v: Word): string => {
  if (isTagged(v, "primitive")) return `#[primitive ${(v.payload as { name: string }).name}]`;
  if (isTagged(v, "procedure")) return "#[compound-procedure]";
  if (isTagged(v, "condition")) return String((v.payload as { detail: string }).detail);
  if (isNil(v)) return "()";
  if (isPair(v)) {
    const xs: string[] = [];
    let p: Word = v;
    while (isPair(p)) {
      xs.push(render(p.car));
      p = p.cdr;
    }
    return isNil(p) ? `(${xs.join(" ")})` : `(${xs.join(" ")} . ${render(p)})`;
  }
  if (typeof v === "object" && "symbol" in v) return v.symbol;
  if (typeof v === "boolean") return v ? "#t" : "#f";
  return String(v);
};

export class EvaluatorFault extends Error {
  constructor(message: string) {
    super(message);
    this.name = "EvaluatorFault";
  }
}
export const INPUT_EXHAUSTED = "the evaluator's input queue is empty";

const tokenize = (source: string): string[] => {
  const out: string[] = [];
  let i = 0;
  while (i < source.length) {
    const ch = source[i] ?? "";
    if (/\s/.test(ch)) {
      i += 1;
      continue;
    }
    if (ch === ";") {
      while (i < source.length && (source[i] ?? "") !== "\n") i += 1;
      continue;
    }
    if ("()'".includes(ch)) {
      out.push(ch);
      i += 1;
      continue;
    }
    if (ch === '"') {
      let s = "";
      i += 1;
      while (i < source.length && (source[i] ?? "") !== '"') {
        const current = source[i] ?? "";
        if (current === "\\") {
          i += 1;
          s += source[i] ?? "";
        } else s += current;
        i += 1;
      }
      if ((source[i] ?? "") !== '"') throw new EvaluatorFault("unterminated string");
      i += 1;
      out.push(JSON.stringify(s));
      continue;
    }
    let j = i;
    while (j < source.length && !/\s/.test(source[j] ?? "") && !"()'".includes(source[j] ?? ""))
      j += 1;
    out.push(source.slice(i, j));
    i = j;
  }
  return out;
};
const readDatum = (tokens: string[], at: { n: number }): Word => {
  const t = tokens[at.n];
  if (t === undefined) throw new EvaluatorFault("unexpected end of input");
  at.n += 1;
  if (t === "'") return list([symbol("quote"), readDatum(tokens, at)]);
  if (t === "(") {
    const xs: Word[] = [];
    while (tokens[at.n] !== ")") {
      if (tokens[at.n] === undefined) throw new EvaluatorFault("missing right parenthesis");
      xs.push(readDatum(tokens, at));
    }
    at.n += 1;
    return list(xs);
  }
  if (t === ")") throw new EvaluatorFault("unexpected right parenthesis");
  if (t === "#t" || t === "true") return true;
  if (t === "#f" || t === "false") return false;
  if (/^-?(?:\d+\.?\d*|\.\d+)$/.test(t)) return Number(t);
  if (t.startsWith('"')) return symbol(t.slice(1, -1));
  return symbol(t);
};
export const readProgram = (source: string): Word[] => {
  const ts = tokenize(source),
    at = { n: 0 },
    out: Word[] = [];
  while (at.n < ts.length) out.push(readDatum(ts, at));
  return out;
};

interface Frame {
  readonly bindings: Map<string, Word>;
  readonly parent: number | null;
}
interface State {
  readonly frames: Frame[];
  readonly input: Word[];
  readonly output: string[];
  machine?: Machine;
}
const envWord = (index: number): TaggedWord => tagged("environment", index);
const envIndex = (w: Word): number => {
  if (!isTagged(w, "environment") || typeof w.payload !== "number")
    throw new EvaluatorFault("expected an environment word");
  return w.payload;
};
const primitiveWord = (name: string): TaggedWord => tagged("primitive", { name });
const procedureWord = (params: string[], body: Word[], env: Word): TaggedWord =>
  tagged("procedure", { params, body, env: envIndex(env) });
const conditionWord = (kind: string, detail: string): TaggedWord =>
  tagged("condition", { kind, detail });

const nameOf = (v: Word): string => {
  if (typeof v === "object" && v !== null && "symbol" in v && !("wordTag" in v) && !isPair(v))
    return v.symbol;
  throw new EvaluatorFault("expected a variable");
};
const taggedForm = (name: string, v: Word): v is PairWord =>
  isPair(v) &&
  !isPair(v.car) &&
  typeof v.car === "object" &&
  "symbol" in v.car &&
  v.car.symbol === name;
const nth = (v: Word, n: number): Word => {
  const xs = items(v);
  const value = xs[n];
  if (value === undefined) throw new EvaluatorFault("malformed form");
  return value;
};
const _seqWord = (body: Word[]): Word =>
  body.length === 1 ? (body[0] ?? nil) : list([symbol("begin"), ...body]);

const baseOperations = (state: State): Record<string, Operation> => {
  const one =
    (name: string, f: (a: Word) => Word): Operation =>
    (args) => {
      if (args.length !== 1) throw new EvaluatorFault(`${name} needs one argument`);
      return f(args[0] as Word) as Value;
    };
  const two =
    (name: string, f: (a: Word, b: Word) => Word): Operation =>
    (args) => {
      if (args.length !== 2) throw new EvaluatorFault(`${name} needs two arguments`);
      return f(args[0] as Word, args[1] as Word) as Value;
    };
  const three =
    (name: string, f: (a: Word, b: Word, c: Word) => Word): Operation =>
    (args) => {
      if (args.length !== 3) throw new EvaluatorFault(`${name} needs three arguments`);
      return f(args[0] as Word, args[1] as Word, args[2] as Word) as Value;
    };
  const bool = (x: boolean): Value => x;
  const global = state.frames[0];
  if (!global) throw new EvaluatorFault("missing global frame");
  const find = (env: Word, name: string): Frame => {
    let i = envIndex(env);
    while (true) {
      const f = state.frames[i];
      if (!f) throw new EvaluatorFault("bad environment index");
      if (f.bindings.has(name)) return f;
      if (f.parent === null) throw new EvaluatorFault(`unbound variable: ${name}`);
      i = f.parent;
    }
  };
  const syntax: Record<string, Operation> = {
    "self-evaluating?": one("self-evaluating?", (w) =>
      bool(typeof w === "number" || typeof w === "string" || typeof w === "boolean"),
    ),
    "variable?": one("variable?", (w) =>
      bool(
        typeof w === "object" &&
          !isPair(w) &&
          !isTagged(w, "environment") &&
          !isTagged(w, "procedure") &&
          !isTagged(w, "primitive") &&
          !isTagged(w, "condition") &&
          !isNil(w),
      ),
    ),
    "quoted?": one("quoted?", (w) => bool(taggedForm("quote", w))),
    "assignment?": one("assignment?", (w) => bool(taggedForm("set!", w))),
    "definition?": one("definition?", (w) => bool(taggedForm("define", w))),
    "if?": one("if?", (w) => bool(taggedForm("if", w))),
    "lambda?": one("lambda?", (w) => bool(taggedForm("lambda", w))),
    "begin?": one("begin?", (w) => bool(taggedForm("begin", w))),
    "application?": one("application?", (w) => bool(isPair(w))),
    "text-of-quotation": one("text-of-quotation", (w) => nth(w, 1)),
    "if-predicate": one("if-predicate", (w) => nth(w, 1)),
    "if-consequent": one("if-consequent", (w) => nth(w, 2)),
    "if-alternative": one("if-alternative", (w) => {
      const xs = items(w);
      return xs[3] ?? false;
    }),
    "begin-actions": one("begin-actions", (w) => {
      if (!isPair(w)) throw new EvaluatorFault("begin-actions needs a list");
      return w.cdr;
    }),
    "lambda-parameters": one("lambda-parameters", (w) => nth(w, 1)),
    "lambda-body": one("lambda-body", (w) => {
      const xs = items(w);
      return list(xs.slice(2));
    }),
    operator: one("operator", (w) => nth(w, 0)),
    operands: one("operands", (w) => {
      if (!isPair(w)) throw new EvaluatorFault("operands needs a combination");
      return w.cdr;
    }),
    "assignment-variable": one("assignment-variable", (w) => nth(w, 1)),
    "assignment-value": one("assignment-value", (w) => nth(w, 2)),
    "definition-variable": one("definition-variable", (w) => {
      const target = nth(w, 1);
      return isPair(target) ? target.car : target;
    }),
    "definition-value": one("definition-value", (w) => {
      const xs = items(w),
        target = xs[1] ?? nil;
      if (isPair(target)) return list([symbol("lambda"), target.cdr, ...xs.slice(2)]);
      return xs[2] ?? nil;
    }),
    "first-exp": one("first-exp", (w) => nth(w, 0)),
    "rest-exps": one("rest-exps", (w) => {
      const xs = items(w);
      return list(xs.slice(1));
    }),
    "last-exp?": one("last-exp?", (w) => bool(items(w).length === 1)),
    "no-more-exps?": one("no-more-exps?", (w) => bool(isNil(w))),
    "no-operands?": one("no-operands?", (w) => bool(isNil(w))),
    "first-operand": one("first-operand", (w) => nth(w, 0)),
    "rest-operands": one("rest-operands", (w) => list(items(w).slice(1))),
    "last-operand?": one("last-operand?", (w) => bool(items(w).length === 1)),
    "empty-arglist": () => nil,
    "adjoin-arg": two("adjoin-arg", (a, l) => list([...items(l), a])),
    "primitive-procedure?": one("primitive-procedure?", (w) => bool(isTagged(w, "primitive"))),
    "compound-procedure?": one("compound-procedure?", (w) => bool(isTagged(w, "procedure"))),
    "procedure-parameters": one("procedure-parameters", (w) =>
      list(((w as TaggedWord).payload as { params: string[] }).params.map(symbol)),
    ),
    "procedure-body": one("procedure-body", (w) =>
      list(((w as TaggedWord).payload as { body: Word[] }).body),
    ),
    "procedure-environment": one("procedure-environment", (w) =>
      envWord(((w as TaggedWord).payload as { env: number }).env),
    ),
    "true?": one("true?", (w) => bool(w !== false)),
    "make-procedure": three("make-procedure", (p, b, e) =>
      procedureWord(items(p).map(nameOf), items(b), e),
    ),
    "apply-primitive-procedure": two("apply-primitive-procedure", (p, args) => {
      const name = ((p as TaggedWord).payload as { name: string }).name;
      return applyPrimitive(name, items(args));
    }),
  };
  const envOps: Record<string, Operation> = {
    "get-global-environment": () => envWord(0) as Value,
    "lookup-variable-value": two("lookup-variable-value", (v, e) => {
      const found = find(e, nameOf(v)).bindings.get(nameOf(v));
      if (found === undefined) throw new EvaluatorFault(`unbound variable: ${nameOf(v)}`);
      return found;
    }),
    "set-variable-value!": three("set-variable-value!", (v, value, e) => {
      find(e, nameOf(v)).bindings.set(nameOf(v), value);
      return nil;
    }),
    "define-variable!": three("define-variable!", (v, value, e) => {
      const frame = state.frames[envIndex(e)];
      if (!frame) throw new EvaluatorFault("bad environment index");
      frame.bindings.set(nameOf(v), value);
      return nil;
    }),
    "extend-environment": three("extend-environment", (p, a, e) => {
      const ps = items(p).map(nameOf),
        as = items(a);
      if (ps.length !== as.length)
        throw new EvaluatorFault(`arity mismatch: expected ${ps.length}, given ${as.length}`);
      const i = state.frames.length;
      state.frames.push({
        bindings: new Map(ps.map((n, j) => [n, as[j] as Word])),
        parent: envIndex(e),
      });
      return envWord(i);
    }),
    read: () => {
      const value = state.input.shift();
      if (value === undefined) throw new EvaluatorFault(INPUT_EXHAUSTED);
      return value as Value;
    },
    "prompt-for-input": () => {
      state.output.push(";;; EC-Eval input:");
      return nil;
    },
    "announce-output": () => {
      state.output.push(";;; EC-Eval value:");
      return nil;
    },
    "user-print": one("user-print", (w) => {
      state.output.push(render(w));
      return w;
    }),
    "signal-error": one("signal-error", (w) => {
      throw new EvaluatorFault(`signal-error: ${render(w)}`);
    }),
  };
  const applyPrimitive = (name: string, args: Word[]): Word => {
    const n = (w: Word): number => {
      if (typeof w !== "number") throw new EvaluatorFault(`${name}: not a number`);
      return w;
    };
    switch (name) {
      case "cons":
        if (args.length !== 2) throw new EvaluatorFault("arity mismatch");
        return pair(args[0] as Word, args[1] as Word);
      case "car":
        if (!isPair(args[0] as Word)) throw new EvaluatorFault("type error: car");
        return (args[0] as PairWord).car;
      case "cdr":
        if (!isPair(args[0] as Word)) throw new EvaluatorFault("type error: cdr");
        return (args[0] as PairWord).cdr;
      case "null?":
        return isNil(args[0] as Word);
      case "pair?":
        return isPair(args[0] as Word);
      case "symbol?":
        return (
          typeof args[0] === "object" &&
          !isPair(args[0] as Word) &&
          !isTagged(args[0] as Word, "environment")
        );
      case "number?":
        return typeof args[0] === "number";
      case "not":
        return args[0] === false;
      case "eq?":
        return args[0] === args[1] || (typeof args[0] === "number" && args[0] === args[1]);
      case "list":
        return list(args);
      case "+":
        return args.reduce<number>((a, b) => a + n(b), 0);
      case "-":
        return args.length === 1
          ? -n(args[0] as Word)
          : args.slice(1).reduce<number>((a, b) => a - n(b), n(args[0] as Word));
      case "*":
        return args.reduce<number>((a, b) => a * n(b), 1);
      case "/": {
        let x = n(args[0] as Word);
        for (const a of args.slice(1)) {
          const d = n(a);
          if (d === 0) throw new EvaluatorFault("division by zero");
          x /= d;
        }
        return x;
      }
      case "=":
        return n(args[0] as Word) === n(args[1] as Word);
      case "<":
        return n(args[0] as Word) < n(args[1] as Word);
      case ">":
        return n(args[0] as Word) > n(args[1] as Word);
      case "remainder":
        return n(args[0] as Word) % n(args[1] as Word);
      default:
        throw new EvaluatorFault(`unknown primitive procedure: ${name}`);
    }
  };
  return { ...syntax, ...envOps };
};

const controller: ControllerLine[] = [
  mark("read-eval-print-loop"),
  perform("initialize-stack"),
  perform("prompt-for-input"),
  assign("exp", op("read")),
  assign("env", op("get-global-environment")),
  assign("continue", c({ label: "print-result" })),
  jump("eval-dispatch"),
  mark("print-result"),
  perform("announce-output"),
  perform("user-print", reg("val")),
  jump("read-eval-print-loop"),
  mark("eval-dispatch"),
  test("self-evaluating?", reg("exp")),
  branch("ev-self-eval"),
  test("variable?", reg("exp")),
  branch("ev-variable"),
  test("quoted?", reg("exp")),
  branch("ev-quoted"),
  test("assignment?", reg("exp")),
  branch("ev-assignment"),
  test("definition?", reg("exp")),
  branch("ev-definition"),
  test("if?", reg("exp")),
  branch("ev-if"),
  test("lambda?", reg("exp")),
  branch("ev-lambda"),
  test("begin?", reg("exp")),
  branch("ev-begin"),
  test("application?", reg("exp")),
  branch("ev-application"),
  jump("unknown-expression-type"),
  mark("ev-self-eval"),
  assign("val", reg("exp")),
  jumpReg("continue"),
  mark("ev-variable"),
  assign("val", op("lookup-variable-value", reg("exp"), reg("env"))),
  jumpReg("continue"),
  mark("ev-quoted"),
  assign("val", op("text-of-quotation", reg("exp"))),
  jumpReg("continue"),
  mark("ev-lambda"),
  assign("unev", op("lambda-parameters", reg("exp"))),
  assign("exp", op("lambda-body", reg("exp"))),
  assign("val", op("make-procedure", reg("unev"), reg("exp"), reg("env"))),
  jumpReg("continue"),
  mark("ev-application"),
  save("continue"),
  save("env"),
  assign("unev", op("operands", reg("exp"))),
  save("unev"),
  assign("exp", op("operator", reg("exp"))),
  assign("continue", c({ label: "ev-appl-did-operator" })),
  jump("eval-dispatch"),
  mark("ev-appl-did-operator"),
  restore("unev"),
  restore("env"),
  assign("argl", op("empty-arglist")),
  assign("proc", reg("val")),
  test("no-operands?", reg("unev")),
  branch("apply-dispatch"),
  save("proc"),
  mark("ev-appl-operand-loop"),
  save("argl"),
  assign("exp", op("first-operand", reg("unev"))),
  test("last-operand?", reg("unev")),
  branch("ev-appl-last-arg"),
  save("env"),
  save("unev"),
  assign("continue", c({ label: "ev-appl-accumulate-arg" })),
  jump("eval-dispatch"),
  mark("ev-appl-accumulate-arg"),
  restore("unev"),
  restore("env"),
  restore("argl"),
  assign("argl", op("adjoin-arg", reg("val"), reg("argl"))),
  assign("unev", op("rest-operands", reg("unev"))),
  jump("ev-appl-operand-loop"),
  mark("ev-appl-last-arg"),
  assign("continue", c({ label: "ev-appl-accum-last-arg" })),
  jump("eval-dispatch"),
  mark("ev-appl-accum-last-arg"),
  restore("argl"),
  assign("argl", op("adjoin-arg", reg("val"), reg("argl"))),
  restore("proc"),
  jump("apply-dispatch"),
  mark("apply-dispatch"),
  test("primitive-procedure?", reg("proc")),
  branch("primitive-apply"),
  test("compound-procedure?", reg("proc")),
  branch("compound-apply"),
  jump("unknown-procedure-type"),
  mark("primitive-apply"),
  assign("val", op("apply-primitive-procedure", reg("proc"), reg("argl"))),
  restore("continue"),
  jumpReg("continue"),
  mark("compound-apply"),
  assign("unev", op("procedure-parameters", reg("proc"))),
  assign("env", op("procedure-environment", reg("proc"))),
  assign("env", op("extend-environment", reg("unev"), reg("argl"), reg("env"))),
  assign("unev", op("procedure-body", reg("proc"))),
  jump("ev-sequence"),
  mark("ev-begin"),
  assign("unev", op("begin-actions", reg("exp"))),
  save("continue"),
  jump("ev-sequence"),
  mark("ev-sequence"),
  assign("exp", op("first-exp", reg("unev"))),
  test("last-exp?", reg("unev")),
  branch("ev-sequence-last-exp"),
  save("unev"),
  save("env"),
  assign("continue", c({ label: "ev-sequence-continue" })),
  jump("eval-dispatch"),
  mark("ev-sequence-continue"),
  restore("env"),
  restore("unev"),
  assign("unev", op("rest-exps", reg("unev"))),
  jump("ev-sequence"),
  mark("ev-sequence-last-exp"),
  restore("continue"),
  jump("eval-dispatch"),
  mark("ev-if"),
  save("exp"),
  save("env"),
  save("continue"),
  assign("continue", c({ label: "ev-if-decide" })),
  assign("exp", op("if-predicate", reg("exp"))),
  jump("eval-dispatch"),
  mark("ev-if-decide"),
  restore("continue"),
  restore("env"),
  restore("exp"),
  test("true?", reg("val")),
  branch("ev-if-consequent"),
  mark("ev-if-alternative"),
  assign("exp", op("if-alternative", reg("exp"))),
  jump("eval-dispatch"),
  mark("ev-if-consequent"),
  assign("exp", op("if-consequent", reg("exp"))),
  jump("eval-dispatch"),
  mark("ev-assignment"),
  assign("unev", op("assignment-variable", reg("exp"))),
  save("unev"),
  assign("exp", op("assignment-value", reg("exp"))),
  save("env"),
  save("continue"),
  assign("continue", c({ label: "ev-assignment-1" })),
  jump("eval-dispatch"),
  mark("ev-assignment-1"),
  restore("continue"),
  restore("env"),
  restore("unev"),
  perform("set-variable-value!", reg("unev"), reg("val"), reg("env")),
  assign("val", c(symbol("ok"))),
  jumpReg("continue"),
  mark("ev-definition"),
  assign("unev", op("definition-variable", reg("exp"))),
  save("unev"),
  assign("exp", op("definition-value", reg("exp"))),
  save("env"),
  save("continue"),
  assign("continue", c({ label: "ev-definition-1" })),
  jump("eval-dispatch"),
  mark("ev-definition-1"),
  restore("continue"),
  restore("env"),
  restore("unev"),
  perform("define-variable!", reg("unev"), reg("val"), reg("env")),
  assign("val", c(symbol("ok"))),
  jumpReg("continue"),
  mark("unknown-expression-type"),
  assign("val", c(symbol("unknown-expression-type-error"))),
  jump("signal-error"),
  mark("unknown-procedure-type"),
  assign("val", c(symbol("unknown-procedure-type-error"))),
  jump("signal-error"),
  mark("signal-error"),
  perform("signal-error", reg("val")),
  jump("read-eval-print-loop"),
];
export const evaluatorController = controller;
export const monitoredEvaluatorController = controller.map((line) => line);
export const evaluatorRegisters = [
  "exp",
  "env",
  "val",
  "continue",
  "proc",
  "argl",
  "unev",
] as const;
export const controllerText = controller.map((x) => (x.tag === "label" ? x.name : "")).join("\n");

export interface Evaluator {
  readonly machine: Machine;
  readonly state: State;
  readonly transcript: readonly string[];
  run: () => readonly string[];
}
export const makeEvaluator = (
  source: string,
  customOperations: Readonly<Record<string, Operation>> = {},
  _monitored = false,
): Evaluator => {
  const state: State = {
    frames: [{ bindings: new Map(), parent: null }],
    input: readProgram(source),
    output: [],
  };
  const global = state.frames[0];
  if (!global) throw new EvaluatorFault("missing global frame");
  for (const n of [
    "cons",
    "car",
    "cdr",
    "null?",
    "pair?",
    "symbol?",
    "number?",
    "not",
    "eq?",
    "list",
    "+",
    "-",
    "*",
    "/",
    "=",
    "<",
    ">",
    "remainder",
  ])
    global.bindings.set(n, primitiveWord(n));
  global.bindings.set("true", true);
  global.bindings.set("false", false);
  const machine = makeNewMachine(evaluatorRegisters, {
    ...baseOperations(state),
    ...customOperations,
  });
  state.machine = machine;
  const assembled = assemble(controller, machine);
  if (!assembled.ok) throw new EvaluatorFault(JSON.stringify(assembled.error));
  machine.install(assembled.value);
  let ran = false;
  const run = (): readonly string[] => {
    if (ran) return state.output;
    ran = true;
    try {
      const result = machine.start();
      if (!result.ok) throw new EvaluatorFault(JSON.stringify(result.error));
    } catch (e) {
      if (!(e instanceof EvaluatorFault) || e.message !== INPUT_EXHAUSTED) throw e;
    }
    return state.output;
  };
  return {
    machine,
    state,
    get transcript() {
      return state.output;
    },
    run,
  };
};
export const runEvaluator = (
  source: string,
  customOperations: Readonly<Record<string, Operation>> = {},
): readonly string[] => makeEvaluator(source, customOperations).run();
export const formatWord = render;
export const parse = readProgram;
export const makeTaggedWord = tagged;
export const isTaggedWord = isTagged;
export const makeEnvironmentWord = envWord;
export const makeConditionWord = conditionWord;
