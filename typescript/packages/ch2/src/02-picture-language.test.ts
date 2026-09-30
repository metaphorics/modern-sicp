// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.2

import { describe, expect, it } from "vitest";

import {
  accumulate,
  append,
  car,
  cdr,
  cons,
  cornerSplit,
  countLeaves,
  enumerateInterval,
  enumerateTree,
  evenFibs,
  evenFibsViaSequence,
  filter,
  flatmap,
  flippedPairs,
  flippedPairsViaSquareOfFour,
  flipVert,
  frameCoordMap,
  getOrElse,
  isNull,
  type List,
  leaf,
  length,
  list,
  listFibSquares,
  listRef,
  makePairSum,
  makeVect,
  map,
  nil,
  node,
  permutations,
  primeSumPairs,
  productOfSquaresOfOddElements,
  renderSvg,
  rightSplit,
  rogers,
  salaryOfHighestPaidProgrammer,
  scaleList,
  scaleTree,
  scaleTreeViaMap,
  showList,
  showTree,
  showVect,
  squareLimit,
  squareLimitViaSquareOfFour,
  sumOddSquares,
  sumOddSquaresViaSequence,
  type Tree,
  transformPainter,
  unitSquare,
  wave,
} from "./02-picture-language.js";

describe("section 2.2: hierarchical data and the closure property", () => {
  it("the nested conses build the chain of figure 2.4", () => {
    expect(showList(cons(1, cons(2, cons(3, cons(4, nil)))))).toBe("[1, 2, 3, 4]");
  });

  it("the selectors read the list like the book's interactions", () => {
    const oneThroughFour = list(1, 2, 3, 4);
    expect(showList(oneThroughFour)).toBe("[1, 2, 3, 4]");
    expect(car(oneThroughFour)).toStrictEqual({ _tag: "Some", value: 1 });
    expect(car(getOrElse(cdr(oneThroughFour), nil))).toStrictEqual({ _tag: "Some", value: 2 });
    expect(showList(getOrElse(cdr(oneThroughFour), nil))).toBe("[2, 3, 4]");
    expect(showList(cons(10, oneThroughFour))).toBe("[10, 1, 2, 3, 4]");
    expect(showList(cons(5, oneThroughFour))).toBe("[5, 1, 2, 3, 4]");
    expect(isNull(nil)).toBe(true);
    expect(isNull(oneThroughFour)).toBe(false);
    expect(car(nil)).toStrictEqual({ _tag: "None" });
  });

  it("list-ref cdrs down to the n-th item", () => {
    const squares = list(1, 4, 9, 16, 25);
    expect(listRef(squares, 3)).toStrictEqual({ _tag: "Some", value: 16 });
    expect(listRef(squares, 9)).toStrictEqual({ _tag: "None" });
  });

  it("length counts and append conses up a new spine", () => {
    const squares = list(1, 4, 9, 16, 25);
    const odds = list(1, 3, 5, 7);
    expect(length(odds)).toBe(4);
    expect(showList(append(squares, odds))).toBe("[1, 4, 9, 16, 25, 1, 3, 5, 7]");
    expect(showList(append(odds, squares))).toBe("[1, 3, 5, 7, 1, 4, 9, 16, 25]");
  });

  it("scale-list and map transform a list to a list", () => {
    expect(showList(scaleList(list(1, 2, 3, 4, 5), 10))).toBe("[10, 20, 30, 40, 50]");
    expect(showList(map(Math.abs, list(-10, 2.5, -11.6, 17)))).toBe("[10, 2.5, 11.6, 17]");
    expect(showList(map((x) => x * x, list(1, 2, 3, 4)))).toBe("[1, 4, 9, 16]");
  });

  it("trees count leaves and rescale leaf by leaf", () => {
    const x: Tree<number> = node(node(leaf(1), leaf(2)), leaf(3), leaf(4));
    expect(showTree(x)).toBe("[[1, 2], 3, 4]");
    expect(countLeaves(x)).toBe(4);
    const xx = node(x, x);
    expect(showTree(xx)).toBe("[[[1, 2], 3, 4], [[1, 2], 3, 4]]");
    expect(countLeaves(xx)).toBe(8);

    const t = node(leaf(1), node(leaf(2), node(leaf(3), leaf(4)), leaf(5)), node(leaf(6), leaf(7)));
    expect(showTree(scaleTree(t, 10))).toBe("[10, [20, [30, 40], 50], [60, 70]]");
    expect(showTree(scaleTreeViaMap(t, 10))).toBe("[10, [20, [30, 40], 50], [60, 70]]");
  });

  it("the raw spellings of sum-odd-squares and even-fibs agree with the flow plans", () => {
    const t = node(leaf(1), node(leaf(2), node(leaf(3), leaf(4)), leaf(5)), node(leaf(6), leaf(7)));
    expect(sumOddSquares(t)).toBe(84);
    expect(sumOddSquaresViaSequence(t)).toBe(84);
    expect(showList(evenFibs(10))).toBe("[0, 2, 8, 34]");
    expect(showList(evenFibsViaSequence(10))).toBe("[0, 2, 8, 34]");
  });

  it("the sequence operations reproduce the book's interactions", () => {
    expect(showList(map((x) => x * x, list(1, 2, 3, 4, 5)))).toBe("[1, 4, 9, 16, 25]");
    expect(showList(filter((n: number) => n % 2 === 1, list(1, 2, 3, 4, 5)))).toBe("[1, 3, 5]");
    expect(accumulate((a: number, b: number) => a + b, 0, list(1, 2, 3, 4, 5))).toBe(15);
    expect(accumulate((a: number, b: number) => a * b, 1, list(1, 2, 3, 4, 5))).toBe(120);
    expect(
      showList(accumulate((x: number, y: List<number>) => cons(x, y), nil, list(1, 2, 3, 4, 5))),
    ).toBe("[1, 2, 3, 4, 5]");
    expect(showList(enumerateInterval(2, 7))).toBe("[2, 3, 4, 5, 6, 7]");
    const t = node(leaf(1), node(leaf(2), node(leaf(3), leaf(4)), leaf(5)), leaf(6));
    expect(showList(enumerateTree(t))).toBe("[1, 2, 3, 4, 5, 6]");
  });

  it("the rearranged sequence programs print the book's values", () => {
    expect(showList(listFibSquares(10))).toBe("[0, 1, 1, 4, 9, 25, 64, 169, 441, 1156, 3025]");
    expect(productOfSquaresOfOddElements(list(1, 2, 3, 4, 5))).toBe(225);
    const records = list(
      { name: "Bitdiddle", salary: 60000, isProgrammer: true },
      { name: "Scrooge", salary: 250000, isProgrammer: false },
      { name: "Hacker", salary: 80000, isProgrammer: true },
    );
    expect(salaryOfHighestPaidProgrammer(records)).toBe(80000);
  });

  it("the nested mappings generate pairs and permutations", () => {
    expect(showList(map((p) => showList(p), primeSumPairs(6)))).toBe(
      '["[2, 1, 3]", "[3, 2, 5]", "[4, 1, 5]", "[4, 3, 7]", "[5, 2, 7]", "[6, 1, 7]", "[6, 5, 11]"]',
    );
    expect(showList(map((p) => showList(p), permutations(list(1, 2, 3))))).toBe(
      '["[1, 2, 3]", "[1, 3, 2]", "[2, 1, 3]", "[2, 3, 1]", "[3, 1, 2]", "[3, 2, 1]"]',
    );
    const p = list(3, 4);
    expect(showList(makePairSum(p))).toBe("[3, 4, 7]");
  });

  it("the frame coordinate map places the unit square's corners", () => {
    const frame = { origin: makeVect(1, 1), edge1: makeVect(2, 0), edge2: makeVect(0, 2) };
    const m = frameCoordMap(frame);
    expect(showVect(m(makeVect(0, 0)))).toBe("(1, 1)");
    expect(showVect(m(makeVect(1, 1)))).toBe("(3, 3)");
    expect(showVect(m(makeVect(0.5, 0.5)))).toBe("(2, 2)");
    expect(showVect(m(makeVect(0, 0)))).toBe(showVect(frame.origin));
  });

  it("the wave painter carries its segment list through the frame map", () => {
    const p = wave();
    const segments = p(unitSquare);
    expect(length(segments)).toBe(18);
    const flipped = flipVert(p);
    expect(length(flipped(unitSquare))).toBe(18);
    const shrunk = transformPainter(p, makeVect(0.5, 0.5), makeVect(1, 0.5), makeVect(0.5, 1));
    expect(length(shrunk(unitSquare))).toBe(18);
    const corner = cornerSplit(p, 4);
    expect(length(corner(unitSquare))).toBe(1962);
  });

  it("the combinations stay closed and agree between spellings", () => {
    const p = wave();
    expect(length(rightSplit(p, 1)(unitSquare))).toBe(54);
    expect(length(flippedPairs(p)(unitSquare))).toBe(72);
    expect(length(flippedPairsViaSquareOfFour(p)(unitSquare))).toBe(72);
    expect(showList(enumerateInterval(1, 0))).toBe("[]");
    const a = squareLimit(p, 2)(unitSquare);
    const b = squareLimitViaSquareOfFour(p, 2)(unitSquare);
    expect(showList(a)).toBe(showList(b));
    expect(length(a)).toBe(1368);
    const r = rogers();
    expect(length(r(unitSquare))).toBe(17);
  });

  it("the svg rendering is deterministic with a fixed attribute order", () => {
    const svg = renderSvg(wave(), 200);
    expect(
      svg.startsWith(
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="200" height="200">',
      ),
    ).toBe(true);
    expect(svg).toContain('<rect width="200" height="200" fill="white"/>');
    expect(svg).toContain(
      '<g stroke="#1a1a1a" stroke-width="1.2" stroke-linecap="round" fill="none">',
    );
    expect(svg).toContain('<line x1="0.00" y1="30.00" x2="24.00" y2="76.00"/>');
    expect(svg.endsWith("</g>\n</svg>\n")).toBe(true);
    expect(renderSvg(wave(), 200)).toBe(svg);
  });

  it("flatmap builds the pair sequence the prime-sum program filters", () => {
    const pairs = flatmap(
      (i: number) => map((j: number) => list(i, j), enumerateInterval(1, i - 1)),
      enumerateInterval(1, 4),
    );
    expect(showList(map((p) => showList(p), pairs))).toBe(
      '["[2, 1]", "[3, 1]", "[3, 2]", "[4, 1]", "[4, 2]", "[4, 3]"]',
    );
  });
});
