// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { readFileSync } from "node:fs";

import { describe, expect, it } from "vitest";

import {
  below,
  beside,
  cornerSplit,
  flipVert,
  list,
  makeSegment,
  makeVect,
  type Painter,
  renderSvg,
  rotate90,
  segmentsToPainter,
  squareLimit,
  transformPainter,
  unitSquare,
  wave,
} from "../../packages/ch2/src/02-picture-language.js";

import { cornerSplitVariant, squareLimitVariant, waveWithSmile } from "./ex_2_52.js";

const seg = (x1: number, y1: number, x2: number, y2: number) =>
  makeSegment(makeVect(x1, y1), makeVect(x2, y2));

const outline = (): Painter =>
  segmentsToPainter(list(seg(0, 0, 1, 0), seg(1, 0, 1, 1), seg(1, 1, 0, 1), seg(0, 1, 0, 0)));

const cross = (): Painter => segmentsToPainter(list(seg(0, 0, 1, 1), seg(0, 1, 1, 0)));

const diamond = (): Painter =>
  segmentsToPainter(
    list(seg(0.5, 0, 1, 0.5), seg(1, 0.5, 0.5, 1), seg(0.5, 1, 0, 0.5), seg(0, 0.5, 0.5, 0)),
  );

const rotate270 = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(0, 1), makeVect(0, 0), makeVect(1, 1));

const belowRotated = (painter1: Painter, painter2: Painter): Painter =>
  rotate90(beside(rotate270(painter1), rotate270(painter2)));

const figure = (name: string): string =>
  readFileSync(
    new URL(`../../book/figures/generated/chap2/${name}.std.svg`, import.meta.url),
    "utf8",
  );

describe("exercise 2.52", () => {
  it("the smile adds two segments at the primitive level", () => {
    const segments = waveWithSmile()(unitSquare);
    expect(segments._tag === "Cons").toBe(true);
    let count = 0;
    for (let rest = segments; rest._tag === "Cons"; rest = rest.tail) {
      count += 1;
    }
    expect(count).toBe(20);
  });

  it("the corner-split variant uses single up and right copies", () => {
    const plain = cornerSplit(wave(), 4)(unitSquare);
    const variant = cornerSplitVariant(wave(), 4)(unitSquare);
    expect(variant._tag === "Cons").toBe(true);
    expect(variant).not.toStrictEqual(plain);
  });

  it("the square-limit variant reorders the corners", () => {
    const plain = squareLimit(wave(), 3)(unitSquare);
    const variant = squareLimitVariant(wave(), 3)(unitSquare);
    expect(variant).not.toStrictEqual(plain);
  });

  it("every checked-in figure is byte-identical to a fresh render", () => {
    const p = wave();
    const wave2 = beside(p, flipVert(p));
    const wave4 = below(wave2, wave2);
    const low = segmentsToPainter(list(seg(0.1, 0.2, 0.4, 0.3)));
    const high = segmentsToPainter(list(seg(0.1, 0.7, 0.4, 0.8)));
    const fresh: ReadonlyArray<readonly [string, string]> = [
      ["wave", renderSvg(p, 200)],
      ["wave4", renderSvg(wave4, 200)],
      ["outline", renderSvg(outline(), 200)],
      ["cross", renderSvg(cross(), 200)],
      ["diamond", renderSvg(diamond(), 200)],
      ["wave_smile", renderSvg(waveWithSmile(), 200)],
      ["below_direct", renderSvg(below(low, high), 200)],
      ["below_rotate", renderSvg(belowRotated(low, high), 200)],
      ["corner_split", renderSvg(cornerSplit(p, 4), 200)],
      ["corner_split_variant", renderSvg(cornerSplitVariant(p, 4), 200)],
      ["square_limit", renderSvg(squareLimit(p, 4), 200)],
      ["square_limit_variant", renderSvg(squareLimitVariant(p, 4), 200)],
    ];
    expect(fresh).toHaveLength(12);
    for (const [name, svg] of fresh) {
      expect(svg).toBe(figure(name));
    }
  });
});
