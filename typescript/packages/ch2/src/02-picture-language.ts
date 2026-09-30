// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.2

// The book's missing value: `car` of an empty list has no answer, so the
// selectors return the 0.4 option. This is the shape chapter 0 previewed
// and chapter 4's evaluator reuses for quoted data.
export type Option<A> = { readonly _tag: "Some"; readonly value: A } | { readonly _tag: "None" };

/** Wraps a value that was found. */
export const some = <A>(value: A): Option<A> => ({ _tag: "Some", value });

/** The absence the book signals by erroring. */
export const none: Option<never> = { _tag: "None" };

/** Reads an option with the fallback for the `None` case. */
export const getOrElse = <A>(o: Option<A>, fallback: A): A =>
  o._tag === "Some" ? o.value : fallback;

// ---------------------------------------------------------------------
// 2.2.1 Representing sequences
// ---------------------------------------------------------------------

/** The empty list; the book's `nil`. */
export interface Nil {
  readonly _tag: "Nil";
}

/** A cons cell: a head value and the rest of the list. */
export interface Cons<A> {
  readonly _tag: "Cons";
  readonly head: A;
  readonly tail: List<A>;
}

export type List<A> = Nil | Cons<A>;

/** The empty list, the chain's end-of-list marker. */
export const nil: List<never> = { _tag: "Nil" };

/** Pairs `head` onto an existing `tail`: the book's `cons`. */
export const cons = <A>(head: A, tail: List<A>): List<A> => ({ _tag: "Cons", head, tail });

/** Builds a list from a sequence of arguments: the book's `list`, the
 * rest parameter that replaces dotted-tail notation. */
export const list = <A>(...items: ReadonlyArray<A>): List<A> =>
  items.reduceRight<List<A>>((tail, head) => cons(head, tail), nil);

/** The first item of a list, or nothing for the empty list: the book's `car`. */
export const car = <A>(l: List<A>): Option<A> => (l._tag === "Cons" ? some(l.head) : none);

/** All but the first item, or nothing for the empty list: the book's `cdr`. */
export const cdr = <A>(l: List<A>): Option<List<A>> => (l._tag === "Cons" ? some(l.tail) : none);

/** Tests whether a list is the empty list: the book's `null?`. */
export const isNull = <A>(l: List<A>): boolean => l._tag === "Nil";

/** A cons cell of renderable values: the recursion lives in an
 * interface because a type alias may not reference itself. */
export interface ShowableList extends Cons<Showable> {}

/** What `showList` renders: a value, a vector, a segment, the empty
 * list, or a nested list of them. */
export type Showable = number | string | boolean | Vect | Segment | ShowableList | Nil;

const showOne = (v: Showable): string => {
  if (typeof v !== "object") {
    return typeof v === "string" ? JSON.stringify(v) : String(v);
  }
  if ("start" in v) {
    return `${showVect(v.start)}->${showVect(v.end)}`;
  }
  if ("x" in v) {
    return showVect(v);
  }
  return showList(v);
};

/** Renders a list in bracket-comma notation, preserving nested data. */
export const showList = (xs: List<Showable>): string => {
  const items: string[] = [];
  for (let rest: List<Showable> = xs; rest._tag === "Cons"; rest = rest.tail) {
    items.push(showOne(rest.head));
  }
  return `[${items.join(", ")}]`;
};

/** The book's `list-ref`: cdrs down the list `n` times and takes the car;
 * nothing when `n` runs past the end, the analog of the book's error. */
export const listRef = <A>(items: List<A>, n: number): Option<A> => {
  if (n === 0) {
    return car(items);
  }
  const rest = cdr(items);
  return rest._tag === "Some" ? listRef(rest.value, n - 1) : none;
};

/** The book's recursive `length`: 1 plus the length of the `cdr`. */
export const length = <A>(items: List<A>): number =>
  items._tag === "Nil" ? 0 : 1 + length(items.tail);

/** The book's `append`: conses up a new spine for `list1` that ends in
 * `list2` itself, the sharing the book's picture shows. */
