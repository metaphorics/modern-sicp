// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.1

/**
 * Register-machine data (host-subsets grammar section 7): controllers are
 * typed constructor data — instructions, inputs, and machine values are
 * ordinary recursively typed unions checked by `tsc`, never quoted
 * controller text and never a parser fallback. Unknown operation names,
 * registers, labels, invalid operand types, stack underflow, and restore
 * mismatch are the declared `MachineError` variants at run time.
 */

/** A machine value: primitives plus the structured data words. */
export type MachineValue =
  | number
  | string
  | boolean
  | null
  | undefined
  | { readonly tag: "symbol"; readonly name: string }
  | { readonly tag: "list"; readonly items: ReadonlyArray<MachineValue> }
  | { readonly tag: "record"; readonly fields: ReadonlyArray<readonly [string, MachineValue]> }
  | { readonly tag: "array"; readonly items: ReadonlyArray<MachineValue> }
  | { readonly tag: "map"; readonly entries: ReadonlyArray<readonly [MachineValue, MachineValue]> }
  | { readonly tag: "set"; readonly items: ReadonlyArray<MachineValue> };

/** An instruction input: a register, a constant, or a label address. */
export type Input<W = MachineValue> =
  | { readonly tag: "reg"; readonly name: string }
  | { readonly tag: "const"; readonly value: W }
  | { readonly tag: "label"; readonly name: string };

/** An operation call in source position; arguments nest recursively. */
export type OperationCall<W = MachineValue> = {
  readonly tag: "op";
  readonly operation: string;
  readonly args: ReadonlyArray<Source<W>>;
};

/** Anything assignable to a register or accepted by an operation. */
export type Source<W = MachineValue> = Input<W> | OperationCall<W>;

/** One controller instruction (grammar section 7). */
export type Instruction<W = MachineValue> =
  | { readonly tag: "assign"; readonly register: string; readonly source: Source<W> }
  | { readonly tag: "test"; readonly operation: string; readonly args: ReadonlyArray<Source<W>> }
  | { readonly tag: "branch"; readonly label: string }
  | { readonly tag: "goto-label"; readonly label: string }
  | { readonly tag: "goto-register"; readonly register: string }
  | { readonly tag: "save"; readonly register: string }
  | { readonly tag: "restore"; readonly register: string }
  | {
      readonly tag: "perform";
      readonly operation: string;
      readonly args: ReadonlyArray<Source<W>>;
    };

/** One assembled controller line: an instruction or a label marker. */
export type MachineStatement<W = MachineValue> =
  | Instruction<W>
  | { readonly tag: "label"; readonly name: string };

/** Every declared machine failure. */
export type MachineError =
  | { readonly tag: "unknown-label"; readonly name: string }
  | { readonly tag: "duplicate-label"; readonly name: string }
  | { readonly tag: "unknown-register"; readonly name: string }
  | { readonly tag: "unknown-operation"; readonly name: string }
  | { readonly tag: "bad-target"; readonly detail: string }
  | { readonly tag: "stack-underflow"; readonly detail: string }
  | {
      readonly tag: "restore-mismatch";
      readonly register: string;
      readonly expected: string;
      readonly found: string;
    }
  | { readonly tag: "out-of-steps"; readonly steps: number };

/** A machine operation over evaluated inputs. */
export type Operation<W = MachineValue> = (args: ReadonlyArray<W | undefined>) => W | undefined;

/** A register input. */
export const register = <W = MachineValue>(name: string): Input<W> => ({ tag: "reg", name });
/** A constant input. */
export const constant = <W = MachineValue>(value: W): Input<W> => ({ tag: "const", value });
/** A label address input. */
export const labelRef = <W = MachineValue>(name: string): Input<W> => ({ tag: "label", name });
/** An operation call source. */
export const op = <W = MachineValue>(
  operation: string,
  ...args: ReadonlyArray<Source<W>>
): OperationCall<W> => ({
  tag: "op",
  operation,
  args,
});

