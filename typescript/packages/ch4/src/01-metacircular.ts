// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * The metacircular evaluator: the book's section 4.1 in this edition's
 * rendering. Expressions are values - the same cons-list data `read.ts`
 * produces - so the syntax predicates do the book's car/cdr surgery, and
 * `evaluate` is the book's case analysis over that data. Evaluation runs
 * in `Effect` with the checked failures of `errors.ts`; environments are
 * the mutable frame chains of `env.ts`. Sections 4.1.1 through 4.1.7 each
 * own their block, in the book's order.
 */
import { Effect } from "effect";
import type {
  BooleanValue,
  CompoundProc,
  Env,
  Evaluate,
  Primitive,
  SymbolValue,
  Value,
} from "./core.js";
import {
  defineVariable as defineInFrame,
  extendEnv,
  lookupVariable,
  makeGlobalEnv,
  setVariable,
} from "./env.js";
import {
  ArityMismatch,
  type EvaluationError,
  NotAProcedure,
  RuntimeError,
  UnknownSyntax,
} from "./errors.js";
import { type Cons, cons, type List, nil, toArray } from "./list.js";
import { format, ReadError, read } from "./read.js";

// ---------------------------------------------------------------------
// Values and the truth test (4.1.3)
// ---------------------------------------------------------------------

/** The book's `true` object. */
export const trueValue: BooleanValue = { _tag: "Boolean", b: true };

/** The book's `false` object: the only false value there is. */
export const falseValue: BooleanValue = { _tag: "Boolean", b: false };

/** The value `display` and `newline` answer: nothing worth printing. */
export const unspecified: Value = { _tag: "Unspecified" };

/** What `define` and `set!` answer, the book's `ok` symbol. */
export const ok: SymbolValue = { _tag: "Symbol", name: "ok" };

/** Builds a symbol value; the reader interns nothing, names compare by text. */
export const symbol = (name: string): SymbolValue => ({ _tag: "Symbol", name });

/** The book's `true?`: false is the false object, everything else is true. */
export const isTrue = (value: Value): boolean => !(value._tag === "Boolean" && !value.b);

/** The book's `eq?` on values: symbols and leaves compare by content,
 * pairs and procedures by identity. */
export const eqValue = (a: Value, b: Value): boolean => {
  switch (a._tag) {
    case "Symbol":
      return b._tag === "Symbol" && a.name === b.name;
    case "Number":
      return b._tag === "Number" && a.n === b.n;
    case "String":
      return b._tag === "String" && a.s === b.s;
    case "Boolean":
      return b._tag === "Boolean" && a.b === b.b;
    case "Nil":
      return b._tag === "Nil";
    case "Unspecified":
      return b._tag === "Unspecified";
    default:
      return a === b;
  }
};

/** The book's `equal?`: structural comparison over pairs. */
export const equalValue = (a: Value, b: Value): boolean => {
  if (a._tag === "Cons" && b._tag === "Cons") {
    return equalValue(a.head, b.head) && equalValue(a.tail, b.tail);
  }
  return eqValue(a, b);
};

// ---------------------------------------------------------------------
// Pair accessors: the book's car, cdr, and friends (4.1.2)
// ---------------------------------------------------------------------

/** The book's `car` of a pair; callers narrow with `isPair` first. */
export const car = (p: Cons<Value>): Value => p.head;

/** The book's `cdr` of a pair. */
export const cdr = (p: Cons<Value>): List<Value> => p.tail;

/** Whether a value is a pair, the book's `pair?`. */
export const isPair = (v: Value): v is Cons<Value> => v._tag === "Cons";

/** Whether a value is a symbol, the book's `symbol?`. */
export const isSymbol = (v: Value): v is SymbolValue => v._tag === "Symbol";

// Syntax accessors below assume the well-formed shapes the reader
// produces, as the book assumes them of `read`; a malformed shape yields
// a name no predicate accepts and evaluation ends in a checked error.

const cadr = (p: Cons<Value>): Value => {
  const tail = p.tail;
  return tail._tag === "Cons" ? tail.head : symbol("<malformed>");
};

