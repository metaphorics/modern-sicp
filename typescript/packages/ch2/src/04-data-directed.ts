// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.4

import { err, ok, type Result } from "./01-data-abstraction.js";
import { none, type Option, some } from "./02-picture-language.js";

// ---------------------------------------------------------------------
// 2.4.1 Representations for Complex Numbers
// ---------------------------------------------------------------------

// The abstract complex number: four selectors and two constructors, here
// one interface. Ben's and Alyssa's packages are two values of that
// interface type; the arithmetic below is written once against it.

/** The book's untyped pair: Ben stores it as (real, imaginary), Alyssa
 * as (magnitude, angle). */
export type ComplexPair = readonly [number, number];

/**
 * The complex-number interface of this section: the four selectors and
 * two constructors the arithmetic is written against. The type parameter
 * is the representation's datum shape, so the same interface covers the
 * bare pairs of 2.4.1 and the tagged datums of 2.4.2.
 */
export interface ComplexArith<Z> {
  readonly realPart: (z: Z) => number;
  readonly imagPart: (z: Z) => number;
  readonly magnitude: (z: Z) => number;
  readonly angle: (z: Z) => number;
  readonly makeFromRealImag: (x: number, y: number) => Z;
  readonly makeFromMagAng: (r: number, a: number) => Z;
}

/** Adds coordinatewise and rebuilds in the argument's representation:
 * the book's add-complex. */
export const addComplex = <Z>(arith: ComplexArith<Z>, z1: Z, z2: Z): Z =>
  arith.makeFromRealImag(
    arith.realPart(z1) + arith.realPart(z2),
    arith.imagPart(z1) + arith.imagPart(z2),
  );

/** Subtracts coordinatewise: the book's sub-complex. */
export const subComplex = <Z>(arith: ComplexArith<Z>, z1: Z, z2: Z): Z =>
  arith.makeFromRealImag(
    arith.realPart(z1) - arith.realPart(z2),
    arith.imagPart(z1) - arith.imagPart(z2),
  );

/** Multiplies magnitudes and adds angles: the book's mul-complex. */
export const mulComplex = <Z>(arith: ComplexArith<Z>, z1: Z, z2: Z): Z =>
  arith.makeFromMagAng(
    arith.magnitude(z1) * arith.magnitude(z2),
    arith.angle(z1) + arith.angle(z2),
  );

/** Divides magnitudes and subtracts angles: the book's div-complex. */
export const divComplex = <Z>(arith: ComplexArith<Z>, z1: Z, z2: Z): Z =>
  arith.makeFromMagAng(
    arith.magnitude(z1) / arith.magnitude(z2),
    arith.angle(z1) - arith.angle(z2),
  );

/** Ben Bitdiddle's rectangular representation, written in isolation:
 * the pair is (real part, imaginary part) and the trigonometry runs in
 * the selectors. */
export const benRectangular: ComplexArith<ComplexPair> = {
  realPart: (z) => z[0],
  imagPart: (z) => z[1],
  magnitude: (z) => Math.sqrt(z[0] * z[0] + z[1] * z[1]),
  angle: (z) => Math.atan2(z[1], z[0]),
  makeFromRealImag: (x, y) => [x, y],
  makeFromMagAng: (r, a) => [r * Math.cos(a), r * Math.sin(a)],
};

/** Alyssa P. Hacker's polar representation: the pair is (magnitude,
 * angle) and the trigonometry runs in the rectangular selectors. */
export const alyssaPolar: ComplexArith<ComplexPair> = {
  realPart: (z) => z[0] * Math.cos(z[1]),
  imagPart: (z) => z[0] * Math.sin(z[1]),
  magnitude: (z) => z[0],
  angle: (z) => z[1],
  makeFromRealImag: (x, y) => [Math.sqrt(x * x + y * y), Math.atan2(y, x)],
  makeFromMagAng: (r, a) => [r, a],
};

// ---------------------------------------------------------------------
// 2.4.2 Tagged Data
// ---------------------------------------------------------------------

