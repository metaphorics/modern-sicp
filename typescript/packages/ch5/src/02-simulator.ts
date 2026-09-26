// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.2

/**
 * The register-machine simulator of section 5.2. `makeMachine` turns a
 * controller into a machine model: named registers, a monitored stack, the
 * operations table, and an assembled instruction sequence whose steps are
 * the execution procedures the assembler installs. The monitoring seams the
 * 5.2.4 exercises use sit at the machine's construction: a register factory
 * (exercise 5.18's traced registers), a per-instruction hook (exercises 5.15
 * to 5.17's counting and tracing), a park gate (exercise 5.19's
 * breakpoints), and an assembler that recomposes (exercises 5.9 to 5.11).
 */

export type Value = number | boolean | { readonly symbol: string };

/** The contents of a register the machine has not written yet. */
export const unassigned: Value = { symbol: "*unassigned*" };

/** Renders a machine word the way transcripts report it. */
export const renderValue = (value: Value): string =>
  typeof value === "object" ? value.symbol : String(value);

/** The simulator's one typed fault: assembly-time and run-time, matching
 * the errors the book's 5.2 signals. */
export type MachineError =
  | { readonly tag: "DuplicateLabel"; readonly label: string }
  | { readonly tag: "UnknownLabel"; readonly label: string }
  | { readonly tag: "UnknownRegister"; readonly reg: string }
  | { readonly tag: "UnknownOperation"; readonly op: string }
  | { readonly tag: "LabelOperand"; readonly label: string }
  | { readonly tag: "StackUnderflow"; readonly reg: string }
  | { readonly tag: "RestoreMismatch"; readonly wanted: string; readonly found: string }
  | { readonly tag: "BadGotoTarget"; readonly reg: string; readonly value: Value }
  | { readonly tag: "BranchWithoutTest" }
  | { readonly tag: "OutOfSteps"; readonly steps: number };

/** Renders a fault the way the machine's reports read. */
export const renderMachineError = (error: MachineError): string => {
  switch (error.tag) {
    case "DuplicateLabel":
      return `the label ${error.label} is used twice`;
    case "UnknownLabel":
      return `undefined label: ${error.label}`;
    case "UnknownRegister":
      return `no such register: ${error.reg}`;
    case "UnknownOperation":
      return `unknown operation: ${error.op}`;
    case "LabelOperand":
      return `an operation input is written (reg r) or (const c), not (label ${error.label})`;
    case "StackUnderflow":
      return `restore ${error.reg} from an empty stack`;
    case "RestoreMismatch":
      return `restore ${error.wanted} but the stack holds ${error.found}`;
    case "BadGotoTarget":
      return `register ${error.reg} does not hold a label address: ${renderValue(error.value)}`;
    case "BranchWithoutTest":
      return "branch read the flag before a test set it";
    case "OutOfSteps":
      return `the machine ran past ${error.steps} steps`;
  }
};

export type Outcome<A> =
  | { readonly ok: true; readonly value: A }
  | { readonly ok: false; readonly error: MachineError };
export const ok = <A>(value: A): Outcome<A> => ({ ok: true, value });
export const fail = <A>(error: MachineError): Outcome<A> => ({ ok: false, error });

// The controller language: the register-machine instructions of 5.1 as data.

/** An instruction operand: a register, a constant, or a label reference. */
export type Arg =
  | { readonly tag: "reg"; readonly name: string }
  | { readonly tag: "const"; readonly value: Value }
  | { readonly tag: "label"; readonly name: string };

/** The right-hand side of an assign: an operand, or an operation call. */
export type Source = Arg | { readonly tag: "op"; readonly op: string; readonly args: Arg[] };

/** Where a goto lands: a label named in the controller, or a register. */
export type GotoTarget =
  | { readonly tag: "label"; readonly name: string }
  | { readonly tag: "reg"; readonly name: string };

export type Instruction =
  | { readonly tag: "assign"; readonly reg: string; readonly source: Source }
  | { readonly tag: "test"; readonly op: string; readonly args: Arg[] }
  | { readonly tag: "branch"; readonly label: string }
  | { readonly tag: "goto"; readonly target: GotoTarget }
  | { readonly tag: "save"; readonly reg: string }
  | { readonly tag: "restore"; readonly reg: string }
  | { readonly tag: "perform"; readonly op: string; readonly args: Arg[] };