const caddr = (p: Cons<Value>): Value => {
  const tail = p.tail;
  if (tail._tag === "Cons") {
    const rest = tail.tail;
    return rest._tag === "Cons" ? rest.head : symbol("<malformed>");
  }
  return symbol("<malformed>");
};

/** Converts a parameter value to its frame name; parameters are symbols. */
const nameOf = (v: Value): string => (isSymbol(v) ? v.name : "<non-symbol-parameter>");

// ---------------------------------------------------------------------
// 4.1.2 Representing expressions
// ---------------------------------------------------------------------

/** The book's `tagged-list?`: a pair whose car is the given symbol. */
export const taggedList = (tag: string, exp: Value): exp is Cons<Value> =>
  isPair(exp) && isSymbol(exp.head) && exp.head.name === tag;

/** The book's `self-evaluating?`: numbers, strings, and booleans. */
export const isSelfEvaluating = (exp: Value): boolean =>
  exp._tag === "Number" || exp._tag === "String" || exp._tag === "Boolean";

/** The book's `variable?`: a symbol. */
export const isVariable = (exp: Value): exp is SymbolValue => isSymbol(exp);

/** The book's `quoted?`. */
export const isQuoted = (exp: Value): exp is Cons<Value> => taggedList("quote", exp);

/** The book's `text-of-quotation`. */
export const textOfQuotation = (exp: Cons<Value>): Value => cadr(exp);

/** The book's `assignment?`. */
export const isAssignment = (exp: Value): exp is Cons<Value> => taggedList("set!", exp);

/** The book's `assignment-variable`. */
export const assignmentVariable = (exp: Cons<Value>): SymbolValue => {
  const target = cadr(exp);
  return isSymbol(target) ? target : symbol("<malformed>");
};

/** The book's `assignment-value`. */
export const assignmentValue = (exp: Cons<Value>): Value => caddr(exp);

/** The book's `definition?`. */
export const isDefinition = (exp: Value): exp is Cons<Value> => taggedList("define", exp);

/** The book's `definition-variable`: the name, or the procedure's name
 * in the `(define (f args) ...)` sugar. */
export const definitionVariable = (exp: Cons<Value>): SymbolValue => {
  const target = cadr(exp);
  return isSymbol(target)
    ? target
    : isPair(target) && isSymbol(target.head)
      ? target.head
      : symbol("<malformed>");
};

/** The book's `definition-value`: the expression, or a lambda built from
 * the parameter list and body of the sugared form. */
export const definitionValue = (exp: Cons<Value>): Value => {
  const target = cadr(exp);
  if (isSymbol(target)) {
    return caddr(exp);
  }
  if (isPair(target)) {
    return makeLambda(target.tail, cddr(exp));
  }
  return symbol("<malformed>");
};

const cddr = (p: Cons<Value>): List<Value> => {
  const tail = p.tail;
  return tail._tag === "Cons" ? tail.tail : nil;
};

/** The book's `lambda?`. */
export const isLambda = (exp: Value): exp is Cons<Value> => taggedList("lambda", exp);

/** The book's `lambda-parameters`: the symbol list. */
export const lambdaParameters = (exp: Cons<Value>): List<Value> => cadrOfList(cadr(exp));

const cadrOfList = (v: Value): List<Value> => (v._tag === "Cons" || v._tag === "Nil" ? v : nil);

/** The book's `lambda-body`: the list of body expressions. */
export const lambdaBody = (exp: Cons<Value>): List<Value> => cddr(exp);

/** The book's `make-lambda`: builds the lambda expression as data. */
export const makeLambda = (parameters: List<Value>, body: List<Value>): Value =>
  cons(symbol("lambda"), cons(parameters, body));

/** The book's `if?`. */
export const isIf = (exp: Value): exp is Cons<Value> => taggedList("if", exp);

/** The book's `if-predicate`. */
export const ifPredicate = (exp: Cons<Value>): Value => cadr(exp);

