// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.2

/**
 * The lazy evaluator: the book's section 4.2 in this edition's rendering.
 * The language is the one `01-metacircular.ts` evaluates except that
 * compound procedures are non-strict in each argument: applying one delays
 * its operands into thunks, and a thunk's expression evaluates only when
 * its value is demanded, by a strict primitive, an `if` predicate, an
 * operator position, or the driver loop before printing. Primitives stay
 * strict; quoted data stays ordinary data.
 *
 * The module reuses the 4.1 exports wholesale: the syntax predicates and
 * selectors, the environment operations, `makeProcedure`,
 * `applyPrimitiveProcedure`, and the primitive table of
 * `setupEnvironment`. What is new here is the thunk machinery (4.2.2),
 * the modified dispatch, the `L-Eval` driver, and the lazy-pair helpers
 * of 4.2.3. The dispatch is built once by `makeLazyEvaluator` over a
 * `LazyTuning` record; the section's evaluator is the default, and each
 * tuning knob is the seam one exercise needs (the memoization flip of
 * 4.29, Cy's sequence of 4.30, the lifted quotes of 4.33, the printable
 * pairs of 4.34, and the declared-strictness lookup of 4.31).
 */
import { Effect } from "effect";
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
  firstExp,
  firstOperand,
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
  isLastExp,
  isQuoted,
  isSelfEvaluating,
  isTrue,
  isVariable,
  lambdaBody,
  lambdaParameters,
  lookupVariableValue,
  makeProcedure,
  noOperands,
  ok,
  operands,
  operator,
  restExps,
  restOperands,
  type Sink,
  setupEnvironment,
  setVariableValue,
  symbol,
  taggedList,
  textOfQuotation,
} from "./01-metacircular.js";
import type { CompoundProc, Env, Evaluate, ThunkValue, Value } from "./core.js";
import { type EvaluationError, NotAProcedure, RuntimeError, UnknownSyntax } from "./errors.js";
import { type Cons, cons, type List, nil, toArray } from "./list.js";
import { format, ReadError, read } from "./read.js";

// ---------------------------------------------------------------------
// 4.2.2 Representing thunks
// ---------------------------------------------------------------------

/** The book's `thunk?`: exactly the values `delayIt` builds. */
export const isThunk = (value: Value): value is ThunkValue => value._tag === "Thunk";

/** The book's `thunk-exp`: the delayed expression, or the stored value
 * of an already forced memoized thunk. */
export const thunkExp = (thunk: ThunkValue): Value => thunk.exp;

/** The book's `thunk-env`. */
export const thunkEnv = (thunk: ThunkValue): Env => thunk.env;

/** The book's `delay-it`: packages the operand expression with the
 * environment of the application that delayed it. The section's thunks
 * memoize; `memoized = false` delays into a wrapper whose every forcing
 * re-evaluates, the discipline exercise 4.29 measures and exercise 4.31
 * declares per parameter. */
export const delayIt = (exp: Value, env: Env, memoized = true): ThunkValue => ({
  _tag: "Thunk",
  exp,
  env,
  evaluated: false,
  memoized,
});

// ---------------------------------------------------------------------
// 4.2.2 The evaluator changes
// ---------------------------------------------------------------------

/**
 * The knobs the exercise variants flip; the section leaves every knob at
 * its default. `delayOperand` decides how one operand of a compound
 * procedure binds: `true` delays it into a memoizing thunk, `false` into
 * a recomputing one, and `undefined` evaluates it now, the strict
 * upward-compatible default of exercise 4.31. `cySequence` is Cy D.
 * Fect's proposal of exercise 4.30: non-final expressions of a sequence
 * force instead of only evaluating. `liftQuotedLists` is exercise 4.33:
 * a quoted proper list lifts into the `cons` chain that builds the same
 * elements as true lazy pairs. `printablePairs` is exercise 4.34:
 * `(cons a b)` builds the tagged two-thunk pair the printer identifies.
 */
export interface LazyTuning {
  readonly delayOperand?: (procedure: CompoundProc, position: number) => boolean | undefined;
  readonly cySequence?: boolean;
  readonly liftQuotedLists?: boolean;
  readonly printablePairs?: boolean;
}

/**
 * One lazy evaluator: the dispatch and the forcing machinery bound to
 * each other, the book's mutually recursive `eval`, `apply`,
 * `actual-value`, and `force-it` as first-class procedures. Exercise
 * variants build a variant by tuning; the section's evaluator is
 * `lazyEvaluator`.
 */
