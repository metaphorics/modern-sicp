// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.1

/**
 * A transcription model for the controllers in section 5.1. It resolves labels,
 * steps instructions, and records stack events for the book's hand simulations.
 * It is deliberately smaller than the section 5.2 assembler and simulator.
 */

export type Value = number | boolean | { readonly label: string };
export type Input = { readonly reg: string } | { readonly constant: Value };
export type Source = Input | { readonly op: string; readonly inputs: ReadonlyArray<Input> };
export type Instruction =
  | { readonly tag: "assign"; readonly reg: string; readonly source: Source }
  | { readonly tag: "test"; readonly op: string; readonly inputs: ReadonlyArray<Input> }
  | { readonly tag: "branch"; readonly label: string }
  | { readonly tag: "goto-label"; readonly label: string }
  | { readonly tag: "goto-reg"; readonly reg: string }
  | { readonly tag: "save"; readonly reg: string }
  | { readonly tag: "restore"; readonly reg: string };
export type ControllerLine = Instruction | { readonly tag: "label"; readonly name: string };
export type Machine = {
  readonly registers: ReadonlyArray<string>;
  readonly controller: ReadonlyArray<ControllerLine>;
};
export type MachineError =
  | { readonly tag: "DuplicateLabel"; readonly label: string }
  | { readonly tag: "UnknownLabel"; readonly label: string; readonly step: number }
  | { readonly tag: "UnknownRegister"; readonly reg: string; readonly step: number }
  | { readonly tag: "UnknownOperation"; readonly op: string; readonly step: number }
  | { readonly tag: "TypeMismatch"; readonly op: string; readonly step: number }
  | { readonly tag: "StackUnderflow"; readonly reg: string; readonly step: number }
  | {
      readonly tag: "MismatchedRestore";
      readonly reg: string;
      readonly found: string;
      readonly step: number;
    }
  | { readonly tag: "OutOfSteps"; readonly steps: number };
export type StackEvent =
  | {
      readonly tag: "save";
      readonly step: number;
      readonly label: string;
      readonly reg: string;
      readonly value: Value;
      readonly stack: ReadonlyArray<string>;
    }
  | {
      readonly tag: "restore";
      readonly step: number;
      readonly label: string;
      readonly reg: string;
      readonly value: Value;
      readonly matchedSave: number;
      readonly stack: ReadonlyArray<string>;
    };
export type MachineRun = {
  readonly registers: Readonly<Record<string, Value>>;
  readonly events: ReadonlyArray<StackEvent>;
  readonly instructions: number;
  readonly maxDepth: number;
};

export type Outcome<A> =
  | { readonly ok: true; readonly value: A }
  | { readonly ok: false; readonly error: MachineError };
const fail = (error: MachineError): Outcome<never> => ({ ok: false, error });
const ok = <A>(value: A): Outcome<A> => ({ ok: true, value });
const label = (name: string): Value => ({ label: name });
const reg = (name: string): Input => ({ reg: name });
const c = (constant: Value): Input => ({ constant });
const op = (name: string, ...inputs: Input[]): Source => ({ op: name, inputs });
const assign = (name: string, source: Source): Instruction => ({
  tag: "assign",
  reg: name,
  source,
});
const test = (name: string, ...inputs: Input[]): Instruction => ({ tag: "test", op: name, inputs });
const branch = (name: string): Instruction => ({ tag: "branch", label: name });
const jump = (name: string): Instruction => ({ tag: "goto-label", label: name });
const jumpReg = (name: string): Instruction => ({ tag: "goto-reg", reg: name });
const save = (name: string): Instruction => ({ tag: "save", reg: name });
const restore = (name: string): Instruction => ({ tag: "restore", reg: name });
const mark = (name: string): ControllerLine => ({ tag: "label", name });

