// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.3

import { err, ok, type Result } from "./01-data-abstraction.js";
import {
  append,
  cons,
  type List,
  list,
  nil,
  none,
  type Option,
  some,
} from "./02-picture-language.js";

// ---------------------------------------------------------------------
// 2.3.1 Quotation
// ---------------------------------------------------------------------

// Symbols: the book's `'a`. TypeScript never confuses a name with the
// value it stands for, so the section has no `quote` operator; a symbol
// is a branded string, and quoting a list is a helper that builds the
// list of symbols.

/** Brands a string as a section symbol, so a symbol can never be
 * confused with an ordinary string. */
declare const symbolBrand: unique symbol;

/** A symbol: the book's quoted `'a`, as a branded string. */
export type Symb = string & { readonly [symbolBrand]: true };

/** Builds the symbol printed as `name`: the book's `'name`. */
export const sym = (name: string): Symb => name as Symb;

/** What the book's `quote` produces, as a discriminated union: a
 * symbol, a number, or a list of quoted data. The tags let every
 * consumer dispatch with an exhaustive `switch`. */
export type Datum =
  | { readonly _tag: "Sym"; readonly name: Symb }
  | { readonly _tag: "Num"; readonly n: number }
  | { readonly _tag: "Lst"; readonly items: List<Datum> };

/** Builds the symbol datum printed as `name`. */
export const symDatum = (name: string): Datum => ({ _tag: "Sym", name: sym(name) });

/** Builds the number datum holding `n`. */
export const numDatum = (n: number): Datum => ({ _tag: "Num", n });

/** Builds the list datum of `items`: a helper that builds the list directly. */
export const listDatum = (...items: ReadonlyArray<Datum>): Datum => ({
  _tag: "Lst",
  items: list(...items),
});

/** Renders quoted data: symbols bare, numbers in decimal, lists in brackets. */
export const showDatum = (d: Datum): string =>
  d._tag === "Sym" ? d.name : d._tag === "Num" ? String(d.n) : showDataList(d.items);

/** Renders quoted data in bracket-comma notation. */
export const showDataList = (items: List<Datum>): string => {
  const parts: string[] = [];
  for (let rest = items; rest._tag === "Cons"; rest = rest.tail) {
    parts.push(showDatum(rest.head));
  }
  return `[${parts.join(", ")}]`;
};

/** Renders symbols in bracket-comma notation. */
export const showSymbols = (items: List<Symb>): string => {
  const parts: string[] = [];
  for (let rest = items; rest._tag === "Cons"; rest = rest.tail) {
    parts.push(rest.head);
  }
  return `[${parts.join(", ")}]`;
};

/** The book's `eq?` over quoted data: true for the same symbol or the
 * same number; two separately built lists are never `eq?`, just as in
 * Scheme. */
export const eqQ = (a: Datum, b: Datum): boolean => {
  if (a._tag === "Sym" && b._tag === "Sym") {
    return a.name === b.name;
  }
  if (a._tag === "Num" && b._tag === "Num") {
    return a.n === b.n;
  }
  return false;
};

/** The book's `memq`: the sublist of `x` beginning with the first item
 * `eq?` to `item`, or nothing when the symbol is absent. */
export const memq = (item: Datum, x: List<Datum>): Option<List<Datum>> => {
  if (x._tag === "Nil") {
    return none;
  }
  if (eqQ(item, x.head)) {
    return some(x);
  }
  return memq(item, x.tail);
};

// ---------------------------------------------------------------------
// 2.3.2 Example: Symbolic differentiation
// ---------------------------------------------------------------------

// The expression type: the book's list-structure representation with
// `'+`/`'*` tags becomes a discriminated union. The book's predicates
// (`sum?`, `product?`, `number?`) collapse into tag checks, and the
// book's selectors (`addend`, `augend`, `multiplier`, `multiplicand`)
// collapse into reading the union's fields.

/** An algebraic expression built from variables, numeric constants,
 * two-argument sums, and two-argument products. */
export type Expr =
  | { readonly _tag: "Var"; readonly name: Symb }
  | { readonly _tag: "Num"; readonly n: number }
  | { readonly _tag: "Sum"; readonly addend: Expr; readonly augend: Expr }
  | {
      readonly _tag: "Prod";
      readonly multiplier: Expr;
      readonly multiplicand: Expr;
    };

/** Builds the variable named `name`: the book's `'x` standing for x. */
export const variable = (name: string): Expr => ({ _tag: "Var", name: sym(name) });

/** Builds the numeric constant `n`. */
export const constant = (n: number): Expr => ({ _tag: "Num", n });

