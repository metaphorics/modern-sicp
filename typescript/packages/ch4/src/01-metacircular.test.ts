// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { runEvaluator } from "../../ch5/src/04-eceval.ts";
import { compileAndRun } from "../../ch5/src/05-compilation.ts";
import { runAnalyzedSource, runSource, Session } from "./01-metacircular.ts";
import { makeRecord } from "./runtime/value.ts";

const KERNEL_SOURCE = readFileSync(
  new URL(
    "../../../../spec/host-subsets/typescript/witnesses/section-9-kernel.ts",
    import.meta.url,
  ),
  "utf8",
);

describe("section 9 guest evaluator kernel", () => {
  it("prints 120 under each teaching engine", () => {
    const direct = runSource(KERNEL_SOURCE);
    expect(direct.outcome.tag).toBe("ok");
    expect(direct.transcript).toEqual(["120"]);

    const analyzed = runAnalyzedSource(KERNEL_SOURCE);
    expect(analyzed.outcome.tag).toBe("ok");
    expect(analyzed.transcript).toEqual(direct.transcript);

    const eceval = runEvaluator(KERNEL_SOURCE);
    expect(eceval.outcome.tag).toBe("ok");
    expect(eceval.transcript).toEqual(direct.transcript);

    const compiled = compileAndRun(KERNEL_SOURCE);
    expect(compiled.outcome.tag).toBe("ok");
    expect(compiled.transcript).toEqual(direct.transcript);
  });

  it("distinguishes absent record fields from present undefined values", () => {
    const session = new Session("core");
    const record = makeRecord([["present", undefined]]);

    expect(session.memberGet(record, "missing")).toEqual({
      tag: "error",
      error: { tag: "unknown-field", field: "missing" },
    });
    expect(session.memberGet(record, "present")).toEqual({ tag: "ok", value: undefined });
  });
});

describe("static named imports", () => {
  const VALUE_IMPORT = 'import { Effect as E } from "effect";\nconsole.log(E);';
  const TYPE_IMPORT = 'import { type Effect } from "effect";\nconsole.log(1);';
  const modules = { effect: { Effect: 41 } };

  it("binds linked value imports under their local name in every engine", () => {
    for (const result of [
      runSource(VALUE_IMPORT, "core", modules),
      runAnalyzedSource(VALUE_IMPORT, "core", modules),
      runEvaluator(VALUE_IMPORT, {}, modules),
      compileAndRun(VALUE_IMPORT, modules),
    ]) {
      expect(result.outcome.tag).toBe("ok");
      expect(result.transcript).toEqual(["41"]);
    }
  });

  it("reports an unlinked value import instead of discarding it, and erases type imports", () => {
    const unresolved = { tag: "unresolved-import", module: "effect", name: "Effect" };
    for (const result of [
      runSource(VALUE_IMPORT),
      runAnalyzedSource(VALUE_IMPORT),
      runEvaluator(VALUE_IMPORT),
      compileAndRun(VALUE_IMPORT),
    ]) {
      expect(result.outcome).toEqual({ tag: "error", error: unresolved });
      expect(result.transcript).toEqual([]);
    }
    for (const result of [
      runSource(TYPE_IMPORT),
      runAnalyzedSource(TYPE_IMPORT),
      runEvaluator(TYPE_IMPORT),
      compileAndRun(TYPE_IMPORT),
    ]) {
      expect(result.outcome.tag).toBe("ok");
      expect(result.transcript).toEqual(["1"]);
    }
  });
});
