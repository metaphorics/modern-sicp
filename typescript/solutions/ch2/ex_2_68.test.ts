// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { list } from "../../packages/ch2/src/02-picture-language.js";
import { sym } from "../../packages/ch2/src/03-symbolic-data.js";
import { decodedSample, sampleMessage, sampleTree } from "./ex_2_67.js";
import { encode, encodeSymbol } from "./ex_2_68.js";

describe("exercise 2.68", () => {
  it("re-encodes the 2.67 result to the original sample message", () => {
    if (decodedSample._tag !== "Ok") {
      throw new Error("the sample decode must succeed");
    }
    expect(encode(decodedSample.value, sampleTree)).toStrictEqual({
      _tag: "Ok",
      value: sampleMessage,
    });
  });

  it("signals an error for a symbol outside the tree", () => {
    expect(encodeSymbol(sym("E"), sampleTree)._tag).toBe("Error");
    expect(encode(list(sym("A"), sym("E")), sampleTree)._tag).toBe("Error");
  });
});
