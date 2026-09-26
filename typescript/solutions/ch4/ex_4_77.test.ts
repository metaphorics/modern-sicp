// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { frameOf, makeQueryEngine, readQuery } from "../../packages/ch4/src/04-logic.js";
import { delayNot, ready, runDelayedNot } from "./ex_4_77.js";

describe("4.77", () =>
  it("waits for bindings then performs negation", () => {
    const e = makeQueryEngine();
    e.load("(blocked alice)");
    const x = readQuery("?x"),
      d = delayNot(readQuery("(blocked ?x)"), [x]);
    expect(ready(frameOf(), d)).toBe(false);
    expect(() => runDelayedNot(e, frameOf(), d)).toThrow("waits");
    expect(runDelayedNot(e, frameOf([[x, readQuery("bob")]]), d)).toBe(true);
    expect(runDelayedNot(e, frameOf([[x, readQuery("alice")]]), d)).toBe(false);
  }));
