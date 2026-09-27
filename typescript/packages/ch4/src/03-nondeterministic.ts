// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.3

/**
 * The amb evaluator: the book's section 4.3 in this edition's rendering. The
 * language is the one `01-metacircular.ts` evaluates plus the `amb` special
 * form: an expression may have more than one possible value, a dead end
 * backtracks to the most recent choice point, and the driver can ask for the
 * next alternative with `try-again`. The book builds this evaluator on the
 * analyzing evaluator of 4.1.7, and the edition keeps that shape: every
 * expression analyzes once into an execution procedure that takes the
 * environment and two continuation procedures, a success continuation of
 * `(value, fail)` and a failure continuation of no arguments. Dead ends
 * travel through the failure continuations only; the error channel stays
 * reserved for program bugs (an unbound variable, a bad primitive
 * application), which are not failed choices.
 *
 * The module reuses the 4.1 exports wholesale: the syntax predicates and
 * selectors, the environment operations, `makeProcedure`,
 * `applyPrimitiveProcedure`, `condToIf`, and the primitive table of
 * `setupEnvironment`. Three seams are added here and nowhere else:
 *
 * - the failure-continuation seam itself: `Fail` and `Succeed` thread the
 *   two continuations through every execution procedure, the one change the
 *   section makes to the 4.1.7 machinery;
 * - the `let` clause: the section's programs bind with `let` (the book
 *   notes the evaluator "supports let, see exercise 4.22"), which the 4.1
 *   dispatch lacks; `letToApplication` rewrites it into the lambda
 *   application of exercise 4.22 before analysis;
 * - the tuning knobs of the exercise variants: `ramb` (4.50),
 *   `permanent-set!` (4.51), `if-fail` (4.52), and `require` as a special
 *   form (4.54), each a clause the dispatch gains under its knob, plus the
 *   optional `failures` counter the measurement exercises read and the
 *   `random` generator `ramb` shuffles with.
 *
 * The dispatch is built once by `makeAmbEvaluator` over an `AmbTuning`
 * record; the section's evaluator is the default, and no exercise forks the
 * evaluator.
 */
import { Effect, type Result } from "effect";
import {
  applyPrimitiveProcedure,
  assignmentValue,
  assignmentVariable,
  beginActions,
  condToIf,
  defineVariableValue,
  definitionValue,
  definitionVariable,
  extendEnvironment,
  ifAlternative,
  ifConsequent,
  ifPredicate,
  isApplication,
  isAssignment,
  isBegin,
  isCond,
  isDefinition,
  isIf,
  isLambda,
  isQuoted,
  isSelfEvaluating,
  isTrue,
  isVariable,
  lambdaBody,
  lambdaParameters,
  lookupVariableValue,
  makeLambda,
  makeProcedure,
  ok,
  setupEnvironment,
  setVariableValue,
  symbol,
  taggedList,
  textOfQuotation,
} from "./01-metacircular.js";
import type { Env, SymbolValue, Value } from "./core.js";
import { type EvaluationError, NotAProcedure, RuntimeError, UnknownSyntax } from "./errors.js";
import { type Cons, cons, type List, nil, toArray } from "./list.js";
import { format, ReadError, read, readAll } from "./read.js";

// ---------------------------------------------------------------------
// 4.3.1 Amb and search: the syntax of the special form
// ---------------------------------------------------------------------

/** The book's `amb?`: the tagged `amb` form. */
export const isAmb = (exp: Value): exp is Cons<Value> => taggedList("amb", exp);

/** The book's `amb-choices`: the alternatives, unevaluated. */
export const ambChoices = (exp: Cons<Value>): List<Value> => exp.tail;

/** The book's `let?`: the tagged `let` form, the clause this module adds. */
export const isLet = (exp: Value): exp is Cons<Value> => taggedList("let", exp);

// Selectors below follow the 4.1 convention: shapes the reader produces are
// assumed well formed, and a malformed shape yields a `<malformed>` symbol no
// predicate accepts, so evaluation ends in a checked error.

const cadrOf = (v: Value): Value => (v._tag === "Cons" ? v.head : symbol("<malformed>"));

