// SPDX-License-Identifier: GPL-3.0-only
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const ECEVAL_C = (): string => readFileSync(new URL("./eceval_5_51.c", import.meta.url), "utf8");

const PROGRAM =
  "(define (factorial n)\n  (if (= n 1)\n      1\n      (* (factorial (- n 1)) n)))\n(factorial 5)\n";

/** Exercise 5.51: the explicit-control evaluator of 5.4 translated
 * into C, built by the system C compiler, and run on the book's
 * factorial session: the machine answers ok and then 120. */
export const ex_5_51 = (): readonly string[] => {
  const dir = mkdtempSync(join(tmpdir(), "sicp_ts_5_51_"));
  try {
    const source = join(dir, "eceval.c");
    const binary = join(dir, "eceval");
    const program = join(dir, "program.scm");
    writeFileSync(source, ECEVAL_C());
    writeFileSync(program, PROGRAM);
    const build = spawnSync("cc", ["-O1", "-o", binary, source], { encoding: "utf8" });
    if (build.status !== 0) throw new Error(`the C translation failed to build: ${build.stderr}`);
    const run = spawnSync(binary, [program], { encoding: "utf8" });
    const output = `${run.stdout}${run.stderr}`;
    if (run.status !== 0) throw new Error(`the C translation failed: ${output}`);
    if (!output.includes("ok")) throw new Error(`the session lost ok: ${output}`);
    if (!output.includes("120")) throw new Error(`the session lost 120: ${output}`);
    return [output];
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
};