/** Assigns a source to a register. */
export const assign = <W = MachineValue>(
  registerName: string,
  source: Source<W>,
): Instruction<W> => ({
  tag: "assign",
  register: registerName,
  source,
});
/** Tests an operation into the flag register. */
export const test = <W = MachineValue>(
  operation: string,
  ...args: ReadonlyArray<Source<W>>
): Instruction<W> => ({
  tag: "test",
  operation,
  args,
});
/** Branches to a label when the flag is set. */
export const branch = <W = MachineValue>(label: string): Instruction<W> => ({
  tag: "branch",
  label,
});
/** Jumps to a label. */
export const gotoLabel = <W = MachineValue>(label: string): Instruction<W> => ({
  tag: "goto-label",
  label,
});
/** Jumps to the address in a register. */
export const gotoRegister = <W = MachineValue>(registerName: string): Instruction<W> => ({
  tag: "goto-register",
  register: registerName,
});
/** Pushes a register on the stack. */
export const save = <W = MachineValue>(registerName: string): Instruction<W> => ({
  tag: "save",
  register: registerName,
});
/** Pops the stack into a register. */
export const restore = <W = MachineValue>(registerName: string): Instruction<W> => ({
  tag: "restore",
  register: registerName,
});
/** Performs an operation for effect. */
export const perform = <W = MachineValue>(
  operation: string,
  ...args: ReadonlyArray<Source<W>>
): Instruction<W> => ({
  tag: "perform",
  operation,
  args,
});

const formatMachineWord = (value: unknown): string => {
  if (value === null) {
    return "null";
  }
  if (value === undefined) {
    return "undefined";
  }
  if (typeof value === "string") {
    return JSON.stringify(value);
  }
  if (typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  if (Array.isArray(value)) {
    return `[${value.map(formatMachineWord).join(", ")}]`;
  }
  if (typeof value === "object" && "tag" in value) {
    if (value.tag === "symbol" && "name" in value) {
      return String(value.name);
    }
    if (value.tag === "pointer" && "index" in value) {
      return `p${String(value.index)}`;
    }
    if (value.tag === "scalar" && "value" in value) {
      return formatMachineWord(value.value);
    }
    if (
      (value.tag === "list" || value.tag === "array" || value.tag === "set") &&
      "items" in value &&
      Array.isArray(value.items)
    ) {
      return `[${value.items.map(formatMachineWord).join(", ")}]`;
    }
    if (value.tag === "record" && "fields" in value && Array.isArray(value.fields)) {
      return `{ ${value.fields.map((entry) => (Array.isArray(entry) ? `${String(entry[0])}: ${formatMachineWord(entry[1])}` : "")).join(", ")} }`;
    }
    if (value.tag === "map" && "entries" in value && Array.isArray(value.entries)) {
      return `map(${value.entries.map((entry) => (Array.isArray(entry) ? `${formatMachineWord(entry[0])}: ${formatMachineWord(entry[1])}` : "")).join(", ")})`;
    }
  }
  return String(value);
};

/** Renders one machine value in the edition's neutral notation. */
export const formatMachineValue = (value: MachineValue): string => formatMachineWord(value);

/** Renders one machine statement as its controller line. */
export const formatMachineStatement = <W = MachineValue>(
  statement: MachineStatement<W>,
): string => {
  switch (statement.tag) {
    case "label":
      return `${statement.name}:`;
    case "assign":
      return `assign ${statement.register} <- ${formatSource(statement.source)}`;
    case "test":
      return `test ${statement.operation}(${statement.args.map(formatSource).join(", ")})`;
    case "branch":
      return `branch ${statement.label}`;
    case "goto-label":
      return `goto ${statement.label}`;
    case "goto-register":
      return `goto-reg ${statement.register}`;
    case "save":
      return `save ${statement.register}`;
    case "restore":
      return `restore ${statement.register}`;
    case "perform":
      return `perform ${statement.operation}(${statement.args.map(formatSource).join(", ")})`;
  }
};

/** Renders one source input. */
export const formatSource = <W = MachineValue>(source: Source<W>): string => {
  if (source.tag === "reg") {
    return source.name;
  }
  if (source.tag === "const") {
    return formatMachineWord(source.value);
  }
  if (source.tag === "label") {
    return `label ${source.name}`;
  }
  return `${source.operation}(${source.args.map(formatSource).join(", ")})`;
};
