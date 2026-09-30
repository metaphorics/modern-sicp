// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { runSource } from "../../packages/ch4/src/01-metacircular.ts";
import type { Program } from "../../packages/ch4/src/syntax/ast.ts";
import { readProgram } from "../../packages/ch5/src/04-eceval.ts";

const ECEVAL_C = (): string => readFileSync(new URL("./eceval_5_51.c", import.meta.url), "utf8");

/** The pinned session: the guest factorial of the book's 5.51 run, in
 * the checked guest source of this edition. */
const PROGRAM_SOURCE = [
  "function factorial(n: number): number {",
  "  return n === 1 ? 1 : factorial(n - 1) * n;",
  "}",
  "console.log(factorial(5));",
].join("\n");

/** The JSON projection of the shared Program data the C evaluator
 * reads: the tagged syntax records of the shared AST with their spans
 * dropped, serialized as typed data. It is data, never source text. */
const serializeProgram = (program: Program): string =>
  JSON.stringify(
    {
      kind: "program",
      forms: program.map((form) =>
        JSON.parse(
          JSON.stringify(form, (key, value: unknown) => (key === "span" ? undefined : value)),
        ),
      ),
    },
    undefined,
    1,
  );

/** Exercise 5.51: the explicit-control evaluator of 5.4 translated into
 * C, over the guest runtime's object world. The boundary driver parses
 * the guest program with the shared reader, serializes the typed syntax
 * data, builds the C translation with the system C compiler, and runs
 * it; the C process transcript must equal the direct evaluator's
 * transcript for the same program. */
export const ex_5_51 = (): readonly string[] => {
  const reference = runSource(PROGRAM_SOURCE);
  if (reference.outcome.tag !== "ok") {
    throw new Error(
      `the direct evaluator faulted on the pinned program: ${JSON.stringify(reference.outcome.error)}`,
    );
  }
  const dir = mkdtempSync(join(tmpdir(), "sicp_ts_5_51_"));
  try {
    const source = join(dir, "eceval.c");
    const binary = join(dir, "eceval");
    const input = join(dir, "program.json");
    writeFileSync(source, ECEVAL_C());
    writeFileSync(input, serializeProgram(readProgram(PROGRAM_SOURCE)));
    const build = spawnSync("cc", ["-std=gnu23", "-O1", "-o", binary, source, "-lm"], {
      encoding: "utf8",
    });
    if (build.status !== 0) throw new Error(`the C translation failed to build: ${build.stderr}`);
    const run = spawnSync(binary, [input], { encoding: "utf8" });
    if (run.status !== 0) throw new Error(`the C translation failed: ${run.stdout}${run.stderr}`);
    const produced = run.stdout.split("\n").filter((line) => line.length > 0);
    const expected = reference.transcript.filter((line) => line.length > 0);
    for (let i = 0; i < Math.max(produced.length, expected.length); i += 1) {
      if (produced[i] !== expected[i]) {
        throw new Error(
          `the C transcript diverged from the direct transcript at line ${i}: ${produced[i] ?? "<missing>"} != ${expected[i] ?? "<missing>"}`,
        );
      }
    }
    return produced;
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
};
