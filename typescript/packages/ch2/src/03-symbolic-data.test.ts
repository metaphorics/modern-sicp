// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.3

import { describe, expect, it } from "vitest";

import { err } from "./01-data-abstraction.js";
import {
  car,
  cdr,
  cons,
  getOrElse,
  type List,
  list,
  nil,
  showList,
} from "./02-picture-language.js";
import {
  adjoinSetTree,
  adjoinSetUnordered,
  adjoinSetWeighted,
  chooseBranch,
  constant,
  type Datum,
  decode,
  deriv,
  derivVia,
  elementOfSetOrdered,
  elementOfSetTree,
  elementOfSetUnordered,
  emptyTreeSet,
  equalsNumber,
  intersectionSetOrdered,
  intersectionSetUnordered,
  lookupUnordered,
  makeCodeTree,
  makeLeaf,
  makeLeafSet,
  makeProduct,
  makeSum,
  makeTreeSet,
  memq,
  naiveConstructors,
  qlist,
  qnum,
  qsym,
  showDataList,
  showDatum,
  showExpr,
  showSymbols,
  sym,
  variable,
  weightOf,
} from "./03-symbolic-data.js";

const abcd = list(sym("a"), sym("b"), sym("c"), sym("d"));
const nameAges = qlist(
  qlist(qsym("Norah"), qnum(12)),
  qlist(qsym("Molly"), qnum(9)),
  qlist(qsym("Anna"), qnum(7)),
  qlist(qsym("Lauren"), qnum(6)),
  qlist(qsym("Charlotte"), qnum(4)),
);
const abc = list(qsym("a"), qsym("b"), qsym("c"));

