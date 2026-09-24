// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.6

import { describe, expect, it } from "vitest";

import { err, ok, parsePort, renderError } from "./06-errors.js";

describe("errors as values", () => {
  it("digits with surrounding spaces parse to their value", () => {
    expect(parsePort(" 8080 ")).toEqual(ok(8080));
  });

  it("blank input is a Blank error value", () => {
    expect(parsePort("   ")).toEqual(err({ _tag: "Blank" }));
  });

  it("a stray character is a NotDigits error carrying the input", () => {
    expect(parsePort("80x0")).toEqual(err({ _tag: "NotDigits", input: "80x0" }));
  });

  it("a value outside the port range is an OutOfRange error", () => {
    expect(parsePort("99999")).toEqual(err({ _tag: "OutOfRange", n: 99999 }));
  });

  it("the caller dispatches on the error union, exhaustively", () => {
    const rendered = (s: string) => {
      const r = parsePort(s);
      return r._tag === "Ok" ? String(r.value) : renderError(r.error);
    };
    expect(rendered(" 8080 ")).toBe("8080");
    expect(rendered("   ")).toBe("blank input");
    expect(rendered(" 1x ")).toBe("not digits:  1x ");
    expect(rendered("99999")).toBe("out of range: 99999");
  });
});
