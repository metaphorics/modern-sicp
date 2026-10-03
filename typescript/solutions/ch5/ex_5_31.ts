// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { readProgram } from "../../packages/ch5/src/04-eceval.ts";
import type { CompiledProgram } from "../../packages/ch5/src/05-compilation.ts";
import { compileProgram, isCompileError } from "../../packages/ch5/src/05-compilation.ts";

/** One save the analysis reports: its register and step, and whether
 * the region it protects modifies the register before the matching
 * restore. A save whose register is untouched across its region keeps
 * the old value alive for nothing: that is the superfluous one. */
export type SaveReport = {
  readonly register: string;
  readonly step: number;
  readonly superfluous: boolean;
};

const savedRegisters = (statement: CompiledProgram["instructions"][number]): string | null =>
  statement.tag === "save" ? statement.register : null;

/** Walks one compiled program and pairs each save with its restore,
 * then reports whether the enclosed statements modify the register. */
export const analyzeSaves = (source: string): readonly SaveReport[] => {
  const compiled = compileProgram(readProgram(source));
  if (isCompileError(compiled)) {
    throw new Error(`compilation failed: ${JSON.stringify(compiled)}`);
  }
  const statements = compiled.instructions;
  const reports: SaveReport[] = [];
  const stack: { register: string; step: number }[] = [];
  for (let step = 0; step < statements.length; step += 1) {
    const statement = statements[step];
    if (statement === undefined) continue;
    const saved = savedRegisters(statement);
    if (saved !== null) {
      stack.push({ register: saved, step });
      continue;
    }
    if (statement.tag !== "restore") continue;
    const frame = stack.pop();
    if (frame === undefined || frame.register !== statement.register) continue;
    let modified = false;
    for (let between = frame.step + 1; between < step; between += 1) {
      const inner = statements[between];
      if (inner !== undefined && inner.tag === "assign" && inner.register === frame.register) {
        modified = true;
      }
    }
    reports.push({ register: frame.register, step: frame.step, superfluous: !modified });
  }
  return reports;
};

/** Exercise 5.31's answer for the compiled factorial: the saves whose
 * registers the protected region never rewrites. */
export const ex_5_31 = (): readonly string[] => {
  const reports = analyzeSaves(
    "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }",
  );
  return reports.map(
    (report) =>
      `save ${report.register} at ${report.step}: ${report.superfluous ? "superfluous" : "needed"}`,
  );
};