/** Tests whether an expression is the constant `n`: the book's
 * `=number?`. */
export const equalsNumber = (e: Expr, n: number): boolean => e._tag === "Num" && e.n === n;

/** Tests whether an expression is the variable `v`: the book's
 * `same-variable?` after the `variable?` check. */
export const sameVariableQ = (e: Expr, v: Symb): boolean => e._tag === "Var" && e.name === v;

/** Renders an expression in infix notation with minimal precedence parens. */
export const showExpr = (e: Expr): string => {
  const precedence = (expr: Expr): number =>
    expr._tag === "Sum" ? 1 : expr._tag === "Prod" ? 2 : 3;
  const render = (
    expr: Expr,
    parentPrecedence: number,
    childSide: "root" | "left" | "right",
  ): string => {
    if (expr._tag === "Var") {
      return expr.name;
    }
    if (expr._tag === "Num") {
      return String(expr.n);
    }
    const currentPrecedence = precedence(expr);
    const text =
      expr._tag === "Sum"
        ? `${render(expr.addend, currentPrecedence, "left")} + ${render(expr.augend, currentPrecedence, "right")}`
        : `${render(expr.multiplier, currentPrecedence, "left")} * ${render(expr.multiplicand, currentPrecedence, "right")}`;
    return currentPrecedence < parentPrecedence ||
      (childSide === "right" && currentPrecedence === parentPrecedence)
      ? `(${text})`
      : text;
  };
  return render(e, 0, "root");
};

/** The two constructors `deriv` builds results with. Swapping the
 * naive pair for the simplifying pair is the book's "we won't change
 * `deriv` at all" demonstration, made executable. */
export interface SumProductConstructors {
  readonly makeSum: (a1: Expr, a2: Expr) => Expr;
  readonly makeProduct: (m1: Expr, m2: Expr) => Expr;
}

/** The book's first constructors: `list('+ a1 a2)` and `list('* m1 m2)`,
 * with no simplification. */
export const naiveConstructors: SumProductConstructors = {
  makeSum: (a1, a2) => ({ _tag: "Sum", addend: a1, augend: a2 }),
  makeProduct: (m1, m2) => ({ _tag: "Prod", multiplier: m1, multiplicand: m2 }),
};

/** The book's revised `make-sum`: adds two constants, absorbs 0. */
export const makeSum = (a1: Expr, a2: Expr): Expr => {
  if (equalsNumber(a1, 0)) {
    return a2;
  }
  if (equalsNumber(a2, 0)) {
    return a1;
  }
  if (a1._tag === "Num" && a2._tag === "Num") {
    return constant(a1.n + a2.n);
  }
  return { _tag: "Sum", addend: a1, augend: a2 };
};

/** The book's revised `make-product`: absorbs 0 and 1, multiplies two
 * constants. */
export const makeProduct = (m1: Expr, m2: Expr): Expr => {
  if (equalsNumber(m1, 0) || equalsNumber(m2, 0)) {
    return constant(0);
  }
  if (equalsNumber(m1, 1)) {
    return m2;
  }
  if (equalsNumber(m2, 1)) {
    return m1;
  }
  if (m1._tag === "Num" && m2._tag === "Num") {
    return constant(m1.n * m2.n);
  }
  return { _tag: "Prod", multiplier: m1, multiplicand: m2 };
};

/** The simplifying constructor pair of the section's final program. */
export const simplifyingConstructors: SumProductConstructors = {
  makeSum,
  makeProduct,
};

/** The book's `deriv`: one exhaustive switch over the expression
 * union. The book's final `(else (error ...))` clause is gone: the
 * union has no other shape, and the compiler flags a missing case. */
export const derivVia = (exp: Expr, v: Symb, ctors: SumProductConstructors): Expr => {
  switch (exp._tag) {
    case "Num":
      return constant(0);
    case "Var":
      return sameVariableQ(exp, v) ? constant(1) : constant(0);
    case "Sum":
      return ctors.makeSum(derivVia(exp.addend, v, ctors), derivVia(exp.augend, v, ctors));
    case "Prod":
      return ctors.makeSum(
        ctors.makeProduct(exp.multiplier, derivVia(exp.multiplicand, v, ctors)),
        ctors.makeProduct(derivVia(exp.multiplier, v, ctors), exp.multiplicand),
      );
  }
};

/** Differentiates with the section's simplifying constructors: the
 * program of the final examples. */
export const deriv = (exp: Expr, v: Symb): Expr => derivVia(exp, v, simplifyingConstructors);

