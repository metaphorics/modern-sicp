// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.2

/**
 * The register-machine simulator (host-subsets grammar section 7): labels
 * resolve to instruction indices at assembly, operations look up by name,
 * save/restore run over one stack with mismatch detection, and every run
 * reports ordered instruction traces, stack statistics, and the declared
 * `MachineError` values. Execution is an index-based loop inside `run`;
 * `singleStep` and breakpoints expose the same machine for the tracing and
 * debugging exercises.
 */
import {
  formatMachineStatement,
  type MachineError,
  type MachineStatement,
  type MachineValue,
  type Operation,
  type Source,
} from "./01-register-machines.ts";

/** One assembled controller: statements plus resolved label indices. */
export interface Assembled<W = MachineValue> {
  readonly statements: ReadonlyArray<MachineStatement<W>>;
  readonly labels: Readonly<Record<string, number>>;
}

/** The result of assembling a controller. */
export type AssemblyResult<W = MachineValue> =
  | { readonly ok: true; readonly value: Assembled<W> }
  | { readonly ok: false; readonly error: MachineError };

/** One run's observable state. */
export interface MachineRun<W = MachineValue> {
  readonly registers: Readonly<Record<string, W | undefined>>;
  readonly stackStats: { readonly pushes: number; readonly maxDepth: number };
  readonly instructionCount: number;
  readonly trace: ReadonlyArray<string>;
  readonly error: MachineError | null;
}

/** The machine's observable lifecycle state. */
export type MachineState = "ready" | "running" | "halted" | "stopped" | "faulted";

/** Resolves labels and operations against one word type. */
export const assemble = <W = MachineValue>(
  controller: ReadonlyArray<MachineStatement<W>>,
  operations: Readonly<Record<string, Operation<W>>>,
): AssemblyResult<W> => {
  const labels: Record<string, number> = {};
  for (let i = 0; i < controller.length; i += 1) {
    const statement = controller[i];
    if (statement === undefined || statement.tag !== "label") {
      continue;
    }
    if (labels[statement.name] !== undefined) {
      return { ok: false, error: { tag: "duplicate-label", name: statement.name } };
    }
    labels[statement.name] = i;
  }
  for (const statement of controller) {
    const error = checkStatement(statement, labels, operations);
    if (error !== null) {
      return { ok: false, error };
    }
  }
  return { ok: true, value: { statements: controller, labels } };
};

const checkStatement = <W>(
  statement: MachineStatement<W>,
  labels: Readonly<Record<string, number>>,
  operations: Readonly<Record<string, Operation<W>>>,
): MachineError | null => {
  if (statement.tag === "label") {
    return null;
  }
  if (statement.tag === "branch" || statement.tag === "goto-label") {
    return labels[statement.label] === undefined
      ? { tag: "unknown-label", name: statement.label }
      : null;
  }
  if (statement.tag === "test" || statement.tag === "perform") {
    return operations[statement.operation] === undefined
      ? { tag: "unknown-operation", name: statement.operation }
      : checkSources(statement.args, labels, operations);
  }
  if (statement.tag === "assign") {
    return checkSource(statement.source, labels, operations);
  }
  return null;
};

const checkSources = <W>(
  sources: ReadonlyArray<Source<W>>,
  labels: Readonly<Record<string, number>>,
  operations: Readonly<Record<string, Operation<W>>>,
): MachineError | null => {
  for (const source of sources) {
    const error = checkSource(source, labels, operations);
    if (error !== null) {
      return error;
    }
  }
  return null;
};

const checkSource = <W>(
  source: Source<W>,
  labels: Readonly<Record<string, number>>,
  operations: Readonly<Record<string, Operation<W>>>,
): MachineError | null => {
  if (source.tag === "label") {
    return labels[source.name] === undefined ? { tag: "unknown-label", name: source.name } : null;
  }
  if (source.tag === "op") {
    return operations[source.operation] === undefined
      ? { tag: "unknown-operation", name: source.operation }
      : checkSources(source.args, labels, operations);
  }
  return null;
};

