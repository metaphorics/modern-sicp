// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import type { Connector } from "../../packages/ch3/src/03-mutable-data.js";

import {
  makeTracedConnector,
  type TraceEvent,
  tracedAdder,
  tracedConstant,
  tracedMultiplier,
} from "./ex_3_36.js";

/** The module's celsius topology wired from traced parts, in the
 * module's wiring order: multiplier(c, w, u), multiplier(v, x, u),
 * adder(v, y, f), then the three constants. */
const buildTracedCelsiusNetwork = (log: TraceEvent[]): { c: Connector; f: Connector } => {
  const c = makeTracedConnector("C", log);
  const f = makeTracedConnector("F", log);
  const u = makeTracedConnector("u", log);
  const v = makeTracedConnector("v", log);
  const w = makeTracedConnector("w", log);
  const x = makeTracedConnector("x", log);
  const y = makeTracedConnector("y", log);
  tracedMultiplier("mul-cwu", c, w, u, log);
  tracedMultiplier("mul-vxu", v, x, u, log);
  tracedAdder("add-vyf", v, y, f, log);
  tracedConstant("const-9", 9, w, log);
  tracedConstant("const-5", 5, x, log);
  tracedConstant("const-32", 32, y, log);
  return { c, f };
};

describe("exercise 3.36: the connector's environment, as a trace", () => {
  it("set-value! on C traces the for-each-except arrows the book drew", () => {
    const log: TraceEvent[] = [];
    const { c, f } = buildTracedCelsiusNetwork(log);
    expect(log.slice(0, 6)).toEqual([
      { from: "const-9", to: "w", what: "setValue" },
      { from: "w", to: "mul-cwu", what: "informAboutValue" },
      { from: "const-5", to: "x", what: "setValue" },
      { from: "x", to: "mul-vxu", what: "informAboutValue" },
      { from: "const-32", to: "y", what: "setValue" },
      { from: "y", to: "add-vyf", what: "informAboutValue" },
    ]);
    c.setValue(25, "user");
    expect(f.getValue()).toBe(77);
    expect(log).toEqual([
      { from: "const-9", to: "w", what: "setValue" },
      { from: "w", to: "mul-cwu", what: "informAboutValue" },
      { from: "const-5", to: "x", what: "setValue" },
      { from: "x", to: "mul-vxu", what: "informAboutValue" },
      { from: "const-32", to: "y", what: "setValue" },
      { from: "y", to: "add-vyf", what: "informAboutValue" },
      { from: "user", to: "C", what: "setValue" },
      { from: "C", to: "mul-cwu", what: "informAboutValue" },
      { from: "mul-cwu", to: "u", what: "setValue" },
      { from: "u", to: "mul-vxu", what: "informAboutValue" },
      { from: "mul-vxu", to: "v", what: "setValue" },
      { from: "v", to: "add-vyf", what: "informAboutValue" },
      { from: "add-vyf", to: "F", what: "setValue" },
    ]);
  });

  it("forget-value! ripples the lost notices down the spine, draining it", () => {
    const log: TraceEvent[] = [];
    const { c, f } = buildTracedCelsiusNetwork(log);
    c.setValue(25, "user");
    expect(f.getValue()).toBe(77);
    const settled = log.length;
    c.forgetValue("user");
    expect(log.slice(settled)).toEqual([
      { from: "C", to: "mul-cwu", what: "informAboutNoValue" },
      { from: "u", to: "mul-vxu", what: "informAboutNoValue" },
      { from: "v", to: "add-vyf", what: "informAboutNoValue" },
    ]);
    expect(c.hasValue()).toBe(false);
    expect(f.hasValue()).toBe(false);
  });

  it("setting C again walks the value arrows down the rebuilt spine", () => {
    const log: TraceEvent[] = [];
    const { c, f } = buildTracedCelsiusNetwork(log);
    c.setValue(25, "user");
    expect(f.getValue()).toBe(77);
    c.forgetValue("user");
    const drained = log.length;
    c.setValue(25, "user");
    expect(f.getValue()).toBe(77);
    expect(log.slice(drained)).toEqual([
      { from: "user", to: "C", what: "setValue" },
      { from: "C", to: "mul-cwu", what: "informAboutValue" },
      { from: "mul-cwu", to: "u", what: "setValue" },
      { from: "u", to: "mul-vxu", what: "informAboutValue" },
      { from: "mul-vxu", to: "v", what: "setValue" },
      { from: "v", to: "add-vyf", what: "informAboutValue" },
      { from: "add-vyf", to: "F", what: "setValue" },
    ]);
  });
});
