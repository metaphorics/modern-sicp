import { describe, expect, it } from "vitest";
import { errorSignals } from "./ex_5_30.js";

describe("exercise 5.30", () => {
  it("signals primitive failure", () =>
    expect(errorSignals().join(" ")).toContain("division by zero"));
});
