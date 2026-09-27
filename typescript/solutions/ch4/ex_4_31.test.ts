// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import { answers, ex_4_31 } from "./ex_4_31.js";

describe("exercise 4.31: lazy and lazy-memo parameter declarations", () => {
  it("pins the declared session: taken, the f run, and the two probes", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed[3]).toBe("taken");
    expect(observed[5]).toBe("(1 5 5 4 30 30)");
    expect(observed[6]).toBe("5");
    expect(observed[8]).toBe("(10 10)");
    expect(observed[9]).toBe("7");
    expect(observed[11]).toBe("(10 10)");
    expect(observed[12]).toBe("8");
  });

  it("keeps ordinary definitions strict", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed.slice(0, 3)).toStrictEqual(["ok", "ok", "ok"]);
  });

  it("reports the modes and their counts", () => {
    expect(ex_4_31()).toContain("lazy-memo");
  });
});