/** The book's attach-tag: a tag glued to contents so two
 * representations can coexist. The tag is a string, the book's symbol. */
export type Tagged<T extends string, A> = { readonly _tag: T; readonly contents: A };

/** Builds the tagged datum carrying `tag` over `contents`. */
export const attachTag = <T extends string, A>(tag: T, contents: A): Tagged<T, A> => ({
  _tag: tag,
  contents,
});

/**
 * A tagged complex number. The book builds tagged data with cons cells
 * and a run-time type-tag check that raises "Bad tagged datum" on a
 * bare pair; this union is closed, so the bad-datum arm is
 * unrepresentable and the compiler flags every dispatch that forgets a
 * representation.
 */
export type TaggedComplex = Tagged<"rectangular", ComplexPair> | Tagged<"polar", ComplexPair>;

/** The book's rectangular?: true for Ben's representation. */
export const isRectangular = (z: TaggedComplex): boolean => z._tag === "rectangular";

/** The book's polar?: true for Alyssa's representation. */
export const isPolar = (z: TaggedComplex): boolean => z._tag === "polar";

// Ben's revised procedures, the 2.4.1 internals under their suffixed
// names, working on the bare pair.

/** The book's real-part-rectangular. */
export const realPartRectangular = (z: ComplexPair): number => z[0];

/** The book's imag-part-rectangular. */
export const imagPartRectangular = (z: ComplexPair): number => z[1];

/** The book's magnitude-rectangular. */
export const magnitudeRectangular = (z: ComplexPair): number =>
  Math.sqrt(z[0] * z[0] + z[1] * z[1]);

/** The book's angle-rectangular. */
export const angleRectangular = (z: ComplexPair): number => Math.atan2(z[1], z[0]);

/** The book's make-from-real-imag-rectangular: Ben tags what he builds. */
export const makeFromRealImagRectangular = (
  x: number,
  y: number,
): Tagged<"rectangular", ComplexPair> => ({
  _tag: "rectangular",
  contents: [x, y],
});

/** The book's make-from-mag-ang-rectangular: converted to the stored
 * coordinates, then tagged. */
export const makeFromMagAngRectangular = (
  r: number,
  a: number,
): Tagged<"rectangular", ComplexPair> => ({
  _tag: "rectangular",
  contents: [r * Math.cos(a), r * Math.sin(a)],
});

// Alyssa's revised procedures, polar names.

/** The book's real-part-polar. */
export const realPartPolar = (z: ComplexPair): number => z[0] * Math.cos(z[1]);

/** The book's imag-part-polar. */
export const imagPartPolar = (z: ComplexPair): number => z[0] * Math.sin(z[1]);

/** The book's magnitude-polar. */
export const magnitudePolar = (z: ComplexPair): number => z[0];

/** The book's angle-polar. */
export const anglePolar = (z: ComplexPair): number => z[1];

/** The book's make-from-real-imag-polar: converted to magnitude and
 * angle, then tagged. */
export const makeFromRealImagPolar = (x: number, y: number): Tagged<"polar", ComplexPair> => ({
  _tag: "polar",
  contents: [Math.sqrt(x * x + y * y), Math.atan2(y, x)],
});

/** The book's make-from-mag-ang-polar. */
export const makeFromMagAngPolar = (r: number, a: number): Tagged<"polar", ComplexPair> => ({
  _tag: "polar",
  contents: [r, a],
});

// The generic selectors, dispatching explicitly on the tag. Each switch
// is exhaustive over the two-member union, so the book's else arm ---
// "Unknown type" --- has no spelling here: a missing case fails the
// compile instead.

/** The generic real-part: the book's first data-directed spelling. */
export const realPartExplicit = (z: TaggedComplex): number =>
  z._tag === "rectangular" ? realPartRectangular(z.contents) : realPartPolar(z.contents);

/** The generic imag-part. */
export const imagPartExplicit = (z: TaggedComplex): number =>
  z._tag === "rectangular" ? imagPartRectangular(z.contents) : imagPartPolar(z.contents);