/** One simulated machine over an assembled controller. */
export class Machine<W = MachineValue> {
  readonly #assembled: Assembled<W>;
  readonly #operations: Readonly<Record<string, Operation<W>>>;
  readonly #registers = new Map<string, W | undefined>();
  readonly #stack: Array<{ readonly value: W | undefined; readonly from: string }> = [];
  readonly #breakpoints = new Set<string>();
  readonly #trace: string[] = [];
  #flag = false;
  #pc = 0;
  #steps = 0;
  #pushes = 0;
  #maxDepth = 0;
  #state: MachineState = "ready";
  #error: MachineError | null = null;

  constructor(
    registers: ReadonlyArray<string>,
    operations: Readonly<Record<string, Operation<W>>>,
    controller: ReadonlyArray<MachineStatement<W>>,
  ) {
    this.#operations = operations;
    for (const name of registers) {
      this.#registers.set(name, undefined);
    }
    const assembly = assemble(controller, operations);
    if (assembly.ok) {
      this.#assembled = assembly.value;
    } else {
      this.#assembled = { statements: controller, labels: {} };
      this.#error = assembly.error;
      this.#state = "faulted";
    }
  }

  /** The machine's lifecycle state. */
  get state(): MachineState {
    return this.#state;
  }

  /** A register's current value. */
  readRegister(name: string): W | undefined {
    return this.#registers.get(name);
  }

  /** Sets a register; unknown registers are a machine error. */
  writeRegister(name: string, value: W | undefined): MachineError | null {
    if (!this.#registers.has(name)) {
      this.#error = { tag: "unknown-register", name };
      this.#state = "faulted";
      return this.#error;
    }
    this.#registers.set(name, value);
    return null;
  }

  /** Stops before the next execution of `label`'s first instruction. */
  setBreakpoint(label: string): void {
    this.#breakpoints.add(label);
  }

  /** Clears one breakpoint. */
  clearBreakpoint(label: string): void {
    this.#breakpoints.delete(label);
  }

  /** Executes one instruction; false when the machine cannot continue. */
  singleStep(): boolean {
    if (this.#state === "faulted" || this.#state === "halted") {
      return false;
    }
    const statement = this.#assembled.statements[this.#pc];
    if (statement === undefined) {
      this.#state = "halted";
      return false;
    }
    if (
      statement.tag === "label" &&
      this.#breakpoints.has(statement.name) &&
      this.#state === "running"
    ) {
      this.#state = "stopped";
      return false;
    }
    this.#state = "running";
    this.#trace.push(formatMachineStatement(statement));
    this.#steps += 1;
    this.execute(statement);
    return this.#state === "running";
  }

  /** Runs the machine to halt, fault, breakpoint, or `maxSteps` exhaustion. */
  run(maxSteps = 100_000): MachineRun<W> {
    if (this.#state === "faulted") {
      return this.result();
    }
    for (;;) {
      if (this.#steps >= maxSteps) {
        this.#error = { tag: "out-of-steps", steps: maxSteps };
        this.#state = "faulted";
        break;
      }
      if (!this.singleStep()) {
        break;
      }
    }
    return this.result();
  }

