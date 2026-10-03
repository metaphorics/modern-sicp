// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { runSource } from "../../packages/ch4/src/01-metacircular.ts";
import {
  ArrayValue,
  Closure,
  ErrorValue,
  MapValue,
  PrimitiveProcedure,
  RecordValue,
  SetValue,
  ThunkValue,
} from "../../packages/ch4/src/runtime/value.ts";
import type { Source } from "../../packages/ch5/src/01-register-machines.ts";
import { MachineErrorValue, readProgram, type Word } from "../../packages/ch5/src/04-eceval.ts";
import type { CompiledProgram } from "../../packages/ch5/src/05-compilation.ts";
import { compileProgram, isCompileError } from "../../packages/ch5/src/05-compilation.ts";

const RUNTIME_C = (): string =>
  readFileSync(new URL("./metacircular_backend_5_52.c", import.meta.url), "utf8");

/** The edition's checked guest evaluator source: the self-interpreter
 * the exercise compiles. */
const METACIRCULAR_SOURCE = (): string =>
  readFileSync(
    new URL(
      "../../../spec/host-subsets/typescript/witnesses/metacircular-evaluator.ts",
      import.meta.url,
    ),
    "utf8",
  );

/** A C identifier for a controller label or register name. */
const cIdent = (name: string): string => name.replaceAll(/[^a-zA-Z0-9_]/g, "_");

const cString = (text: string): string =>
  `"${text.replaceAll("\\", "\\\\").replaceAll('"', '\\"').replaceAll("\n", "\\n")}"`;

const C_OPERATION_NAMES: Readonly<Record<string, string>> = {
  "adjoin-arg": "adjoinArg",
  "apply-primitive-procedure": "applyProcedure",
  "array-index": "arrayIndex",
  "array-length": "arrayLength",
  "array-new": "arrayNew",
  "boolean-not": "booleanNot",
  "define-variable": "defineVariableValue",
  "empty-argl": "emptyArgList",
  "error-new": "errorNew",
  "extend-environment": "extendEnvironment",
  "is-error-value": "isErrorValue",
  "is-primitive-procedure": "isPrimitiveProcedure",
  "is-true": "isTrue",
  "lookup-variable-value": "lookupVariableValue",
  "map-new": "mapNew",
  "number-add": "numberAdd",
  "number-less": "numberLess",
  "number-less-equal": "numberLessEqual",
  "number-multiply": "numberMultiply",
  "number-subtract": "numberSubtract",
  "parent-environment": "parentEnvironment",
  "print-value": "printValue",
  "procedure-environment": "compiledProcedureEnvironment",
  "procedure-entry": "procedureEntryWord",
  "procedure-parameters": "compiledProcedureParameters",
  "record-from": "recordFrom",
  "record-get": "recordGetValue",
  "record-set": "recordSetValue",
  "set-variable-value": "setVariableValue",
  "string-concat": "stringConcat",
  "throw-box": "throwBox",
  "throw-error-pending": "throwErrorPending",
  "throw-pending": "throwPending",
  "typeof-value": "typeofValue",
  "value-equal": "valueEqual",
  "value-not-equal": "valueNotEqual",
};

const cOperation = (name: string): string => C_OPERATION_NAMES[name] ?? cIdent(name);

const globalRegisters: Readonly<Record<string, true>> = {
  exp: true,
  env: true,
  val: true,
  proc: true,
  argl: true,
  unev: true,
  arg1: true,
  arg2: true,
  continue: true,
};

type SequenceTag = "list" | "array" | "set";

const unreachable = (value: never): never => {
  throw new Error(`unhandled C sequence tag: ${String(value)}`);
};

const cSequenceTag = (tag: SequenceTag): string => {
  switch (tag) {
    case "list":
      return "T_LIST";
    case "array":
      return "T_ARRAY";
    case "set":
      return "T_SET";
    default:
      return unreachable(tag);
  }
};

