// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { list } from "../../packages/ch2/src/02-picture-language.js";
import { sym } from "../../packages/ch2/src/03-symbolic-data.js";
import { sampleTree } from "./ex_2_67.js";
import { generateHuffmanTree } from "./ex_2_69.js";

describe("exercise 2.69", () => {
  it("regenerates the book's sample tree from its pairs", () => {
    const pairs = list(
      [sym("A"), 4] as const,
      [sym("B"), 2] as const,
      [sym("C"), 1] as const,
      [sym("D"), 1] as const,
    );
    // The ordered-set merge with ties inserted after equal weights
    // rebuilds exactly the book's sample tree.
    expect(generateHuffmanTree(pairs)).toStrictEqual({ _tag: "Ok", value: sampleTree });
  });

  it("refuses an empty pair list", () => {
    expect(generateHuffmanTree(list())._tag).toBe("Error");
  });
});