export interface LazyEvaluator {
  /** The book's lazy `eval`: one case analysis over the expression. */
  readonly evaluate: Evaluate;
  /** The book's `actual-value`: `evaluate` followed by `force`. */
  readonly actualValue: (exp: Value, env: Env) => Effect.Effect<Value, EvaluationError>;
  /** The book's `force-it`: a thunk computes once and, if memoized,
   * stores its value; anything else answers unchanged. */
  readonly force: (value: Value) => Effect.Effect<Value, EvaluationError>;
  /** The book's lazy `apply`: the operands arrive unevaluated, and a
   * primitive forces them while a compound procedure delays them. */
  readonly applyProcedure: (
    procedure: Value,
    argumentExps: List<Value>,
    env: Env,
  ) => Effect.Effect<Value, EvaluationError>;
}

const quoted = (datum: Value): Value => cons(symbol("quote"), cons(datum, nil));

/** The items of a non-empty proper list, or nothing for any other datum:
 * the quote shapes exercise 4.33 lifts into lazy pairs. */
const properListItems = (datum: Value): ReadonlyArray<Value> | undefined => {
  if (datum._tag !== "Cons" && datum._tag !== "Nil") {
    return undefined;
  }
  const items = toArray(datum);
  return items.length > 0 ? items : undefined;
};

/** Exercise 4.33's lift: a quotation of a non-empty proper list rewrites
 * into the `cons` chain of its quoted elements, so the list the driver
 * hands out is built by the object language's own `cons`; atoms, the
 * empty list, and dotted tails stay ordinary data. */
export const liftQuotedList = (datum: Value): Value | undefined => {
  const items = properListItems(datum);
  if (items === undefined) {
    return undefined;
  }
  return items.reduceRight<Value>(
    (tail, item) => cons(symbol("cons"), cons(quoted(item), cons(tail, nil))),
    quoted(nil),
  );
};

const pairOperands = (exps: List<Value>): readonly [Value, Value] | undefined => {
  if (exps._tag !== "Cons" || exps.tail._tag !== "Cons" || exps.tail.tail._tag !== "Nil") {
    return undefined;
  }
  return [exps.head, exps.tail.head];
};

/** The tagged lazy pair of exercise 4.34: `(lazy-pair <thunk> <thunk>)`,
 * a list-structured pair whose two slots are the delayed operands, so
 * the printer and the lazy `car` and `cdr` can identify it without
 * forcing either slot. */
export const makeLazyPair = (headExp: Value, tailExp: Value, env: Env): Value => {
  const slots: List<Value> = cons(delayIt(headExp, env), cons(delayIt(tailExp, env), nil));
  return cons(symbol("lazy-pair"), slots);
};

/** The two delayed slots of a tagged lazy pair, or nothing for any other
 * value. */
export const lazyPairSlots = (
  value: Value,
): { readonly head: ThunkValue; readonly tail: ThunkValue } | undefined => {
  if (!taggedList("lazy-pair", value)) {
    return undefined;
  }
  const rest = value.tail;
  if (rest._tag !== "Cons" || rest.tail._tag !== "Cons" || rest.tail.tail._tag !== "Nil") {
    return undefined;
  }
  const head = rest.head;
  const tail = rest.tail.head;
  if (!isThunk(head) || !isThunk(tail)) {
    return undefined;
  }
  return { head, tail };
};

/** Builds one lazy evaluator over the tuning. The internal procedures
 * close over the dispatch, so a thunk's expression evaluates under the
 * same evaluator that delayed it, and a forcing anywhere in that
 * evaluation uses the same forcing discipline. */