export const append = <A>(list1: List<A>, list2: List<A>): List<A> =>
  list1._tag === "Nil" ? list2 : cons(list1.head, append(list1.tail, list2));

/** The book's `scale-list` in its first, explicit spelling. */
export const scaleList = (items: List<number>, factor: number): List<number> =>
  items._tag === "Nil" ? nil : cons(items.head * factor, scaleList(items.tail, factor));

/** The book's `map`: applies `f` to each element and conses up the list
 * of results. The edition's map is unary over one list, as in the book. */
export const map = <A, B>(f: (x: A) => B, items: List<A>): List<B> =>
  items._tag === "Nil" ? nil : cons(f(items.head), map(f, items.tail));

// ---------------------------------------------------------------------
// 2.2.2 Hierarchical structures
// ---------------------------------------------------------------------

/** A leaf of the edition's tree union: one value. */
export interface Leaf<A> {
  readonly _tag: "Leaf";
  readonly value: A;
}

/** A branch of the edition's tree union: the sequence of its subtrees,
 * the book's nested list viewed as a tree. */
export interface TreeNode<A> {
  readonly _tag: "Node";
  readonly subtrees: ReadonlyArray<Tree<A>>;
}

/**
 * The edition's tree: the book builds trees from nested lists, and this
 * recursive union replaces them — a leaf holds a value, a branch holds
 * the sequence of its subtrees.
 */
export type Tree<A> = Leaf<A> | TreeNode<A>;

/** Builds a leaf. */
export const leaf = <A>(value: A): Tree<A> => ({ _tag: "Leaf", value });

/** Builds a branch from its subtrees. */
export const node = <A>(...subtrees: ReadonlyArray<Tree<A>>): Tree<A> => ({
  _tag: "Node",
  subtrees,
});

/** Renders a tree in the book's nested bracket-comma notation. */
export const showTree = <A>(tree: Tree<A>): string =>
  tree._tag === "Leaf" ? String(tree.value) : `[${tree.subtrees.map(showTree).join(", ")}]`;

/** The book's `count-leaves`: the car of a tree may itself be a tree, so
 * the reduction adds the counts of both sides until leaves count 1. */
export const countLeaves = <A>(tree: Tree<A>): number =>
  tree._tag === "Leaf" ? 1 : tree.subtrees.reduce((sum, sub) => sum + countLeaves(sub), 0);

/** The book's direct `scale-tree`: the recursive plan of `count-leaves`,
 * multiplying each leaf and rebuilding the same shape. */
export const scaleTree = (tree: Tree<number>, factor: number): Tree<number> =>
  tree._tag === "Leaf"
    ? leaf(tree.value * factor)
    : node(...tree.subtrees.map((sub) => scaleTree(sub, factor)));

/** The book's second `scale-tree`: the tree regarded as a sequence of
 * sub-trees, mapped over with the leaf case decided inside. */
export const scaleTreeViaMap = (tree: Tree<number>, factor: number): Tree<number> =>
  tree._tag === "Leaf"
    ? leaf(tree.value * factor)
    : node(
        ...tree.subtrees.map((sub) =>
          sub._tag === "Leaf" ? leaf(sub.value * factor) : scaleTreeViaMap(sub, factor),
        ),
      );

// ---------------------------------------------------------------------
// 2.2.3 Sequences as conventional interfaces
// ---------------------------------------------------------------------

/** The book's `square`, the map argument of this subsection. */
export const square = (x: number): number => x * x;

/** The book's `odd?`. */
export const isOdd = (n: number): boolean => Math.abs(n % 2) === 1;

/** The book's `even?`. */
export const isEven = (n: number): boolean => n % 2 === 0;

/** The Fibonacci number of `k`, the 1.2.2 tree-recursive process. */
export const fib = (k: number): number => (k < 2 ? k : fib(k - 1) + fib(k - 2));

/** The book's `prime?`, trial division per the 1.2.6 plan. */
export const isPrime = (n: number): boolean => {
  if (n < 2) {
    return false;
  }
  const find = (d: number): number => (d * d > n ? n : n % d === 0 ? d : find(d + 1));
  return find(2) === n;
};

