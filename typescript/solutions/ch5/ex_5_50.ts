// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { readFileSync } from "node:fs";
import type { RunResult } from "../../packages/ch4/src/01-metacircular.ts";
import type { Value } from "../../packages/ch4/src/runtime/value.ts";
import { runSelfInterpretation } from "../../packages/ch5/src/06-selfinterp.ts";

const SELF_INTERPRETER_SOURCE = readFileSync(
  new URL(
    "../../../spec/host-subsets/typescript/witnesses/metacircular-evaluator.ts",
    import.meta.url,
  ),
  "utf8",
);

/** One clean engine session: the outcome is ok, with its value and
 * transcript. */
type SelfRun = {
  readonly value: Value;
  readonly transcript: ReadonlyArray<string>;
};

/** Structural rendering used only to compare two engine results: maps
 * and sets keep their entries, everything else its JSON shape. */
const renderResult = (value: Value): string =>
  JSON.stringify(value, (_key, item: Value | Map<Value, Value> | Set<Value> | object) =>
    item instanceof Map
      ? { mapEntries: [...item] }
      : item instanceof Set
        ? { setItems: [...item] }
        : item,
  );

const session = (engine: "direct" | "eceval" | "compiled"): SelfRun => {
  const run: RunResult = runSelfInterpretation(engine, SELF_INTERPRETER_SOURCE);
  if (run.outcome.tag !== "ok") {
    throw new Error(
      `the ${engine} self-interpretation run faulted: ${JSON.stringify(run.outcome.error)}`,
    );
  }
  return { value: run.outcome.value, transcript: run.transcript };
};

/** Exercise 5.50: the metacircular evaluator, written as ordinary
 * checked guest source, runs as a guest program through three engines:
 * the direct evaluator, the explicit-control evaluator, and the
 * evaluator compiled by the 5.5 compiler and executed on the register
 * machine. The solution requires the three runs to agree on both the
 * result value and the whole transcript. The compiled run interprets
 * the guest evaluator itself, one more interpretation level than the
 * direct run, by construction. The observable output is the shared
 * guest session transcript. */
export const ex_5_50 = (): readonly string[] => {
  const direct = session("direct");
  const eceval = session("eceval");
  const compiled = session("compiled");
  const value = renderResult(direct.value);
  if (renderResult(eceval.value) !== value || renderResult(compiled.value) !== value) {
    throw new Error("the engines disagreed on the guest program's result value");
  }
  for (const [engine, run] of [
    ["eceval", eceval],
    ["compiled", compiled],
  ] as const) {
    const same =
      run.transcript.length === direct.transcript.length &&
      run.transcript.every((line, i) => line === direct.transcript[i]);
    if (!same) throw new Error(`the ${engine} transcript diverged from the direct transcript`);
  }
  return direct.transcript;
};