const listValue = (v: Value): List<Value> => (v._tag === "Cons" || v._tag === "Nil" ? v : nil);

/** The book's `let-bindings`: the `((v e) ...)` list. */
export const letBindings = (exp: Cons<Value>): List<Value> => listValue(cadrOf(exp.tail));

/** The book's `let-body`: the body expressions. */
export const letBody = (exp: Cons<Value>): List<Value> =>
  exp.tail._tag === "Cons" ? listValue(exp.tail.tail) : nil;

/** Exercise 4.22's derivation: `(let ((v e) ...) body...)` becomes
 * `((lambda (v ...) body...) e ...)`, so the combination machinery does the
 * binding and no let machinery runs at execution time. */
export const letToApplication = (exp: Cons<Value>): Value => {
  const pairs = toArray(letBindings(exp));
  return cons(
    makeLambda(
      pairs.reduceRight<List<Value>>((tail, pair) => {
        const name = cadrOf(pair);
        return cons(isVariable(name) ? name : symbol("<malformed>"), tail);
      }, nil),
      letBody(exp),
    ),
    pairs.reduceRight<List<Value>>(
      (tail, pair) => cons(cadrOf(pair._tag === "Cons" ? pair.tail : nil), tail),
      nil,
    ),
  );
};

// ---------------------------------------------------------------------
// 4.3.3 Execution procedures and continuations
// ---------------------------------------------------------------------

/** The book's failure continuation: no arguments, tries another branch. The
 * driver installs the outermost one; every dead end in the program ends at
 * one of these. */
export type Fail = () => Effect.Effect<Value, EvaluationError>;

/** The book's success continuation: receives the value just obtained and
 * another failure continuation to call if that value leads to a dead end. */
export type Succeed = (value: Value, fail: Fail) => Effect.Effect<Value, EvaluationError>;

/** The book's execution procedure in the amb evaluator: environment plus
 * the two continuations, where the 4.1.7 execution procedure took only the
 * environment. This is the failure-continuation seam. */
export type AmbExecute = (
  env: Env,
  succeed: Succeed,
  fail: Fail,
) => Effect.Effect<Value, EvaluationError>;

/** The book's `analyze` over the three-argument execution procedures. */
export type AmbAnalyze = (exp: Value) => AmbExecute;

/** The failure the driver sees when the outermost failure continuation runs:
 * the whole search ran dry, and there are no more values. The book's driver
 * answers with a report; the helpers of this module catch the class and
 * report exhaustion instead. It rides the error channel as a `RuntimeError`
 * so a search that dries in a definition position still fails the session
 * through the checked channel. */
export class AmbExhausted extends RuntimeError {
  constructor() {
    super({ message: "the amb search ran dry", detail: "there are no more values" });
    this.name = "AmbExhausted";
  }
}

// ---------------------------------------------------------------------
// The seeded generator of exercise 4.50
// ---------------------------------------------------------------------

/** The stateful host generator `ramb` shuffles with: a 32-bit xorshift
 * stream over `[0, 1)`. The seed must be a nonzero 32-bit integer; a zero
 * seed would fix the stream at zero. Two generators from one seed draw the
 * same stream, which is what makes a ramb session reproducible. */
export const makeXorshift32 = (seed: number): (() => number) => {
  if (!Number.isInteger(seed) || seed <= 0 || seed > 0xffffffff) {
    throw new RangeError("the xorshift seed must be a nonzero 32-bit integer");
  }
  let state = seed >>> 0;
  return () => {
    state ^= state << 13;
    state >>>= 0;
    state ^= state >>> 17;
    state ^= state << 5;
    state >>>= 0;
    return state / 0x100000000;
  };
};

// The shuffle `ramb` applies to its alternatives before the search descends:
// Fisher-Yates over a copy, one draw per position, the order the generator
// chooses. The generator is an argument, so one evaluator's ramb order is
// exactly its tuning's stream and nothing hides in module state.
const shuffled = <A>(items: ReadonlyArray<A>, draw: () => number): ReadonlyArray<A> => {
  const out = [...items];
  for (let i = out.length - 1; i > 0; i -= 1) {
    const j = Math.floor(draw() * (i + 1));
    const swap = out[i];
    const target = out[j];
    if (swap !== undefined && target !== undefined) {
      out[i] = target;
      out[j] = swap;
    }
  }
  return out;
};

