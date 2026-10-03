// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.3

import { describe, expect, it } from "vitest";
import { runAmbAnswers } from "./03-nondeterministic.ts";
import { format } from "./read.ts";

describe("section 4.3: the named search experiment", () => {
  it("undoes ordinary assignments before trying another alternative", () => {
    const run = runAmbAnswers(
      `
        let count = 0;
        choose(count = count + 1, count = count + 1);
        count;
      `,
      "amb-depth-first-experiment",
    );

    expect(run.answers).toEqual([1, 1]);
    expect(run.status).toBe("completed");
  });

  it("restores array length after an out-of-range assignment", () => {
    const run = runAmbAnswers(
      `
        let values: number[] = [];
        choose((values[2] = 1), values.length);
      `,
      "amb-depth-first-experiment",
    );

    expect(run.answers).toEqual([1, 0]);
  });

  it("preserves a permanent RHS write when the outer assignment is undone", () => {
    const run = runAmbAnswers(
      `
        let count = 0;
        choose(
          (count = (() => {
            permanentAssign(count, 1);
            return count + 1;
          })()),
          count,
        );
      `,
      "amb-depth-first-experiment",
    );

    expect(run.answers).toEqual([2, 1]);
  });
  it("returns from a function before its remaining statements", () => {
    const run = runAmbAnswers(
      `
        let after = 0;
        const chooseValue = (flag: boolean): number => {
          if (flag) {
            return 1;
          }
          after = after + 1;
          return 2;
        };
        const result = chooseValue(true);
        [result, after];
      `,
      "amb-depth-first-experiment",
    );

    expect(run.answers.map(format)).toEqual(["[1, 0]"]);
  });

  it("keeps permanent writes while backtracking", () => {
    const run = runAmbAnswers(
      `
        let count = 0;
        choose(
          permanentAssign(count, count + 1),
          permanentAssign(count, count + 1),
        );
        count;
      `,
      "amb-depth-first-experiment",
    );

    expect(run.answers).toEqual([1, 2]);
  });

  it("runs ifFail's fallback once after every primary answer is exhausted", () => {
    const run = runAmbAnswers("ifFail(choose(1, 2), 99);", "amb-depth-first-experiment");

    expect(run.answers).toEqual([1, 2, 99]);
    expect(run.failures).toBe(0);
    expect(run.status).toBe("completed");
  });

  it("counts a failed requirement once, not its exhausted continuation", () => {
    const run = runAmbAnswers("require(false);", "amb-depth-first-experiment");

    expect(run.answers).toEqual([]);
    expect(run.failures).toBe(1);
    expect(run.steps).toBe(1);
  });

  it("counts failure separately from successful resumes and stops before the answer limit", () => {
    const complete = runAmbAnswers("choose(1, 2);", "amb-depth-first-experiment");
    expect(complete.answers).toEqual([1, 2]);
    expect(complete.failures).toBe(0);
    expect(complete.steps).toBe(3);
    expect(complete.status).toBe("completed");

    const limited = runAmbAnswers("choose(1, 2);", "amb-depth-first-experiment", 1, {
      maxAnswers: 1,
    });
    expect(limited.answers).toEqual([1]);
    expect(limited.steps).toBe(0);
    expect(limited.status).toBe("cut-off");
  });

  it("rejects a permanent write without a variable or property target", () => {
    const run = runAmbAnswers("permanentAssign(1, 1);", "amb-depth-first-experiment");

    expect(run.outcome).toEqual({
      tag: "error",
      error: { tag: "unknown-syntax", construct: "invalid-assignment-target" },
    });
  });

  it("does not evaluate guest effects when maxAnswers is zero", () => {
    const run = runAmbAnswers('console.log("visible"); 1;', "amb-depth-first-experiment", 1, {
      maxAnswers: 0,
    });

    expect(run.answers).toEqual([]);
    expect(run.transcript).toEqual([]);
    expect(run.steps).toBe(0);
    expect(run.status).toBe("cut-off");
  });
});