const operation = (name: string, values: ReadonlyArray<Value>, step: number): Outcome<Value> => {
  const numbers = values.every((value) => typeof value === "number");
  if (name === "=" || name === "<" || name === ">") {
    const left = values[0];
    const right = values[1];
    if (typeof left !== "number" || typeof right !== "number")
      return fail({ tag: "TypeMismatch", op: name, step });
    return ok(name === "=" ? left === right : name === "<" ? left < right : left > right);
  }
  if (name === "abs") {
    const value = values[0];
    return typeof value === "number"
      ? ok(Math.abs(value))
      : fail({ tag: "TypeMismatch", op: name, step });
  }
  if (!numbers || values.length < 1) return fail({ tag: "TypeMismatch", op: name, step });
  const ns = values.filter((value): value is number => typeof value === "number");
  if (name === "+") return ok(ns.slice(1).reduce((sum, value) => sum + value, ns[0] ?? 0));
  if (name === "*") return ok(ns.slice(1).reduce((product, value) => product * value, ns[0] ?? 1));
  if (name === "-") return ok(ns.slice(1).reduce((result, value) => result - value, ns[0] ?? 0));
  if (name === "/") {
    if (ns.length !== 2 || ns[1] === 0) return fail({ tag: "TypeMismatch", op: name, step });
    return ok((ns[0] ?? 0) / (ns[1] ?? 1));
  }
  return fail({ tag: "UnknownOperation", op: name, step });
};

/** Executes a machine from caller-supplied register values. */
export function runMachine(
  machine: Machine,
  initial: Readonly<Record<string, Value>>,
  maxSteps = 100_000,
  customOperations: Readonly<Record<string, (args: ReadonlyArray<Value>) => Value>> = {},
): Outcome<MachineRun> {
  const addresses = new Map<string, number>();
  const code: Array<{ instruction: Instruction; label: string }> = [];
  let currentLabel = "";
  for (const line of machine.controller) {
    if (line.tag === "label") {
      if (addresses.has(line.name)) return fail({ tag: "DuplicateLabel", label: line.name });
      addresses.set(line.name, code.length);
      currentLabel = line.name;
    } else code.push({ instruction: line, label: currentLabel });
  }
  const registers: Record<string, Value> = {};
  for (const name of machine.registers) registers[name] = initial[name] ?? 0;
  for (const name of Object.keys(initial))
    if (!machine.registers.includes(name))
      return fail({ tag: "UnknownRegister", reg: name, step: 0 });
  const stack: Array<{ reg: string; value: Value; step: number }> = [];
  const events: StackEvent[] = [];
  let flag = false;
  let pc = 0;
  let steps = 0;
  let maxDepth = 0;
  const get = (name: string): Outcome<Value> =>
    name in registers
      ? ok(registers[name] ?? 0)
      : fail({ tag: "UnknownRegister", reg: name, step: steps });
  const readInput = (input: Input): Outcome<Value> =>
    "constant" in input ? ok(input.constant) : get(input.reg);
  const address = (name: string): Outcome<number> =>
    addresses.has(name)
      ? ok(addresses.get(name) ?? 0)
      : fail({ tag: "UnknownLabel", label: name, step: steps });
  while (pc < code.length) {
    if (steps >= maxSteps) return fail({ tag: "OutOfSteps", steps });
    const slot = code[pc];
    if (!slot) return fail({ tag: "OutOfSteps", steps });
    const ins = slot.instruction;
    let next = pc + 1;
    if (ins.tag === "assign") {
      let value: Outcome<Value>;
      if ("op" in ins.source) {
        const args: Value[] = [];
        for (const input of ins.source.inputs) {
          const result = readInput(input);
          if (!result.ok) return result;
          args.push(result.value);
        }
        value =
          ins.source.op in customOperations
            ? ok(customOperations[ins.source.op]?.(args) ?? false)
            : operation(ins.source.op, args, steps);
      } else value = readInput(ins.source);
      if (!value.ok) return value;
      if (!(ins.reg in registers))
        return fail({ tag: "UnknownRegister", reg: ins.reg, step: steps });
      registers[ins.reg] = value.value;
    } else if (ins.tag === "test") {
      const args: Value[] = [];
      for (const input of ins.inputs) {
        const result = readInput(input);
        if (!result.ok) return result;
        args.push(result.value);
      }
      const result =
        ins.op in customOperations
          ? ok(customOperations[ins.op]?.(args) ?? false)
          : operation(ins.op, args, steps);
      if (!result.ok) return result;
      flag = result.value === true;
    } else if (ins.tag === "branch") {
      if (flag) {
        const result = address(ins.label);
        if (!result.ok) return result;
        next = result.value;
      }
    } else if (ins.tag === "goto-label") {
      const result = address(ins.label);
      if (!result.ok) return result;
      next = result.value;
    } else if (ins.tag === "goto-reg") {
      const value = get(ins.reg);
      if (!value.ok) return value;
      if (typeof value.value !== "object" || value.value === null || !("label" in value.value))
        return fail({ tag: "TypeMismatch", op: "goto", step: steps });
      const result = address(value.value.label);
      if (!result.ok) return result;
      next = result.value;
    } else if (ins.tag === "save") {
      const value = get(ins.reg);
      if (!value.ok) return value;
      stack.push({ reg: ins.reg, value: value.value, step: steps });
      maxDepth = Math.max(maxDepth, stack.length);
      events.push({
        tag: "save",
        step: steps,
        label: slot.label,
        reg: ins.reg,
        value: value.value,
        stack: stack.map((entry) => entry.reg),
      });
    } else {
      const saved = stack.pop();
      if (!saved) return fail({ tag: "StackUnderflow", reg: ins.reg, step: steps });
      if (saved.reg !== ins.reg)
        return fail({ tag: "MismatchedRestore", reg: ins.reg, found: saved.reg, step: steps });
      registers[ins.reg] = saved.value;
      events.push({
        tag: "restore",
        step: steps,
        label: slot.label,
        reg: ins.reg,
        value: saved.value,
        matchedSave: saved.step,
        stack: stack.map((entry) => entry.reg),
      });
    }
    pc = next;
    steps += 1;
  }
  return ok({ registers, events, instructions: steps, maxDepth });
}