/** The book's `sum-odd-squares` in its raw tree-recursive spelling. */
export const sumOddSquares = (tree: Tree<number>): number => {
  if (tree._tag === "Leaf") {
    return isOdd(tree.value) ? square(tree.value) : 0;
  }
  return tree.subtrees.reduce((sum, sub) => sum + sumOddSquares(sub), 0);
};

/** The book's first `even-fibs`, the raw recursive spelling with the
 * internal `next` walk over the integers. */
export const evenFibs = (n: number): List<number> => {
  const next = (k: number): List<number> => {
    if (k > n) {
      return nil;
    }
    const f = fib(k);
    return isEven(f) ? cons(f, next(k + 1)) : next(k + 1);
  };
  return next(0);
};

/** The book's `filter`: keeps the elements satisfying `predicate`. */
export const filter = <A>(predicate: (x: A) => boolean, sequence: List<A>): List<A> => {
  if (sequence._tag === "Nil") {
    return nil;
  }
  return predicate(sequence.head)
    ? cons(sequence.head, filter(predicate, sequence.tail))
    : filter(predicate, sequence.tail);
};

/** The book's `accumulate`, also known as `fold-right`. */
export const accumulate = <A, B>(op: (x: A, y: B) => B, initial: B, sequence: List<A>): B =>
  sequence._tag === "Nil" ? initial : op(sequence.head, accumulate(op, initial, sequence.tail));

/** The book's `enumerate-interval`: the integers `low` through `high`. */
export const enumerateInterval = (low: number, high: number): List<number> =>
  low > high ? nil : cons(low, enumerateInterval(low + 1, high));

/** The book's `enumerate-tree`, the `fringe` of exercise 2.28 renamed to
 * sit in the family of sequence operations. */
export const enumerateTree = <A>(tree: Tree<A>): List<A> => {
  if (tree._tag === "Leaf") {
    return list(tree.value);
  }
  return tree.subtrees.reduce<List<A>>((acc, sub) => append(acc, enumerateTree(sub)), nil);
};

/** The personnel record the salary example assumes. */
export interface PersonnelRecord {
  readonly name: string;
  readonly salary: number;
  readonly isProgrammer: boolean;
}

/** The book's `salary` selector, assumed by the record example. */
export const salary = (record: PersonnelRecord): number => record.salary;

/** The book's `programmer?` predicate, assumed by the record example. */
export const isProgrammer = (record: PersonnelRecord): boolean => record.isProgrammer;

/** The book's `sum-odd-squares` reformulated as a signal-flow plan. */
export const sumOddSquaresViaSequence = (tree: Tree<number>): number =>
  accumulate((a: number, b: number) => a + b, 0, map(square, filter(isOdd, enumerateTree(tree))));

/** The book's `even-fibs` reformulated as a signal-flow plan. */
export const evenFibsViaSequence = (n: number): List<number> =>
  accumulate(
    (x: number, y: List<number>) => cons(x, y),
    nil,
    filter(isEven, map(fib, enumerateInterval(0, n))),
  );

/** The book's `list-fib-squares`: the pieces rearranged. */
export const listFibSquares = (n: number): List<number> =>
  accumulate(
    (x: number, y: List<number>) => cons(x, y),
    nil,
    map(square, map(fib, enumerateInterval(0, n))),
  );

/** The book's `product-of-squares-of-odd-elements`. */
export const productOfSquaresOfOddElements = (sequence: List<number>): number =>
  accumulate((a: number, b: number) => a * b, 1, map(square, filter(isOdd, sequence)));

/** The book's `salary-of-highest-paid-programmer`, a conventional
 * data-processing application over sequence operations. */
export const salaryOfHighestPaidProgrammer = (records: List<PersonnelRecord>): number =>
  accumulate(
    (a: number, b: number) => Math.max(a, b),
    0,
    map(salary, filter(isProgrammer, records)),
  );

/** The book's `flatmap`: mapping and accumulating with `append`. */
export const flatmap = <A, B>(f: (x: A) => List<B>, sequence: List<A>): List<B> =>
  accumulate((x: List<B>, y: List<B>) => append(x, y), nil, map(f, sequence));