/** The generic magnitude. */
export const magnitudeExplicit = (z: TaggedComplex): number =>
  z._tag === "rectangular" ? magnitudeRectangular(z.contents) : magnitudePolar(z.contents);

/** The generic angle. */
export const angleExplicit = (z: TaggedComplex): number =>
  z._tag === "rectangular" ? angleRectangular(z.contents) : anglePolar(z.contents);

/** The 2.4.1 arithmetic over the tagged datums: the book's point that
 * add-complex "is still" the same procedure, now reading the generic
 * selectors. Rectangular is constructed from parts, polar from
 * magnitude and angle, as the book chooses. */
export const taggedArith: ComplexArith<TaggedComplex> = {
  realPart: realPartExplicit,
  imagPart: imagPartExplicit,
  magnitude: magnitudeExplicit,
  angle: angleExplicit,
  makeFromRealImag: makeFromRealImagRectangular,
  makeFromMagAng: makeFromMagAngPolar,
};

/** The book's add-complex after 2.4.2: the same body, reading the
 * generic selectors. */
export const addComplexTagged = (z1: TaggedComplex, z2: TaggedComplex): TaggedComplex =>
  addComplex(taggedArith, z1, z2);

// ---------------------------------------------------------------------
// 2.4.3 Data-Directed Programming and Additivity
// ---------------------------------------------------------------------

/**
 * A complex-table entry. The selectors are unary and answer a number;
 * the constructors are binary and answer a bare pair for tagging. One
 * table holds both, keyed by operation name and tag list, and each
 * fetcher narrows to the shape it asked for.
 */
export type ComplexHandler =
  | { readonly _tag: "Selector"; readonly fn: (z: ComplexPair) => number }
  | {
      readonly _tag: "Constructor";
      readonly fn: (x: number, y: number) => TaggedComplex;
    };

/** The operation-and-type table: op name, then the ordered tag list,
 * then the handler (D19). */
export type OpTable<H> = Map<string, Map<string, H>>;

/** The key for one ordered tag list: the tags in order, comma-joined. */
const tagsKey = (tags: ReadonlyArray<string>): string => tags.join(",");

/** An empty operation-and-type table. */
export const makeOpTable = <H>(): OpTable<H> => new Map();

/** The book's put: installs `item` under `op` and `tags`; installing
 * again over the same keys overwrites. */
export const put = <H>(
  table: OpTable<H>,
  op: string,
  tags: ReadonlyArray<string>,
  item: H,
): void => {
  const inner = table.get(op) ?? new Map<string, H>();
  inner.set(tagsKey(tags), item);
  table.set(op, inner);
};

/** The book's get: the handler under `op` and `tags`, or nothing ---
 * never a falsy stand-in for a handler (D19). */
export const get = <H>(table: OpTable<H>, op: string, tags: ReadonlyArray<string>): Option<H> => {
  const inner = table.get(op);
  if (inner === undefined) {
    return none;
  }
  const item = inner.get(tagsKey(tags));
  return item === undefined ? none : some(item);
};

/** Why a complex-table lookup cannot answer: the book's "No method for
 * these types", carrying the operation and the tag list it asked for. */
export type ApplyGenericError = {
  readonly _tag: "NoMethod";
  readonly op: string;
  readonly tags: ReadonlyArray<string>;
};

/** The book's error line, rendered: the message, the operation, and the
 * tag list as the book prints them. */
export const showApplyGenericError = (e: ApplyGenericError): string =>
  `No method for these types: APPLY-GENERIC (${e.op} (${e.tags.join(" ")}))`;

/** The selector under `op` and `tags`, or nothing when the keys are
 * absent or name a constructor. */
const getSelector = (
  table: OpTable<ComplexHandler>,
  op: string,
  tags: ReadonlyArray<string>,
): Option<(z: ComplexPair) => number> => {
  const entry = get(table, op, tags);
  return entry._tag === "Some" && entry.value._tag === "Selector" ? some(entry.value.fn) : none;
};

