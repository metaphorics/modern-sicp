// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.2

import type { Value } from "@sicp-ts/ch4/core";
/**
 * The register-machine simulator's word types. Assembling and running a
 * machine is section 5.2's lesson; the spine fixes the vocabulary the
 * controllers, the explicit-control evaluator, and the compiler share.
 */
import type { Effect, HashMap, Ref, Result } from "effect";
import { Schema } from "effect";

export class AssembleError extends Schema.TaggedError<AssembleError>()("AssembleError", {
  message: Schema.String,
}) {}

export class SimError extends Schema.TaggedError<SimError>()("SimError", {
  message: Schema.String,
}) {}

/** An instruction operand: a register, a constant, or a label reference. */
export type Arg =
  | { readonly _tag: "Reg"; readonly name: string }
  | { readonly _tag: "Const"; readonly value: Value }
  | { readonly _tag: "LabelRef"; readonly label: string };

/** The right-hand side of an `assign`: an `Arg`, or an operation applied to args. */
export type Src =
  | Arg
  | { readonly _tag: "OpCall"; readonly op: string; readonly args: ReadonlyArray<Arg> };

/** Where a `goto` lands: a label named in the controller text. */
export interface LabelTarget {
  readonly _tag: "Label";
  readonly name: string;
}

export type Instr =
  | { readonly _tag: "Assign"; readonly reg: string; readonly src: Src }
  | { readonly _tag: "Test"; readonly op: string; readonly args: ReadonlyArray<Arg> }
  | { readonly _tag: "Branch"; readonly label: string }
  | { readonly _tag: "Goto"; readonly target: LabelTarget }
  | { readonly _tag: "Save"; readonly reg: string }
  | { readonly _tag: "Restore"; readonly reg: string }
  | { readonly _tag: "Perform"; readonly op: string; readonly args: ReadonlyArray<Arg> };

/** One controller line: a label the assembler resolves to an index, or an instruction. */
export type AssembleLine =
  | { readonly _tag: "Label"; readonly name: string }
  | { readonly _tag: "Instruction"; readonly instr: Instr };

/** The assembler's output: instructions in execution order, plus each label's index. */
export interface Program {
  readonly instrs: ReadonlyArray<Instr>;
  readonly labels: ReadonlyMap<string, number>;
}

/** Assembles controller lines; duplicate or dangling labels are `AssembleError`s. */
export type Assemble = (
  lines: ReadonlyArray<AssembleLine>,
) => Result.Result<Program, AssembleError>;

export interface Machine {
  readonly regs: HashMap.HashMap<string, Ref.Ref<Value>>;
  readonly stack: Ref.Ref<Array<Value>>;
  readonly ops: HashMap.HashMap<string, (...args: ReadonlyArray<Value>) => Value>;
  readonly run: Effect.Effect<Value, SimError>;
}

/** Builds a machine from named operations and controller lines, ready to `run`. */
export type MakeMachine = (
  ops: Iterable<readonly [string, (...args: ReadonlyArray<Value>) => Value]>,
  lines: ReadonlyArray<AssembleLine>,
) => Result.Result<Machine, AssembleError>;