/** The book's `if-consequent`. */
export const ifConsequent = (exp: Cons<Value>): Value => caddr(exp);

/** The book's `if-alternative`: `false` when the expression omits it. */
export const ifAlternative = (exp: Cons<Value>): Value => {
  const rest = cddr(exp);
  const tail = rest._tag === "Cons" ? rest.tail : nil;
  return tail._tag === "Cons" ? tail.head : falseValue;
};

/** The book's `make-if`. */
export const makeIf = (predicate: Value, consequent: Value, alternative: Value): Value =>
  cons(symbol("if"), cons(predicate, cons(consequent, cons(alternative, nil))));

/** The book's `begin?`. */
export const isBegin = (exp: Value): exp is Cons<Value> => taggedList("begin", exp);

/** The book's `begin-actions`. */
export const beginActions = (exp: Cons<Value>): List<Value> => cdr(exp);

/** The book's `last-exp?`: the rest is the empty list. */
export const isLastExp = (seq: List<Value>): boolean => {
  const tail = seq._tag === "Cons" ? seq.tail : seq;
  return tail._tag === "Nil";
};

/** The book's `first-exp`. */
export const firstExp = (seq: List<Value>): Value =>
  seq._tag === "Cons" ? seq.head : symbol("<malformed>");

/** The book's `rest-exps`. */
export const restExps = (seq: List<Value>): List<Value> => (seq._tag === "Cons" ? seq.tail : nil);

/** The book's `sequence->exp`: one expression, or a begin of them. */
export const sequenceToExp = (seq: List<Value>): Value => {
  if (seq._tag === "Nil") {
    return seq;
  }
  return isLastExp(seq) ? firstExp(seq) : makeBegin(seq);
};

/** The book's `make-begin`. */
export const makeBegin = (seq: List<Value>): Value => cons(symbol("begin"), seq);

/** The book's `cond?`. */
export const isCond = (exp: Value): exp is Cons<Value> => taggedList("cond", exp);

/** The book's `cond-clauses`. */
export const condClauses = (exp: Cons<Value>): List<Value> => cdr(exp);

/** The book's `cond-else-clause?`. */
export const isCondElseClause = (clause: Value): boolean =>
  isPair(clause) && isSymbol(clause.head) && clause.head.name === "else";

/** The book's `cond-predicate`. */
export const condPredicate = (clause: Cons<Value>): Value => car(clause);

/** The book's `cond-actions`. */
export const condActions = (clause: Cons<Value>): List<Value> => cdr(clause);

/** The book's `expand-clauses`: folds the clause list into nested ifs. */
const expandClauses = (clauses: List<Value>): Value => {
  if (clauses._tag === "Nil") {
    return falseValue;
  }
  const first = clauses.head;
  if (!isPair(first)) {
    return falseValue;
  }
  if (isCondElseClause(first)) {
    return sequenceToExp(condActions(first));
  }
  return makeIf(
    condPredicate(first),
    sequenceToExp(condActions(first)),
    expandClauses(clauses.tail),
  );
};

/** The book's `cond->if`. */
export const condToIf = (exp: Cons<Value>): Value => expandClauses(condClauses(exp));

/** The book's `application?`: any other pair. */
export const isApplication = (exp: Value): exp is Cons<Value> => isPair(exp);

/** The book's `operator`. */
export const operator = (exp: Cons<Value>): Value => car(exp);

/** The book's `operands`: the raw operand expression list. */
export const operands = (exp: Cons<Value>): List<Value> => cdr(exp);

/** The book's `no-operands?`. */
export const noOperands = (ops: List<Value>): boolean => ops._tag === "Nil";

/** The book's `first-operand`. */
export const firstOperand = (ops: List<Value>): Value => firstExp(ops);

/** The book's `rest-operands`. */
export const restOperands = (ops: List<Value>): List<Value> => restExps(ops);

// ---------------------------------------------------------------------
// 4.1.1 The core of the evaluator
// ---------------------------------------------------------------------

