// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.3

import {
  applyBinaryOperation,
  applyUnaryOperation,
  type RunResult,
  Session,
  splitParams,
} from "./01-metacircular.ts";
import type { Cell, Env } from "./runtime/env.ts";
import { child, findCell, makeCell } from "./runtime/env.ts";
import type { GuestError, Outcome } from "./runtime/errors.ts";
import { fail, ok } from "./runtime/errors.ts";
import {
  isArrayValue,
  isClosure,
  isMapValue,
  isPrimitive,
  isRecordValue,
  makeArray,
  makeClosure,
  makeErrorValue,
  makeMap,
  makeRecord,
  makeSet,
  type Value,
} from "./runtime/value.ts";
/**
 * The named search experiments (host-subsets grammar section 6):
 * `amb-depth-first-experiment` runs alternatives left-to-right depth-first
 * over the shared checked syntax; `amb-ramb-experiment` runs them in a seeded
 * deterministic random order and is a separately named experiment. `require`
 * fails the current branch; failure re-enters the nearest untried choice.
 * Assignments are undone newest-first during backtracking; `permanentAssign`
 * retains explicit writes, and `ifFail` runs its fallback only after the
 * primary expression exhausts its solutions. Search uses only its named
 * `choose`/`require`/`ramb`/`permanentAssign`/`ifFail` forms, never core
 * TypeScript behavior or the native oracle as if JavaScript were nondeterministic.
 * Every continuation threads its resume: each success carries the failure
 * continuation that re-enters the choice it came from.
 */
import type { Arg, Decl, Expr, Stmt } from "./syntax/ast.ts";
import { admitSource } from "./syntax/check.ts";

/** The two search execution modes. */
export type SearchMode = "amb-depth-first-experiment" | "amb-ramb-experiment";

/** One search run: every answer found, in search order, with its transcript. */
export interface SearchRun {
  readonly answers: ReadonlyArray<Value>;
  readonly transcript: ReadonlyArray<string>;
  readonly outcome: Outcome;
  /** Failed candidate computations that scheduled backtracking; exhaustion propagation does not count. */
  readonly failures: number;
  /** Deferred continuations actually executed, including successful resumptions. */
  readonly steps: number;
  /** `completed` when the retry queue drained; `cut-off` when a limit stopped it. */
  readonly status: "completed" | "cut-off";
}

/** Limits checked before each pending continuation; `maxSteps` excludes initial evaluation. */
export interface SearchLimits {
  /** Maximum answers returned; zero stops before evaluating guest source. */
  readonly maxAnswers?: number;
  /** Maximum deferred continuations executed. */
  readonly maxSteps?: number;
}

/** Success receives the found value and the continuation that resumes the search. */
type Success = (value: Value, next: Failure) => void;
/** Failure re-enters the nearest untried alternative. */
type Failure = () => void;
/** Prior target state used to undo one successful assignment on backtracking. */
interface TargetSnapshot {
  readonly restore: () => Outcome;
}
type TargetCapture =
  | { readonly tag: "snapshot"; readonly snapshot: TargetSnapshot }
  | { readonly tag: "error"; readonly error: GuestError };
/** A resolved target reference, captured before evaluating the right-hand side. */
interface TargetLocation {
  readonly capture: () => TargetCapture;
  readonly write: (value: Value) => Outcome;
}

class Searcher {
  readonly session = new Session("core");
  readonly #ramb: boolean;
  #randomState: number;

  constructor(ramb: boolean, seed: number) {
    this.#ramb = ramb;
    this.#randomState = seed >>> 0 || 1;
  }

  readonly #deferred: Failure[] = [];
  #failures = 0;
  #steps = 0;

  /** Defers a failed candidate computation and records one actual failure. */
  retry(fail: Failure): void {
    this.#failures += 1;
    this.defer(fail);
  }

  /** Defers a continuation without counting a new failure. */
  defer(continuation: Failure): void {
    this.#deferred.push(continuation);
  }

  /** Whether another deferred continuation can be executed. */
  get hasPending(): boolean {
    return this.#deferred.length > 0;
  }

