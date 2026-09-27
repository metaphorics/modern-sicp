// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

import { describe, expect, it } from "vitest";

import { format, ReadError, read, readAll } from "./read.js";

describe("section 4.1: the reader and printer", () => {
  it("reads atoms into the value kinds", () => {
    expect(read("42")).toStrictEqual({ _tag: "Number", n: 42 });
    expect(read("-3.5")).toStrictEqual({ _tag: "Number", n: -3.5 });
    expect(read("#t")).toStrictEqual({ _tag: "Boolean", b: true });
    expect(read("#f")).toStrictEqual({ _tag: "Boolean", b: false });
    expect(read("lambda")).toStrictEqual({ _tag: "Symbol", name: "lambda" });
    expect(read('"hi there"')).toStrictEqual({ _tag: "String", s: "hi there" });
  });

  it("reads lists, nesting, and quote sugar", () => {
    expect(format(read("(a b c)"))).toBe("(a b c)");
    expect(format(read("(if (< n 10) 'small 'big)"))).toBe(
      "(if (< n 10) (quote small) (quote big))",
    );
    expect(format(read("'x"))).toBe("(quote x)");
    expect(format(read("''x"))).toBe("(quote (quote x))");
    expect(format(read("(1 2 . (3 4))"))).toBe("(1 2 3 4)");
    expect(format(read("()"))).toBe("()");
    expect(format(read("(define (f x) (* x x))"))).toBe("(define (f x) (* x x))");
  });

  it("skips comments and whitespace between forms", () => {
    expect(format(read("; a comment\n(+ 1 2) ; trailing\n"))).toBe("(+ 1 2)");
    const forms = readAll("(define x 1)\n(set! x 2)\nx");
    expect(forms.length).toBe(3);
    const last = forms[2];
    if (last !== undefined) {
      expect(format(last)).toBe("x");
    } else {
      expect.unreachable("readAll must return every form");
    }
  });

  it("prints the book's display forms", () => {
    expect(format(read("5"))).toBe("5");
    expect(format(read("(1 (2 3) 4)"))).toBe("(1 (2 3) 4)");
    expect(format(read('"str"'))).toBe("str");
  });

  it("rejects malformed input with a ReadError", () => {
    expect(() => read("(a b")).toThrow(ReadError);
    expect(() => read("a)")).toThrow(ReadError);
    expect(() => read("(a) (b)")).toThrow(ReadError);
    expect(() => read('"open')).toThrow(ReadError);
  });
});