/** The book's `list-of-values` as the edition writes it: operands are
 * evaluated left to right, the order the host fixes (exercise 4.1). */
export const listOfValues = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> => {
  if (noOperands(exps)) {
    return Effect.succeed(nil);
  }
  return Effect.flatMap(evaluate(firstOperand(exps), env), (first) =>
    Effect.map(listOfValues(restOperands(exps), env), (rest) => cons(first, rest)),
  );
};

/** The book's `eval`: one case analysis over the expression data. */
export const evaluate: Evaluate = (exp, env) => {
  if (isSelfEvaluating(exp)) {
    return Effect.succeed(exp);
  }
  if (isVariable(exp)) {
    return lookupVariableValue(exp, env);
  }
  if (isQuoted(exp)) {
    return Effect.succeed(textOfQuotation(exp));
  }
  if (isAssignment(exp)) {
    return evalAssignment(exp, env);
  }
  if (isDefinition(exp)) {
    return evalDefinition(exp, env);
  }
  if (isIf(exp)) {
    return evalIf(exp, env);
  }
  if (isLambda(exp)) {
    return Effect.succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env));
  }
  if (isBegin(exp)) {
    return evalSequence(beginActions(exp), env);
  }
  if (isCond(exp)) {
    return evaluate(condToIf(exp), env);
  }
  if (isApplication(exp)) {
    return Effect.flatMap(evaluate(operator(exp), env), (procedure) =>
      Effect.flatMap(listOfValues(operands(exp), env), (args) => applyProcedure(procedure, args)),
    );
  }
  return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

/** The book's `make-procedure` (4.1.3): packages a compound procedure. */
export const makeProcedure = (
  parameters: List<Value>,
  body: List<Value>,
  env: Env,
): CompoundProc => ({
  _tag: "Compound",
  params: parameters,
  body,
  env,
});

/** The book's `apply` over evaluator values. */
export const applyProcedure = (
  procedure: Value,
  args: List<Value>,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
      evalSequence(procedure.body, newEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};

/** The book's `apply-primitive-procedure`: runs the host function. */
export const applyPrimitiveProcedure = (
  procedure: Extract<Value, { _tag: "Primitive" }>,
  args: List<Value>,
): Effect.Effect<Value, EvaluationError> => procedure.fn(args);

/** The book's `eval-if`; a missing alternative arms false, so `(if p c)`
 * behaves as the edition defines it. */
export const evalIf = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluate(ifPredicate(exp), env), (predicate) =>
    isTrue(predicate) ? evaluate(ifConsequent(exp), env) : evaluate(ifAlternative(exp), env),
  );

/** The book's `eval-sequence`: every expression but the last for effect. */
export const evalSequence = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<Value, EvaluationError> => {
  if (exps._tag === "Nil") {
    return Effect.fail(new RuntimeError({ message: "Empty sequence: EVAL", detail: "" }));
  }
  if (isLastExp(exps)) {
    return evaluate(firstExp(exps), env);
  }
  return Effect.flatMap(evaluate(firstExp(exps), env), () => evalSequence(restExps(exps), env));
};

/** The book's `eval-assignment`: computes, writes the found frame. */
export const evalAssignment = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluate(assignmentValue(exp), env), (value) =>
    Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
  );

/** The book's `eval-definition`: computes, binds in this frame. */
export const evalDefinition = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(evaluate(definitionValue(exp), env), (value) =>
    Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
  );

// ---------------------------------------------------------------------
// 4.1.3 Evaluator data structures: environment operations
// ---------------------------------------------------------------------

/** The book's `lookup-variable-value`. */
export const lookupVariableValue = (
  variable: SymbolValue,
  env: Env,
): Effect.Effect<Value, EvaluationError> => lookupVariable(env, variable.name);

/** The book's `set-variable-value!`: writes the frame that defines it. */
export const setVariableValue = (
  variable: SymbolValue,
  value: Value,
  env: Env,
): Effect.Effect<void, EvaluationError> => setVariable(env, variable.name, value);