/** One controller line: a label the assembler resolves, or an instruction. */
export type ControllerLine = Instruction | { readonly tag: "label"; readonly name: string };

// The constructors of the controller language.

export const reg = (name: string): Arg => ({ tag: "reg", name });
export const c = (value: Value): Arg => ({ tag: "const", value });
export const lbl = (name: string): Arg => ({ tag: "label", name });
export const op = (name: string, ...args: Arg[]): Source => ({ tag: "op", op: name, args });
export const assign = (name: string, source: Source): Instruction => ({
  tag: "assign",
  reg: name,
  source,
});
export const test = (name: string, ...args: Arg[]): Instruction => ({
  tag: "test",
  op: name,
  args,
});
export const branch = (name: string): Instruction => ({ tag: "branch", label: name });
export const jump = (name: string): Instruction => ({
  tag: "goto",
  target: { tag: "label", name },
});
export const jumpReg = (name: string): Instruction => ({
  tag: "goto",
  target: { tag: "reg", name },
});
export const save = (name: string): Instruction => ({ tag: "save", reg: name });
export const restore = (name: string): Instruction => ({ tag: "restore", reg: name });
export const perform = (name: string, ...args: Arg[]): Instruction => ({
  tag: "perform",
  op: name,
  args,
});
export const mark = (name: string): ControllerLine => ({ tag: "label", name });

// Rendering: one controller line the way the book writes it.

const renderArg = (arg: Arg): string =>
  arg.tag === "reg"
    ? `(reg ${arg.name})`
    : arg.tag === "const"
      ? `(const ${renderValue(arg.value)})`
      : `(label ${arg.name})`;

const renderOperands = (args: ReadonlyArray<Arg>): string => args.map(renderArg).join(" ");

/** An operation call in a source's nested form, `((op *) (reg n) (reg val))`. */
const renderCallNested = (name: string, args: ReadonlyArray<Arg>): string => {
  const head = `(op ${name})`;
  const operands = renderOperands(args);
  return operands === "" ? `(${head})` : `(${head} ${operands})`;
};

/** An operation call the way it stands inside an instruction, `(op *) (reg n) (reg val)`. */
const renderCallFlat = (name: string, args: ReadonlyArray<Arg>): string => {
  const head = `(op ${name})`;
  const operands = renderOperands(args);
  return operands === "" ? head : `${head} ${operands}`;
};

/** An operand or source in the nested form. */
export const renderSource = (source: Source): string =>
  source.tag === "op" ? renderCallNested(source.op, source.args) : renderArg(source);

const renderSourceInInstruction = (source: Source): string =>
  source.tag === "op" ? renderCallFlat(source.op, source.args) : renderArg(source);

/** One instruction the way the book writes it, `(assign val (op *) ...)`;
 * a label renders as its bare name. */
export const renderInstruction = (line: ControllerLine): string => {
  switch (line.tag) {
    case "label":
      return line.name;
    case "assign":
      return `(assign ${line.reg} ${renderSourceInInstruction(line.source)})`;
    case "test":
      return `(test ${renderCallFlat(line.op, line.args)})`;
    case "branch":
      return `(branch (label ${line.label}))`;
    case "goto":
      return line.target.tag === "label"
        ? `(goto (label ${line.target.name}))`
        : `(goto (reg ${line.target.name}))`;
    case "save":
      return `(save ${line.reg})`;
    case "restore":
      return `(restore ${line.reg})`;
    case "perform":
      return `(perform ${renderCallFlat(line.op, line.args)})`;
  }
};

// Registers: one named cell, `store` the single write path.

export interface Register {
  readonly name: string;
  readonly content: () => Value;
  readonly store: (value: Value) => void;
}

export const makeRegister = (name: string): Register => {
  let cell: Value = unassigned;
  return { name, content: () => cell, store: (value) => (cell = value) };
};

// The stack of 5.2.4: the saved words plus the push and depth counters.