// ---------------------------------------------------------------------
// 4.3.3 Structure of the evaluator
// ---------------------------------------------------------------------

/** Exercise 4.50's `ramb?`: the tagged `ramb` form. */
export const isRamb = (exp: Value): exp is Cons<Value> => taggedList("ramb", exp);

/** Exercise 4.50's `ramb-choices`. */
export const rambChoices = (exp: Cons<Value>): List<Value> => exp.tail;

/** Exercise 4.51's `permanent-set!?`. */
export const isPermanentSet = (exp: Value): exp is Cons<Value> => taggedList("permanent-set!", exp);

/** Exercise 4.52's `if-fail?`. */
export const isIfFail = (exp: Value): exp is Cons<Value> => taggedList("if-fail", exp);

/** Exercise 4.52's first expression, the one whose search is caught. */
export const ifFailExp = (exp: Cons<Value>): Value => cadrOf(exp.tail);

/** Exercise 4.52's alternative, the value of a dry search. */
export const ifFailAlternative = (exp: Cons<Value>): Value => {
  const rest = exp.tail._tag === "Cons" ? exp.tail.tail : nil;
  return cadrOf(rest);
};

/** Exercise 4.54's `require?`: the tagged `require` form. */
export const isRequire = (exp: Value): exp is Cons<Value> => taggedList("require", exp);

/** Exercise 4.54's `require-predicate`. */
export const requirePredicate = (exp: Cons<Value>): Value => cadrOf(exp.tail);

/**
 * The knobs the exercise variants flip; the section leaves every knob off.
 * `ramb` is exercise 4.50: the dispatch recognizes a `ramb` form whose
 * alternatives shuffle through `random` before the search descends.
 * `permanentSet` is exercise 4.51: the dispatch recognizes
 * `permanent-set!`, an assignment no backtrack undoes. `ifFail` is exercise
 * 4.52: the dispatch recognizes `if-fail`, whose alternative evaluates when
 * the first expression's search runs dry. `requireForm` is exercise 4.54:
 * the dispatch recognizes `(require p)` as a special form analyzed by
 * `analyzeRequire`, instead of require being an ordinary procedure the
 * program defines. `failures`, when supplied, counts the Fail deliveries the
 * choice frames see: one per alternative replay and one per frame
 * exhaustion, the measurement the timing exercises (4.37, 4.39, 4.40) read.
 * `random` is the stateful generator `ramb` draws from; it defaults to a
 * nonzero seed, and one generator shared by two evaluators makes them ramble
 * in step.
 */
export interface AmbTuning {
  readonly ramb?: boolean;
  readonly permanentSet?: boolean;
  readonly ifFail?: boolean;
  readonly requireForm?: boolean;
  readonly random?: () => number;
  readonly failures?: { count: number };
}

/**
 * Exercise 4.54's `analyze-require`, the definition the exercise completes:
 * the first blank is the truth test on the predicate's value, the second is
 * the `(fail2)` that rejects the branch when the predicate fails, so the
 * predicate's own alternatives are tried before the failure propagates. A
 * true predicate succeeds with the symbol `ok`.
 */
export const analyzeRequire = (exp: Cons<Value>, analyze: AmbAnalyze): AmbExecute => {
  const pproc = analyze(requirePredicate(exp));
  return (env, succeed, fail) =>
    pproc(env, (predValue, fail2) => (isTrue(predValue) ? succeed(ok, fail2) : fail2()), fail);
};

/**
 * One amb evaluator: the dispatch and the continuation machinery bound to
 * each other, the book's `analyze` and `ambeval` as first-class procedures.
 * Exercise variants build an evaluator by tuning; the section's evaluator is
 * `ambEvaluator`.
 */