export const makeLazyEvaluator = (tuning: LazyTuning = {}): LazyEvaluator => {
  const delayOperand = tuning.delayOperand;
  const cySequence = tuning.cySequence === true;
  const liftQuotedLists = tuning.liftQuotedLists === true;
  const printablePairs = tuning.printablePairs === true;

  const force = (value: Value): Effect.Effect<Value, EvaluationError> => {
    if (!isThunk(value)) {
      return Effect.succeed(value);
    }
    if (value.evaluated) {
      return Effect.succeed(value.exp);
    }
    return Effect.flatMap(evaluate(value.exp, value.env), (result) =>
      Effect.map(force(result), (finalValue) => {
        if (value.memoized) {
          value.exp = finalValue;
          value.evaluated = true;
        }
        return finalValue;
      }),
    );
  };

  const actualValue = (exp: Value, env: Env): Effect.Effect<Value, EvaluationError> =>
    Effect.flatMap(evaluate(exp, env), force);

  /** The book's `list-of-arg-values`: strict primitives see values. */
  const listOfArgValues = (
    exps: List<Value>,
    env: Env,
  ): Effect.Effect<List<Value>, EvaluationError> => {
    if (noOperands(exps)) {
      return Effect.succeed(nil);
    }
    return Effect.flatMap(actualValue(firstOperand(exps), env), (first) =>
      Effect.map(listOfArgValues(restOperands(exps), env), (rest) => cons(first, rest)),
    );
  };

  /** Binds one operand of a compound procedure per the tuning: memoized,
   * recomputing, or (the strict default of exercise 4.31) evaluated now. */
  const listOfDelayedArgs = (
    procedure: CompoundProc,
    exps: List<Value>,
    env: Env,
  ): Effect.Effect<List<Value>, EvaluationError> => {
    const loop = (
      position: number,
      rest: List<Value>,
    ): Effect.Effect<List<Value>, EvaluationError> => {
      if (noOperands(rest)) {
        return Effect.succeed(nil);
      }
      const exp = firstOperand(rest);
      const mode = delayOperand?.(procedure, position) ?? true;
      const bound =
        mode === true
          ? Effect.succeed(delayIt(exp, env))
          : mode === false
            ? Effect.succeed(delayIt(exp, env, false))
            : actualValue(exp, env);
      return Effect.flatMap(bound, (value) =>
        Effect.map(loop(position + 1, restOperands(rest)), (others) => cons(value, others)),
      );
    };
    return loop(0, exps);
  };

  /** The book's `eval-sequence`: every expression but the last is
   * evaluated, none forced. Cy's proposal (exercise 4.30) forces the
   * non-final expressions instead. */
  const evalSequence = (exps: List<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
    if (exps._tag === "Nil") {
      return Effect.fail(new RuntimeError({ message: "Empty sequence: L-EVAL", detail: "" }));
    }
    if (isLastExp(exps)) {
      return evaluate(firstExp(exps), env);
    }
    const first = cySequence ? actualValue(firstExp(exps), env) : evaluate(firstExp(exps), env);
    return Effect.flatMap(first, () => evalSequence(restExps(exps), env));
  };

  /** The book's `eval-if`: the predicate is forced before the branches
   * are chosen; the chosen branch is evaluated, not forced. */
  const evalIf = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
    Effect.flatMap(actualValue(ifPredicate(exp), env), (predicate) =>
      isTrue(predicate) ? evaluate(ifConsequent(exp), env) : evaluate(ifAlternative(exp), env),
    );

  /** The book's `eval-assignment`: the value expression is evaluated,
   * not forced, so an assignment can store a thunk. */
  const evalAssignment = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
    Effect.flatMap(evaluate(assignmentValue(exp), env), (value) =>
      Effect.map(setVariableValue(assignmentVariable(exp), value, env), () => ok),
    );

  /** The book's `eval-definition`. */
  const evalDefinition = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
    Effect.flatMap(evaluate(definitionValue(exp), env), (value) =>
      Effect.map(defineVariableValue(definitionVariable(exp), value, env), () => ok),
    );

  /** The book's quoted clause; under exercise 4.33's tuning a quoted
   * proper list evaluates as the `cons` chain of its quoted elements. */
  const evalQuoted = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
    const datum = textOfQuotation(exp);
    if (liftQuotedLists) {
      const lifted = liftQuotedList(datum);
      if (lifted !== undefined) {
        return evaluate(lifted, env);
      }
    }
    return Effect.succeed(datum);
  };

  /** The book's lazy `apply`: a primitive is strict, so its operands
   * are forced before the call; a compound procedure is non-strict, so
   * every operand binds per the tuning and the body runs in the extended
   * environment. */
  const applyProcedure = (
    procedure: Value,
    argumentExps: List<Value>,
    env: Env,
  ): Effect.Effect<Value, EvaluationError> => {
    if (procedure._tag === "Primitive") {
      return Effect.flatMap(listOfArgValues(argumentExps, env), (args) => {
        if (printablePairs) {
          const addressed = addressLazySlot(procedure, args);
          if (addressed !== undefined) {
            return addressed;
          }
        }
        return applyPrimitiveProcedure(procedure, args);
      });
    }
    if (procedure._tag === "Compound") {
      return Effect.flatMap(listOfDelayedArgs(procedure, argumentExps, env), (args) =>
        Effect.flatMap(extendEnvironment(procedure.params, args, procedure.env), (newEnv) =>
          evalSequence(procedure.body, newEnv),
        ),
      );
    }
    return Effect.fail(new NotAProcedure({ value: format(procedure) }));
  };

  /** Exercise 4.34's `car` and `cdr`: on a tagged lazy pair the
   * addressed slot forces and answers; any other argument falls through
   * to the host function, which fails on it as on ordinary data. */
  const addressLazySlot = (
    procedure: Extract<Value, { _tag: "Primitive" }>,
    args: List<Value>,
  ): Effect.Effect<Value, EvaluationError> | undefined => {
    if (procedure.name !== "car" && procedure.name !== "cdr") {
      return undefined;
    }
    if (args._tag !== "Cons" || args.tail._tag !== "Nil") {
      return undefined;
    }
    const slots = lazyPairSlots(args.head);
    if (slots === undefined) {
      return undefined;
    }
    return procedure.name === "car" ? force(slots.head) : force(slots.tail);
  };

  const application = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> =>
    Effect.flatMap(actualValue(operator(exp), env), (procedure) =>
      applyProcedure(procedure, operands(exp), env),
    );

  /** Exercise 4.34's cons clause: a two-operand application of the
   * variable `cons`, resolved to the installed `cons` primitive, builds
   * the tagged lazy pair instead of running the strict constructor, so
   * neither slot computes and the pair prints on demand. */
  const printableCons = (exp: Cons<Value>, env: Env): Effect.Effect<Value, EvaluationError> => {
    const op = operator(exp);
    if (!isVariable(op) || op.name !== "cons") {
      return application(exp, env);
    }
    const pair = pairOperands(operands(exp));
    if (pair === undefined) {
      return application(exp, env);
    }
    return Effect.flatMap(lookupVariableValue(op, env), (consValue) => {
      if (consValue._tag !== "Primitive" || consValue.name !== "cons") {
        return application(exp, env);
      }
      return Effect.succeed(makeLazyPair(pair[0], pair[1], env));
    });
  };

  const evaluate: Evaluate = (exp, env) => {
    if (isSelfEvaluating(exp)) {
      return Effect.succeed(exp);
    }
    if (isVariable(exp)) {
      return lookupVariableValue(exp, env);
    }
    if (isQuoted(exp)) {
      return evalQuoted(exp, env);
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
      return printablePairs ? printableCons(exp, env) : application(exp, env);
    }
    return Effect.fail(new UnknownSyntax({ expr: format(exp) }));
  };

  return { evaluate, actualValue, force, applyProcedure };
};

