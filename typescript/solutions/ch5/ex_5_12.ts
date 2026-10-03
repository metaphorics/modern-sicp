// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  formatMachineStatement,
  formatSource,
  type MachineStatement,
} from "../../packages/ch5/src/01-register-machines.ts";
import { type AssemblyResult, assemble } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, gcdController } from "./ex_5_07.ts";

/** The assembler's summary of exercise 5.12: the instructions grouped
 * by type with duplicates removed, the registers the controller names,
 * the entry-point registers, the saved and restored registers, the
 * assign sources of each register, and the defined labels. Every list
 * is deterministic: groups in first-appearance order, names and source
 * texts sorted, and labels in definition order. */
const renderNames = (label: string, names: ReadonlyArray<string>): string =>
  `${label}:${names.length === 0 ? "" : ` ${names.join(" ")}`}`;

export const summarize = (controller: readonly MachineStatement[]): readonly string[] => {
  const groups = new Map<string, Set<string>>();
  const registers = new Set<string>();
  const entryPoints = new Set<string>();
  const saved = new Set<string>();
  const restored = new Set<string>();
  const sources = new Map<string, Set<string>>();
  const labels: string[] = [];
  for (const statement of controller) {
    if (statement.tag === "label") {
      labels.push(statement.name);
      continue;
    }
    const group = groups.get(statement.tag) ?? new Set<string>();
    group.add(formatMachineStatement(statement));
    groups.set(statement.tag, group);
    if (statement.tag === "assign") {
      registers.add(statement.register);
      const owned = sources.get(statement.register) ?? new Set<string>();
      owned.add(formatSource(statement.source));
      sources.set(statement.register, owned);
      if (statement.source.tag === "reg") registers.add(statement.source.name);
      if (statement.source.tag === "op") {
        for (const arg of statement.source.args) {
          if (arg.tag === "reg") registers.add(arg.name);
        }
      }
    }
    if (statement.tag === "goto-register") {
      registers.add(statement.register);
      entryPoints.add(statement.register);
    }
    if (statement.tag === "save") {
      registers.add(statement.register);
      saved.add(statement.register);
    }
    if (statement.tag === "restore") {
      registers.add(statement.register);
      restored.add(statement.register);
    }
    if (statement.tag === "test" || statement.tag === "perform") {
      for (const arg of statement.args) {
        if (arg.tag === "reg") registers.add(arg.name);
      }
    }
    if (statement.tag === "branch" || statement.tag === "goto-label") {
      // labels are named by the jump targets, not registers
    }
  }
  const lines: string[] = [];
  for (const [tag, texts] of groups) {
    lines.push(`${tag}: ${[...texts].sort().join(", ")}`);
  }
  lines.push(renderNames("registers", [...registers].sort()));
  lines.push(renderNames("entry-points", [...entryPoints].sort()));
  lines.push(renderNames("saved", [...saved].sort()));
  lines.push(renderNames("restored", [...restored].sort()));
  for (const registerName of [...sources.keys()].sort()) {
    const owned = sources.get(registerName);
    lines.push(`${registerName} <- ${(owned === undefined ? [] : [...owned]).sort().join(", ")}`);
  }
  lines.push(`labels: ${labels.join(" ")}`);
  return lines;
};

/** Exercise 5.12: the summary is computed from a fresh assembly of the
 * gcd controller the 5.7 machine runs. */
export const ex_5_12 = (): readonly string[] => {
  const assembly: AssemblyResult = assemble(gcdController, arithmeticOperations);
  if (!assembly.ok) {
    throw new Error(`the gcd controller failed to assemble: ${JSON.stringify(assembly.error)}`);
  }
  return summarize(assembly.value.statements);
};