export interface StackStatistics {
  readonly pushes: number;
  readonly maxDepth: number;
}

export interface MachineStack {
  readonly push: (value: Value) => void;
  readonly pop: (reg: string) => Outcome<Value>;
  readonly initialize: () => void;
  readonly statistics: () => StackStatistics;
  readonly statisticsLine: () => string;
}

export const makeStack = (): MachineStack => {
  let items: Value[] = [];
  let pushes = 0;
  let maxDepth = 0;
  return {
    push: (value) => {
      items.push(value);
      pushes += 1;
      if (items.length > maxDepth) maxDepth = items.length;
    },
    pop: (regName) => {
      const top = items.pop();
      return top === undefined ? fail({ tag: "StackUnderflow", reg: regName }) : ok(top);
    },
    initialize: () => {
      items = [];
      pushes = 0;
      maxDepth = 0;
    },
    statistics: () => ({ pushes, maxDepth }),
    statisticsLine: () => `(total-pushes = ${pushes} maximum-depth = ${maxDepth})`,
  };
};

// The machine model.

/** One machine operation: the host computation behind an operation name. */
export type Operation = (args: ReadonlyArray<Value>) => Value;

/** One execution procedure: calling it simulates the instruction. */
export type Exec = () => Outcome<null>;

/** One assembled instruction: its text (tracing reads it, 5.16) and the
 * execution procedure the assembler installed. */
export interface AssembledInstruction {
  readonly text: string;
  readonly exec: Exec;
}

/** The assembled program: the instruction sequence, the label table, and
 * the 5.12 summary the assembler gathered. */
export interface Program {
  readonly insts: readonly AssembledInstruction[];
  readonly labels: ReadonlyMap<string, number>;
  readonly summary: AssemblySummary;
}

/** The seam a variant machine plugs: a register factory (5.18), a
 * per-instruction hook (5.15 to 5.17), a park gate with its re-arm (5.19),
 * register derivation (5.13), and a step ceiling for a runaway controller. */
export interface MachineOptions {
  readonly deriveRegisters?: boolean;
  readonly makeRegister?: (name: string) => Register;
  readonly onInstruction?: (machine: Machine, inst: AssembledInstruction) => void;
  readonly gate?: (machine: Machine) => string | null;
  readonly onRestart?: () => void;
  readonly stepLimit?: number;
}

export interface Machine {
  readonly registers: ReadonlyMap<string, Register>;
  readonly stack: MachineStack;
  readonly operations: ReadonlyMap<string, Operation>;
  readonly transcript: string[];
  readonly flag: Register;
  pc: number;
  labels: ReadonlyMap<string, number>;
  /** The breakpoint label the machine is parked at, or null. */
  parkedAt: string | null;
  readonly install: (program: Program) => void;
  readonly registerFor: (name: string) => Outcome<Register>;
  readonly labelInEffect: (address: number) => string | null;
  readonly start: () => Outcome<null>;
  readonly proceed: () => Outcome<null>;
}

