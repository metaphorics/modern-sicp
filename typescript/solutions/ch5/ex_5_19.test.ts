// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_19, makeBreakpointMachine } from "./ex_5_19.ts";

describe("exercise 5.19 breakpoints", () => {
  it("parks before each n-th execution, proceed resumes, cancel releases", () => {
    const lines = ex_5_19();
    expect(lines[0]).toBe("break at test-b: a = 40, b = 6");
    expect(lines[1]).toBe("break at test-b: a = 4, b = 2");
    expect(lines[lines.length - 2]).toContain("finished: gcd(206, 40) = 2");
    expect(lines[lines.length - 1]).toContain("cancel and restart: gcd(206, 40) = 2");
  });
  it("with n = 1 the machine parks at every arrival until the branch", () => {
    const session = makeBreakpointMachine();
    session.machine.writeRegister("a", 206);
    session.machine.writeRegister("b", 40);
    session.setBreakpoint("test-b", 1);
    expect(session.start()).toContain("break at test-b:");
    expect(session.parkedAt()).toBe("test-b");
  });
});