/** The book's `prime-sum?` over a pair represented as a two-element list. */
export const primeSum = (pair: List<number>): boolean =>
  pair._tag === "Cons" && pair.tail._tag === "Cons" && isPrime(pair.head + pair.tail.head);

/** The book's `make-pair-sum`: the pair plus its sum, as a three-element list. */
export const makePairSum = (pair: List<number>): List<number> =>
  pair._tag === "Cons" && pair.tail._tag === "Cons"
    ? list(pair.head, pair.tail.head, pair.head + pair.tail.head)
    : pair;

/** The book's `prime-sum-pairs`, the nested-mapping program assembled. */
export const primeSumPairs = (n: number): List<List<number>> =>
  map(
    makePairSum,
    filter(
      primeSum,
      flatmap(
        (i: number) => map((j: number) => list(i, j), enumerateInterval(1, i - 1)),
        enumerateInterval(1, n),
      ),
    ),
  );

/** The book's `remove`, the filter behind `permutations`. */
export const remove = <A>(item: A, sequence: List<A>): List<A> =>
  filter((x: A) => x !== item, sequence);

/** The book's `permutations` over the edition's lists. */
export const permutations = <A>(s: List<A>): List<List<A>> =>
  s._tag === "Nil"
    ? list(nil)
    : flatmap((x: A) => map((p: List<A>) => cons(x, p), permutations(remove(x, s))), s);

// ---------------------------------------------------------------------
// 2.2.4 Example: a picture language
// ---------------------------------------------------------------------

/** A two-dimensional vector: the book's `make-vect` abstraction of
 * exercise 2.46. */
export interface Vect {
  readonly x: number;
  readonly y: number;
}

/** The book's `make-vect`. */
export const makeVect = (x: number, y: number): Vect => ({ x, y });

/** The book's `add-vect`. */
export const addVect = (v: Vect, w: Vect): Vect => makeVect(v.x + w.x, v.y + w.y);

/** The book's `sub-vect`. */
export const subVect = (v: Vect, w: Vect): Vect => makeVect(v.x - w.x, v.y - w.y);

/** The book's `scale-vect`. */
export const scaleVect = (s: number, v: Vect): Vect => makeVect(s * v.x, s * v.y);

/** Renders a vector as the book displays it. */
export const showVect = (v: Vect): string => `(${v.x}, ${v.y})`;

/** A frame: an origin vector and two edge vectors, the book's
 * `make-frame` abstraction; exercise 2.47 supplies alternative
 * constructors over other representations. */
export interface Frame {
  readonly origin: Vect;
  readonly edge1: Vect;
  readonly edge2: Vect;
}

/** The book's `make-frame`. */
export const makeFrame = (origin: Vect, edge1: Vect, edge2: Vect): Frame => ({
  origin,
  edge1,
  edge2,
});

/** The book's `origin-frame`. */
export const originFrame = (frame: Frame): Vect => frame.origin;

/** The book's `edge1-frame`. */
export const edge1Frame = (frame: Frame): Vect => frame.edge1;

/** The book's `edge2-frame`. */
export const edge2Frame = (frame: Frame): Vect => frame.edge2;

/** The frame the book's figures draw in: the unit square itself. */
export const unitSquare: Frame = makeFrame(makeVect(0, 0), makeVect(1, 0), makeVect(0, 1));

/** The book's `frame-coord-map`: maps a unit-square vector to the frame,
 * `Origin + x * Edge1 + y * Edge2`. */
export const frameCoordMap =
  (frame: Frame) =>
  (v: Vect): Vect =>
    addVect(
      originFrame(frame),
      addVect(scaleVect(v.x, edge1Frame(frame)), scaleVect(v.y, edge2Frame(frame))),
    );

/** A directed line segment: the book's `make-segment` abstraction of
 * exercise 2.48. */
export interface Segment {
  readonly start: Vect;
  readonly end: Vect;
}

/** The book's `make-segment`. */
export const makeSegment = (start: Vect, end: Vect): Segment => ({ start, end });