// ---------------------------------------------------------------------
// The section's evaluator and the lazy driver
// ---------------------------------------------------------------------

/** The section's evaluator: compound procedures delay every operand
 * into a memoizing thunk, sequences follow the text's rule, quoted data
 * stays ordinary, and `cons` stays strict. */
export const lazyEvaluator = makeLazyEvaluator();

/** The section's lazy `eval`. */
export const evaluateLazy = lazyEvaluator.evaluate;

/** The book's `actual-value` of the section. */
export const actualValue = lazyEvaluator.actualValue;

/** The book's `force-it` of the section. */
export const forceIt = lazyEvaluator.force;

/** The book's lazy `apply` of the section. */
export const applyLazy = lazyEvaluator.applyProcedure;

/** Reads one form and evaluates it with `actual-value`, the driver's
 * step; a read failure lands on the error channel as a `RuntimeError`. */
export const evalLazyString = (text: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(
    Effect.try({
      try: () => read(text),
      catch: (e) =>
        new RuntimeError({
          message: "read failed",
          detail: e instanceof ReadError || e instanceof Error ? e.message : String(e),
        }),
    }),
    (exp) => actualValue(exp, env),
  );

/** The book's lazy driver loop over a finite session: prompts with the
 * section's `;;; L-Eval` lines and forces each answer before printing,
 * so a delayed value never reaches the surface. */
export const lazyDriverLoop = (
  env: Env,
  inputs: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.forEach(inputs, (input) =>
    Effect.flatMap(evalLazyString(input, env), (value) =>
      Effect.succeed([";;; L-Eval input:", input, ";;; L-Eval value:", format(value)]),
    ),
  ).pipe(Effect.map((lines) => lines.flat()));

/** Drives a finite session with any evaluator of this module, the
 * section driver's rule over a tuned dispatch: every answer is forced
 * before printing, and `display` and `newline` write to the sink. The
 * exercise variants (the unmemoized twin, Cy's sequence, the lifted
 * quotes) drive their sessions through this loop. */
export const lazyDriverWith = (
  evaluator: LazyEvaluator,
  inputs: ReadonlyArray<string>,
  sink: Sink = () => {},
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupLazyEnvironment(sink), (env) =>
    Effect.forEach(inputs, (input) =>
      Effect.flatMap(
        Effect.flatMap(evaluator.evaluate(read(input), env), evaluator.force),
        (value) => Effect.succeed([";;; L-Eval input:", input, ";;; L-Eval value:", format(value)]),
      ),
    ).pipe(Effect.map((lines) => lines.flat())),
  );

/** The printable driver of exercise 4.34: the section driver with the
 * lazy-pair rendering in place of `format`, so a lazy list prints its
 * budgeted prefix and an infinite one still answers. */
export const printableDriverLoop = (
  evaluator: LazyEvaluator,
  inputs: ReadonlyArray<string>,
  budget: number = LAZY_PRINT_BUDGET,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupLazyEnvironment(), (env) =>
    Effect.forEach(inputs, (input) =>
      Effect.flatMap(
        Effect.flatMap(evaluator.evaluate(read(input), env), evaluator.force),
        (value) =>
          Effect.map(renderLazyValue(evaluator, value, budget), (rendered) => [
            ";;; L-Eval input:",
            input,
            ";;; L-Eval value:",
            rendered,
          ]),
      ),
    ).pipe(Effect.map((lines) => lines.flat())),
  );

/** A fresh global environment for a lazy session: the 4.1 setup, the
 * same primitive table the book's `setup-environment` builds. */
export const setupLazyEnvironment = setupEnvironment;

// ---------------------------------------------------------------------
// 4.2.3 Lazy pairs and the printing budget
// ---------------------------------------------------------------------

/** The lazy-printing budget of exercise 4.34: a lazy list prints its
 * first ten elements, each forced once, and the unprinted tail prints as
 * the ellipsis. The printer never forces past the budget, nor the tail
 * of an unprinted element, so an infinite lazy list still renders. */
export const LAZY_PRINT_BUDGET = 10;

const renderForced = (
  evaluator: LazyEvaluator,
  value: Value,
  budget: number,
): Effect.Effect<string, EvaluationError> => {
  const slots = lazyPairSlots(value);
  if (slots === undefined) {
    return Effect.succeed(format(value));
  }
  return Effect.flatMap(evaluator.force(slots.head), (head) =>
    Effect.flatMap(renderForced(evaluator, head, budget), (headText) => {
      if (budget <= 1) {
        return Effect.succeed(`(${headText} ...)`);
      }
      return Effect.flatMap(evaluator.force(slots.tail), (tail) =>
        Effect.map(renderTail(evaluator, tail, budget), (tailText) => `(${headText}${tailText}`),
      );
    }),
  );
};

const renderTail = (
  evaluator: LazyEvaluator,
  tail: Value,
  budget: number,
): Effect.Effect<string, EvaluationError> => {
  if (lazyPairSlots(tail) !== undefined) {
    return Effect.map(renderForced(evaluator, tail, budget - 1), (rest) => ` ${rest.slice(1)}`);
  }
  if (tail._tag === "Nil") {
    return Effect.succeed(")");
  }
  if (tail._tag === "Cons") {
    // An ordinary pair renders inside the same parentheses.
    return Effect.succeed(` ${format(tail).slice(1, -1)})`);
  }
  return Effect.succeed(` . ${format(tail)})`);
};

/** Renders one forced top-level value the printable driver prints: lazy
 * pairs render their prefix under the budget, every other value per
 * `format`. An element that is itself a lazy pair gets the full budget
 * again, so an infinite tree of lazy pairs still renders finitely. */
export const renderLazyValue = (
  evaluator: LazyEvaluator,
  value: Value,
  budget: number = LAZY_PRINT_BUDGET,
): Effect.Effect<string, EvaluationError> =>
  Effect.flatMap(evaluator.force(value), (forced) => renderForced(evaluator, forced, budget));