const valueC = (value: unknown): string => {
  if (typeof value === "number") return `make_num(${value})`;
  if (typeof value === "string") return `make_str(${cString(value)})`;
  if (typeof value === "boolean") return value ? "V_TRUE" : "V_FALSE";
  if (value === null) return "V_NULL";
  if (value === undefined) return "V_UNDEF";
  if (value instanceof ArrayValue) return sequenceC("T_ARRAY", value.items);
  if (value instanceof SetValue) return sequenceC("T_SET", [...value.items]);
  if (value instanceof RecordValue) return recordC([...value.fields]);
  if (value instanceof MapValue) return mapC([...value.entries]);
  if (
    value instanceof Closure ||
    value instanceof PrimitiveProcedure ||
    value instanceof ThunkValue ||
    value instanceof ErrorValue ||
    value instanceof MachineErrorValue
  ) {
    throw new Error("a runtime object cannot be embedded as a compiled C literal");
  }
  if (Array.isArray(value)) return sequenceC("T_ARRAY", value);
  if (typeof value === "object" && value !== null) {
    if ("bindings" in value && value.bindings instanceof Map && "parent" in value) {
      throw new Error("a runtime environment cannot be embedded as a compiled C literal");
    }
    if ("tag" in value) {
      if (value.tag === "symbol" && "name" in value && typeof value.name === "string") {
        return `make_sym(${cString(value.name)})`;
      }
      if (
        (value.tag === "list" || value.tag === "array" || value.tag === "set") &&
        "items" in value &&
        Array.isArray(value.items)
      ) {
        const items: unknown[] = value.items;
        return sequenceC(cSequenceTag(value.tag), items);
      }
      if (value.tag === "record" && "fields" in value) return recordC(recordFields(value.fields));
      if (value.tag === "map" && "entries" in value) return mapC(mapEntries(value.entries));
    }
    return recordC(Object.entries(value));
  }
  throw new Error("unsupported compiled C literal");
};

const sequenceC = (tag: string, items: ReadonlyArray<unknown>): string =>
  `make_seq(${tag}, ${items.length === 0 ? "NULL" : `(Val *[]){${items.map(valueC).join(", ")}}`}, ${items.length})`;

const recordC = (fields: ReadonlyArray<readonly [string, unknown]>): string => {
  const names = fields.map(([name]) => cString(name)).join(", ");
  const values = fields.map(([, item]) => valueC(item)).join(", ");
  return `make_record((const char *[]){${names}}, (Val *[]){${values}}, ${fields.length})`;
};

const mapC = (entries: ReadonlyArray<readonly [unknown, unknown]>): string => {
  const keys = entries.map(([key]) => valueC(key)).join(", ");
  const values = entries.map(([, item]) => valueC(item)).join(", ");
  return `make_map((Val *[]){${keys}}, (Val *[]){${values}}, ${entries.length})`;
};

const isPair = (entry: unknown): entry is readonly [unknown, unknown] =>
  Array.isArray(entry) && entry.length === 2;

const recordFields = (fields: unknown): ReadonlyArray<readonly [string, unknown]> => {
  if (!Array.isArray(fields)) throw new Error("a C record literal must contain field pairs");
  const result: Array<readonly [string, unknown]> = [];
  for (const entry of fields) {
    if (!isPair(entry) || typeof entry[0] !== "string") {
      throw new Error("a C record literal has an invalid field pair");
    }
    result.push([entry[0], entry[1]]);
  }
  return result;
};

const mapEntries = (entries: unknown): ReadonlyArray<readonly [unknown, unknown]> => {
  if (!Array.isArray(entries)) throw new Error("a C map literal must contain entry pairs");
  const result: Array<readonly [unknown, unknown]> = [];
  for (const entry of entries) {
    if (!isPair(entry)) throw new Error("a C map literal has an invalid entry pair");
    result.push([entry[0], entry[1]]);
  }
  return result;
};

/** One Source as a C expression: inputs first, then operation calls
 * with their arguments resolved recursively, the exchange contract's
 * nesting. */
const sourceC = (source: Source<Word>): string => {
  if (source.tag === "reg") return `R_${cIdent(source.name)}`;
  if (source.tag === "const") {
    const value = source.value;
    if (
      typeof value === "object" &&
      value !== null &&
      "tag" in value &&
      value.tag === "symbol" &&
      "name" in value &&
      typeof value.name === "string"
    ) {
      return `addr_word(&&${cIdent(value.name)})`;
    }
    return valueC(value);
  }
  if (source.tag === "label") return `addr_word(&&${cIdent(source.name)})`;
  if (source.operation === "make-procedure") {
    const entry = source.args[0];
    const params = source.args[1];
    const env = source.args[2];
    if (
      entry === undefined ||
      entry.tag !== "const" ||
      typeof entry.value !== "string" ||
      params === undefined ||
      env === undefined
    ) {
      throw new Error("a compiled C procedure needs its entry label, parameters, and environment");
    }
    return `makeCompiledProcedure(&&${cIdent(entry.value)}, ${sourceC(params)}, ${sourceC(env)})`;
  }
  return `${cOperation(source.operation)}(${source.args.map(sourceC).join(", ")})`;
};