export const makeNewMachine = (
  registerNames: ReadonlyArray<string>,
  userOperations: Readonly<Record<string, Operation>>,
  options: MachineOptions = {},
): Machine => {
  const newRegister = options.makeRegister ?? makeRegister;
  const registers = new Map<string, Register>();
  const stack = makeStack();
  const transcript: string[] = [];
  const flag = makeRegister("flag");
  const machine: Machine = {
    registers,
    stack,
    operations: new Map([
      [
        "initialize-stack",
        () => {
          stack.initialize();
          return 0;
        },
      ],
      [
        "print-stack-statistics",
        () => {
          transcript.push(stack.statisticsLine());
          return 0;
        },
      ],
      ...Object.entries(userOperations),
    ]),
    transcript,
    flag,
    pc: 0,
    labels: new Map(),
    parkedAt: null,
    install: (program) => {
      machine.labels = program.labels;
      insts = program.insts;
      labelInEffect = buildLabelInEffect(program.labels, insts.length);
    },
    registerFor: (name) => {
      const found = registers.get(name);
      if (found) return ok(found);
      if (options.deriveRegisters) return ok(allocateRegister(name));
      return fail({ tag: "UnknownRegister", reg: name });
    },
    labelInEffect: (address) => labelInEffect[address] ?? null,
    start: () => {
      machine.pc = 0;
      machine.parkedAt = null;
      steps = 0;
      options.onRestart?.();
      return runLoop();
    },
    proceed: () => {
      machine.parkedAt = null;
      return runLoop();
    },
  };
  const allocateRegister = (name: string): Register => {
    if (registers.has(name)) throw new Error(`multiply defined register: ${name}`);
    const register = newRegister(name);
    registers.set(name, register);
    return register;
  };
  const limit = options.stepLimit ?? 100_000;
  let steps = 0;
  let insts: readonly AssembledInstruction[] = [];
  let labelInEffect: ReadonlyArray<string | null> = [];
  const runLoop = (): Outcome<null> => {
    while (machine.pc < insts.length) {
      const parked = options.gate?.(machine) ?? null;
      if (parked !== null) {
        machine.parkedAt = parked;
        return ok(null);
      }
      machine.parkedAt = null;
      const inst = insts[machine.pc];
      if (!inst) return fail({ tag: "OutOfSteps", steps });
      options.onInstruction?.(machine, inst);
      if (steps >= limit) return fail({ tag: "OutOfSteps", steps });
      const ran = inst.exec();
      if (!ran.ok) return ran;
      steps += 1;
    }
    return ok(null);
  };
  for (const name of registerNames) allocateRegister(name);
  return machine;
};

const buildLabelInEffect = (
  labels: ReadonlyMap<string, number>,
  length: number,
): ReadonlyArray<string | null> => {
  const at = new Map<number, string>();
  for (const [name, address] of labels) at.set(address, name);
  const effect: Array<string | null> = [];
  let current: string | null = null;
  for (let address = 0; address < length; address += 1) {
    const here = at.get(address);
    if (here !== undefined) current = here;
    effect.push(current);
  }
  return effect;
};

// The assembler.

/** Exercise 5.12's summary: the instructions grouped by type with
 * duplicates removed, the registers used, the entry-point and stack
 * registers, each register's assign sources, and the labels. Lists are in
 * a deterministic order: types and register groups in first-appearance
 * order, instruction texts and sources sorted, labels in definition
 * order. */
export interface AssemblySummary {
  readonly instructionCount: number;
  readonly instructionsByType: ReadonlyMap<string, readonly string[]>;
  readonly registersUsed: readonly string[];
  readonly entryPointRegisters: readonly string[];
  readonly stackRegisters: readonly string[];
  readonly sourcesByRegister: ReadonlyMap<string, readonly string[]>;
  readonly labels: readonly string[];
}

/** Renders the summary as one part per line. */
export const renderAssemblySummary = (summary: AssemblySummary): string =>
  [
    `(instructions ${[...summary.instructionsByType]
      .map(([kind, texts]) => `(${kind} ${texts.join(" ")})`)
      .join(" ")})`,
    `(registers ${summary.registersUsed.join(" ")})`,
    `(entry-point registers ${summary.entryPointRegisters.join(" ")})`,
    `(stack registers ${summary.stackRegisters.join(" ")})`,
    `(sources ${[...summary.sourcesByRegister]
      .map(([name, sources]) => `(${name} ${sources.join(" ")})`)
      .join(" ")})`,
    `(labels ${summary.labels.join(" ")})`,
  ].join("\n");

/** The registers a controller names: every assign target, every register
 * source or operand, every goto register, every save and restore
 * register. Exercise 5.13 builds a machine's register list from this
 * scan. */
export const controllerRegisters = (controller: ReadonlyArray<ControllerLine>): string[] => {
  const names = new Set<string>();
  const fromSource = (source: Source): void => {
    if (source.tag === "reg") names.add(source.name);
    if (source.tag === "op") source.args.forEach(fromSource);
  };
  for (const line of controller) {
    if (line.tag === "label") continue;
    switch (line.tag) {
      case "assign": {
        names.add(line.reg);
        fromSource(line.source);
        break;
      }
      case "test":
      case "perform": {
        for (const arg of line.args) fromSource(arg);
        break;
      }
      case "goto": {
        if (line.target.tag === "reg") names.add(line.target.name);
        break;
      }
      case "save":
      case "restore": {
        names.add(line.reg);
        break;
      }
    }
  }
  return [...names].sort();
};