/** The constructor under `op` and `tag`, or nothing when the keys are
 * absent or name a selector. */
const getConstructor = (
  table: OpTable<ComplexHandler>,
  op: string,
  tag: string,
): Option<(x: number, y: number) => TaggedComplex> => {
  const entry = get(table, op, [tag]);
  return entry._tag === "Some" && entry.value._tag === "Constructor" ? some(entry.value.fn) : none;
};

/** Ben's rectangular package: the 2.4.1 procedures as internals, the
 * table entries as the interface. */
export const installRectangularPackage = (table: OpTable<ComplexHandler>): void => {
  // internal procedures
  const realPart = (z: ComplexPair): number => z[0];
  const imagPart = (z: ComplexPair): number => z[1];
  const magnitude = (z: ComplexPair): number => Math.sqrt(z[0] * z[0] + z[1] * z[1]);
  const angle = (z: ComplexPair): number => Math.atan2(z[1], z[0]);
  const makeFromRealImag = (x: number, y: number): ComplexPair => [x, y];
  const makeFromMagAng = (r: number, a: number): ComplexPair => [r * Math.cos(a), r * Math.sin(a)];
  // interface to the rest of the system
  const tag = (x: ComplexPair): TaggedComplex => attachTag("rectangular", x);
  put(table, "real-part", ["rectangular"], { _tag: "Selector", fn: realPart });
  put(table, "imag-part", ["rectangular"], { _tag: "Selector", fn: imagPart });
  put(table, "magnitude", ["rectangular"], { _tag: "Selector", fn: magnitude });
  put(table, "angle", ["rectangular"], { _tag: "Selector", fn: angle });
  put(table, "make-from-real-imag", ["rectangular"], {
    _tag: "Constructor",
    fn: (x, y) => tag(makeFromRealImag(x, y)),
  });
  put(table, "make-from-mag-ang", ["rectangular"], {
    _tag: "Constructor",
    fn: (r, a) => tag(makeFromMagAng(r, a)),
  });
};

/** Alyssa's polar package, analogous to Ben's. */
export const installPolarPackage = (table: OpTable<ComplexHandler>): void => {
  // internal procedures
  const magnitude = (z: ComplexPair): number => z[0];
  const angle = (z: ComplexPair): number => z[1];
  const makeFromMagAng = (r: number, a: number): ComplexPair => [r, a];
  const realPart = (z: ComplexPair): number => z[0] * Math.cos(z[1]);
  const imagPart = (z: ComplexPair): number => z[0] * Math.sin(z[1]);
  const makeFromRealImag = (x: number, y: number): ComplexPair => [
    Math.sqrt(x * x + y * y),
    Math.atan2(y, x),
  ];
  // interface to the rest of the system
  const tag = (x: ComplexPair): TaggedComplex => attachTag("polar", x);
  put(table, "real-part", ["polar"], { _tag: "Selector", fn: realPart });
  put(table, "imag-part", ["polar"], { _tag: "Selector", fn: imagPart });
  put(table, "magnitude", ["polar"], { _tag: "Selector", fn: magnitude });
  put(table, "angle", ["polar"], { _tag: "Selector", fn: angle });
  put(table, "make-from-real-imag", ["polar"], {
    _tag: "Constructor",
    fn: (x, y) => tag(makeFromRealImag(x, y)),
  });
  put(table, "make-from-mag-ang", ["polar"], {
    _tag: "Constructor",
    fn: (r, a) => tag(makeFromMagAng(r, a)),
  });
};

/** The system's table, both packages installed, as the book's running
 * system assumes. */
const complexTable: OpTable<ComplexHandler> = makeOpTable();
installRectangularPackage(complexTable);
installPolarPackage(complexTable);

/** The value of a successful dispatch, or `fallback` when the table had
 * no method: the book's "applies the resulting procedure if one is
 * present" as an expression. */