describe("section 2.3: symbolic data", () => {
  it("2.3.1 builds lists of symbols and prints them like the book", () => {
    expect(showSymbols(abcd)).toBe("(a b c d)");
    expect(showList(list(23, 45, 17))).toBe("(23 45 17)");
    expect(showDatum(nameAges)).toBe("((Norah 12) (Molly 9) (Anna 7) (Lauren 6) (Charlotte 4))");
    expect(
      showExpr(
        naiveConstructors.makeProduct(
          naiveConstructors.makeSum(constant(23), constant(45)),
          naiveConstructors.makeSum(variable("x"), constant(9)),
        ),
      ),
    ).toBe("(* (+ 23 45) (+ x 9))");
  });

  it("2.3.1 distinguishes symbols from the values of names", () => {
    const a = 1;
    const b = 2;
    expect(showList(list(a, b))).toBe("(1 2)");
    expect(showSymbols(list(sym("a"), sym("b")))).toBe("(a b)");
    expect(showDatum(qlist(qsym("a"), qnum(b)))).toBe("(a 2)");
  });

  it("2.3.1 takes quoted lists apart with the list primitives", () => {
    expect(showDatum(getOrElse(car(abc), qsym("")))).toBe("a");
    expect(showDataList(getOrElse(cdr(abc), nil))).toBe("(b c)");
  });

  it("2.3.1 memq returns the sublist from the first eq? hit", () => {
    const noApple = list(qsym("pear"), qsym("banana"), qsym("prune"));
    expect(memq(qsym("apple"), noApple)).toStrictEqual({ _tag: "None" });
    const withApple = list(
      qsym("x"),
      qlist(qsym("apple"), qsym("sauce")),
      qsym("y"),
      qsym("apple"),
      qsym("pear"),
    );
    const hit = memq(qsym("apple"), withApple);
    expect(hit).toStrictEqual({
      _tag: "Some",
      value: list(qsym("apple"), qsym("pear")),
    });
    expect(showDataList(getOrElse(hit, nil))).toBe("(apple pear)");
  });

  it("2.3.2 prints the unsimplified derivatives of the book's three examples", () => {
    const x = variable("x");
    const y = variable("y");
    const sumX3 = naiveConstructors.makeSum(x, constant(3));
    const prodXY = naiveConstructors.makeProduct(x, y);
    const third = naiveConstructors.makeProduct(prodXY, sumX3);
    const n = naiveConstructors;
    expect(showExpr(derivVia(sumX3, sym("x"), n))).toBe("(+ 1 0)");
    expect(showExpr(derivVia(prodXY, sym("x"), n))).toBe("(+ (* x 0) (* 1 y))");
    expect(showExpr(derivVia(third, sym("x"), n))).toBe(
      "(+ (* (* x y) (+ 1 0)) (* (+ (* x 0) (* 1 y)) (+ x 3)))",
    );
  });

  it("2.3.2 prints the simplified derivatives of the same examples", () => {
    const x = variable("x");
    const y = variable("y");
    const sumX3 = makeSum(x, constant(3));
    const prodXY = makeProduct(x, y);
    const third = makeProduct(prodXY, sumX3);
    expect(showExpr(deriv(sumX3, sym("x")))).toBe("1");
    expect(showExpr(deriv(prodXY, sym("x")))).toBe("y");
    expect(showExpr(deriv(third, sym("x")))).toBe("(+ (* x y) (* y (+ x 3)))");
  });

  it("2.3.2 the simplifying constructors fold constants and absorb 0 and 1", () => {
    expect(showExpr(makeSum(constant(2), constant(3)))).toBe("5");
    expect(showExpr(makeSum(constant(0), variable("y")))).toBe("y");
    expect(showExpr(makeProduct(constant(1), variable("y")))).toBe("y");
    expect(showExpr(makeProduct(constant(0), variable("y")))).toBe("0");
    expect(equalsNumber(constant(0), 0)).toBe(true);
    expect(equalsNumber(variable("x"), 0)).toBe(false);
  });

  it("2.3.3 the unordered representation scans and conses", () => {
    expect(elementOfSetUnordered(6, list(1, 3, 6, 10))).toBe(true);
    expect(elementOfSetUnordered(7, list(1, 3, 6, 10))).toBe(false);
    expect(showList(adjoinSetUnordered(5, list(3, 7)))).toBe("(5 3 7)");
    expect(showList(adjoinSetUnordered(3, list(3, 7)))).toBe("(3 7)");
    expect(showList(intersectionSetUnordered(list(1, 3, 6, 10), list(3, 6, 7)))).toBe("(3 6)");
  });

  it("2.3.3 the ordered representation stops early and intersects in one pass", () => {
    expect(elementOfSetOrdered(6, list(1, 3, 6, 10))).toBe(true);
    expect(elementOfSetOrdered(0, list(1, 3, 6, 10))).toBe(false);
    expect(elementOfSetOrdered(11, list(1, 3, 6, 10))).toBe(false);
    expect(showList(intersectionSetOrdered(list(1, 3, 6, 10), list(3, 6, 7)))).toBe("(3 6)");
    expect(showList(intersectionSetOrdered(list(1, 3, 6, 10), list(11, 13)))).toBe("()");
  });

  it("2.3.3 the tree representation narrows its way down", () => {
    let t = emptyTreeSet;
    for (const n of [1, 3, 6, 10]) {
      t = adjoinSetTree(n, t);
    }
    // Adjoining 1, 3, 6, 10 in sequence: 1 at the root, 3 to its right,
    // 6 right of 3, 10 right of 6 (the book's Figure 2.17 skew).
    expect(t).toStrictEqual({
      _tag: "Node",
      entry: 1,
      left: { _tag: "Empty" },
      right: {
        _tag: "Node",
        entry: 3,
        left: { _tag: "Empty" },
        right: {
          _tag: "Node",
          entry: 6,
          left: { _tag: "Empty" },
          right: { _tag: "Node", entry: 10, left: { _tag: "Empty" }, right: { _tag: "Empty" } },
        },
      },
    });
    expect(elementOfSetTree(6, t)).toBe(true);
    expect(elementOfSetTree(7, t)).toBe(false);
    const balanced = makeTreeSet(
      6,
      makeTreeSet(1, emptyTreeSet, makeTreeSet(3, emptyTreeSet, emptyTreeSet)),
      makeTreeSet(10, makeTreeSet(7, emptyTreeSet, emptyTreeSet), emptyTreeSet),
    );
    expect(elementOfSetTree(7, balanced)).toBe(true);
    expect(adjoinSetTree(6, balanced)).toStrictEqual(balanced);
  });

  it("2.3.3 lookup walks the unordered record list", () => {
    const records = list<[number, Datum]>([1, qsym("Anna")], [2, qsym("Norah")]);
    expect(lookupUnordered(2, records)).toStrictEqual({
      _tag: "Some",
      value: [2, qsym("Norah")],
    });
    expect(lookupUnordered(3, records)).toStrictEqual({ _tag: "None" });
  });

  it("2.3.4 the leaf set comes out ordered by weight", () => {
    const leaves = makeLeafSet(
      list(
        [sym("A"), 4] as const,
        [sym("B"), 2] as const,
        [sym("C"), 1] as const,
        [sym("D"), 1] as const,
      ),
    );
    const weights: List<number> = list(1, 1, 2, 4);
    for (
      let l = leaves, w = weights;
      l._tag === "Cons" && w._tag === "Cons";
      l = l.tail, w = w.tail
    ) {
      expect(weightOf(l.head)).toBe(w.head);
    }
    const first = getOrElse(car(leaves), makeLeaf(sym("?"), 0));
    // make-leaf-set adjoins the last pair first, so the weight-1 tie
    // puts D ahead of C.
    expect(first._tag === "Leaf" ? first.symbol : "?").toBe("D");
  });

  it("2.3.4 decode walks bits down the tree and restarts at leaves", () => {
    const tree = makeCodeTree(
      makeLeaf(sym("a"), 2),
      makeCodeTree(makeLeaf(sym("b"), 1), makeLeaf(sym("c"), 1)),
    );
    const decoded = decode(list(0, 1, 0, 1, 1), tree);
    expect(decoded).toStrictEqual({
      _tag: "Ok",
      value: list(sym("a"), sym("b"), sym("c")),
    });
    expect(decoded._tag === "Ok" ? showSymbols(decoded.value) : "").toBe("(a b c)");
    expect(decode(list(2), tree)).toStrictEqual(err("bad bit: CHOOSE-BRANCH 2"));
    expect(chooseBranch(0, makeLeaf(sym("a"), 1))).toStrictEqual(
      err("bad position: CHOOSE-BRANCH cannot branch at a leaf"),
    );
  });

  it("2.3.4 the weighted adjoin keeps trees ordered by weight", () => {
    const leafA = makeLeaf(sym("A"), 4);
    const leafB = makeLeaf(sym("B"), 2);
    const merged = makeCodeTree(leafB, leafA);
    const set = cons(leafB, cons(leafA, nil));
    expect(adjoinSetWeighted(merged, set)).toStrictEqual(
      cons(leafB, cons(leafA, cons(merged, nil))),
    );
  });
});