const summarize = (
  instructions: ReadonlyArray<Instruction>,
  labels: ReadonlyMap<string, number>,
): AssemblySummary => {
  const byType = new Map<string, Set<string>>();
  const sources = new Map<string, Set<string>>();
  const entryPoints = new Set<string>();
  const stackRegs = new Set<string>();
  const classify = (kind: string, instr: Instruction): void => {
    const group = byType.get(kind) ?? new Set<string>();
    group.add(renderInstruction(instr));
    byType.set(kind, group);
  };
  for (const instr of instructions) {
    switch (instr.tag) {
      case "assign": {
        const text = renderSource(instr.source);
        const seen = sources.get(instr.reg) ?? new Set<string>();
        seen.add(text);
        sources.set(instr.reg, seen);
        classify("assign", instr);
        break;
      }
      case "test":
        classify("test", instr);
        break;
      case "branch":
        classify("branch", instr);
        break;
      case "goto": {
        if (instr.target.tag === "reg") entryPoints.add(instr.target.name);
        classify("goto", instr);
        break;
      }
      case "save":
        stackRegs.add(instr.reg);
        classify("save", instr);
        break;
      case "restore":
        stackRegs.add(instr.reg);
        classify("restore", instr);
        break;
      case "perform":
        classify("perform", instr);
        break;
    }
  }
  const sorted = (set: Set<string>): string[] => [...set].sort();
  return {
    instructionCount: instructions.length,
    instructionsByType: new Map([...byType].map(([kind, group]) => [kind, sorted(group)])),
    registersUsed: controllerRegisters(instructions),
    entryPointRegisters: [...entryPoints].sort(),
    stackRegisters: [...stackRegs].sort(),
    sourcesByRegister: new Map([...sources].map(([name, group]) => [name, sorted(group)])),
    labels: [...labels.keys()],
  };
};

export const lookupLabel = (labels: ReadonlyMap<string, number>, name: string): Outcome<number> => {
  const address = labels.get(name);
  return address === undefined ? fail({ tag: "UnknownLabel", label: name }) : ok(address);
};

/** The label scan: separates the labels from the instructions, pointing
 * each label at the instruction that follows it (a trailing label names
 * the stop address, one past the last instruction). A label used twice is
 * exercise 5.8's assembly error. */
export const extractLabels = (
  controller: ReadonlyArray<ControllerLine>,
): Outcome<{
  readonly instructions: Instruction[];
  readonly labels: ReadonlyMap<string, number>;
}> => {
  const instructions: Instruction[] = [];
  const labels = new Map<string, number>();
  for (const line of controller) {
    if (line.tag === "label") {
      if (labels.has(line.name)) return fail({ tag: "DuplicateLabel", label: line.name });
      labels.set(line.name, instructions.length);
    } else {
      instructions.push(line);
    }
  }
  return ok({ instructions, labels });
};

export interface AssemblerOptions {
  /** Exercise 5.9: refuse `(label x)` operands of machine operations. */
  readonly strictLabels?: boolean;
  /** Exercise 5.11's seam: a builder for save and restore instructions;
   * null falls through to the standard builder, an error refuses the
   * controller. */
  readonly saveRestore?: (instr: Instruction) => Outcome<Exec | null>;
}

// The execution-procedure generators, one per instruction type.

const primitiveExp = (
  exp: Source,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
  options: AssemblerOptions,
): Outcome<() => Outcome<Value>> => {
  if (exp.tag === "reg") {
    const source = machine.registerFor(exp.name);
    return source.ok ? ok(() => ok(source.value.content())) : source;
  }
  if (exp.tag === "const") {
    const value = exp.value;
    return ok(() => ok(value));
  }
  if (exp.tag === "label") {
    const address = lookupLabel(labels, exp.name);
    return address.ok ? ok(() => ok(address.value)) : address;
  }
  return operationExp(exp.op, exp.args, machine, labels, options);
};