/** The book's `define-variable!`: binds in this frame, shadowing outer ones. */
export const defineVariableValue = (
  variable: SymbolValue,
  value: Value,
  env: Env,
): Effect.Effect<void> => defineInFrame(env, variable.name, value);

/** The book's `add-binding-to-frame!`: one more binding in this frame. */
export const addBindingToFrame = (
  variable: SymbolValue,
  value: Value,
  env: Env,
): Effect.Effect<void> => defineInFrame(env, variable.name, value);

const fillFrame = (
  env: Env,
  vars: List<Value>,
  vals: List<Value>,
): Effect.Effect<Env, EvaluationError> => {
  if (vars._tag === "Cons" && vals._tag === "Cons") {
    return Effect.flatMap(defineInFrame(env, nameOf(vars.head), vals.head), () =>
      fillFrame(env, vars.tail, vals.tail),
    );
  }
  if (vars._tag === "Nil" && vals._tag === "Nil") {
    return Effect.succeed(env);
  }
  return Effect.fail(new ArityMismatch({ expected: countList(vars), given: countList(vals) }));
};

const countList = (items: List<Value>): number => toArray(items).length;

/** The book's `extend-environment`: a fresh frame binding `vars` to `vals`. */
export const extendEnvironment = (
  vars: List<Value>,
  vals: List<Value>,
  baseEnv: Env,
): Effect.Effect<Env, EvaluationError> =>
  Effect.flatMap(extendEnv(baseEnv), (env) => fillFrame(env, vars, vals));

// ---------------------------------------------------------------------
// 4.1.4 Running the evaluator as a program
// ---------------------------------------------------------------------

/** Where `display` and `newline` write; a session collects its transcript. */
export type Sink = (chunk: string) => void;

const numberArgs = (
  args: List<Value>,
  who: string,
): Effect.Effect<ReadonlyArray<number>, EvaluationError> => {
  const nums: number[] = [];
  let rest: List<Value> = args;
  while (rest._tag === "Cons") {
    const head = rest.head;
    if (head._tag !== "Number") {
      return Effect.fail(
        new RuntimeError({ message: `${who}: expected numbers`, detail: format(head) }),
      );
    }
    nums.push(head.n);
    rest = rest.tail;
  }
  return Effect.succeed(nums);
};

const primitive = (name: string, fn: Primitive): [string, Primitive] => [name, fn];

const foldNumbers =
  (who: string, zero: number, step: (a: number, b: number) => number): Primitive =>
  (args) =>
    Effect.flatMap(numberArgs(args, who), (ns) => {
      if (ns.length === 0) {
        return Effect.succeed({ _tag: "Number", n: zero });
      }
      return Effect.succeed({ _tag: "Number", n: ns.reduce(step) });
    });

const compareNumbers =
  (who: string, holds: (a: number, b: number) => boolean): Primitive =>
  (args) =>
    Effect.flatMap(numberArgs(args, who), (ns) => {
      let ordered = true;
      let previous: number | undefined;
      for (const n of ns) {
        if (previous !== undefined && !holds(previous, n)) {
          ordered = false;
        }
        previous = n;
      }
      return Effect.succeed({ _tag: "Boolean", b: ordered });
    });

/** Builds the book's primitive-procedures table: name plus host function.
 * `display` and `newline` write to the given sink. */