const factorialController: ControllerLine[] = [
  assign("product", c(1)),
  assign("counter", c(1)),
  mark("test-counter"),
  test(">", reg("counter"), reg("n")),
  branch("factorial-done"),
  assign("product", op("*", reg("counter"), reg("product"))),
  assign("counter", op("+", reg("counter"), c(1))),
  jump("test-counter"),
  mark("factorial-done"),
];
/** Exercises 5.1 and 5.2: iterative factorial controller. */
export const factorialIterative: Machine = {
  registers: ["n", "counter", "product"],
  controller: factorialController,
};

/** Exercise 5.3, first design with primitive good-enough? and improve operations. */
export const sqrtPrimitive: Machine = {
  registers: ["x", "guess"],
  controller: [
    assign("guess", c(1)),
    mark("test-guess"),
    test("good-enough?", reg("guess"), reg("x")),
    branch("sqrt-done"),
    assign("guess", op("improve", reg("guess"), reg("x"))),
    jump("test-guess"),
    mark("sqrt-done"),
  ],
};
/** Exercise 5.3, expanded Newton iteration using arithmetic operations. */
export const sqrtExpanded: Machine = {
  registers: ["x", "guess", "t"],
  controller: [
    assign("guess", c(1)),
    mark("test-guess"),
    assign("t", op("*", reg("guess"), reg("guess"))),
    assign("t", op("-", reg("t"), reg("x"))),
    assign("t", op("abs", reg("t"))),
    test("<", reg("t"), c(0.001)),
    branch("sqrt-done"),
    assign("t", op("/", reg("x"), reg("guess"))),
    assign("t", op("+", reg("guess"), reg("t"))),
    assign("guess", op("/", reg("t"), c(2))),
    jump("test-guess"),
    mark("sqrt-done"),
  ],
};

