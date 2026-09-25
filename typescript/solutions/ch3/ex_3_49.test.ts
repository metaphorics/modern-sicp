// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import {
  clerkAuditorDeadlocks,
  clerkProgram,
  makeAuditSystem,
  twoPhaseScenarioCompletes,
} from "./ex_3_49.js";

describe("exercise 3.49: when lock ordering cannot help", () => {
  it("the clerk and the auditor deadlock despite every resource being lockable in a fixed order", async () => {
    const deadlocks = await Effect.runPromise(clerkAuditorDeadlocks());
    expect(deadlocks).toBe(true);
  });

  it("the two-phase auditor backs off and the same scenario completes", async () => {
    const completes = await Effect.runPromise(twoPhaseScenarioCompletes());
    expect(completes).toBe(true);
  });

  it("the clerk alone moves the money and releases both resources", async () => {
    const system = makeAuditSystem();
    await Effect.runPromise(clerkProgram(system));
    expect(system.balance.value).toBe(75);
  });
});
