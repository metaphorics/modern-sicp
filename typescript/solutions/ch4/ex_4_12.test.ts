// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { child, type Env, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  defineDirect,
  forEachBinding,
  lookupTraversing,
  setTraversing,
  walkEnvChain,
} from "./ex_4_12.js";

/** The observable result: the rendered value, or the fault category. */
const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? String(outcome.value) : `error:${outcome.error.tag}`;

/** A three-frame chain: global a = 1, middle b = 2, inner c = 3. */
const chain = (): { global: Env; middle: Env; inner: Env } => {
  const global: Env = child(null);
  global.bindings.set("a", makeCell(1, true));
  const middle = child(global);
  middle.bindings.set("b", makeCell(2, true));
  const inner = child(middle);
  inner.bindings.set("c", makeCell(3, true));
  return { global, middle, inner };
};

describe("exercise 4.12: abstract environment traversals", () => {
  it("the traversing lookup walks inner, middle, then global", () => {
    const { inner } = chain();
    expect(shown(lookupTraversing("c", inner))).toBe("3");
    expect(shown(lookupTraversing("b", inner))).toBe("2");
    expect(shown(lookupTraversing("a", inner))).toBe("1");
    expect(shown(lookupTraversing("zz", inner))).toBe("error:unbound-name");
  });

  it("a traversing set writes the frame that binds the name", () => {
    const { middle, inner } = chain();
    expect(shown(setTraversing("b", 20, inner))).toBe("20");
    expect(shown(lookupTraversing("b", inner))).toBe("20");
    expect(shown(lookupTraversing("b", middle))).toBe("20");
  });

  it("define writes the current frame only", () => {
    const { global, middle, inner } = chain();
    defineDirect("d", 4, inner);
    expect(shown(lookupTraversing("d", inner))).toBe("4");
    expect(shown(lookupTraversing("d", middle))).toBe("error:unbound-name");
    expect(shown(lookupTraversing("a", inner))).toBe("1");
    expect(shown(lookupTraversing("a", global))).toBe("1");
  });

  it("forEachBinding visits one frame only", () => {
    const { inner } = chain();
    const visited: string[] = [];
    forEachBinding(inner, (binding) => {
      visited.push(binding.name);
    });
    expect(visited).toEqual(["c"]);
  });

  it("walkEnvChain stops at the first probe answer", () => {
    const { middle, inner } = chain();
    const frames: string[] = [];
    const answer = walkEnvChain(inner, (frame: Env) => {
      frames.push(frame.bindings.has("b") ? "b-frame" : "other");
      return frame.bindings.get("b") === undefined ? undefined : "found";
    });
    expect(answer).toBe("found");
    expect(frames).toEqual(["other", "b-frame"]);
  });
});