export const valueOr = <A, E>(r: Result<A, E>, fallback: A): A =>
  r._tag === "Ok" ? r.value : fallback;

/** The book's apply-generic: looks up the operation under the argument
 * tags and applies the handler to the bare contents, or answers why it
 * could not. The complex system's operations are unary, so the lookup
 * is made with one argument; the rest parameter keeps the book's
 * variadic shape. */
export const applyGeneric = (
  op: string,
  ...args: ReadonlyArray<Tagged<string, ComplexPair>>
): Result<number, ApplyGenericError> => {
  const tags = args.map((a) => a._tag);
  const proc = getSelector(complexTable, op, tags);
  if (proc._tag === "None") {
    return err({ _tag: "NoMethod", op, tags });
  }
  const [z] = args;
  return z === undefined ? err({ _tag: "NoMethod", op, tags }) : ok(proc.value(z.contents));
};

/** The generic real-part over the table: the book's final selector. */
export const realPart = (z: TaggedComplex): Result<number, ApplyGenericError> =>
  applyGeneric("real-part", z);

/** The generic imag-part over the table. */
export const imagPart = (z: TaggedComplex): Result<number, ApplyGenericError> =>
  applyGeneric("imag-part", z);

/** The generic magnitude over the table. */
export const magnitude = (z: TaggedComplex): Result<number, ApplyGenericError> =>
  applyGeneric("magnitude", z);

/** The generic angle over the table. */
export const angle = (z: TaggedComplex): Result<number, ApplyGenericError> =>
  applyGeneric("angle", z);

/** Builds a rectangular number through the table, as the book's
 * make-from-real-imag extracts the constructor Ben installed. */
export const makeFromRealImag = (
  x: number,
  y: number,
): Result<TaggedComplex, ApplyGenericError> => {
  const ctor = getConstructor(complexTable, "make-from-real-imag", "rectangular");
  return ctor._tag === "None"
    ? err({ _tag: "NoMethod", op: "make-from-real-imag", tags: ["rectangular"] })
    : ok(ctor.value(x, y));
};

/** Builds a polar number through the table, from Alyssa's installed
 * constructor. */
export const makeFromMagAng = (r: number, a: number): Result<TaggedComplex, ApplyGenericError> => {
  const ctor = getConstructor(complexTable, "make-from-mag-ang", "polar");
  return ctor._tag === "None"
    ? err({ _tag: "NoMethod", op: "make-from-mag-ang", tags: ["polar"] })
    : ok(ctor.value(r, a));
};

// Message passing: the table decomposed into columns. A complex number
// is a procedure that answers operation names; apply-generic just hands
// over the message.

/** The messages a message-passing complex number answers. */
export type ComplexMessage = "real-part" | "imag-part" | "magnitude" | "angle";

/** Why a message-passing object could not answer: the book's "Unknown
 * op", carrying the message and the constructor that received it. */
export type MessageError = {
  readonly _tag: "UnknownMessage";
  readonly op: string;
  readonly source: string;
};

/** A message-passing complex number: the book's dispatch procedure. */
export type MessageObject = (msg: string) => Result<number, MessageError>;

/** The book's message-passing make-from-real-imag: a closure that
 * answers the four selectors from its captured parts. */
export const makeFromRealImagMessage = (x: number, y: number): MessageObject => {
  const dispatch = (op: string): Result<number, MessageError> => {
    switch (op) {
      case "real-part":
        return ok(x);
      case "imag-part":
        return ok(y);
      case "magnitude":
        return ok(Math.sqrt(x * x + y * y));
      case "angle":
        return ok(Math.atan2(y, x));
      default:
        return err({ _tag: "UnknownMessage", op, source: "MAKE-FROM-REAL-IMAG" });
    }
  };
  return dispatch;
};

/** The book's one-line apply-generic for message passing: feed the
 * operation name to the object and let it work. */
export const applyGenericMessage = (
  object: MessageObject,
  op: ComplexMessage,
): Result<number, MessageError> => object(op);