export const makePrimitiveProcedures = (
  sink: Sink,
): ReadonlyArray<readonly [string, Primitive]> => [
  primitive("car", (args) => {
    const first = toArray(args)[0];
    return first !== undefined && isPair(first)
      ? Effect.succeed(car(first))
      : Effect.fail(new RuntimeError({ message: "car: expected a pair", detail: argsText(args) }));
  }),
  primitive("cdr", (args) => {
    const first = toArray(args)[0];
    return first !== undefined && isPair(first)
      ? Effect.succeed(cdr(first))
      : Effect.fail(new RuntimeError({ message: "cdr: expected a pair", detail: argsText(args) }));
  }),
  primitive("cons", (args) => {
    const [a, b] = toArray(args);
    return a !== undefined && b !== undefined
      ? Effect.succeed(cons(a, asList(b)))
      : Effect.fail(
          new RuntimeError({ message: "cons: expected two arguments", detail: argsText(args) }),
        );
  }),
  primitive("null?", (args) => {
    const first = toArray(args)[0];
    return Effect.succeed({ _tag: "Boolean", b: first !== undefined && first._tag === "Nil" });
  }),
  primitive("pair?", (args) => {
    const first = toArray(args)[0];
    return Effect.succeed({ _tag: "Boolean", b: first !== undefined && isPair(first) });
  }),
  primitive("eq?", (args) => {
    const [a, b] = toArray(args);
    return Effect.succeed({
      _tag: "Boolean",
      b: a !== undefined && b !== undefined && eqValue(a, b),
    });
  }),
  primitive("equal?", (args) => {
    const [a, b] = toArray(args);
    return Effect.succeed({
      _tag: "Boolean",
      b: a !== undefined && b !== undefined && equalValue(a, b),
    });
  }),
  primitive("symbol?", (args) => {
    const first = toArray(args)[0];
    return Effect.succeed({ _tag: "Boolean", b: first !== undefined && isSymbol(first) });
  }),
  primitive("number?", (args) => {
    const first = toArray(args)[0];
    return Effect.succeed({ _tag: "Boolean", b: first !== undefined && first._tag === "Number" });
  }),
  primitive("string?", (args) => {
    const first = toArray(args)[0];
    return Effect.succeed({ _tag: "Boolean", b: first !== undefined && first._tag === "String" });
  }),
  primitive("boolean?", (args) => {
    const first = toArray(args)[0];
    return Effect.succeed({ _tag: "Boolean", b: first !== undefined && first._tag === "Boolean" });
  }),
  primitive(
    "+",
    foldNumbers("+", 0, (a, b) => a + b),
  ),
  primitive(
    "-",
    foldNumbers("-", 0, (a, b) => a - b),
  ),
  primitive(
    "*",
    foldNumbers("*", 1, (a, b) => a * b),
  ),
  primitive("/", (args) =>
    Effect.flatMap(numberArgs(args, "/"), (ns) => {
      const [head, ...rest] = ns;
      if (head === undefined || rest.length === 0 || rest.some((d) => d === 0)) {
        return Effect.fail(
          new RuntimeError({
            message: "/: expects a nonzero divisor list",
            detail: argsText(args),
          }),
        );
      }
      return Effect.succeed({ _tag: "Number", n: rest.reduce((a, b) => a / b, head) });
    }),
  ),
  primitive(
    "=",
    compareNumbers("=", (a, b) => a === b),
  ),
  primitive(
    "<",
    compareNumbers("<", (a, b) => a < b),
  ),
  primitive(
    ">",
    compareNumbers(">", (a, b) => a > b),
  ),
  primitive("not", (args) => {
    const first = toArray(args)[0];
    return Effect.succeed({ _tag: "Boolean", b: first === undefined || !isTrue(first) });
  }),
  primitive("list", (args) => Effect.succeed(args)),
  primitive("append", (args) => Effect.succeed(appendLists(toArray(args)))),
  primitive("display", (args) => {
    const first = toArray(args)[0];
    sink(first === undefined ? "()" : format(first));
    return Effect.succeed(unspecified);
  }),
  primitive("newline", () => {
    sink("\n");
    return Effect.succeed(unspecified);
  }),
  primitive("error", (args) => {
    const items = toArray(args);
    const message =
      items[0] !== undefined && items[0]._tag === "String" ? items[0].s : format(items[0] ?? nil);
    return Effect.fail(new RuntimeError({ message, detail: items.slice(1).map(format).join(" ") }));
  }),
];

const argsText = (args: List<Value>): string => (args._tag === "Cons" ? format(args.head) : "()");

const asList = (v: Value): List<Value> => (v._tag === "Cons" || v._tag === "Nil" ? v : nil);

