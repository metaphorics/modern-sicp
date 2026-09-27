// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** One local layer of a rule-application environment over values of type T. */
export type Bindings<T> = Readonly<Record<string, T>>;

/**
 * Applies a rule body in a fresh local environment: parameters bind the
 * call's arguments in a layer of their own, so a caller and a callee
 * parameter sharing one name never collide. Lookups fall through to the
 * outer bindings, the query frame the application extends.
 */
export function applyRuleLocally<T, R>(
  params: ReadonlyArray<string>,
  args: ReadonlyArray<T>,
  body: (env: Bindings<T>) => R,
  outer: Bindings<T> = {},
): R {
  if (params.length !== args.length) throw new Error("arity mismatch");
  const local: Record<string, T> = { ...outer };
  params.forEach((param, index) => {
    const arg = args[index];
    if (arg !== undefined) local[param] = arg;
  });
  return body(local);
}

/** Rewrites one infix addition into prefix operator position. */
export function rewriteInfix(text: string): string {
  return text.replace(/\(([\w?]+)\s+\+\s+([\w?]+)\)/g, "(+ $1 $2)");
}

export function ex_4_79(): string {
  return "A local environment for each application prevents caller and callee parameters with the same name from colliding.";
}