// ---------------------------------------------------------------------
// 2.3.3 Example: Representing sets
// ---------------------------------------------------------------------

// Sets as unordered lists. The book's elements "need not be symbols"
// because `equal?` compares them; this edition keeps the section's
// numeric examples, so plain `===` plays the role of `equal?`.

/** The book's `element-of-set?` over an unordered list: scans the
 * whole list in the worst case, Θ(n) for a set of n elements. */
export const elementOfSetUnordered = (x: number, set: List<number>): boolean => {
  if (set._tag === "Nil") {
    return false;
  }
  if (x === set.head) {
    return true;
  }
  return elementOfSetUnordered(x, set.tail);
};

/** The book's `adjoin-set` over an unordered list: conses unless the
 * element is already present. */
export const adjoinSetUnordered = (x: number, set: List<number>): List<number> =>
  elementOfSetUnordered(x, set) ? set : cons(x, set);

/** The book's `intersection-set` over unordered lists: keeps the
 * elements of `set1` that are also elements of `set2`, Θ(n²) for two
 * sets of size n. */
export const intersectionSetUnordered = (set1: List<number>, set2: List<number>): List<number> => {
  if (set1._tag === "Nil" || set2._tag === "Nil") {
    return nil;
  }
  if (elementOfSetUnordered(set1.head, set2)) {
    return cons(set1.head, intersectionSetUnordered(set1.tail, set2));
  }
  return intersectionSetUnordered(set1.tail, set2);
};

// Sets as ordered lists.

/** The book's ordered-list `element-of-set?`: stops as soon as it
 * passes the position where `x` would have to be. */
export const elementOfSetOrdered = (x: number, set: List<number>): boolean => {
  if (set._tag === "Nil") {
    return false;
  }
  if (x === set.head) {
    return true;
  }
  return x < set.head ? false : elementOfSetOrdered(x, set.tail);
};

/** The book's ordered-list `intersection-set`: advances whichever
 * list carries the smaller head, Θ(n + m) rather than Θ(n · m). */
export const intersectionSetOrdered = (set1: List<number>, set2: List<number>): List<number> => {
  if (set1._tag === "Nil" || set2._tag === "Nil") {
    return nil;
  }
  const x1 = set1.head;
  const x2 = set2.head;
  if (x1 === x2) {
    return cons(x1, intersectionSetOrdered(set1.tail, set2.tail));
  }
  return x1 < x2
    ? intersectionSetOrdered(set1.tail, set2)
    : intersectionSetOrdered(set1, set2.tail);
};

// Sets as binary trees. The book's three-item list `(entry left
// right)` becomes a discriminated union: an `Empty` tag replaces the
// empty-list subtree, so every read narrows before it descends.

/** A binary-tree set: empty, or a node holding one entry with smaller
 * elements to the left and larger elements to the right. */
export type TreeSet =
  | { readonly _tag: "Empty" }
  | {
      readonly _tag: "Node";
      readonly entry: number;
      readonly left: TreeSet;
      readonly right: TreeSet;
    };

/** The empty tree set: the book's `'()` as a tree. */
export const emptyTreeSet: TreeSet = { _tag: "Empty" };

/** The book's `make-tree`. */
export const makeTreeSet = (entry: number, left: TreeSet, right: TreeSet): TreeSet => ({
  _tag: "Node",
  entry,
  left,
  right,
});

/** The book's tree `element-of-set?`: one comparison per level, Θ(log n)
 * for a balanced tree. */
export const elementOfSetTree = (x: number, set: TreeSet): boolean => {
  if (set._tag === "Empty") {
    return false;
  }
  if (x === set.entry) {
    return true;
  }
  return x < set.entry ? elementOfSetTree(x, set.left) : elementOfSetTree(x, set.right);
};

/** The book's tree `adjoin-set`: rebuilds the spine it descends,
 * Θ(log n) for a balanced tree. */
export const adjoinSetTree = (x: number, set: TreeSet): TreeSet => {
  if (set._tag === "Empty") {
    return makeTreeSet(x, emptyTreeSet, emptyTreeSet);
  }
  if (x === set.entry) {
    return set;
  }
  return x < set.entry
    ? makeTreeSet(set.entry, adjoinSetTree(x, set.left), set.right)
    : makeTreeSet(set.entry, set.left, adjoinSetTree(x, set.right));
};

/** A record of the information-retrieval data base: a numerical key
 * and the value stored under it. */
export type RecordOf<Value> = readonly [key: number, value: Value];

/** The book's unordered-list `lookup`: the record with the given key,
 * or nothing. */