  /** The machine's observable result state. */
  result(): MachineRun<W> {
    const registers: Record<string, W | undefined> = {};
    for (const [name, value] of this.#registers) {
      registers[name] = value;
    }
    return {
      registers,
      stackStats: { pushes: this.#pushes, maxDepth: this.#maxDepth },
      instructionCount: this.#steps,
      trace: [...this.#trace],
      error: this.#error,
    };
  }

  execute(statement: MachineStatement<W>): void {
    switch (statement.tag) {
      case "label":
        this.#pc += 1;
        return;
      case "assign": {
        const value = this.resolve(statement.source);
        if (this.#state === "faulted") {
          return;
        }
        this.store(statement.register, value);
        return;
      }
      case "test": {
        const result = this.applyOperation(statement.operation, statement.args);
        if (this.#state === "faulted") {
          return;
        }
        this.#flag = result === true;
        this.#pc += 1;
        return;
      }
      case "perform": {
        this.applyOperation(statement.operation, statement.args);
        this.#pc += 1;
        return;
      }
      case "branch": {
        const target = this.#assembled.labels[statement.label];
        if (target === undefined) {
          this.fault({ tag: "unknown-label", name: statement.label });
          return;
        }
        this.#pc = this.#flag ? target : this.#pc + 1;
        return;
      }
      case "goto-label": {
        const target = this.#assembled.labels[statement.label];
        if (target === undefined) {
          this.fault({ tag: "unknown-label", name: statement.label });
          return;
        }
        this.#pc = target;
        return;
      }
      case "goto-register": {
        if (!this.#registers.has(statement.register)) {
          this.fault({ tag: "unknown-register", name: statement.register });
          return;
        }
        const destination = this.addressOf(this.#registers.get(statement.register));
        if (destination === undefined) {
          this.fault({ tag: "bad-target", detail: `register ${statement.register}` });
          return;
        }
        this.#pc = destination;
        return;
      }
      case "save": {
        const value = this.#registers.get(statement.register);
        if (value === undefined && !this.#registers.has(statement.register)) {
          this.fault({ tag: "unknown-register", name: statement.register });
          return;
        }
        this.#stack.push({ value, from: statement.register });
        this.#pushes += 1;
        this.#maxDepth = Math.max(this.#maxDepth, this.#stack.length);
        this.#pc += 1;
        return;
      }
      case "restore": {
        const entry = this.#stack.pop();
        if (entry === undefined) {
          this.fault({ tag: "stack-underflow", detail: `restore ${statement.register}` });
          return;
        }
        if (entry.from !== statement.register) {
          this.fault({
            tag: "restore-mismatch",
            register: statement.register,
            expected: entry.from,
            found: statement.register,
          });
          return;
        }
        this.#registers.set(statement.register, entry.value);
        this.#pc += 1;
        return;
      }
    }
  }

  private store(name: string, value: W | undefined): void {
    if (!this.#registers.has(name)) {
      this.fault({ tag: "unknown-register", name });
      return;
    }
    this.#registers.set(name, value);
    this.#pc += 1;
  }

  private resolve(source: Source<W>): W | undefined {
    if (source.tag === "const") {
      return source.value;
    }
    if (source.tag === "reg") {
      const value = this.#registers.get(source.name);
      if (value === undefined && !this.#registers.has(source.name)) {
        this.fault({ tag: "unknown-register", name: source.name });
        return undefined;
      }
      return value;
    }
    if (source.tag === "label") {
      const target = this.#assembled.labels[source.name];
      return target === undefined
        ? (this.fault({ tag: "unknown-label", name: source.name }), undefined)
        : (target as W);
    }
    return this.applyOperation(source.operation, source.args);
  }

  private applyOperation(name: string, args: ReadonlyArray<Source<W>>): W | undefined {
    const operation = this.#operations[name];
    if (operation === undefined) {
      this.fault({ tag: "unknown-operation", name });
      return undefined;
    }
    const values: Array<W | undefined> = [];
    for (const source of args) {
      const value = this.resolve(source);
      if (value === undefined && this.#state === "faulted") {
        return undefined;
      }
      values.push(value);
    }
    return operation(values);
  }

  private addressOf(target: W | undefined): number | undefined {
    if (typeof target === "number") {
      return target;
    }
    if (
      typeof target === "object" &&
      target !== null &&
      "tag" in target &&
      target.tag === "symbol" &&
      "name" in target &&
      typeof target.name === "string"
    ) {
      return this.#assembled.labels[target.name];
    }
    return undefined;
  }

  private fault(error: MachineError): void {
    this.#error = error;
    this.#state = "faulted";
  }
}

/** Builds a machine from its registers, operations, and controller. */
export const makeMachine = <W = MachineValue>(spec: {
  readonly registers: ReadonlyArray<string>;
  readonly operations: Readonly<Record<string, Operation<W>>>;
  readonly controller: ReadonlyArray<MachineStatement<W>>;
}): Machine<W> => new Machine<W>(spec.registers, spec.operations, spec.controller);
