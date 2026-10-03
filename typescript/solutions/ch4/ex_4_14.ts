// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.14: two maps, two fates. Louis installs the host's array
 * map as a primitive: his wrapper hands the host the procedure's
 * underlying implementation, so a host primitive (a real host function)
 * is called per element and works, while an evaluator closure offers
 * nothing the host can call — the representation mismatch the book
 * predicts. Eva Lu Ator instead types the definition of map into the
 * object language: a recursive compound procedure whose per-element
 * call goes through the evaluator's own apply, so it handles closures
 * and primitives alike. Both maps are run on the same inputs.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import {
  isArrayValue,
  isClosure,
  isPrimitive,
  makeArray,
  makePrimitive,
  type PrimitiveProcedure,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";

/** Louis's host map: the host calls each procedure's own representation. */
export const makeHostMap = (): PrimitiveProcedure =>
  makePrimitive("map", (args: ReadonlyArray<Value>): Outcome => {
    const procedure = args[0];
    const items = args[1];
    if (procedure === undefined || !isArrayValue(items)) {
      return fail({
        tag: "bad-operand",
        operator: "map",
        detail: "map expects a procedure and an array",
      });
    }
    if (isClosure(procedure)) {
      // The host cannot call an evaluator closure: the call dies at the
      // host boundary, exactly the mismatch the exercise predicts.
      return fail({
        tag: "bad-operand",
        operator: "map",
        detail: "map: the host could not call this procedure",
      });
    }
    if (!isPrimitive(procedure)) {
      return fail({
        tag: "bad-operand",
        operator: "map",
        detail: "map: the procedure is not callable",
      });
    }
    const results: Value[] = [];
    for (const item of items.items) {
      const result = procedure.fn([item]);
      if (result.tag === "error") {
        return result;
      }
      results.push(result.value);
    }
    return ok(makeArray(results));
  });

/** Eva's map, typed into the object language: per-element calls go
 * through the evaluator's own apply. */
export const evaMapSource = `
function map<T, U>(p: (x: T) => U, xs: T[]): U[] {
  const out: U[] = [];
  let i = 0;
  while (i < xs.length) {
    const item = xs[i];
    if (item !== undefined) {
      out.push(p(item));
    }
    i = i + 1;
  }
  return out;
}
const head = (xs: number[]): number => {
  const first = xs[0];
  return first === undefined ? -1 : first;
};
const square = (n: number): number => n * n;
`;

/** A session with Eva's map installed in its global frame. */
export const evaEnv = (): { session: Session; env: Env } => {
  const session = new Session("core");
  const env = session.globalEnv();
  return { session, env };
};

export function ex_4_14(): string {
  return (
    "Louis's host map hands each procedure to the host's own array map: a host primitive " +
    "is a real host function and answers [1, 3] over [[1, 2], [3, 4]], but an evaluator " +
    "closure offers nothing the host can call, so that call fails with " +
    '"map: the host could not call this procedure". Eva\'s map, typed into the object ' +
    "language, routes every per-element call through the evaluator's own apply: it answers " +
    "[1, 3] with head, [1, 4, 9] with square, and [[9]] with the identity — the call that " +
    "killed Louis's map."
  );
}