const operationExp = (
  name: string,
  args: ReadonlyArray<Arg>,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
  options: AssemblerOptions,
): Outcome<() => Outcome<Value>> => {
  const prim = machine.operations.get(name);
  if (!prim) return fail({ tag: "UnknownOperation", op: name });
  const argProcs: Array<() => Outcome<Value>> = [];
  for (const arg of args) {
    if (arg.tag === "label" && options.strictLabels === true) {
      return fail({ tag: "LabelOperand", label: arg.name });
    }
    const proc = primitiveExp(arg, machine, labels, options);
    if (!proc.ok) return proc;
    argProcs.push(proc.value);
  }
  return ok(() => {
    const values: Value[] = [];
    for (const proc of argProcs) {
      const value = proc();
      if (!value.ok) return value;
      values.push(value.value);
    }
    return ok(prim(values));
  });
};

const assignExec = (
  instr: Extract<Instruction, { tag: "assign" }>,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
  options: AssemblerOptions,
): Outcome<Exec> => {
  const target = machine.registerFor(instr.reg);
  if (!target.ok) return target;
  const source = primitiveExp(instr.source, machine, labels, options);
  if (!source.ok) return source;
  const valueProc = source.value;
  const register = target.value;
  return ok(() => {
    const value = valueProc();
    if (!value.ok) return value;
    register.store(value.value);
    machine.pc += 1;
    return ok(null);
  });
};

const testExec = (
  instr: Extract<Instruction, { tag: "test" }>,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
  options: AssemblerOptions,
): Outcome<Exec> => {
  const condition = operationExp(instr.op, instr.args, machine, labels, options);
  if (!condition.ok) return condition;
  const condProc = condition.value;
  return ok(() => {
    const value = condProc();
    if (!value.ok) return value;
    machine.flag.store(value.value);
    machine.pc += 1;
    return ok(null);
  });
};

const branchExec = (
  instr: Extract<Instruction, { tag: "branch" }>,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
): Outcome<Exec> => {
  const address = lookupLabel(labels, instr.label);
  if (!address.ok) return address;
  const dest = address.value;
  return ok(() => {
    const flag = machine.flag.content();
    if (typeof flag !== "boolean") return fail({ tag: "BranchWithoutTest" });
    machine.pc = flag ? dest : machine.pc + 1;
    return ok(null);
  });
};

const gotoExec = (
  instr: Extract<Instruction, { tag: "goto" }>,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
): Outcome<Exec> => {
  if (instr.target.tag === "label") {
    const address = lookupLabel(labels, instr.target.name);
    if (!address.ok) return address;
    const dest = address.value;
    return ok(() => {
      machine.pc = dest;
      return ok(null);
    });
  }
  const target = machine.registerFor(instr.target.name);
  if (!target.ok) return target;
  const register = target.value;
  return ok(() => {
    const value = register.content();
    if (typeof value !== "number") return fail({ tag: "BadGotoTarget", reg: register.name, value });
    machine.pc = value;
    return ok(null);
  });
};

const saveExec = (
  instr: Extract<Instruction, { tag: "save" }>,
  machine: Machine,
): Outcome<Exec> => {
  const source = machine.registerFor(instr.reg);
  if (!source.ok) return source;
  const register = source.value;
  return ok(() => {
    machine.stack.push(register.content());
    machine.pc += 1;
    return ok(null);
  });
};

const restoreExec = (
  instr: Extract<Instruction, { tag: "restore" }>,
  machine: Machine,
): Outcome<Exec> => {
  const target = machine.registerFor(instr.reg);
  if (!target.ok) return target;
  const register = target.value;
  return ok(() => {
    const value = machine.stack.pop(instr.reg);
    if (!value.ok) return value;
    register.store(value.value);
    machine.pc += 1;
    return ok(null);
  });
};

const performExec = (
  instr: Extract<Instruction, { tag: "perform" }>,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
  options: AssemblerOptions,
): Outcome<Exec> => {
  const action = operationExp(instr.op, instr.args, machine, labels, options);
  if (!action.ok) return action;
  const actionProc = action.value;
  return ok(() => {
    const value = actionProc();
    if (!value.ok) return value;
    machine.pc += 1;
    return ok(null);
  });
};