export interface AmbEvaluator {
  /** The book's `analyze`: syntax once, an `AmbExecute` to run many times. */
  readonly analyze: AmbAnalyze;
  /** The book's `ambeval`: analyze the expression, apply the execution
   * procedure to the environment and the two continuations. */
  readonly ambeval: (
    exp: Value,
    env: Env,
    succeed: Succeed,
    fail: Fail,
  ) => Effect.Effect<Value, EvaluationError>;
}

/** Builds one amb evaluator over the tuning. The internal procedures close
 * over the dispatch, so an object-level procedure body analyzes and runs
 * under the same evaluator that recognized its forms. */
export const makeAmbEvaluator = (tuning: AmbTuning = {}): AmbEvaluator => {
  const ramb = tuning.ramb === true;
  const permanentSet = tuning.permanentSet === true;
  const ifFail = tuning.ifFail === true;
  const requireForm = tuning.requireForm === true;
  const random = tuning.random ?? makeXorshift32(0x5eed);
  const noteFailure = (): void => {
    if (tuning.failures !== undefined) {
      tuning.failures.count += 1;
    }
  };

  const analyzeSelfEvaluating =
    (exp: Value): AmbExecute =>
    (_env, succeed, fail) =>
      succeed(exp, fail);

  const analyzeQuoted = (exp: Cons<Value>): AmbExecute => {
    const qval = textOfQuotation(exp);
    return (_env, succeed, fail) => succeed(qval, fail);
  };

  const analyzeVariable =
    (exp: SymbolValue): AmbExecute =>
    (env, succeed, fail) =>
      Effect.flatMap(lookupVariableValue(exp, env), (value) => succeed(value, fail));

  /** The book's `analyze-lambda`: the procedure records the environment of
   * the execution that runs the lambda form. */
  const analyzeLambda =
    (exp: Cons<Value>): AmbExecute =>
    (env, succeed, fail) =>
      succeed(makeProcedure(lambdaParameters(exp), lambdaBody(exp), env), fail);

  const analyzeIf = (exp: Cons<Value>): AmbExecute => {
    const pproc = analyze(ifPredicate(exp));
    const cproc = analyze(ifConsequent(exp));
    const aproc = analyze(ifAlternative(exp));
    return (env, succeed, fail) =>
      pproc(
        env,
        (predValue, fail2) =>
          isTrue(predValue) ? cproc(env, succeed, fail2) : aproc(env, succeed, fail2),
        fail,
      );
  };

  const analyzeSequence = (exps: List<Value>): AmbExecute => {
    if (exps._tag === "Nil") {
      return () =>
        Effect.fail(new RuntimeError({ message: "Empty sequence: AMB-EVAL", detail: "" }));
    }
    const procs = toArray(exps).map(analyze);
    const sequentially =
      (a: AmbExecute, b: AmbExecute): AmbExecute =>
      (env, succeed, fail) =>
        a(env, (_value, fail2) => b(env, succeed, fail2), fail);
    return procs.reduce(sequentially);
  };

  /** The book's `analyze-definition`: the value computes, then the name
   * defines, then the success propagates with the new failure continuation. */
  const analyzeDefinition = (exp: Cons<Value>): AmbExecute => {
    const variable = definitionVariable(exp);
    const vproc = analyze(definitionValue(exp));
    return (env, succeed, fail) =>
      vproc(
        env,
        (value, fail2) =>
          Effect.flatMap(defineVariableValue(variable, value, env), () => succeed(ok, fail2)),
        fail,
      );
  };

  /** The write of the book's `analyze-assignment` plus its undo record:
   * the old value is read before the write, and the success is handed a
   * failure continuation that restores the old value before propagating,
   * so a backtrack unwrites the branch's assignments, newest first. */
  const assignWithUndo = (
    variable: SymbolValue,
    value: Value,
    env: Env,
    succeed: Succeed,
    fail2: Fail,
  ): Effect.Effect<Value, EvaluationError> =>
    Effect.flatMap(lookupVariableValue(variable, env), (oldValue) =>
      Effect.flatMap(setVariableValue(variable, value, env), () =>
        succeed(ok, () => Effect.flatMap(setVariableValue(variable, oldValue, env), () => fail2())),
      ),
    );

  /** The book's `analyze-assignment`: the first place the continuations do
   * real work. The old value is read before the write, and the success is
   * handed a failure continuation that undoes the assignment before
   * propagating, so backtracking unwrites every assignment made on the
   * branch, newest first. */
  const analyzeAssignment = (exp: Cons<Value>): AmbExecute => {
    const variable = assignmentVariable(exp);
    const vproc = analyze(assignmentValue(exp));
    return (env, succeed, fail) =>
      vproc(env, (value, fail2) => assignWithUndo(variable, value, env, succeed, fail2), fail);
  };

  /** Exercise 4.51's `analyze-permanent-set!`: the assignment skips the undo
   * record, so no backtrack ever restores the old value. */
  const analyzePermanentSet = (exp: Cons<Value>): AmbExecute => {
    const variable = assignmentVariable(exp);
    const vproc = analyze(assignmentValue(exp));
    return (env, succeed, fail) =>
      vproc(
        env,
        (value, fail2) =>
          Effect.flatMap(setVariableValue(variable, value, env), () => succeed(ok, fail2)),
        fail,
      );
  };

  /** Exercise 4.52's `analyze-if-fail`: the first expression runs against
   * the continuation as usual; only its outermost failure is intercepted,
   * and the alternative evaluates against the same success and the same
   * outer failure. A failure after the first expression has succeeded is
   * past this boundary and propagates untouched. */
  const analyzeIfFail = (exp: Cons<Value>): AmbExecute => {
    const pproc = analyze(ifFailExp(exp));
    const aproc = analyze(ifFailAlternative(exp));
    return (env, succeed, fail) => pproc(env, succeed, () => aproc(env, succeed, fail));
  };

  /** The choice machinery of `analyze-amb` and `analyze-ramb`: the analyzed
   * alternatives run in order, each handed a failure continuation that tries
   * the next; when the alternatives run out, the form's own failure
   * continuation propagates. Each delivery of a failure continuation to the
   * frame counts one Fail when the tuning measures: one per alternative
   * replay, one per frame exhaustion. */
  const tryChoices = (
    cprocs: ReadonlyArray<AmbExecute>,
    env: Env,
    succeed: Succeed,
    fail: Fail,
  ): Effect.Effect<Value, EvaluationError> => {
    const tryNext = (rest: ReadonlyArray<AmbExecute>): Effect.Effect<Value, EvaluationError> => {
      const first = rest[0];
      if (first === undefined) {
        return fail();
      }
      return first(env, succeed, () => {
        noteFailure();
        return tryNext(rest.slice(1));
      });
    };
    return tryNext(cprocs);
  };

  const analyzeAmb = (exp: Cons<Value>): AmbExecute => {
    const cprocs = toArray(ambChoices(exp)).map(analyze);
    return (env, succeed, fail) => tryChoices(cprocs, env, succeed, fail);
  };

  /** Exercise 4.50's `analyze-ramb`: the amb machinery over a shuffled copy
   * of the alternatives, the order the generator draws. The shuffle runs per
   * execution, so a ramb inside a recursive procedure draws again each time
   * the search re-enters it. */
  const analyzeRamb = (exp: Cons<Value>): AmbExecute => {
    const cprocs = toArray(rambChoices(exp)).map(analyze);
    return (env, succeed, fail) => tryChoices(shuffled(cprocs, random), env, succeed, fail);
  };

  /** The book's `get-args`: the operand execution procedures run left to
   * right, each handed a success continuation that accumulates the argument
   * and continues the walk, so the operand order is the walk order. */
  const getArgs =
    (aprocs: ReadonlyArray<AmbExecute>): AmbExecute =>
    (env, succeed, fail) => {
      const first = aprocs[0];
      if (first === undefined) {
        return succeed(nil, fail);
      }
      return first(
        env,
        (arg, fail2) =>
          getArgs(aprocs.slice(1))(
            env,
            (args, fail3) => succeed(cons(arg, listValue(args)), fail3),
            fail2,
          ),
        fail,
      );
    };

  /** The book's `execute-application`: a primitive sees the values; a
   * compound procedure's body runs in the extended environment against the
   * same continuations. Anything else is the ordinary checked error. */
  const executeApplication = (
    procedure: Value,
    args: List<Value>,
    succeed: Succeed,
    fail: Fail,
  ): Effect.Effect<Value, EvaluationError> => {
    if (procedure._tag === "Primitive") {
      return Effect.flatMap(applyPrimitiveProcedure(procedure, args), (value) =>
        succeed(value, fail),
      );
    }
    if (procedure._tag === "Compound") {
      return Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
        analyzeSequence(procedure.body)(newEnv, succeed, fail),
      );
    }
    return Effect.fail(new NotAProcedure({ value: format(procedure) }));
  };

  const analyzeApplication = (exp: Cons<Value>): AmbExecute => {
    const fproc = analyze(exp.head);
    const aprocs = toArray(exp.tail).map(analyze);
    return (env, succeed, fail) =>
      fproc(
        env,
        (proc, fail2) =>
          getArgs(aprocs)(
            env,
            (args, fail3) => executeApplication(proc, listValue(args), succeed, fail3),
            fail2,
          ),
        fail,
      );
  };

  const analyze: AmbAnalyze = (exp: Value): AmbExecute => {
    if (isSelfEvaluating(exp)) {
      return analyzeSelfEvaluating(exp);
    }
    if (isVariable(exp)) {
      return analyzeVariable(exp);
    }
    if (isQuoted(exp)) {
      return analyzeQuoted(exp);
    }
    if (isAssignment(exp)) {
      return analyzeAssignment(exp);
    }
    if (permanentSet && isPermanentSet(exp)) {
      return analyzePermanentSet(exp);
    }
    if (isDefinition(exp)) {
      return analyzeDefinition(exp);
    }
    if (ifFail && isIfFail(exp)) {
      return analyzeIfFail(exp);
    }
    if (isIf(exp)) {
      return analyzeIf(exp);
    }
    if (isLambda(exp)) {
      return analyzeLambda(exp);
    }
    if (isLet(exp)) {
      return analyze(letToApplication(exp));
    }
    if (isBegin(exp)) {
      return analyzeSequence(beginActions(exp));
    }
    if (isCond(exp)) {
      return analyze(condToIf(exp));
    }
    if (isAmb(exp)) {
      return analyzeAmb(exp);
    }
    if (ramb && isRamb(exp)) {
      return analyzeRamb(exp);
    }
    if (requireForm && isRequire(exp)) {
      return analyzeRequire(exp, analyze);
    }
    if (isApplication(exp)) {
      return analyzeApplication(exp);
    }
    return () => Effect.fail(new UnknownSyntax({ expr: format(exp) }));
  };

  return {
    analyze,
    ambeval: (exp, env, succeed, fail) => analyze(exp)(env, succeed, fail),
  };
};