/** Exercise 5.4a: recursive exponentiation. */
export const exponentRecursive: Machine = {
  registers: ["b", "n", "val", "continue"],
  controller: [
    assign("continue", c(label("expt-done"))),
    mark("expt-loop"),
    test("=", reg("n"), c(0)),
    branch("base-case"),
    save("continue"),
    save("n"),
    assign("n", op("-", reg("n"), c(1))),
    assign("continue", c(label("after-expt"))),
    jump("expt-loop"),
    mark("after-expt"),
    restore("n"),
    restore("continue"),
    assign("val", op("*", reg("b"), reg("val"))),
    jumpReg("continue"),
    mark("base-case"),
    assign("val", c(1)),
    jumpReg("continue"),
    mark("expt-done"),
  ],
};
/** Exercise 5.4b: iterative exponentiation. */
export const exponentIterative: Machine = {
  registers: ["b", "n", "counter", "product"],
  controller: [
    assign("counter", reg("n")),
    assign("product", c(1)),
    mark("expt-iter"),
    test("=", reg("counter"), c(0)),
    branch("expt-done"),
    assign("product", op("*", reg("b"), reg("product"))),
    assign("counter", op("-", reg("counter"), c(1))),
    jump("expt-iter"),
    mark("expt-done"),
  ],
};

/** Figure 5.11 recursive factorial. */
export const factorialRecursive: Machine = {
  registers: ["n", "val", "continue"],
  controller: [
    assign("continue", c(label("fact-done"))),
    mark("fact-loop"),
    test("=", reg("n"), c(1)),
    branch("base-case"),
    save("continue"),
    save("n"),
    assign("n", op("-", reg("n"), c(1))),
    assign("continue", c(label("after-fact"))),
    jump("fact-loop"),
    mark("after-fact"),
    restore("n"),
    restore("continue"),
    assign("val", op("*", reg("n"), reg("val"))),
    jumpReg("continue"),
    mark("base-case"),
    assign("val", c(1)),
    jumpReg("continue"),
    mark("fact-done"),
  ],
};
const fibonacciController: ControllerLine[] = [
  assign("continue", c(label("fib-done"))),
  mark("fib-loop"),
  test("<", reg("n"), c(2)),
  branch("immediate-answer"),
  save("continue"),
  assign("continue", c(label("afterfib-n-1"))),
  save("n"),
  assign("n", op("-", reg("n"), c(1))),
  jump("fib-loop"),
  mark("afterfib-n-1"),
  restore("n"),
  restore("continue"),
  assign("n", op("-", reg("n"), c(2))),
  save("continue"),
  assign("continue", c(label("afterfib-n-2"))),
  save("val"),
  jump("fib-loop"),
  mark("afterfib-n-2"),
  assign("n", reg("val")),
  restore("val"),
  restore("continue"),
  assign("val", op("+", reg("val"), reg("n"))),
  jumpReg("continue"),
  mark("immediate-answer"),
  assign("val", reg("n")),
  jumpReg("continue"),
  mark("fib-done"),
];
/** Figure 5.12 Fibonacci machine. */
export const fibonacciRecursive: Machine = {
  registers: ["n", "val", "continue"],
  controller: fibonacciController,
};
export const fibonacciReduced: Machine = {
  registers: ["n", "val", "continue"],
  controller: [
    assign("continue", c(label("fib-done"))),
    mark("fib-loop"),
    test("<", reg("n"), c(2)),
    branch("immediate-answer"),
    save("continue"),
    assign("continue", c(label("afterfib-n-1"))),
    save("n"),
    assign("n", op("-", reg("n"), c(1))),
    jump("fib-loop"),
    mark("afterfib-n-1"),
    restore("n"),
    assign("n", op("-", reg("n"), c(2))),
    assign("continue", c(label("afterfib-n-2"))),
    save("val"),
    jump("fib-loop"),
    mark("afterfib-n-2"),
    assign("n", reg("val")),
    restore("val"),
    restore("continue"),
    assign("val", op("+", reg("val"), reg("n"))),
    jumpReg("continue"),
    mark("immediate-answer"),
    assign("val", reg("n")),
    jumpReg("continue"),
    mark("fib-done"),
  ],
};