/** The book's `start-segment`. */
export const startSegment = (segment: Segment): Vect => segment.start;

/** The book's `end-segment`. */
export const endSegment = (segment: Segment): Vect => segment.end;

/**
 * A painter: a function from a frame to the segments it draws there.
 * This is the book's procedural representation of painters — the plan's
 * frame-to-segments decision — and painters stay closed under the
 * language's means of combination because the result type is the
 * argument type again.
 */
export type Painter = (frame: Frame) => List<Segment>;

/** Builds a painter from a list of segments in unit-square coordinates:
 * the book's `segments->painter`. Each endpoint goes through the frame
 * coordinate map; the book's `draw-line` calls become the returned
 * segments, which any renderer can draw. */
export const segmentsToPainter =
  (segmentList: List<Segment>): Painter =>
  (frame) => {
    const m = frameCoordMap(frame);
    const out: Segment[] = [];
    for (let rest = segmentList; rest._tag === "Cons"; rest = rest.tail) {
      out.push(makeSegment(m(startSegment(rest.head)), m(endSegment(rest.head))));
    }
    return out.reduceRight<List<Segment>>((tail, head) => cons(head, tail), nil);
  };

/** The segment list of the `wave` painter, the hand-drawn figure the
 * book takes as given (its construction is exercise 2.49d). */
export const waveSegments = (): List<Segment> => {
  const s = (x1: number, y1: number, x2: number, y2: number): Segment =>
    makeSegment(makeVect(x1, y1), makeVect(x2, y2));
  return list(
    s(0.0, 0.85, 0.12, 0.62), // left side of the head
    s(0.12, 0.62, 0.3, 0.68), // up to the crown
    s(0.3, 0.68, 0.42, 0.72), // crown peak, left half
    s(0.42, 0.72, 0.52, 0.7), // crown peak, right half
    s(0.52, 0.7, 0.6, 0.6), // down to the neck
    s(0.6, 0.6, 0.65, 0.45), // right shoulder
    s(0.65, 0.45, 1.0, 0.4), // right arm out
    s(0.3, 0.68, 0.25, 0.55), // left shoulder
    s(0.25, 0.55, 0.0, 0.65), // left arm out
    s(0.6, 0.6, 0.72, 0.62), // right hand back up
    s(0.72, 0.62, 0.85, 0.55), // right hand wave
    s(0.4, 0.45, 0.42, 0.2), // left leg
    s(0.42, 0.2, 0.35, 0.0), // left foot
    s(0.4, 0.45, 0.55, 0.45), // hips
    s(0.55, 0.45, 0.6, 0.2), // right leg
    s(0.6, 0.2, 0.7, 0.0), // right foot
    s(0.32, 0.62, 0.28, 0.55), // left hand
    s(0.36, 0.52, 0.46, 0.52), // belt
  );
};

/** The book's `wave` painter. */
export const wave = (): Painter => segmentsToPainter(waveSegments());

/** The segment list of this edition's `rogers`: the book paints a
 * photograph, and this edition stands it in with a line portrait, the
 * only kind the segment painter can draw. */
export const rogersSegments = (): List<Segment> => {
  const s = (x1: number, y1: number, x2: number, y2: number): Segment =>
    makeSegment(makeVect(x1, y1), makeVect(x2, y2));
  return list(
    s(0.3, 0.95, 0.25, 0.8), // left side of the head
    s(0.25, 0.8, 0.3, 0.62), // left jaw
    s(0.3, 0.62, 0.5, 0.55), // chin
    s(0.5, 0.55, 0.7, 0.62), // right jaw
    s(0.7, 0.62, 0.75, 0.8), // right side of the head
    s(0.75, 0.8, 0.7, 0.95), // right temple
    s(0.7, 0.95, 0.5, 1.0), // crown, right half
    s(0.5, 1.0, 0.3, 0.95), // crown, left half
    s(0.36, 0.85, 0.42, 0.85), // left eye
    s(0.58, 0.85, 0.64, 0.85), // right eye
    s(0.5, 0.8, 0.5, 0.68), // nose bridge
    s(0.44, 0.66, 0.56, 0.66), // mouth
    s(0.5, 0.55, 0.5, 0.45), // neck
    s(0.2, 0.3, 0.5, 0.45), // left collar
    s(0.8, 0.3, 0.5, 0.45), // right collar
    s(0.2, 0.3, 0.15, 0.0), // left shoulder down
    s(0.8, 0.3, 0.85, 0.0), // right shoulder down
  );
};