/** The book's make-execution-procedure: one generator per instruction
 * type; a label never reaches here, the scan removed it. */
export const executionProcedure = (
  instr: Instruction,
  machine: Machine,
  labels: ReadonlyMap<string, number>,
  options: AssemblerOptions = {},
): Outcome<Exec> => {
  if (options.saveRestore) {
    const variant = options.saveRestore(instr);
    if (!variant.ok) return variant;
    if (variant.value) return ok(variant.value);
  }
  switch (instr.tag) {
    case "assign":
      return assignExec(instr, machine, labels, options);
    case "test":
      return testExec(instr, machine, labels, options);
    case "branch":
      return branchExec(instr, machine, labels);
    case "goto":
      return gotoExec(instr, machine, labels);
    case "save":
      return saveExec(instr, machine);
    case "restore":
      return restoreExec(instr, machine);
    case "perform":
      return performExec(instr, machine, labels, options);
  }
};

/** The assembler: scans the controller, pairs each instruction's text
 * with its execution procedure, and gathers the 5.12 summary. */
export const assemble = (
  controller: ReadonlyArray<ControllerLine>,
  machine: Machine,
  options: AssemblerOptions = {},
): Outcome<Program> => {
  const scanned = extractLabels(controller);
  if (!scanned.ok) return scanned;
  const { instructions, labels } = scanned.value;
  const insts: AssembledInstruction[] = [];
  for (const instr of instructions) {
    const exec = executionProcedure(instr, machine, labels, options);
    if (!exec.ok) return exec;
    insts.push({ text: renderInstruction(instr), exec: exec.value });
  }
  return ok({ insts, labels, summary: summarize(instructions, labels) });
};

// The section's interface procedures.

export const makeMachine = (
  registerNames: ReadonlyArray<string>,
  operations: Readonly<Record<string, Operation>>,
  controller: ReadonlyArray<ControllerLine>,
  options: MachineOptions = {},
): Outcome<Machine> => {
  const machine = makeNewMachine(registerNames, operations, options);
  const program = assemble(controller, machine);
  if (!program.ok) return program;
  machine.install(program.value);
  return ok(machine);
};

/** Exercise 5.13's interface: no register list, the controller decides. */
export const makeMachineDerivingRegisters = (
  operations: Readonly<Record<string, Operation>>,
  controller: ReadonlyArray<ControllerLine>,
): Outcome<Machine> => makeMachine([], operations, controller, { deriveRegisters: true });

export const getRegisterContents = (machine: Machine, name: string): Outcome<Value> => {
  const register = machine.registerFor(name);
  return register.ok ? ok(register.value.content()) : register;
};

export const setRegisterContents = (
  machine: Machine,
  name: string,
  value: Value,
): Outcome<null> => {
  const register = machine.registerFor(name);
  if (!register.ok) return register;
  register.value.store(value);
  return ok(null);
};

// The section's shared arithmetic, the operations the chapter's machines assume.

const asNumber = (op: string, value: Value): number => {
  if (typeof value === "number") return value;
  throw new Error(`${op}: not a number: ${renderValue(value)}`);
};

const arith2 =
  (name: string, f: (a: number, b: number) => Value): Operation =>
  (args) => {
    const a = asNumber(name, args[0] ?? unassigned);
    const b = asNumber(name, args[1] ?? unassigned);
    return f(a, b);
  };

const compare2 =
  (f: (a: number, b: number) => boolean): Operation =>
  (args) => {
    const a = asNumber("=", args[0] ?? unassigned);
    const b = asNumber("=", args[1] ?? unassigned);
    return f(a, b);
  };

export const arithmeticOperations: Record<string, Operation> = {
  "+": arith2("+", (a, b) => a + b),
  "-": arith2("-", (a, b) => a - b),
  "*": arith2("*", (a, b) => a * b),
  "/": arith2("/", (a, b) => a / b),
  rem: arith2("rem", (a, b) => a % b),
  "=": compare2((a, b) => a === b),
  "<": compare2((a, b) => a < b),
  ">": compare2((a, b) => a > b),
};