/** One controller statement as C lines over the backend runtime. */
const statementC = (statement: CompiledProgram["instructions"][number]): string[] => {
  switch (statement.tag) {
    case "label":
      return [`${cIdent(statement.name)}: ;`];
    case "assign":
      return [`R_${cIdent(statement.register)} = ${sourceC(statement.source)};`];
    case "test":
      return [
        `flag = ${cOperation(statement.operation)}(${statement.args.map(sourceC).join(", ")}) != V_FALSE;`,
      ];
    case "branch":
      return [`if (flag) goto ${cIdent(statement.label)};`];
    case "goto-label":
      return [`goto ${cIdent(statement.label)};`];
    case "goto-register":
      return [`goto *addr_of(R_${cIdent(statement.register)});`];
    case "save":
      return [`push_val(R_${cIdent(statement.register)});`];
    case "restore":
      return [`R_${cIdent(statement.register)} = pop_val();`];
    case "perform":
      return [
        `(void)${cOperation(statement.operation)}(${statement.args.map(sourceC).join(", ")});`,
      ];
  }
};

/** The compiled forms, appended after the runtime file: one
 * `run_compiled` body walking `CompiledProgram.instructions` in order,
 * label markers included. */
const collectSourceRegisters = (source: Source<Word>, registers: Set<string>): void => {
  if (source.tag === "reg") {
    registers.add(source.name);
  } else if (source.tag === "op") {
    for (const argument of source.args) collectSourceRegisters(argument, registers);
  }
};

const programRegisters = (program: CompiledProgram): string[] => {
  const registers = new Set<string>();
  for (const statement of program.instructions) {
    switch (statement.tag) {
      case "assign":
        registers.add(statement.register);
        collectSourceRegisters(statement.source, registers);
        break;
      case "test":
      case "perform":
        for (const argument of statement.args) collectSourceRegisters(argument, registers);
        break;
      case "goto-register":
      case "save":
      case "restore":
        registers.add(statement.register);
        break;
      case "label":
      case "branch":
      case "goto-label":
        break;
    }
  }
  return [...registers].filter((name) => !Object.hasOwn(globalRegisters, name));
};
const formsC = (program: CompiledProgram): string => {
  const lines = program.instructions.flatMap(statementC);
  const declarations = programRegisters(program).map(
    (name) => `  Val *R_${cIdent(name)} = V_UNDEF;`,
  );
  return [
    "",
    "static void run_compiled(void) {",
    ...declarations,
    "  R_env = env_word(global_environment);",
    "  R_continue = addr_word(&&done);",
    ...lines.map((line) => `  ${line}`),
    "done: ;",
    "}",
    "",
  ].join("\n");
};

/** Exercise 5.52: the compiler emits C from its typed instruction
 * stream, the compiled forms are appended after the runtime file, and
 * the resulting guest interpreter in C answers the same session the
 * direct evaluator answers. */
export const ex_5_52 = (): readonly string[] => {
  const source = METACIRCULAR_SOURCE();
  const reference = runSource(source);
  if (reference.outcome.tag !== "ok") {
    throw new Error(
      `the direct evaluator faulted on the guest source: ${JSON.stringify(reference.outcome.error)}`,
    );
  }
  const compiled = compileProgram(readProgram(source));
  if (isCompileError(compiled)) {
    throw new Error(`the guest evaluator did not compile: ${JSON.stringify(compiled)}`);
  }
  const dir = mkdtempSync(join(tmpdir(), "sicp_ts_5_52_"));
  try {
    const unit = join(dir, "metacircular_backend.c");
    const binary = join(dir, "metacircular");
    writeFileSync(unit, `${RUNTIME_C()}${formsC(compiled)}`);
    const build = spawnSync("cc", ["-std=gnu11", "-O1", "-o", binary, unit, "-lm"], {
      encoding: "utf8",
    });
    if (build.status !== 0) throw new Error(`the C backend failed to build: ${build.stderr}`);
    const run = spawnSync(binary, [], { encoding: "utf8" });
    if (run.status !== 0) throw new Error(`the C backend failed: ${run.stdout}${run.stderr}`);
    const produced = run.stdout.split("\n").filter((line) => line.length > 0);
    const expected = reference.transcript.filter((line) => line.length > 0);
    for (let i = 0; i < Math.max(produced.length, expected.length); i += 1) {
      if (produced[i] !== expected[i]) {
        throw new Error(
          `the C interpreter transcript diverged at line ${i}: ${produced[i] ?? "<missing>"} != ${expected[i] ?? "<missing>"}`,
        );
      }
    }
    return produced;
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
};