  /** Failed computations observed so far. */
  get failures(): number {
    return this.#failures;
  }

  /** Deferred continuations executed so far. */
  get steps(): number {
    return this.#steps;
  }

  /** Runs one deferred continuation; does not count an empty queue poll. */
  step(): void {
    const work = this.#deferred.shift();
    if (work === undefined) {
      return;
    }
    this.#steps += 1;
    work();
  }

  #assignWithUndo(target: Expr, valueExpr: Expr, env: Env, succeed: Success, fail: Failure): void {
    this.resolveTarget(
      target,
      env,
      (location, targetNext) => {
        this.evalExpr(
          valueExpr,
          env,
          (value, valueNext) => this.#commitReversible(location, value, valueNext, succeed),
          targetNext,
        );
      },
      fail,
    );
  }

  #commitReversible(location: TargetLocation, value: Value, next: Failure, succeed: Success): void {
    const captured = location.capture();
    if (captured.tag === "error") {
      this.retry(next);
      return;
    }
    if (location.write(value).tag === "error") {
      this.retry(next);
      return;
    }
    succeed(value, () => this.#restoreAndResume(captured.snapshot, next));
  }

  #restoreAndResume(snapshot: TargetSnapshot, next: Failure): void {
    if (snapshot.restore().tag === "error") {
      this.retry(next);
      return;
    }
    next();
  }

  #assignPermanently(
    target: Expr,
    valueExpr: Expr,
    env: Env,
    succeed: Success,
    fail: Failure,
  ): void {
    this.resolveTarget(
      target,
      env,
      (location, targetNext) => {
        this.evalExpr(
          valueExpr,
          env,
          (value, valueNext) => this.#commitPermanent(location, value, valueNext, succeed),
          targetNext,
        );
      },
      fail,
    );
  }

  #commitPermanent(location: TargetLocation, value: Value, next: Failure, succeed: Success): void {
    if (location.write(value).tag === "error") {
      this.retry(next);
      return;
    }
    succeed(undefined, next);
  }

  // ------------------------------------------------------------------
  // Expressions
  // ------------------------------------------------------------------

  evalExpr(expr: Expr, env: Env, succeed: Success, fail: Failure): void {
    switch (expr.tag) {
      case "number":
      case "string":
      case "boolean":
        succeed(expr.value, fail);
        return;
      case "null":
        succeed(null, fail);
        return;
      case "undefined":
        succeed(undefined, fail);
        return;
      case "template":
        this.evalTemplate(expr.chunks, expr.exprs, env, "", succeed, fail);
        return;
      case "variable": {
        const cell = findCell(env, expr.name);
        if (cell === undefined || !cell.initialized) {
          this.retry(fail);
          return;
        }
        succeed(cell.value, fail);
        return;
      }
      case "array":
        this.evalOperands(
          expr.elements,
          env,
          (values, next) => succeed(makeArray(values), next),
          fail,
        );
        return;
      case "object":
        this.evalFields(expr.fields, env, 0, [], succeed, fail);
        return;
      case "unary":
        this.evalExpr(
          expr.operand,
          env,
          (value, next) => {
            const result = applyUnaryOperation(expr.op, value);
            if (result.tag === "error") {
              this.retry(next);
              return;
            }
            succeed(result.value, next);
          },
          fail,
        );
        return;
      case "binary":
        this.evalExpr(
          expr.left,
          env,
          (left, leftNext) => {
            this.evalExpr(
              expr.right,
              env,
              (right, rightNext) => {
                const result = applyBinaryOperation(expr.op, left, right);
                if (result.tag === "error") {
                  this.retry(rightNext);
                  return;
                }
                succeed(result.value, rightNext);
              },
              leftNext,
            );
          },
          fail,
        );
        return;
      case "logical":
        this.evalExpr(
          expr.left,
          env,
          (left, leftNext) => {
            if (typeof left !== "boolean") {
              this.retry(leftNext);
              return;
            }
            const shortCircuits = expr.op === "&&" ? !left : left;
            if (shortCircuits) {
              succeed(left, leftNext);
              return;
            }
            this.evalExpr(
              expr.right,
              env,
              (right, rightNext) => {
                if (typeof right !== "boolean") {
                  this.retry(rightNext);
                  return;
                }
                succeed(right, rightNext);
              },
              leftNext,
            );
          },
          fail,
        );
        return;
      case "conditional":
        this.evalExpr(
          expr.test,
          env,
          (test, testNext) => {
            this.evalExpr(
              test === true ? expr.consequent : expr.alternative,
              env,
              succeed,
              testNext,
            );
          },
          fail,
        );
        return;
      case "assign":
        this.#assignWithUndo(expr.target, expr.value, env, succeed, fail);
        return;
      case "permanent-assign":
        this.#assignPermanently(expr.target, expr.value, env, succeed, fail);
        return;
      case "if-fail":
        this.evalExpr(expr.expression, env, succeed, () => {
          this.evalExpr(expr.fallback, env, succeed, fail);
        });
        return;
      case "arrow": {
        const { params, required, rest } = splitParams(expr.params);
        succeed(makeClosure(params, required, rest, expr.body, env), fail);
        return;
      }
      case "call": {
        const callee = expr.callee;
        if (
          callee.tag === "member" &&
          callee.object.tag === "variable" &&
          callee.object.name === "console" &&
          callee.name === "log"
        ) {
          this.evalOperands(
            expr.args,
            env,
            (values, next) => {
              if (values.length !== 1) {
                this.retry(next);
                return;
              }
              this.session.transcript.push(this.session.render(values[0]));
              succeed(undefined, next);
            },
            fail,
          );
          return;
        }
        if (callee.tag === "member") {
          this.evalExpr(
            callee.object,
            env,
            (receiver, receiverNext) => {
              this.evalOperands(
                expr.args,
                env,
                (values, argsNext) => {
                  const result = this.session.callMember(receiver, callee.name, values);
                  if (result.tag === "error") {
                    this.retry(argsNext);
                    return;
                  }
                  succeed(result.value, argsNext);
                },
                receiverNext,
              );
            },
            fail,
          );
          return;
        }
        this.evalExpr(
          expr.callee,
          env,
          (target, targetNext) => {
            this.evalOperands(
              expr.args,
              env,
              (values, argsNext) => {
                this.apply(target, values, succeed, argsNext);
              },
              targetNext,
            );
          },
          fail,
        );
        return;
      }
      case "member":
        this.evalExpr(
          expr.object,
          env,
          (object, next) => {
            const result = this.session.memberGet(object, expr.name);
            if (result.tag === "error") {
              this.retry(next);
              return;
            }
            succeed(result.value, next);
          },
          fail,
        );
        return;
      case "index":
        this.evalExpr(
          expr.object,
          env,
          (object, objectNext) => {
            this.evalExpr(
              expr.index,
              env,
              (index, indexNext) => {
                const result = this.session.indexGet(object, index);
                if (result.tag === "error") {
                  this.retry(indexNext);
                  return;
                }
                succeed(result.value, indexNext);
              },
              objectNext,
            );
          },
          fail,
        );
        return;
      case "new-error":
        this.evalExpressions(
          expr.args,
          env,
          (values, next) => {
            const message = values[0];
            succeed(
              makeErrorValue(message === undefined ? "" : this.session.render(message)),
              next,
            );
          },
          fail,
        );
        return;
      case "new-map":
        this.evalExpressions(
          expr.args,
          env,
          (values, next) => {
            const entries = values[0];
            if (entries === undefined) {
              succeed(makeMap(), next);
              return;
            }
            if (!isArrayValue(entries)) {
              this.retry(next);
              return;
            }
            const pairs: Array<readonly [Value, Value]> = [];
            for (const pair of entries.items) {
              if (!isArrayValue(pair) || pair.items.length !== 2) {
                this.retry(next);
                return;
              }
              pairs.push([pair.items[0], pair.items[1]]);
            }
            succeed(makeMap(pairs), next);
          },
          fail,
        );
        return;
      case "new-set":
        this.evalExpressions(
          expr.args,
          env,
          (values, next) => {
            const items = values[0];
            if (items === undefined) {
              succeed(makeSet(), next);
              return;
            }
            if (!isArrayValue(items)) {
              this.retry(next);
              return;
            }
            succeed(makeSet(items.items), next);
          },
          fail,
        );
        return;
      case "choose":
      case "ramb": {
        const ordered =
          expr.tag === "ramb" ? this.shuffle([...expr.alternatives]) : expr.alternatives;
        this.evalAlternatives(ordered, 0, env, succeed, fail);
        return;
      }
      case "require":
        this.evalExpr(
          expr.condition,
          env,
          (condition, next) => {
            if (condition === true) {
              succeed(undefined, next);
              return;
            }
            this.retry(next);
          },
          fail,
        );
        return;
      default:
        this.retry(fail);
    }
  }

  evalTemplate(
    chunks: ReadonlyArray<string>,
    exprs: ReadonlyArray<Expr>,
    env: Env,
    collected: string,
    succeed: Success,
    fail: Failure,
  ): void {
    const runAt = (at: number, acc: string, resume: Failure): void => {
      const prefix = acc + (chunks[at] ?? "");
      const inner = exprs[at];
      if (inner === undefined) {
        succeed(prefix, resume);
        return;
      }
      this.evalExpr(
        inner,
        env,
        (value, next) => {
          runAt(at + 1, prefix + this.session.render(value), () => next());
        },
        resume,
      );
    };
    runAt(0, collected, fail);
  }

  evalOperands(
    args: ReadonlyArray<Arg>,
    env: Env,
    succeed: (values: Value[], next: Failure) => void,
    fail: Failure,
  ): void {
    const runAt = (index: number, collected: ReadonlyArray<Value>, resume: Failure): void => {
      const arg = args[index];
      if (arg === undefined) {
        succeed([...collected], resume);
        return;
      }
      this.evalExpr(
        arg.expr,
        env,
        (value, next) => {
          if (arg.kind === "spread") {
            if (!isArrayValue(value)) {
              this.retry(next);
              return;
            }
            runAt(index + 1, [...collected, ...value.items], () => next());
            return;
          }
          runAt(index + 1, [...collected, value], () => next());
        },
        resume,
      );
    };
    runAt(0, [], fail);
  }

  evalExpressions(
    expressions: ReadonlyArray<Expr>,
    env: Env,
    succeed: (values: Value[], next: Failure) => void,
    fail: Failure,
  ): void {
    const runAt = (index: number, collected: ReadonlyArray<Value>, resume: Failure): void => {
      const expression = expressions[index];
      if (expression === undefined) {
        succeed([...collected], resume);
        return;
      }
      this.evalExpr(
        expression,
        env,
        (value, next) => {
          runAt(index + 1, [...collected, value], () => next());
        },
        resume,
      );
    };
    runAt(0, [], fail);
  }

  evalFields(
    fields: ReadonlyArray<{ key: string; value: Expr }>,
    env: Env,
    index: number,
    collected: ReadonlyArray<readonly [string, Value]>,
    succeed: Success,
    fail: Failure,
  ): void {
    const field = fields[index];
    if (field === undefined) {
      succeed(makeRecord(collected), fail);
      return;
    }
    this.evalExpr(
      field.value,
      env,
      (value, next) => {
        const entry: readonly [string, Value] = [field.key, value];
        this.evalFields(fields, env, index + 1, [...collected, entry], succeed, () => next());
      },
      fail,
    );
  }

  evalAlternatives(
    alternatives: ReadonlyArray<Expr>,
    index: number,
    env: Env,
    succeed: Success,
    fail: Failure,
  ): void {
    const alternative = alternatives[index];
    if (alternative === undefined) {
      if (index === 0) {
        this.retry(fail);
      } else {
        this.defer(fail);
      }
      return;
    }
    this.evalExpr(alternative, env, succeed, () => {
      this.evalAlternatives(alternatives, index + 1, env, succeed, fail);
    });
  }

  apply(procedure: Value, args: ReadonlyArray<Value>, succeed: Success, fail: Failure): void {
    if (isPrimitive(procedure)) {
      const result = procedure.fn(args);
      if (result.tag === "error") {
        this.retry(fail);
        return;
      }
      succeed(result.value, fail);
      return;
    }
    if (!isClosure(procedure)) {
      this.retry(fail);
      return;
    }
    const total = procedure.params.length;
    const fits =
      args.length >= procedure.required && (procedure.rest !== null || args.length <= total);
    if (!fits) {
      this.retry(fail);
      return;
    }
    const frame = child(procedure.env);
    for (let i = 0; i < total; i += 1) {
      const name = procedure.params[i];
      if (name !== undefined) {
        frame.bindings.set(name, makeCell(args[i], true));
      }
    }
    if (procedure.rest !== null) {
      frame.bindings.set(procedure.rest, makeCell(makeArray(args.slice(total)), true));
    }
    this.execBody(procedure.body.body, frame, succeed, fail);
  }

  // ------------------------------------------------------------------
  // Statements
  // ------------------------------------------------------------------

  execStmt(
    stmt: Stmt,
    env: Env,
    succeed: Success,
    fail: Failure,
    returnFromBody: Success = succeed,
  ): void {
    switch (stmt.tag) {
      case "block":
        this.execBody(stmt.body, env, succeed, fail, returnFromBody);
        return;
      case "if":
        this.evalExpr(
          stmt.test,
          env,
          (test, testNext) => {
            const branch =
              test === true
                ? stmt.consequent
                : (stmt.alternative ?? { tag: "block" as const, body: [], span: stmt.span });
            this.execStmt(branch, env, succeed, testNext, returnFromBody);
          },
          fail,
        );
        return;
      case "while": {
        const iterate = (resume: Failure): void => {
          this.evalExpr(
            stmt.test,
            env,
            (test, testNext) => {
              if (test !== true) {
                succeed(undefined, testNext);
                return;
              }
              this.execStmt(
                stmt.body,
                env,
                (_value, bodyNext) => {
                  iterate(() => this.defer(bodyNext));
                },
                testNext,
                returnFromBody,
              );
            },
            resume,
          );
        };
        iterate(fail);
        return;
      }
      case "for-of":
        this.evalExpr(
          stmt.iterable,
          env,
          (iterable, iterableNext) => {
            if (!isArrayValue(iterable)) {
              this.retry(iterableNext);
              return;
            }
            const items = iterable.items;
            const visitAt = (index: number, resume: Failure): void => {
              if (index >= items.length) {
                succeed(undefined, resume);
                return;
              }
              const item = items[index];
              const frame = child(env);
              frame.bindings.set(stmt.name, makeCell(item, true));
              this.execStmt(
                stmt.body,
                frame,
                (_value, bodyNext) => {
                  visitAt(index + 1, () => this.defer(bodyNext));
                },
                () => visitAt(index + 1, resume),
                returnFromBody,
              );
            };
            visitAt(0, iterableNext);
          },
          fail,
        );
        return;
      case "return": {
        if (stmt.argument === null) {
          returnFromBody(undefined, fail);
          return;
        }
        this.evalExpr(stmt.argument, env, (value, next) => returnFromBody(value, next), fail);
        return;
      }
      case "expr-stmt":
        this.evalExpr(stmt.expr, env, succeed, fail);
        return;
      default:
        this.retry(fail);
    }
  }

  execBody(
    items: ReadonlyArray<Decl | Stmt>,
    env: Env,
    succeed: Success,
    fail: Failure,
    returnFromBody: Success = succeed,
  ): void {
    const frame = child(env);
    this.session.predeclare(items, frame);
    this.execSequence(items, frame, succeed, fail, returnFromBody);
  }

  execSequence(
    items: ReadonlyArray<Decl | Stmt>,
    frame: Env,
    succeed: Success,
    fail: Failure,
    returnFromBody: Success = succeed,
  ): void {
    const runAt = (index: number, last: Value, resume: Failure): void => {
      const item = items[index];
      if (item === undefined) {
        succeed(last, resume);
        return;
      }
      this.execItem(
        item,
        frame,
        (value, next) => {
          runAt(index + 1, value, () => next());
        },
        resume,
        returnFromBody,
      );
    };
    runAt(0, undefined, fail);
  }

  execItem(
    item: Decl | Stmt,
    frame: Env,
    succeed: Success,
    fail: Failure,
    returnFromBody: Success = succeed,
  ): void {
    if (item.tag === "var-decl") {
      this.evalExpr(
        item.init,
        frame,
        (value, next) => {
          const cell = findCell(frame, item.name);
          if (cell === undefined) {
            frame.bindings.set(item.name, makeCell(value, true, item.kind === "let"));
          } else {
            cell.value = value;
            cell.initialized = true;
          }
          succeed(value, next);
        },
        fail,
      );
      return;
    }
    if (item.tag === "function-decl") {
      this.session.predeclare([item], frame);
      succeed(undefined, fail);
      return;
    }
    if (item.tag === "type-decl" || item.tag === "interface-decl" || item.tag === "import") {
      succeed(undefined, fail);
      return;
    }
    this.execStmt(item, frame, succeed, fail, returnFromBody);
  }

  resolveTarget(
    target: Expr,
    env: Env,
    succeed: (target: TargetLocation, next: Failure) => void,
    fail: Failure,
  ): void {
    if (target.tag === "variable") {
      const cell = findCell(env, target.name);
      if (cell === undefined || !cell.initialized) {
        this.retry(fail);
        return;
      }
      succeed(this.#variableTarget(cell, target.name, env), fail);
      return;
    }
    if (target.tag === "member") {
      this.evalExpr(
        target.object,
        env,
        (object, next) => succeed(this.#memberTarget(object, target.name), next),
        fail,
      );
      return;
    }
    if (target.tag === "index") {
      this.evalExpr(
        target.object,
        env,
        (object, objectNext) => {
          this.evalExpr(
            target.index,
            env,
            (index, next) => succeed(this.#indexTarget(object, index), next),
            objectNext,
          );
        },
        fail,
      );
      return;
    }
    this.retry(fail);
  }

  #variableTarget(cell: Cell, name: string, env: Env): TargetLocation {
    return {
      capture: () => {
        const value = cell.value;
        return {
          tag: "snapshot",
          snapshot: {
            restore: () => {
              cell.value = value;
              return ok(value);
            },
          },
        };
      },
      write: (value) => this.session.setVariableValue(name, value, env),
    };
  }

  #memberTarget(object: Value, name: string): TargetLocation {
    return {
      capture: () => this.#captureMember(object, name),
      write: (value) => this.session.memberSet(object, name, value),
    };
  }

  #captureMember(object: Value, name: string): TargetCapture {
    const previous = this.session.memberGet(object, name);
    if (previous.tag === "error") {
      return { tag: "error", error: previous.error };
    }
    const value = previous.value;
    if (!isRecordValue(object)) {
      return {
        tag: "snapshot",
        snapshot: { restore: () => this.session.memberSet(object, name, value) },
      };
    }
    const existed = object.fields.has(name);
    return {
      tag: "snapshot",
      snapshot: {
        restore: () => {
          if (existed) {
            object.fields.set(name, value);
          } else {
            object.fields.delete(name);
          }
          return ok(value);
        },
      },
    };
  }

  #indexTarget(object: Value, index: Value): TargetLocation {
    return {
      capture: () => this.#captureIndex(object, index),
      write: (value) => this.session.indexSet(object, index, value),
    };
  }

  #captureIndex(object: Value, index: Value): TargetCapture {
    const previous = this.session.indexGet(object, index);
    if (previous.tag === "error") {
      return { tag: "error", error: previous.error };
    }
    const value = previous.value;
    if (isArrayValue(object) && typeof index === "number") {
      const length = object.items.length;
      const existed = Number.isInteger(index) && index >= 0 && index < length;
      return {
        tag: "snapshot",
        snapshot: {
          restore: () => {
            if (existed) {
              object.items[index] = value;
            } else {
              object.items.length = length;
            }
            return ok(value);
          },
        },
      };
    }
    if (isRecordValue(object) && typeof index === "string") {
      const existed = object.fields.has(index);
      return {
        tag: "snapshot",
        snapshot: {
          restore: () => {
            if (existed) {
              object.fields.set(index, value);
            } else {
              object.fields.delete(index);
            }
            return ok(value);
          },
        },
      };
    }
    if (isMapValue(object)) {
      const existed = object.entries.has(index);
      return {
        tag: "snapshot",
        snapshot: {
          restore: () => {
            if (existed) {
              object.entries.set(index, value);
            } else {
              object.entries.delete(index);
            }
            return ok(value);
          },
        },
      };
    }
    return {
      tag: "snapshot",
      snapshot: { restore: () => this.session.indexSet(object, index, value) },
    };
  }

  shuffle(items: ReadonlyArray<Expr>): ReadonlyArray<Expr> {
    const out = [...items];
    for (let i = out.length - 1; i > 0; i -= 1) {
      this.#randomState = (Math.imul(this.#randomState, 1664525) + 1013904223) >>> 0;
      const j = this.#randomState % (i + 1);
      const held = out[i];
      const other = out[j];
      if (held === undefined || other === undefined) {
        continue;
      }
      out[i] = other;
      out[j] = held;
    }
    return out;
  }
}

