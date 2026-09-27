// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

import { describe, expect, it } from "vitest";

import { ArityMismatch, NotAProcedure, UnboundVariable, UnknownSyntax } from "./errors.js";

describe("EvaluationError classes", () => {
  it("carries the tag and fields of each variant", () => {
    expect(new UnboundVariable({ name: "x" })._tag).toBe("UnboundVariable");
    expect(new NotAProcedure({ value: "5" })._tag).toBe("NotAProcedure");
    expect(new UnknownSyntax({ expr: "(if)" })._tag).toBe("UnknownSyntax");
    const arity = new ArityMismatch({ expected: 2, given: 3 });
    expect(arity._tag).toBe("ArityMismatch");
    expect(arity.expected).toBe(2);
    expect(arity.given).toBe(3);
  });

  it("instances satisfy the EvaluationError union by tag", () => {
    const error: UnboundVariable = new UnboundVariable({ name: "sqrt" });
    expect(error.name).toBe("sqrt");
  });
});
