// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { expect, test } from "vitest";

import { InvalidSeedError, Random } from "./random.js";

test("random(1000) from seed 1 yields the edition vector", () => {
  const random = new Random(1n);
  const draws = Array.from({ length: 5 }, () => random.random(1000));
  expect(draws).toStrictEqual([165, 517, 103, 413, 928]);
});

test("Random masks a seed outside 64 bits to its low 64 bits", () => {
  const masked = new Random((1n << 64n) + 1n);
  const plain = new Random(1n);
  expect(masked.random(1000)).toBe(plain.random(1000));
});

test("Random rejects a zero seed with the typed error", () => {
  expect(() => new Random(0n)).toThrow(InvalidSeedError);
});