/** The section's evaluator: amb, plain `set!` with its undo, require as an
 * ordinary procedure, and no exercise forms. */
export const ambEvaluator = makeAmbEvaluator();

/** The book's `ambeval` of the section. */
export const ambeval = ambEvaluator.ambeval;

/** A fresh global environment for an amb session: the 4.1 setup, the same
 * primitive table the book's `setup-environment` builds. Takes the sink the
 * object-language `display` and `newline` write to. */
export const setupAmbEnvironment = setupEnvironment;

// ---------------------------------------------------------------------
// The driver loop
// ---------------------------------------------------------------------

/** The book's `driver-loop` over a finite session. An input equal to
 * `"try-again"` resumes the search for the next alternative; any other
 * input starts a new problem. The transcript carries the book's prompts:
 * each new problem prints `;;; Amb-Eval input:`, the input, and
 * `;;; Starting a new problem`, then the value or the exhaustion report; a
 * resume prints the value or the exhaustion report alone, and a resume with
 * no current problem reports that. A hard error fails the whole session on
 * the error channel. The object-language `display` and `newline` write to
 * the sink the environment was built with. */
export const ambDriverLoop = (
  evaluator: AmbEvaluator,
  env: Env,
  inputs: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.gen(function* () {
    const lines: string[] = [];
    let next: Fail | undefined;
    let current: string | undefined;
    const record: Succeed = (value, fail) => {
      next = fail;
      return Effect.succeed(value);
    };
    const terminalFail: Fail = () => Effect.fail(new AmbExhausted());
    const runForm = (input: string): Effect.Effect<Value, EvaluationError> =>
      Effect.flatMap(
        Effect.try({
          try: () => read(input),
          catch: (e) =>
            new RuntimeError({
              message: "read failed",
              detail: e instanceof ReadError || e instanceof Error ? e.message : String(e),
            }),
        }),
        (exp) => evaluator.ambeval(exp, env, record, terminalFail),
      );
    const settle = (
      outcome: Result.Result<Value, EvaluationError>,
      input: string,
    ): Effect.Effect<void, EvaluationError> => {
      if (outcome._tag === "Success") {
        lines.push(";;; Amb-Eval value:", format(outcome.success));
        return Effect.void;
      }
      if (outcome.failure instanceof AmbExhausted) {
        lines.push(";;; There are no more values of", input);
        next = undefined;
        return Effect.void;
      }
      return Effect.fail(outcome.failure);
    };
    for (const input of inputs) {
      if (input === "try-again") {
        lines.push(";;; Amb-Eval input:", input);
        if (next === undefined) {
          lines.push(";;; There is no current problem");
          continue;
        }
        yield* settle(yield* Effect.result(next()), current ?? input);
        continue;
      }
      current = input;
      next = undefined;
      lines.push(";;; Amb-Eval input:", input, ";;; Starting a new problem");
      yield* settle(yield* Effect.result(runForm(input)), input);
    }
    return lines;
  });

// ---------------------------------------------------------------------
// Answer collection for the exercises
// ---------------------------------------------------------------------

/** One program run: the answers of the last form's search, in order, and
 * whether the search ran dry (false when `limit` cut the run first). */
export interface AmbRun {
  readonly answers: ReadonlyArray<Value>;
  readonly exhausted: boolean;
}

/** Runs a program: every form but the last evaluates for effect (the
 * definitions of the session), then the last form's search yields each
 * answer in turn until it runs dry or `limit` answers have been collected.
 * A definition whose own search runs dry raises `AmbExhausted`. */
export const runAmbForms = (
  evaluator: AmbEvaluator,
  forms: ReadonlyArray<Value>,
  env: Env,
  limit: number = Number.POSITIVE_INFINITY,
): Effect.Effect<AmbRun, EvaluationError> =>
  Effect.gen(function* () {
    const terminalFail: Fail = () => Effect.fail(new AmbExhausted());
    const head = forms.slice(0, -1);
    const last = forms.at(-1);
    for (const form of head) {
      yield* evaluator.ambeval(form, env, (value) => Effect.succeed(value), terminalFail);
    }
    if (last === undefined) {
      return { answers: [], exhausted: true };
    }
    const answers: Value[] = [];
    let exhausted = false;
    let next: Fail | undefined;
    const record: Succeed = (value, fail) => {
      next = fail;
      return Effect.succeed(value);
    };
    while (answers.length < limit && !exhausted) {
      const outcome = yield* Effect.result(
        next === undefined ? evaluator.ambeval(last, env, record, terminalFail) : next(),
      );
      if (outcome._tag === "Failure") {
        if (outcome.failure instanceof AmbExhausted) {
          exhausted = true;
        } else {
          return yield* Effect.fail(outcome.failure);
        }
      } else {
        answers.push(outcome.success);
      }
    }
    return { answers, exhausted };
  });

/** Reads every form of `text` and runs it with `runAmbForms`: the exercises'
 * entry point. */
export const runAmbText = (
  evaluator: AmbEvaluator,
  text: string,
  env: Env,
  limit: number = Number.POSITIVE_INFINITY,
): Effect.Effect<AmbRun, EvaluationError> => runAmbForms(evaluator, readAll(text), env, limit);
