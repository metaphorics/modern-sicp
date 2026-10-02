// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { admitSource } from "./check.ts";
import { parseProgram, ReadError } from "./parse.ts";

const importOf = (text: string) => {
  const program = parseProgram(text);
  const [first] = program;
  if (first?.tag !== "import") {
    throw new Error(`expected one import declaration, got ${String(program[0]?.tag)}`);
  }
  return first;
};

describe("import declarations", () => {
  it("parses a leading type modifier as a whole-declaration type-only import", () => {
    const item = importOf('import type { Effect } from "effect";');
    expect(item.from).toBe("effect");
    expect(item.names).toEqual([{ imported: "Effect", local: "Effect", isType: true }]);
  });

  it("parses mixed per-name and value names with aliases", () => {
    const item = importOf('import { type Effect, run as go } from "effect";');
    expect(item.names).toEqual([
      { imported: "Effect", local: "Effect", isType: true },
      { imported: "run", local: "go", isType: false },
    ]);
  });

  it("rejects the unbraced forms the grammar does not admit", () => {
    expect(() => parseProgram('import type * as ns from "effect";')).toThrow(ReadError);
    expect(() => parseProgram('import Default from "effect";')).toThrow(ReadError);
  });

  it("admits and runs a type-only import through the checker with nothing linked", () => {
    const run = admitSource('import type { Effect } from "effect";\nconsole.log(1);');
    expect(run.ok).toBe(true);
    const [first] = run.ok ? run.program : [];
    expect(first).toMatchObject({
      tag: "import",
      names: [{ imported: "Effect", local: "Effect", isType: true }],
    });
  });
});
