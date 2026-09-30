// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.5 self-interpreter witness

/**
 * The minimum positive self-interpreter kernel (host-subsets grammar
 * section 5): typed recursive syntax data, pattern matching over it, lexical
 * closures, shared recursive binding, conditionals, and arithmetic — enough
 * to evaluate a recursively defined guest factorial. The kernel proper is
 * the `Expr`/`Value`/`Env` data, `findCell`, and `evalExpr`; the file tail
 * (guestFactorial, native comparison, throw, console.log) is boundary driver
 * code under the grammar's boundary rules, not an evaluator primitive.
 */

type Expr =
  | { kind: "num"; value: number }
  | { kind: "var"; name: string }
  | { kind: "add"; left: Expr; right: Expr }
  | { kind: "mul"; left: Expr; right: Expr }
  | { kind: "less"; left: Expr; right: Expr }
  | { kind: "cond"; test: Expr; consequent: Expr; alternative: Expr }
  | { kind: "fun"; param: string; body: Expr }
  | { kind: "call"; callee: Expr; argument: Expr };

type Fn = (argument: Value) => Value;
type Value = number | Fn;

interface Cell {
  value: Value | undefined;
  initialized: boolean;
}

interface Env {
  readonly bindings: Map<string, Cell>;
  readonly parent: Env | null;
}

const childEnv = (parent: Env | null): Env => ({ bindings: new Map(), parent: parent });

const findCell = (env: Env | null, name: string): Cell | undefined => {
  let frame = env;
  while (frame !== null) {
    const own = frame.bindings.get(name);
    if (own !== undefined) {
      return own;
    }
    frame = frame.parent;
  }
  return undefined;
};

const lookup = (env: Env | null, name: string): Value => {
  const cell = findCell(env, name);
  if (cell === undefined || !cell.initialized || cell.value === undefined) {
    throw new Error(`unbound ${name}`);
  }
  return cell.value;
};

const numberValue = (value: Value): number => {
  if (typeof value !== "number") {
    throw new Error("expected number");
  }
  return value;
};

const functionValue = (value: Value): Fn => {
  if (typeof value !== "function") {
    throw new Error("expected function");
  }
  return value;
};

/** The guest evaluator kernel: pattern matching over the typed syntax data. */
const evalExpr = (expr: Expr, env: Env | null): Value => {
  if (expr.kind === "num") {
    return expr.value;
  }
  if (expr.kind === "var") {
    return lookup(env, expr.name);
  }
  if (expr.kind === "add") {
    return numberValue(evalExpr(expr.left, env)) + numberValue(evalExpr(expr.right, env));
  }
  if (expr.kind === "mul") {
    return numberValue(evalExpr(expr.left, env)) * numberValue(evalExpr(expr.right, env));
  }
  if (expr.kind === "less") {
    return numberValue(evalExpr(expr.left, env)) < numberValue(evalExpr(expr.right, env)) ? 1 : 0;
  }
  if (expr.kind === "cond") {
    return numberValue(evalExpr(expr.test, env)) === 1 ? evalExpr(expr.consequent, env) : evalExpr(expr.alternative, env);
  }
  if (expr.kind === "fun") {
    const param = expr.param;
    const body = expr.body;
    const captured = env;
    return (argument: Value) => {
      const frame = childEnv(captured);
      frame.bindings.set(param, { value: argument, initialized: true });
      return evalExpr(body, frame);
    };
  }
  return functionValue(evalExpr(expr.callee, env))(evalExpr(expr.argument, env));
};

/** Shared recursive binding: the guest's `letrec` over one shared cell. */
const letrec = (name: string, functionExpression: Expr, body: Expr): Value => {
  const frame = childEnv(null);
  const cell: Cell = { value: undefined, initialized: false };
  frame.bindings.set(name, cell);
  cell.value = evalExpr(functionExpression, frame);
  cell.initialized = true;
  return evalExpr(body, frame);
};

// Boundary driver: the guest factorial over the kernel, compared to native.

const num = (value: number): Expr => ({ kind: "num", value: value });
const add = (left: Expr, right: Expr): Expr => ({ kind: "add", left: left, right: right });
const mul = (left: Expr, right: Expr): Expr => ({ kind: "mul", left: left, right: right });
const less = (left: Expr, right: Expr): Expr => ({ kind: "less", left: left, right: right });
const cond = (test: Expr, consequent: Expr, alternative: Expr): Expr => ({ kind: "cond", test: test, consequent: consequent, alternative: alternative });
const call = (callee: Expr, argument: Expr): Expr => ({ kind: "call", callee: callee, argument: argument });
const variable = (name: string): Expr => ({ kind: "var", name: name });

const guestFactorial: Expr = {
  kind: "fun",
  param: "n",
  body: cond(
    less(variable("n"), num(2)),
    num(1),
    mul(variable("n"), call(variable("fact"), add(variable("n"), num(-1)))),
  ),
};

const factorialVia = (n: number): number => {
  const result = letrec("fact", guestFactorial, call(variable("fact"), num(n)));
  return numberValue(result);
};

const nativeFactorial = (n: number): number => {
  let acc = 1;
  let index = 2;
  while (index <= n) {
    acc = acc * index;
    index = index + 1;
  }
  return acc;
};

const cases = [0, 1, 5, 7];
for (const n of cases) {
  const guest = factorialVia(n);
  const native = nativeFactorial(n);
  if (guest !== native) {
    throw new Error("self-interpreter mismatch");
  }
  console.log(guest);
}

// A lexical closure escapes and keeps its shared cell across calls.
type Counter = (ignored: number) => number;
const makeCounter = (): Counter => {
  let count = 0;
  return (ignored: number) => {
    count = count + 1;
    return count;
  };
};
const counter = makeCounter();
counter(0);
counter(0);
console.log(counter(0));
