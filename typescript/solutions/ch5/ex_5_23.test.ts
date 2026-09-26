import { describe, expect, it } from "vitest";
import { derivedExpressions } from "./ex_5_23.js";

describe("exercise 5.23", () => {
  it("runs through the evaluator controller", () =>
    expect(derivedExpressions().some((x) => x === "6")).toBe(true));
});
