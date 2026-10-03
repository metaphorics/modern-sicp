// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  defineInAList,
  frameVariables,
  lookupInAList,
  makeAListEnv,
  setInAList,
} from "./ex_4_11.js";

/** The observable result: the rendered value, or the fault category. */
const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? String(outcome.value) : `error:${outcome.error.tag}`;

describe("exercise 4.11: frames as association lists", () => {
  it("lookup scans the frame's entries and walks the parent chain", () => {
    const outer = makeAListEnv();
    defineInAList("a", 1, outer);
    const inner = makeAListEnv(outer);
    defineInAList("b", 2, inner);
    expect(shown(lookupInAList("b", inner))).toBe("2");
    expect(shown(lookupInAList("a", inner))).toBe("1");
    expect(shown(lookupInAList("zz", inner))).toBe("error:unbound-name");
  });

  it("a write lands in the shared entry of the frame that binds the name", () => {
    const outer = makeAListEnv();
    defineInAList("a", 1, outer);
    const inner = makeAListEnv(outer);
    defineInAList("b", 2, inner);
    expect(shown(setInAList("a", 10, inner))).toBe("10");
    expect(shown(lookupInAList("a", outer))).toBe("10");
  });

  it("define conses onto the current frame and shadows outer names", () => {
    const outer = makeAListEnv();
    defineInAList("a", 1, outer);
    const inner = makeAListEnv(outer);
    defineInAList("b", 2, inner);
    defineInAList("c", 3, inner);
    expect(frameVariables(inner)).toEqual(["c", "b"]);
    expect(frameVariables(outer)).toEqual(["a"]);
    expect(shown(lookupInAList("c", outer))).toBe("error:unbound-name");
  });

  it("setting an unbound name fails with unbound-name", () => {
    const outer = makeAListEnv();
    defineInAList("a", 1, outer);
    const inner = makeAListEnv(outer);
    expect(shown(setInAList("zz", 5, inner))).toBe("error:unbound-name");
  });
});
