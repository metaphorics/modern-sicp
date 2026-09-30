// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme evaluator in SICP section 4.1

type Expr =
  | { readonly tag: "number"; readonly value: number }
  | { readonly tag: "variable"; readonly name: string }
  | { readonly tag: "add"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "subtract"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "multiply"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "lessOrEqual"; readonly left: Expr; readonly right: Expr }
  | { readonly tag: "if"; readonly condition: Expr; readonly consequent: Expr; readonly alternative: Expr }
  | { readonly tag: "lambda"; readonly parameter: string; readonly body: Expr }
  | { readonly tag: "call"; readonly operator: Expr; readonly argument: Expr }
  | { readonly tag: "letrec"; readonly name: string; readonly parameter: string; readonly definition: Expr; readonly body: Expr };
type Closure = { readonly tag: "closure"; readonly parameter: string; readonly body: Expr; readonly environment: Env | null };
type Value = number | boolean | Closure;
type EvalError =
  | { readonly tag: "unbound-name"; readonly name: string }
  | { readonly tag: "expected-number"; readonly operator: string }
  | { readonly tag: "expected-boolean" }
  | { readonly tag: "not-callable" };
type Outcome = { readonly tag: "ok"; readonly value: Value }
  | { readonly tag: "error"; readonly error: EvalError };
type Cell = { value: Value | undefined };
type Binding = { readonly name: string; readonly cell: Cell };
type Env = { readonly bindings: Binding[]; readonly parent: Env | null };

const ok = (value: Value): Outcome => ({ tag: "ok", value });
const fail = (error: EvalError): Outcome => ({ tag: "error", error });
const child = (parent: Env | null): Env => ({ bindings: [], parent });
const findCell = (environment: Env | null, name: string): Cell | undefined => {
  let current = environment;
  while (current !== null) {
    for (const binding of current.bindings) {
      if (binding.name === name) return binding.cell;
    }
    current = current.parent;
  }
  return undefined;
};
const evalExpr = (expression: Expr, environment: Env | null): Outcome => {
  switch (expression.tag) {
    case "number":
      return ok(expression.value);
    case "variable": {
      const cell = findCell(environment, expression.name);
      if (cell === undefined || cell.value === undefined)
        return fail({ tag: "unbound-name", name: expression.name });
      return ok(cell.value);
    }
    case "add":
    case "subtract":
    case "multiply": {
      const left = evalExpr(expression.left, environment);
      if (left.tag !== "ok") return left;
      const right = evalExpr(expression.right, environment);
      if (right.tag !== "ok") return right;
      if (typeof left.value !== "number" || typeof right.value !== "number")
        return fail({ tag: "expected-number", operator: expression.tag });
      if (expression.tag === "add") return ok(left.value + right.value);
      if (expression.tag === "subtract") return ok(left.value - right.value);
      return ok(left.value * right.value);
    }
    case "lessOrEqual": {
      const left = evalExpr(expression.left, environment);
      if (left.tag !== "ok") return left;
      const right = evalExpr(expression.right, environment);
      if (right.tag !== "ok") return right;
      if (typeof left.value !== "number" || typeof right.value !== "number")
        return fail({ tag: "expected-number", operator: "lessOrEqual" });
      return ok(left.value <= right.value);
    }
    case "if": {
      const condition = evalExpr(expression.condition, environment);
      if (condition.tag !== "ok") return condition;
      if (typeof condition.value !== "boolean") return fail({ tag: "expected-boolean" });
      return evalExpr(condition.value ? expression.consequent : expression.alternative, environment);
    }
    case "lambda":
      return ok({ tag: "closure", parameter: expression.parameter, body: expression.body, environment: environment });
    case "call": {
      const target = evalExpr(expression.operator, environment);
      if (target.tag !== "ok") return target;
      const argument = evalExpr(expression.argument, environment);
      if (argument.tag !== "ok") return argument;
      if (typeof target.value !== "object" || target.value.tag !== "closure")
        return fail({ tag: "not-callable" });
      const callEnvironment = child(target.value.environment);
      callEnvironment.bindings.push({ name: target.value.parameter, cell: { value: argument.value } });
      return evalExpr(target.value.body, callEnvironment);
    }
    case "letrec": {
      const recursiveEnvironment = child(environment);
      const cell: Cell = { value: undefined };
      recursiveEnvironment.bindings.push({ name: expression.name, cell });
      cell.value = {
        tag: "closure",
        parameter: expression.parameter,
        body: expression.definition,
        environment: recursiveEnvironment,
      };
      return evalExpr(expression.body, recursiveEnvironment);
    }
  }
};
const guestFactorial: Expr = {
  tag: "letrec",
  name: "fact",
  parameter: "n",
  definition: {
    tag: "if",
    condition: {
      tag: "lessOrEqual",
      left: { tag: "variable", name: "n" },
      right: { tag: "number", value: 1 },
    },
    consequent: { tag: "number", value: 1 },
    alternative: {
      tag: "multiply",
      left: { tag: "variable", name: "n" },
      right: {
        tag: "call",
        operator: { tag: "variable", name: "fact" },
        argument: {
          tag: "subtract",
          left: { tag: "variable", name: "n" },
          right: { tag: "number", value: 1 },
        },
      },
    },
  },
  body: {
    tag: "call",
    operator: { tag: "variable", name: "fact" },
    argument: { tag: "number", value: 5 },
  },
};
const nativeFactorial = (n: number): number => n <= 1 ? 1 : n * nativeFactorial(n - 1);
const answer = evalExpr(guestFactorial, null);
if (answer.tag !== "ok" || typeof answer.value !== "number" || answer.value !== nativeFactorial(5))
  throw new Error("native and interpreted factorial disagree");
console.log(answer.value);