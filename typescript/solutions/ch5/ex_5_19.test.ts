// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { getRegisterContents, setRegisterContents } from "../../packages/ch5/src/02-simulator.js";
import { expectOk } from "./ex_5_07.js";
import { breakpointSession, makeBreakpointMachine } from "./ex_5_19.js";

describe("exercise 5.19 breakpoints", () => {
  it("parks before each n-th execution, proceed resumes, cancel releases", () => {
    expect(breakpointSession()).toEqual([
      "break at test-b: a = 40, b = 6",
      "break at test-b: a = 4, b = 2",
      "finished: gcd(206, 40) = 2",
      "cancel and restart: gcd(206, 40) = 2",
    ]);
  });
  it("parks at every arrival for n = 1 and resumes the very run", () => {
    const { machine, setBreakpoint } = makeBreakpointMachine(["a", "b", "t"]);
    expectOk(setRegisterContents(machine, "a", 206));
    expectOk(setRegisterContents(machine, "b", 40));
    setBreakpoint("test-b", 1);
    expectOk(machine.start());
    expect(machine.parkedAt).toBe("test-b");
    for (let stop = 0; stop < 4; stop += 1) {
      expectOk(machine.proceed());
      expect(machine.parkedAt).toBe("test-b");
    }
    expectOk(machine.proceed());
    expect(machine.parkedAt).toBe(null);
    expect(expectOk(getRegisterContents(machine, "a"))).toBe(2);
  });
  it("refuses a breakpoint on an undefined label", () => {
    const { machine, setBreakpoint, cancelAllBreakpoints } = makeBreakpointMachine(["a", "b", "t"]);
    expect(() => setBreakpoint("no-such-label", 1)).toThrow("no such label: no-such-label");
    cancelAllBreakpoints();
    expect(machine.parkedAt).toBe(null);
  });
});