export const lookupUnordered = <Value>(
  givenKey: number,
  setOfRecords: List<RecordOf<Value>>,
): Option<RecordOf<Value>> => {
  if (setOfRecords._tag === "Nil") {
    return none;
  }
  const [key, value] = setOfRecords.head;
  if (givenKey === key) {
    return some([key, value]);
  }
  return lookupUnordered(givenKey, setOfRecords.tail);
};

// ---------------------------------------------------------------------
// 2.3.4 Example: Huffman encoding trees
// ---------------------------------------------------------------------

// The book's leaves `'(leaf symbol weight)` and its general trees
// `'(left right symbols weight)` become one discriminated union. The
// book's generic `symbols`/`weight` procedures become narrowings over
// the union's tags.

/** A Huffman encoding tree: a leaf holding one symbol and its weight,
 * or a branch whose symbol set is the union of its branches' sets and
 * whose weight is their sum. */
export type HuffTree =
  | { readonly _tag: "Leaf"; readonly symbol: Symb; readonly weight: number }
  | {
      readonly _tag: "Branch";
      readonly left: HuffTree;
      readonly right: HuffTree;
      readonly symbols: List<Symb>;
      readonly weight: number;
    };

/** The book's `make-leaf`. */
export const makeLeaf = (symbol: Symb, weight: number): HuffTree => ({
  _tag: "Leaf",
  symbol,
  weight,
});

/** The book's `make-code-tree`: the weight is the sum of the branch
 * weights, the symbol set their append. */
export const makeCodeTree = (left: HuffTree, right: HuffTree): HuffTree => ({
  _tag: "Branch",
  left,
  right,
  symbols: append(symbolsOf(left), symbolsOf(right)),
  weight: weightOf(left) + weightOf(right),
});

/** The book's `symbols`, by narrowing: the leaf's symbol, or the
 * branch's stored set. */
export const symbolsOf = (tree: HuffTree): List<Symb> =>
  tree._tag === "Leaf" ? list(tree.symbol) : tree.symbols;

/** The book's `weight`: both union shapes carry it directly. */
export const weightOf = (tree: HuffTree): number => tree.weight;

/** The book's `choose-branch`: descends left on 0 and right on 1, and
 * refuses any other bit. */
export const chooseBranch = (bit: number, branch: HuffTree): Result<HuffTree, string> => {
  if (branch._tag === "Leaf") {
    return err("chooseBranch: cannot branch at a leaf");
  }
  if (bit === 0) {
    return ok(branch.left);
  }
  if (bit === 1) {
    return ok(branch.right);
  }
  return err(`chooseBranch: bad bit ${bit}`);
};

/** The book's `decode`: walks the tree one bit at a time, restarting
 * at the root each time a leaf is reached. Like the book's version it
 * stops silently if the bits end between symbols; the added exercise
 * 2.67a pins a decoding that refuses such input. */
export const decode = (bits: List<number>, tree: HuffTree): Result<List<Symb>, string> => {
  const walk = (rest: List<number>, current: HuffTree): Result<List<Symb>, string> => {
    if (rest._tag === "Nil") {
      return ok(nil);
    }
    const next = chooseBranch(rest.head, current);
    if (next._tag === "Error") {
      return next;
    }
    if (next.value._tag === "Leaf") {
      const decoded = walk(rest.tail, tree);
      return decoded._tag === "Ok" ? ok(cons(next.value.symbol, decoded.value)) : decoded;
    }
    return walk(rest.tail, next.value);
  };
  return walk(bits, tree);
};

/** The book's weighted `adjoin-set`: inserts by weight into an
 * ordered list of leaves and trees. */
export const adjoinSetWeighted = (x: HuffTree, set: List<HuffTree>): List<HuffTree> => {
  if (set._tag === "Nil") {
    return list(x);
  }
  if (weightOf(x) < weightOf(set.head)) {
    return cons(x, set);
  }
  return cons(set.head, adjoinSetWeighted(x, set.tail));
};

/** A symbol-frequency pair: the book's `((A 4) (B 2) ...)` entry. */
export type SymbolFrequency = readonly [symbol: Symb, frequency: number];

/** The book's `make-leaf-set`: an ordered set of leaves, ready to be
 * merged by the Huffman algorithm. */
export const makeLeafSet = (pairs: List<SymbolFrequency>): List<HuffTree> => {
  if (pairs._tag === "Nil") {
    return nil;
  }
  const [symbol, frequency] = pairs.head;
  return adjoinSetWeighted(makeLeaf(symbol, frequency), makeLeafSet(pairs.tail));
};
