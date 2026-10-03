// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.1: the book asks for both evaluation orders because Lisp
 * leaves the order of operand evaluation unspecified. This host fixes it:
 * the guest grammar evaluates call arguments left to right, so the
 * engine's `listOfValues` is the no-defer left-to-right version. Both
 * book-faithful variants below are plain explicit recursion over the
 * operand syntax; they build the same argument list and differ only in
 * which operand's effects land first. An optional session carries the
 * operand effects into one observable transcript.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { GuestError, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import { makeArray, type Value } from "../../packages/ch4/src/runtime/value.js";
import { call, type Expr, ident, str } from "../../packages/ch4/src/syntax/ast.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";

/** Evaluated operands, or the error that stopped them. */
type Values =
  | { readonly tag: "values"; readonly items: Value[] }
  | { readonly tag: "error"; readonly error: GuestError };

const collectLr = (exps: ReadonlyArray<Expr>, env: Env, session: Session): Values => {
  const first = exps[0];
  if (first === undefined) {
    return { tag: "values", items: [] };
  }
  const head = session.evaluate(first, env);
  if (head.tag === "error") {
    return head;
  }
  const tail = collectLr(exps.slice(1), env, session);
  if (tail.tag === "error") {
    return tail;
  }
  return { tag: "values", items: [head.value, ...tail.items] };
};

const collectRl = (exps: ReadonlyArray<Expr>, env: Env, session: Session): Values => {
  const first = exps[0];
  if (first === undefined) {
    return { tag: "values", items: [] };
  }
  const tail = collectRl(exps.slice(1), env, session);
  if (tail.tag === "error") {
    return tail;
  }
  const head = session.evaluate(first, env);
  if (head.tag === "error") {
    return head;
  }
  return { tag: "values", items: [head.value, ...tail.items] };
};

/** The book's left-to-right `list-of-values`: this operand, then the rest. */
export const listOfValuesLr = (
  exps: ReadonlyArray<Expr>,
  env: Env,
  session: Session = new Session("core"),
): Outcome => {
  const values = collectLr(exps, env, session);
  return values.tag === "error" ? fail(values.error) : ok(makeArray(values.items));
};

/** The book's right-to-left variant: the rest first, then this operand. */
export const listOfValuesRl = (
  exps: ReadonlyArray<Expr>,
  env: Env,
  session: Session = new Session("core"),
): Outcome => {
  const values = collectRl(exps, env, session);
  return values.tag === "error" ? fail(values.error) : ok(makeArray(values.items));
};

/**
 * The admitted guest recorder: `(rec tag)` logs its tag and appends it to
 * the guest `recorded` array, so operand order is observable both in the
 * session transcript and in shared guest state.
 */
export const recorderSource = `
let recorded: string[] = [];
const rec = (tag: string): string => {
  console.log(tag);
  recorded.push(tag);
  return tag;
};
`;

/** A session with the admitted recorder installed in its global frame. */
export const recorderEnv = (): { session: Session; env: Env } => {
  const admission = admitSource(recorderSource);
  if (!admission.ok) {
    throw new Error(
      `recorder source must admit: ${admission.diagnostics[0]?.construct ?? "unknown"}`,
    );
  }
  const session = new Session("core");
  const env = session.globalEnv();
  session.execSequence(admission.program, env);
  return { session, env };
};

/** The three operands of the order probe: `rec("a")`, `rec("b")`, `rec("c")`. */
export const recorderOperands = (): ReadonlyArray<Expr> => [
  call(ident("rec"), [str("a")]),
  call(ident("rec"), [str("b")]),
  call(ident("rec"), [str("c")]),
];

export function ex_4_01(): string {
  return (
    "The guest grammar fixes call-argument order left to right, which is why the book's " +
    "unspecified-order question becomes a reworded premise: the edition's listOfValues is " +
    "the book's no-defer left-to-right version, and the exercise pins it against the two " +
    "book-faithful explicit-recursion variants. listOfValuesLr evaluates each operand before " +
    "the rest, so its transcript reads a, b, c; listOfValuesRl evaluates the rest first, so " +
    "the same operands log c, b, a. Both build the argument list in source order, so the " +
    "right-to-left variant alone disagrees with the specification's effect order."
  );
}