/** Reads, admits, and searches one unit, collecting answers until exhaustion or a limit. */
export const runAmbAnswers = (
  text: string,
  mode: SearchMode,
  seed = 1,
  limits: SearchLimits = {},
): SearchRun => {
  const admission = admitSource(text, mode);
  if (!admission.ok) {
    return {
      answers: [],
      transcript: [],
      outcome: fail({
        tag: "unknown-syntax",
        construct:
          admission.diagnostics[0]?.construct ?? `TS${admission.hostDiagnostics[0]?.code ?? 0}`,
      }),
      failures: 0,
      steps: 0,
      status: "completed",
    };
  }
  if (limits.maxAnswers === 0) {
    return {
      answers: [],
      transcript: [],
      outcome: ok(undefined),
      failures: 0,
      steps: 0,
      status: "cut-off",
    };
  }
  const searcher = new Searcher(mode === "amb-ramb-experiment", seed);
  const globals = searcher.session.globalEnv();
  for (const item of admission.program) {
    // Imports link before any search starts; this runner exposes no linked
    // modules, so any value import is rejected here rather than left unbound.
    const error = item.tag === "import" ? searcher.session.linkImport(item, globals) : null;
    if (error !== null) {
      return {
        answers: [],
        transcript: [],
        outcome: fail(error),
        failures: 0,
        steps: 0,
        status: "completed",
      };
    }
  }
  const answers: Value[] = [];
  searcher.execBody(
    admission.program,
    globals,
    (value, next) => {
      answers.push(value);
      searcher.defer(next);
    },
    () => undefined,
  );
  let status: "completed" | "cut-off" = "completed";
  while (searcher.hasPending) {
    if (limits.maxAnswers !== undefined && answers.length >= limits.maxAnswers) {
      status = "cut-off";
      break;
    }
    if (limits.maxSteps !== undefined && searcher.steps >= limits.maxSteps) {
      status = "cut-off";
      break;
    }
    searcher.step();
  }
  return {
    answers,
    transcript: searcher.session.transcript,
    outcome: ok(answers[answers.length - 1]),
    failures: searcher.failures,
    steps: searcher.steps,
    status,
  };
};

/** Reads, admits, and searches one source unit in a named search mode. */
export const runAmbSource = (text: string, mode: SearchMode, seed = 1): RunResult => {
  const run = runAmbAnswers(text, mode, seed);
  return { outcome: run.outcome, transcript: run.transcript };
};
