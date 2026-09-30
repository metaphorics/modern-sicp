// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import type { Value } from "../../packages/ch4/src/runtime/value.js";
import { isArrayValue } from "../../packages/ch4/src/runtime/value.js";
import { parses } from "./ex_4_45.js";

/** The tree shape as plain nested data, for structural comparison. */
const shape = (value: Value): unknown => (isArrayValue(value) ? value.items.map(shape) : value);

describe("exercise 4.45: five parses", () => {
  it("answers exactly five parses in the pinned order, then exhaustion", () => {
    expect(parses().map(shape)).toEqual([
      [
        "sentence",
        ["simple-noun-phrase", ["article", "the"], ["noun", "professor"]],
        [
          "verb-phrase",
          [
            "verb-phrase",
            [
              "verb-phrase",
              ["verb", "lectures"],
              [
                "prep-phrase",
                ["prep", "to"],
                ["simple-noun-phrase", ["article", "the"], ["noun", "student"]],
              ],
            ],
            [
              "prep-phrase",
              ["prep", "in"],
              ["simple-noun-phrase", ["article", "the"], ["noun", "class"]],
            ],
          ],
          [
            "prep-phrase",
            ["prep", "with"],
            ["simple-noun-phrase", ["article", "the"], ["noun", "cat"]],
          ],
        ],
      ],
      [
        "sentence",
        ["simple-noun-phrase", ["article", "the"], ["noun", "professor"]],
        [
          "verb-phrase",
          [
            "verb-phrase",
            ["verb", "lectures"],
            [
              "prep-phrase",
              ["prep", "to"],
              ["simple-noun-phrase", ["article", "the"], ["noun", "student"]],
            ],
          ],
          [
            "prep-phrase",
            ["prep", "in"],
            [
              "noun-phrase",
              ["simple-noun-phrase", ["article", "the"], ["noun", "class"]],
              [
                "prep-phrase",
                ["prep", "with"],
                ["simple-noun-phrase", ["article", "the"], ["noun", "cat"]],
              ],
            ],
          ],
        ],
      ],
      [
        "sentence",
        ["simple-noun-phrase", ["article", "the"], ["noun", "professor"]],
        [
          "verb-phrase",
          [
            "verb-phrase",
            ["verb", "lectures"],
            [
              "prep-phrase",
              ["prep", "to"],
              [
                "noun-phrase",
                ["simple-noun-phrase", ["article", "the"], ["noun", "student"]],
                [
                  "prep-phrase",
                  ["prep", "in"],
                  ["simple-noun-phrase", ["article", "the"], ["noun", "class"]],
                ],
              ],
            ],
          ],
          [
            "prep-phrase",
            ["prep", "with"],
            ["simple-noun-phrase", ["article", "the"], ["noun", "cat"]],
          ],
        ],
      ],
      [
        "sentence",
        ["simple-noun-phrase", ["article", "the"], ["noun", "professor"]],
        [
          "verb-phrase",
          ["verb", "lectures"],
          [
            "prep-phrase",
            ["prep", "to"],
            [
              "noun-phrase",
              [
                "noun-phrase",
                ["simple-noun-phrase", ["article", "the"], ["noun", "student"]],
                [
                  "prep-phrase",
                  ["prep", "in"],
                  ["simple-noun-phrase", ["article", "the"], ["noun", "class"]],
                ],
              ],
              [
                "prep-phrase",
                ["prep", "with"],
                ["simple-noun-phrase", ["article", "the"], ["noun", "cat"]],
              ],
            ],
          ],
        ],
      ],
      [
        "sentence",
        ["simple-noun-phrase", ["article", "the"], ["noun", "professor"]],
        [
          "verb-phrase",
          ["verb", "lectures"],
          [
            "prep-phrase",
            ["prep", "to"],
            [
              "noun-phrase",
              ["simple-noun-phrase", ["article", "the"], ["noun", "student"]],
              [
                "prep-phrase",
                ["prep", "in"],
                [
                  "noun-phrase",
                  ["simple-noun-phrase", ["article", "the"], ["noun", "class"]],
                  [
                    "prep-phrase",
                    ["prep", "with"],
                    ["simple-noun-phrase", ["article", "the"], ["noun", "cat"]],
                  ],
                ],
              ],
            ],
          ],
        ],
      ],
    ]);
  });
});