/** This edition's `rogers` painter. */
export const rogers = (): Painter => segmentsToPainter(rogersSegments());

/** The book's `transform-painter`: takes a painter and three unit-square
 * corner vectors, and returns the painter that calls the original on
 * the derived frame. */
export const transformPainter =
  (painter: Painter, origin: Vect, corner1: Vect, corner2: Vect): Painter =>
  (frame) => {
    const m = frameCoordMap(frame);
    const newOrigin = m(origin);
    return painter(
      makeFrame(newOrigin, subVect(m(corner1), newOrigin), subVect(m(corner2), newOrigin)),
    );
  };

/** The book's `flip-vert`, the worked example of `transform-painter`. */
export const flipVert = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(0.0, 1.0), makeVect(1.0, 1.0), makeVect(0.0, 0.0));

/** The book's `shrink-to-upper-right`. */
export const shrinkToUpperRight = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(0.5, 0.5), makeVect(1.0, 0.5), makeVect(0.5, 1.0));

/** The book's `rotate90`, a pure rotation only for square frames. */
export const rotate90 = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(1.0, 0.0), makeVect(1.0, 1.0), makeVect(0.0, 0.0));

/** The book's `squash-inwards`, the transform behind the diamond-shaped
 * images of the book's figures. */
export const squashInwards = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(0.0, 0.0), makeVect(0.65, 0.35), makeVect(0.35, 0.65));

/** The book's `beside`: the first painter in the left half of the frame,
 * the second in the right half. */
export const beside = (painter1: Painter, painter2: Painter): Painter => {
  const splitPoint = makeVect(0.5, 0.0);
  const paintLeft = transformPainter(painter1, makeVect(0.0, 0.0), splitPoint, makeVect(0.0, 1.0));
  const paintRight = transformPainter(painter2, splitPoint, makeVect(1.0, 0.0), makeVect(0.5, 1.0));
  return (frame) => append(paintLeft(frame), paintRight(frame));
};

/** The book's `flip-horiz`, which exercise 2.50 derives; the module
 * carries it because `square-limit` calls it. */
export const flipHoriz = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(1.0, 0.0), makeVect(0.0, 0.0), makeVect(1.0, 1.0));

/** The book's `rotate180`, which exercise 2.50 derives; spelled here as
 * the footnote's `compose` of the two flips because `square-limit`
 * calls it. */
export const rotate180 = (painter: Painter): Painter =>
  transformPainter(painter, makeVect(1.0, 1.0), makeVect(0.0, 1.0), makeVect(1.0, 0.0));

/** The book's `below`, which exercise 2.51 derives; the module carries
 * the direct construction because `corner-split` calls it. */
export const below = (painter1: Painter, painter2: Painter): Painter => {
  const splitPoint = makeVect(0.0, 0.5);
  const paintBottom = transformPainter(
    painter1,
    makeVect(0.0, 0.0),
    makeVect(1.0, 0.0),
    splitPoint,
  );
  const paintTop = transformPainter(painter2, splitPoint, makeVect(1.0, 0.5), makeVect(0.0, 1.0));
  return (frame) => append(paintBottom(frame), paintTop(frame));
};

/** The book's `right-split`: split and branch towards the right. */
export const rightSplit = (painter: Painter, n: number): Painter => {
  if (n === 0) {
    return painter;
  }
  const smaller = rightSplit(painter, n - 1);
  return beside(painter, below(smaller, smaller));
};

/** The book's `up-split`, which exercise 2.44 defines; the module
 * carries it because `corner-split` calls it. */
export const upSplit = (painter: Painter, n: number): Painter => {
  if (n === 0) {
    return painter;
  }
  const smaller = upSplit(painter, n - 1);
  return below(painter, beside(smaller, smaller));
};