const appendLists = (lists: ReadonlyArray<Value>): List<Value> => {
  const parts: ReadonlyArray<Value>[] = lists.map((l) =>
    l._tag === "Cons" || l._tag === "Nil" ? toArray(l) : [],
  );
  const out: Value[] = [];
  for (const part of parts) {
    out.push(...part);
  }
  return fromValues(out);
};

const fromValues = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

/** The book's `setup-environment`: the global frame with the primitives
 * and the `true` and `false` variables bound. */
export const setupEnvironment = (sink: Sink = () => {}): Effect.Effect<Env, never> =>
  Effect.gen(function* () {
    const env = yield* makeGlobalEnv();
    for (const [name, fn] of makePrimitiveProcedures(sink)) {
      yield* defineInFrame(env, name, { _tag: "Primitive", name, fn });
    }
    yield* defineInFrame(env, "true", trueValue);
    yield* defineInFrame(env, "false", falseValue);
    return env;
  });

/** The book's driver loop over a finite session: reads each input,
 * evaluates it in the global environment, and prints the value. */
export const driverLoop = (
  env: Env,
  inputs: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.forEach(inputs, (input) =>
    Effect.flatMap(evalString(input, env), (value) =>
      Effect.succeed([";;; M-Eval input:", input, ";;; M-Eval value:", format(value)]),
    ),
  ).pipe(Effect.map((lines) => lines.flat()));

/** Reads one form and evaluates it; a read failure lands on the error
 * channel as a `RuntimeError`. */
export const evalString = (text: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(
    Effect.try({
      try: () => read(text),
      catch: (e) =>
        new RuntimeError({
          message: "read failed",
          detail: e instanceof ReadError || e instanceof Error ? e.message : String(e),
        }),
    }),
    (exp) => evaluate(exp, env),
  );

// ---------------------------------------------------------------------
// 4.1.7 Separating syntactic analysis from execution
// ---------------------------------------------------------------------

/** An execution procedure: the analyzed expression's remaining work. */
export type ExecutionProcedure = (env: Env) => Effect.Effect<Value, EvaluationError>;

/** The book's `analyze`: syntax once, execution many times. */
export const analyze = (exp: Value): ExecutionProcedure => {
  if (isSelfEvaluating(exp)) {
    return analyzeSelfEvaluating(exp);
  }
  if (isQuoted(exp)) {
    return analyzeQuoted(exp);
  }
  if (isVariable(exp)) {
    return analyzeVariable(exp);
  }
  if (isAssignment(exp)) {
    return analyzeAssignment(exp);
  }
  if (isDefinition(exp)) {
    return analyzeDefinition(exp);
  }
  if (isIf(exp)) {
    return analyzeIf(exp);
  }
  if (isLambda(exp)) {
    return analyzeLambda(exp);
  }
  if (isBegin(exp)) {
    return analyzeSequence(beginActions(exp));
  }
  if (isCond(exp)) {
    return analyze(condToIf(exp));
  }
  if (isApplication(exp)) {
    return analyzeApplication(exp);
  }
  return () => Effect.fail(new UnknownSyntax({ expr: format(exp) }));
};

/** The analyzed evaluator's `eval`: analyze once, run once. */
export const evalAnalyzed: Evaluate = (exp, env) => analyze(exp)(env);

/** The book's `analyze-self-evaluating`: the expression, unchanged. */
export const analyzeSelfEvaluating =
  (exp: Value): ExecutionProcedure =>
  () =>
    Effect.succeed(exp);

/** The book's `analyze-quoted`: the text lifted out at analysis time. */
export const analyzeQuoted = (exp: Cons<Value>): ExecutionProcedure => {
  const qval = textOfQuotation(exp);
  return () => Effect.succeed(qval);
};

/** The book's `analyze-variable`: lookup waits for the environment. */
export const analyzeVariable =
  (exp: SymbolValue): ExecutionProcedure =>
  (env) =>
    lookupVariableValue(exp, env);

/** The book's `analyze-assignment`: value analyzed once, write at run time. */
export const analyzeAssignment = (exp: Cons<Value>): ExecutionProcedure => {
  const variable = assignmentVariable(exp);
  const vproc = analyze(assignmentValue(exp));
  return (env) =>
    Effect.flatMap(vproc(env), (value) =>
      Effect.map(setVariableValue(variable, value, env), () => ok),
    );
};

/** The book's `analyze-definition`. */
export const analyzeDefinition = (exp: Cons<Value>): ExecutionProcedure => {
  const variable = definitionVariable(exp);
  const vproc = analyze(definitionValue(exp));
  return (env) =>
    Effect.flatMap(vproc(env), (value) =>
      Effect.map(defineVariableValue(variable, value, env), () => ok),
    );
};

/** The book's `analyze-if`. */
export const analyzeIf = (exp: Cons<Value>): ExecutionProcedure => {
  const pproc = analyze(ifPredicate(exp));
  const cproc = analyze(ifConsequent(exp));
  const aproc = analyze(ifAlternative(exp));
  return (env) =>
    Effect.flatMap(pproc(env), (predicate) => (isTrue(predicate) ? cproc(env) : aproc(env)));
};

/** The book's `analyze-lambda`: body analyzed once for every closure. */
export const analyzeLambda = (exp: Cons<Value>): ExecutionProcedure => {
  const vars = lambdaParameters(exp);
  const bproc = analyzeSequence(lambdaBody(exp));
  return (env) =>
    Effect.succeed(makeProcedure(vars, fromValues([{ _tag: "Execution", run: bproc }]), env));
};

const sequentially =
  (proc1: ExecutionProcedure, proc2: ExecutionProcedure): ExecutionProcedure =>
  (env) =>
    Effect.flatMap(proc1(env), () => proc2(env));

/** The book's `analyze-sequence`: execution procedures folded left, so a
 * one-expression body runs with no sequence machinery at all. */
export const analyzeSequence = (exps: List<Value>): ExecutionProcedure => {
  if (exps._tag === "Nil") {
    return () => Effect.fail(new RuntimeError({ message: "Empty sequence: ANALYZE", detail: "" }));
  }
  const loop = (first: ExecutionProcedure, rest: List<Value>): ExecutionProcedure =>
    rest._tag === "Nil" ? first : loop(sequentially(first, analyze(rest.head)), rest.tail);
  return loop(analyze(exps.head), exps.tail);
};

const runOperands = (
  aprocs: ReadonlyArray<ExecutionProcedure>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> => {
  const args: Value[] = [];
  const runFrom = (i: number): Effect.Effect<List<Value>, EvaluationError> => {
    const aproc = aprocs[i];
    if (aproc === undefined) {
      return Effect.succeed(nil);
    }
    return Effect.flatMap(aproc(env), (value) => {
      args.push(value);
      return runFrom(i + 1);
    });
  };
  return Effect.map(runFrom(0), () => fromValues(args));
};

/** The book's `analyze-application`. */
export const analyzeApplication = (exp: Cons<Value>): ExecutionProcedure => {
  const fproc = analyze(operator(exp));
  const aprocs = toArray(operands(exp)).map(analyze);
  return (env) =>
    Effect.flatMap(fproc(env), (procedure) =>
      Effect.flatMap(runOperands(aprocs, env), (args) => executeApplication(procedure, args)),
    );
};

/** The book's `execute-application`: the compound body is already an
 * execution procedure, so it just runs in the extended environment. */
export const executeApplication = (
  procedure: Value,
  args: List<Value>,
): Effect.Effect<Value, EvaluationError> => {
  if (procedure._tag === "Primitive") {
    return applyPrimitiveProcedure(procedure, args);
  }
  if (procedure._tag === "Compound") {
    const head = procedure.body._tag === "Cons" ? procedure.body.head : undefined;
    if (head !== undefined && head._tag === "Execution") {
      return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), head.run);
    }
    return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
      evalSequence(procedure.body, newEnv),
    );
  }
  return Effect.fail(new NotAProcedure({ value: format(procedure) }));
};
