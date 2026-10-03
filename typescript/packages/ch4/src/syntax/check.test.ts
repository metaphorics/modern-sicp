// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { admitSource } from "./check.ts";

const rejected = (source: string): ReadonlyArray<string> => {
  const run = admitSource(source);
  expect(run.ok).toBe(false);
  return run.ok ? [] : run.diagnostics.map((entry) => entry.construct);
};

const accepted = (source: string): void => {
  const run = admitSource(source);
  expect(run.ok).toBe(true);
};

describe("host-object member admission", () => {
  const cases: ReadonlyArray<readonly [string, string]> = [
    ["eval(1);", "eval"],
    ["new Function(\"x\");", "unsupported-new-target"],
    ["require(\"x\");", "amb-depth-first-experiment"],
    ["fetch(\"x\");", "fetch"],
    ["process.env;", "process.env"],
    ["process.exit(1);", "process.exit"],
    ["globalThis.eval(\"x\");", "globalThis.eval"],
    ["Date.now();", "Date.now"],
    ["Date.UTC(2020, 0, 1);", "Date.UTC"],
    ["performance.now();", "performance.now"],
    ["Math.random();", "Math.random"],
    ["Math.log(1);", "Math.log"],
    ["Math.PI;", "Math.PI"],
    ["console.error(1);", "console"],
    ["console.log(1, 2);", "console"],
  ];
  for (const [source, construct] of cases) {
    it(`rejects ${source}`, () => {
      expect(rejected(source)).toContain(construct);
    });
  }

  it("admits the closed member list of the grammar", () => {
    for (const name of ["abs", "floor", "max", "min", "sqrt", "trunc"]) {
      accepted(`Math.${name}(1);`);
    }
    accepted("Number.isInteger(1);");
    accepted("console.log(1);");
  });
});

describe("subset structure diagnostics", () => {
  it("rejects an import after a statement", () => {
    expect(rejected('console.log(1);\nimport { x } from "./m.ts";')).toContain(
      "leading-imports-only",
    );
  });

  it("rejects reassignment of a const binding", () => {
    const run = admitSource("const x = 1;\nx = 2;");
    expect(run.ok).toBe(false);
  });

  it("rejects a duplicate declaration in one scope", () => {
    expect(rejected("const x = 1;\nconst x = 2;")).toContain("x");
  });

  it("rejects a readonly field write", () => {
    const run = admitSource(
      "interface P { readonly x: number }\nconst p: P = { x: 1 };\np.x = 2;",
    );
    expect(run.ok).toBe(false);
    expect(run.ok ? [] : run.diagnostics.map((d) => d.kind)).toContain("ReadOnlyField");
  });
});