/** The book's `corner-split`: balanced patterns branching upwards as
 * well as towards the right. */
export const cornerSplit = (painter: Painter, n: number): Painter => {
  if (n === 0) {
    return painter;
  }
  const up = upSplit(painter, n - 1);
  const right = rightSplit(painter, n - 1);
  const topLeft = beside(up, up);
  const bottomRight = below(right, right);
  const corner = cornerSplit(painter, n - 1);
  return beside(below(painter, topLeft), below(bottomRight, corner));
};

/** The book's `square-limit`: four `corner-split` copies arranged into
 * the symmetric pattern. */
export const squareLimit = (painter: Painter, n: number): Painter => {
  const quarter = cornerSplit(painter, n);
  const half = beside(flipHoriz(quarter), quarter);
  return below(flipVert(half), half);
};

/** A painter operation: the one-argument painter transformer that
 * `square-of-four` composes. */
export type PainterOp = (painter: Painter) => Painter;

/** The identity painter operation, `square-of-four`'s do-nothing corner. */
export const identityOp: PainterOp = (painter) => painter;

/** The book's `square-of-four`: takes four corner operations and
 * returns the painter operation that arranges the four copies in a
 * square. */
export const squareOfFour =
  (tl: PainterOp, tr: PainterOp, bl: PainterOp, br: PainterOp): PainterOp =>
  (painter) => {
    const top = beside(tl(painter), tr(painter));
    const bottom = beside(bl(painter), br(painter));
    return below(bottom, top);
  };

/** The book's `flipped-pairs` in its first spelling. */
export const flippedPairs = (painter: Painter): Painter => {
  const painter2 = beside(painter, flipVert(painter));
  return below(painter2, painter2);
};

/** The book's `flipped-pairs` via `square-of-four`. */
export const flippedPairsViaSquareOfFour = (painter: Painter): Painter =>
  squareOfFour(identityOp, flipVert, identityOp, flipVert)(painter);

/** The book's `square-limit` via `square-of-four`. */
export const squareLimitViaSquareOfFour = (painter: Painter, n: number): Painter =>
  squareOfFour(flipHoriz, identityOp, rotate180, flipVert)(cornerSplit(painter, n));

// --- deterministic SVG rendering -------------------------------------

/** Renders the segments a painter draws on `frame` into an SVG document:
 * the edition stands in for the book's graphics terminal. Frame
 * coordinates land in the unit square, the `y` axis flips (SVG grows
 * downward), and the output is plain markup with a fixed attribute
 * order and two-decimal precision — no timestamp, no randomness — so a
 * regenerated figure is byte-identical to the checked-in one under
 * `book/figures/generated/chap2/`. */
export const renderSvgFrame = (painter: Painter, frame: Frame, size: number): string => {
  const lines: string[] = [];
  for (let rest = painter(frame); rest._tag === "Cons"; rest = rest.tail) {
    const { start, end } = rest.head;
    lines.push(
      `<line x1="${px(start.x, size)}" y1="${px(1 - start.y, size)}" x2="${px(end.x, size)}" y2="${px(1 - end.y, size)}"/>`,
    );
  }
  const s = size.toFixed(0);
  return [
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${s} ${s}" width="${s}" height="${s}">`,
    `<rect width="${s}" height="${s}" fill="white"/>`,
    `<g stroke="#1a1a1a" stroke-width="1.2" stroke-linecap="round" fill="none">`,
    ...lines,
    `</g>`,
    `</svg>`,
    ``,
  ].join("\n");
};

const px = (value: number, size: number): string => {
  const scaled = value * size * 100;
  const floor = Math.floor(scaled);
  const tie = scaled - floor === 0.5;
  const cents = tie ? (floor % 2 === 0 ? floor : floor + 1) : Math.round(scaled);
  return (cents / 100).toFixed(2);
};

/** Renders a painter on the unit-square frame: the figures' entry point. */
export const renderSvg = (painter: Painter, size: number): string =>
  renderSvgFrame(painter, unitSquare, size);
