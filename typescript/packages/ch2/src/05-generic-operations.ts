// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.5

import { err, ok, type Result } from "./01-data-abstraction.js";
import { none, type Option, some } from "./02-picture-language.js";
import {
  attachTag,
  type ComplexPair,
  formatNoMethodError,
  get as getIn,
  makeOpTable,
  type OpTable,
  put as putIn,
  type Tagged,
} from "./04-data-directed.js";

// ---------------------------------------------------------------------
// 2.5.1 Generic Arithmetic Operations
// ---------------------------------------------------------------------

/**
 * An ordinary number: after exercise 2.78 the tower's base case is the
 * bare `bigint` itself, not a pair whose car is the tag.
 */
export type TsNumber = bigint;

/** The numerator-denominator pair behind a `rational` tag. */
export type RatContents = readonly [bigint, bigint];

/** A rational number: the tagged 2.1.1 pair on exact integers. */
export type Rational = Tagged<"rational", RatContents>;

/** The real level of Figure 2.25's tower, an inexact `number`. Its
 * package entries are the tower exercises' work (2.83 to 2.85). */
export type Real = Tagged<"real", number>;

/** Ben's rectangular contents: the bare (real, imaginary) pair. */
export type Rectangular = Tagged<"rectangular", ComplexPair>;

/** Alyssa's polar contents: the bare (magnitude, angle) pair. */
export type Polar = Tagged<"polar", ComplexPair>;

/** A complex number: the inner representation tagged again --- the
 * section's two-level tag system. */
export type ComplexNumber = Tagged<"complex", Rectangular | Polar>;

/** A term: the book's (order coeff) pair. The coefficient is any datum
 * the tower knows, which is how polynomials over polynomials work. */
export type Term = readonly [order: bigint, coeff: ArithDatum];

/** A term list: the sparse representation, highest order first. */
export type TermList = ReadonlyArray<Term>;

/** The book's poly: the (variable . term-list) pair. */
export type Poly = { readonly variable: string; readonly terms: TermList };

/** A polynomial: the tagged poly. */
export type Polynomial = Tagged<"polynomial", Poly>;

/** The quotient and remainder of a polynomial division: the book's
 * two-element list, one datum here. */
export type PolyPair = readonly [Poly, Poly];

/** The numerator-denominator pair of a rational function (the extended
 * exercise): the parts are tower datums, integers or polynomials
 * alike. */
export type RatFnContents = { readonly numer: ArithDatum; readonly denom: ArithDatum };

/** A rational function: the extended exercise's fraction over the
 * generic operations, same `rational` tag, datum-valued parts. */
export type RationalFunction = Tagged<"rational", RatFnContents>;

/** Every datum the system's packages produce. */
export type ArithDatum =
  | TsNumber
  | Rational
  | Real
  | ComplexNumber
  | Polynomial
  | Tagged<"quotient-remainder", PolyPair>
  | RationalFunction;

/** The bare contents behind each tag: the number itself after 2.78, the
 * numerator-denominator pair, the inner tagged representation, the
 * poly, the quotient-remainder pair, the rational function's parts. */
export type ArithContents =
  | bigint
  | number
  | RatContents
  | Rectangular
  | Polar
  | Poly
  | PolyPair
  | RatFnContents;

/** Why a generic operation could not answer: the book's "No method for
 * these types", or polynomials handed to a procedure that insists on
 * one indeterminate. */
export type GenError =
  | { readonly _tag: "NoMethod"; readonly op: string; readonly tags: ReadonlyArray<string> }
  | {
      readonly _tag: "NotSameVar";
      readonly proc: string;
      readonly left: string;
      readonly right: string;
    };

const noMethod = (op: string, tags: ReadonlyArray<string>): GenError => ({
  _tag: "NoMethod",
  op,
  tags,
});

/** The book's error line for a table miss, rendered. */
export const showNoMethod = (e: Extract<GenError, { _tag: "NoMethod" }>): string =>
  formatNoMethodError(e.op, e.tags);

/** The book's error line for a variable mismatch, rendered. */
export const showNotSameVar = (e: Extract<GenError, { _tag: "NotSameVar" }>): string =>
  `Polys not in same var: ${e.proc}(${e.left}, ${e.right})`;

/** Any system error, rendered the way the book prints it. */
export const showError = (e: GenError): string =>
  e._tag === "NoMethod" ? showNoMethod(e) : showNotSameVar(e);

/** The book's type-tag after 2.78: a bare number names itself
 * `ts-number`; everything else carries its tag. */
export const typeTagOf = (d: ArithDatum): string => (typeof d === "bigint" ? "ts-number" : d._tag);

/** The book's contents after 2.78: a bare number is its own contents. */
export const contentsOf = (d: ArithDatum): ArithContents =>
  typeof d === "bigint" ? d : d.contents;

/**
 * A table entry. The book's table holds procedures of every shape
 * under one regime; the entry carries which shape it is, and each
 * fetcher narrows to the shape it asked for --- the 2.4 table's
 * Selector/Constructor move, generalized to the whole tower.
 */
export type ArithHandler =
  /** An operation applied through `applyGeneric` to bare contents. */
  | {
      readonly _tag: "Op";
      readonly fn: (args: ReadonlyArray<ArithContents>) => Result<ArithDatum, GenError>;
    }
  /** A predicate: `equ?` and `=zero?`. */
  | {
      readonly _tag: "Pred";
      readonly fn: (args: ReadonlyArray<ArithContents>) => Result<boolean, GenError>;
    }
  /** A representation-level selector over the bare complex pair. */
  | { readonly _tag: "RepSelector"; readonly fn: (z: ComplexPair) => number }
  /** The complex-level selector of exercise 2.77: one tag down. */
  | { readonly _tag: "Selector"; readonly fn: (z: Rectangular | Polar) => Result<number, GenError> }
  /** The TypeScript-number constructor. */
  | { readonly _tag: "MakeNumber"; readonly fn: (n: bigint) => TsNumber }
  /** The rational constructor. */
  | { readonly _tag: "MakeRational"; readonly fn: (n: bigint, d: bigint) => Rational }
  /** The representation-level complex constructor from real and
   * imaginary parts: answers the inner tagged representation. */
  | {
      readonly _tag: "MakeInnerRealImag";
      readonly fn: (x: number, y: number) => Result<Rectangular | Polar, GenError>;
    }
  /** The representation-level complex constructor from magnitude and
   * angle. */
  | {
      readonly _tag: "MakeInnerMagAng";
      readonly fn: (r: number, a: number) => Result<Rectangular | Polar, GenError>;
    }
  /** The complex-level constructor from real and imaginary parts. */
  | {
      readonly _tag: "MakeComplexRealImag";
      readonly fn: (x: number, y: number) => Result<ComplexNumber, GenError>;
    }
  /** The complex-level constructor from magnitude and angle. */
  | {
      readonly _tag: "MakeComplexMagAng";
      readonly fn: (r: number, a: number) => Result<ComplexNumber, GenError>;
    }
  /** The polynomial constructor. */
  | { readonly _tag: "MakePoly"; readonly fn: (variable: string, terms: TermList) => Polynomial };

/** Wraps a datum operation as a table entry. */
export const op = (
  fn: (args: ReadonlyArray<ArithContents>) => Result<ArithDatum, GenError>,
): ArithHandler => ({
  _tag: "Op",
  fn,
});

/** Wraps a predicate as a table entry. */
export const pred = (
  fn: (args: ReadonlyArray<ArithContents>) => Result<boolean, GenError>,
): ArithHandler => ({
  _tag: "Pred",
  fn,
});

/** The system's operation-and-type table: one table, every package. */
const arithTable: OpTable<ArithHandler> = makeOpTable();

/** The book's put, bound to the system's table. */
export const put = (op: string, tags: ReadonlyArray<string>, item: ArithHandler): void => {
  putIn(arithTable, op, tags, item);
};

/** The book's get, bound to the system's table: the entry under `op`
 * and the ordered tag list, or nothing. */
export const get = (op: string, tags: ReadonlyArray<string>): Option<ArithHandler> =>
  getIn(arithTable, op, tags);

const getOp = (
  op: string,
  tags: ReadonlyArray<string>,
): Option<(args: ReadonlyArray<ArithContents>) => Result<ArithDatum, GenError>> => {
  const e = getIn(arithTable, op, tags);
  return e._tag === "Some" && e.value._tag === "Op" ? some(e.value.fn) : none;
};

const getPred = (
  op: string,
  tags: ReadonlyArray<string>,
): Option<(args: ReadonlyArray<ArithContents>) => Result<boolean, GenError>> => {
  const e = getIn(arithTable, op, tags);
  return e._tag === "Some" && e.value._tag === "Pred" ? some(e.value.fn) : none;
};

/** The selector under `op` for `z`'s tag, applied to its contents: the
 * representation-level entries take the bare pair, the complex-level
 * entries of exercise 2.77 take the inner tagged datum. */
const selectorAt = (op: string, z: ArithDatum): Result<number, GenError> => {
  const tags = [typeTagOf(z)];
  const e = getIn(arithTable, op, tags);
  if (
    typeof z !== "bigint" &&
    z._tag === "complex" &&
    e._tag === "Some" &&
    e.value._tag === "Selector"
  ) {
    return e.value.fn(z.contents);
  }
  return err(noMethod(op, tags));
};

/** The generic real-part: the 2.4.3 spelling over this system's table. */
export const realPart = (z: ArithDatum): Result<number, GenError> => selectorAt("real-part", z);

/** The generic imag-part. */
export const imagPart = (z: ArithDatum): Result<number, GenError> => selectorAt("imag-part", z);

/** The generic magnitude. */
export const magnitude = (z: ArithDatum): Result<number, GenError> => selectorAt("magnitude", z);

/** The generic angle. */
export const angle = (z: ArithDatum): Result<number, GenError> => selectorAt("angle", z);

/** Exercise 2.78's ordinary number has no runtime wrapper. */
export const attachTagTsNumber = (value: bigint): TsNumber => value;

/** Exercise 2.79's ordinary-number equality. */
export const equTsNumber = (left: TsNumber, right: TsNumber): boolean => left === right;

/** Exercise 2.80's ordinary-number zero test. */
export const isZeroTsNumber = (value: TsNumber): boolean => value === 0n;

/** Lifts a primitive bigint operation into a TypeScript-number entry. The
 * division truncates toward zero: the exactness boundary remains explicit. */
const bigintOp = (name: string, f: (x: bigint, y: bigint) => bigint): ArithHandler =>
  op((args) => {
    const x = args[0];
    const y = args[1];
    if (typeof x !== "bigint" || typeof y !== "bigint") {
      return err(noMethod(name, ["ts-number", "ts-number"]));
    }
    return ok(f(x, y));
  });

/** Installs the ordinary-number package: the language's own arithmetic
 * under the `ts-number` tag, exponentiation as in the 2.81
 * discussion, and the constructor. */
export const installTsNumberPackage = (): void => {
  put(
    "add",
    ["ts-number", "ts-number"],
    bigintOp("add", (x, y) => x + y),
  );
  put(
    "sub",
    ["ts-number", "ts-number"],
    bigintOp("sub", (x, y) => x - y),
  );
  put(
    "mul",
    ["ts-number", "ts-number"],
    bigintOp("mul", (x, y) => x * y),
  );
  put(
    "div",
    ["ts-number", "ts-number"],
    bigintOp("div", (x, y) => x / y),
  );
  put(
    "exp",
    ["ts-number", "ts-number"],
    bigintOp("exp", (x, y) => x ** y),
  );
  put("equ?", ["ts-number", "ts-number"], bigintPred("equ?", equTsNumber));
  put("=zero?", ["ts-number"], zeroBigintPred());
  put("make", ["ts-number"], { _tag: "MakeNumber", fn: attachTagTsNumber });
};

/** Lifts a bigint predicate into a two-argument ts-number entry. */
const bigintPred = (name: string, f: (x: bigint, y: bigint) => boolean): ArithHandler =>
  pred((args) => {
    const x = args[0];
    const y = args[1];
    if (typeof x !== "bigint" || typeof y !== "bigint") {
      return err(noMethod(name, ["ts-number", "ts-number"]));
    }
    return ok(f(x, y));
  });

/** The ts-number =zero? entry. */
const zeroBigintPred = (): ArithHandler =>
  pred((args) => {
    const x = args[0];
    if (typeof x !== "bigint") {
      return err(noMethod("=zero?", ["ts-number"]));
    }
    return ok(isZeroTsNumber(x));
  });

/** The book's make-ts-number: the (untagged, after 2.78) ordinary
 * number through the table's constructor. The entry is present because
 * the package is installed with the system; a missing entry is a
 * system inconsistency, the book's error on a get it cannot survive. */
export const makeTsNumber = (n: bigint): TsNumber => {
  const e = getIn(arithTable, "make", ["ts-number"]);
  if (e._tag === "Some" && e.value._tag === "MakeNumber") {
    return e.value.fn(n);
  }
  throw new Error("make: no constructor for ts-number");
};

/** Euclid's gcd over exact integers, on absolute values: the 1.2.5
 * algorithm the section reuses for rationals and for term lists. */
export const gcdInteger = (a: bigint, b: bigint): bigint => {
  const x = a < 0n ? -a : a;
  const y = b < 0n ? -b : b;
  return y === 0n ? x : gcdInteger(y, x % y);
};

/** The rational contents behind a table argument: the only array shape
 * a rational entry can meet. */
const isRatContents = (c: ArithContents | undefined): c is RatContents =>
  Array.isArray(c) && typeof c[0] === "bigint";

/** Lifts a rational-pair operation into a rational entry. */
const ratOp = (
  name: string,
  f: (x: RatContents, y: RatContents) => Result<ArithDatum, GenError>,
): ArithHandler =>
  op((args) => {
    const x = args[0];
    const y = args[1];
    if (!isRatContents(x) || !isRatContents(y)) {
      return err(noMethod(name, ["rational", "rational"]));
    }
    return f(x, y);
  });

/** Installs the rational package: the 2.1.1 code, unmodified, as the
 * internal procedures, gcd-reducing at construction. */
export const installRationalPackage = (): void => {
  const numer = (x: RatContents): bigint => x[0];
  const denom = (x: RatContents): bigint => x[1];
  const makeRat = (n: bigint, d: bigint): RatContents => {
    const g = gcdInteger(n, d);
    return [n / g, d / g];
  };
  const addRat = (x: RatContents, y: RatContents): RatContents =>
    makeRat(numer(x) * denom(y) + numer(y) * denom(x), denom(x) * denom(y));
  const subRat = (x: RatContents, y: RatContents): RatContents =>
    makeRat(numer(x) * denom(y) - numer(y) * denom(x), denom(x) * denom(y));
  const mulRat = (x: RatContents, y: RatContents): RatContents =>
    makeRat(numer(x) * numer(y), denom(x) * denom(y));
  const divRat = (x: RatContents, y: RatContents): RatContents =>
    makeRat(numer(x) * denom(y), denom(x) * numer(y));
  const tag = (x: RatContents): Rational => attachTag("rational", x);
  const rat2 = (name: string, f: (x: RatContents, y: RatContents) => RatContents): ArithHandler =>
    ratOp(name, (x, y) => ok(tag(f(x, y))));
  put("add", ["rational", "rational"], rat2("add", addRat));
  put("sub", ["rational", "rational"], rat2("sub", subRat));
  put("mul", ["rational", "rational"], rat2("mul", mulRat));
  put("div", ["rational", "rational"], rat2("div", divRat));
  put(
    "equ?",
    ["rational", "rational"],
    pred((args) => {
      const x = args[0];
      const y = args[1];
      if (!isRatContents(x) || !isRatContents(y)) {
        return err(noMethod("equ?", ["rational", "rational"]));
      }
      return ok(numer(x) * denom(y) === numer(y) * denom(x));
    }),
  );
  put(
    "=zero?",
    ["rational"],
    pred((args) => {
      const x = args[0];
      if (!isRatContents(x)) {
        return err(noMethod("=zero?", ["rational"]));
      }
      return ok(numer(x) === 0n);
    }),
  );
  put("make", ["rational"], { _tag: "MakeRational", fn: (n, d) => tag(makeRat(n, d)) });
};

/** The book's make-rational: the tagged rational in lowest terms. */
export const makeRational = (n: bigint, d: bigint): Rational => {
  const e = getIn(arithTable, "make", ["rational"]);
  if (e._tag === "Some" && e.value._tag === "MakeRational") {
    return e.value.fn(n, d);
  }
  throw new Error(showError(noMethod("make", ["rational"])));
};

/** Installs Ben's rectangular entries: the 2.4.1 selectors over the
 * bare pair, the constructors under their own tags. */
export const installRectangularPackage = (): void => {
  put("real-part", ["rectangular"], { _tag: "RepSelector", fn: (z) => z[0] });
  put("imag-part", ["rectangular"], { _tag: "RepSelector", fn: (z) => z[1] });
  put("magnitude", ["rectangular"], {
    _tag: "RepSelector",
    fn: (z) => Math.sqrt(z[0] * z[0] + z[1] * z[1]),
  });
  put("angle", ["rectangular"], { _tag: "RepSelector", fn: (z) => Math.atan2(z[1], z[0]) });
  put("make-from-real-imag", ["rectangular"], {
    _tag: "MakeInnerRealImag",
    fn: (x, y) => {
      const pair: ComplexPair = [x, y];
      return ok(attachTag("rectangular", pair));
    },
  });
  put("make-from-mag-ang", ["rectangular"], {
    _tag: "MakeInnerMagAng",
    fn: (r, a) => {
      const pair: ComplexPair = [r * Math.cos(a), r * Math.sin(a)];
      return ok(attachTag("rectangular", pair));
    },
  });
};

/** Installs Alyssa's polar entries. */
export const installPolarPackage = (): void => {
  put("real-part", ["polar"], { _tag: "RepSelector", fn: (z) => z[0] * Math.cos(z[1]) });
  put("imag-part", ["polar"], { _tag: "RepSelector", fn: (z) => z[0] * Math.sin(z[1]) });
  put("magnitude", ["polar"], { _tag: "RepSelector", fn: (z) => z[0] });
  put("angle", ["polar"], { _tag: "RepSelector", fn: (z) => z[1] });
  put("make-from-real-imag", ["polar"], {
    _tag: "MakeInnerRealImag",
    fn: (x, y) => {
      const pair: ComplexPair = [Math.sqrt(x * x + y * y), Math.atan2(y, x)];
      return ok(attachTag("polar", pair));
    },
  });
  put("make-from-mag-ang", ["polar"], {
    _tag: "MakeInnerMagAng",
    fn: (r, a) => {
      const pair: ComplexPair = [r, a];
      return ok(attachTag("polar", pair));
    },
  });
};

// The 2.4.1 arithmetic over the inner tagged representations, the
// complex package's internal procedures.

const repRealPart = (z: Rectangular | Polar): number =>
  z._tag === "rectangular" ? z.contents[0] : z.contents[0] * Math.cos(z.contents[1]);

const repImagPart = (z: Rectangular | Polar): number =>
  z._tag === "rectangular" ? z.contents[1] : z.contents[0] * Math.sin(z.contents[1]);

const repMagnitude = (z: Rectangular | Polar): number =>
  z._tag === "rectangular"
    ? Math.sqrt(z.contents[0] * z.contents[0] + z.contents[1] * z.contents[1])
    : z.contents[0];

const repAngle = (z: Rectangular | Polar): number =>
  z._tag === "rectangular" ? Math.atan2(z.contents[1], z.contents[0]) : z.contents[1];

const isComplexContents = (c: ArithContents): c is Rectangular | Polar => {
  if (typeof c !== "object" || Array.isArray(c) || !("contents" in c)) {
    return false;
  }
  return c._tag === "rectangular" || c._tag === "polar";
};

/** The complex-level selector of exercise 2.77: dispatch one tag down
 * into the representation packages. */
export const repSelector =
  (op: string) =>
  (z: Rectangular | Polar): Result<number, GenError> => {
    const e = getIn(arithTable, op, [z._tag]);
    if (e._tag === "Some" && e.value._tag === "RepSelector") {
      return ok(e.value.fn(z.contents));
    }
    return err(noMethod(op, [z._tag]));
  };

/** Extracts the inner representation from a complex entry's argument. */
const complexContents = (
  c: ArithContents | undefined,
  op: string,
): Result<Rectangular | Polar, GenError> =>
  c !== undefined && isComplexContents(c) ? ok(c) : err(noMethod(op, ["complex"]));

const mapGen = <A, B>(r: Result<A, GenError>, f: (a: A) => B): Result<B, GenError> =>
  r._tag === "Ok" ? ok(f(r.value)) : r;

const bindGen = <A, B>(
  r: Result<A, GenError>,
  f: (a: A) => Result<B, GenError>,
): Result<B, GenError> => (r._tag === "Ok" ? f(r.value) : r);

const complexOp = (
  name: string,
  f: (z1: Rectangular | Polar, z2: Rectangular | Polar) => Result<ArithDatum, GenError>,
): ArithHandler =>
  op((args) => {
    const c1 = complexContents(args[0], name);
    if (c1._tag === "Error") {
      return c1;
    }
    const c2 = complexContents(args[1], name);
    if (c2._tag === "Error") {
      return c2;
    }
    return f(c1.value, c2.value);
  });

/** Installs the complex package: the 2.4.1 arithmetic as internals, the
 * constructors imported from rectangular and polar through the table,
 * and --- exercise 2.77 --- the complex-level selectors. */
export const installComplexPackage = (): void => {
  // imported procedures from rectangular and polar packages
  const makeFromRealImagT = (x: number, y: number): Result<Rectangular | Polar, GenError> => {
    const e = getIn(arithTable, "make-from-real-imag", ["rectangular"]);
    if (e._tag === "Some" && e.value._tag === "MakeInnerRealImag") {
      return e.value.fn(x, y);
    }
    return err(noMethod("make-from-real-imag", ["rectangular"]));
  };
  const makeFromMagAngT = (r: number, a: number): Result<Rectangular | Polar, GenError> => {
    const e = getIn(arithTable, "make-from-mag-ang", ["polar"]);
    if (e._tag === "Some" && e.value._tag === "MakeInnerMagAng") {
      return e.value.fn(r, a);
    }
    return err(noMethod("make-from-mag-ang", ["polar"]));
  };
  // internal procedures
  const addComplex = (
    z1: Rectangular | Polar,
    z2: Rectangular | Polar,
  ): Result<Rectangular | Polar, GenError> =>
    makeFromRealImagT(repRealPart(z1) + repRealPart(z2), repImagPart(z1) + repImagPart(z2));
  const subComplex = (
    z1: Rectangular | Polar,
    z2: Rectangular | Polar,
  ): Result<Rectangular | Polar, GenError> =>
    makeFromRealImagT(repRealPart(z1) - repRealPart(z2), repImagPart(z1) - repImagPart(z2));
  const mulComplex = (
    z1: Rectangular | Polar,
    z2: Rectangular | Polar,
  ): Result<Rectangular | Polar, GenError> =>
    makeFromMagAngT(repMagnitude(z1) * repMagnitude(z2), repAngle(z1) + repAngle(z2));
  const divComplex = (
    z1: Rectangular | Polar,
    z2: Rectangular | Polar,
  ): Result<Rectangular | Polar, GenError> =>
    makeFromMagAngT(repMagnitude(z1) / repMagnitude(z2), repAngle(z1) - repAngle(z2));
  // interface to rest of the system
  const tag = (z: Rectangular | Polar): ComplexNumber => attachTag("complex", z);
  const tagIn = (r: Result<Rectangular | Polar, GenError>): Result<ArithDatum, GenError> =>
    mapGen(r, tag);
  put(
    "add",
    ["complex", "complex"],
    complexOp("add", (z1, z2) => tagIn(addComplex(z1, z2))),
  );
  put(
    "sub",
    ["complex", "complex"],
    complexOp("sub", (z1, z2) => tagIn(subComplex(z1, z2))),
  );
  put(
    "mul",
    ["complex", "complex"],
    complexOp("mul", (z1, z2) => tagIn(mulComplex(z1, z2))),
  );
  put(
    "div",
    ["complex", "complex"],
    complexOp("div", (z1, z2) => tagIn(divComplex(z1, z2))),
  );
  put(
    "equ?",
    ["complex", "complex"],
    pred((args) => {
      const c1 = complexContents(args[0], "equ?");
      if (c1._tag === "Error") {
        return err(c1.error);
      }
      const c2 = complexContents(args[1], "equ?");
      if (c2._tag === "Error") {
        return err(c2.error);
      }
      return ok(
        repRealPart(c1.value) === repRealPart(c2.value) &&
          repImagPart(c1.value) === repImagPart(c2.value),
      );
    }),
  );
  put(
    "=zero?",
    ["complex"],
    pred((args) => {
      const c = complexContents(args[0], "=zero?");
      if (c._tag === "Error") {
        return err(c.error);
      }
      return ok(repRealPart(c.value) === 0 && repImagPart(c.value) === 0);
    }),
  );
  // exercise 2.77: the complex-number selectors, so that magnitude and
  // friends dispatch through the inner tag
  put("real-part", ["complex"], { _tag: "Selector", fn: repSelector("real-part") });
  put("imag-part", ["complex"], { _tag: "Selector", fn: repSelector("imag-part") });
  put("magnitude", ["complex"], { _tag: "Selector", fn: repSelector("magnitude") });
  put("angle", ["complex"], { _tag: "Selector", fn: repSelector("angle") });
  put("make-from-real-imag", ["complex"], {
    _tag: "MakeComplexRealImag",
    fn: (x, y) => mapGen(makeFromRealImagT(x, y), tag),
  });
  put("make-from-mag-ang", ["complex"], {
    _tag: "MakeComplexMagAng",
    fn: (r, a) => mapGen(makeFromMagAngT(r, a), tag),
  });
};

/** The book's make-complex-from-real-imag. */
export const makeComplexFromRealImag = (x: number, y: number): ComplexNumber => {
  const e = getIn(arithTable, "make-from-real-imag", ["complex"]);
  if (e._tag === "Some" && e.value._tag === "MakeComplexRealImag") {
    const r = e.value.fn(x, y);
    if (r._tag === "Ok") {
      return r.value;
    }
    throw new Error(showError(r.error));
  }
  throw new Error(showError(noMethod("make-from-real-imag", ["complex"])));
};

/** The book's make-complex-from-mag-ang. */
export const makeComplexFromMagAng = (r: number, a: number): ComplexNumber => {
  const e = getIn(arithTable, "make-from-mag-ang", ["complex"]);
  if (e._tag === "Some" && e.value._tag === "MakeComplexMagAng") {
    const z = e.value.fn(r, a);
    if (z._tag === "Ok") {
      return z.value;
    }
    throw new Error(showError(z.error));
  }
  throw new Error(showError(noMethod("make-from-mag-ang", ["complex"])));
};

/** Installs every 2.5.1 package, in dependency order: the system the
 * section's prose assumes whenever it evaluates an interaction. */
export const installGenericArithmetic = (): void => {
  installTsNumberPackage();
  installRationalPackage();
  installRectangularPackage();
  installPolarPackage();
  installComplexPackage();
};

// ---------------------------------------------------------------------
// 2.5.2 Combining Data of Different Types
// ---------------------------------------------------------------------

/** A coercion: one datum viewed as another type. */
export type CoercionFn = (d: ArithDatum) => Result<ArithDatum, GenError>;

const coercionTable: OpTable<CoercionFn> = makeOpTable();

/** The book's put-coercion: installs `fn` under the two type names. */
export const putCoercion = (from: string, to: string, fn: CoercionFn): void => {
  putIn(coercionTable, from, [to], fn);
};

/** The book's get-coercion: the coercion from one type to another, or
 * nothing. */
export const getCoercion = (from: string, to: string): Option<CoercionFn> =>
  getIn(coercionTable, from, [to]);

/** The identity coercion for the ordinary-number representation. */
export const tsNumberToTsNumber: CoercionFn = (value) =>
  typeof value === "bigint"
    ? ok(attachTagTsNumber(value))
    : err(noMethod("ts-number->ts-number", [typeTagOf(value)]));

/** The book's coercion from an ordinary number to a complex number with
 * that real part and zero imaginary part. */
export const tsNumberToComplex: CoercionFn = (n) => {
  const c = contentsOf(n);
  if (typeof c !== "bigint") {
    return err(noMethod("ts-number->complex", [typeTagOf(n)]));
  }
  return ok(makeComplexFromRealImag(Number(c), 0));
};

putCoercion("ts-number", "ts-number", tsNumberToTsNumber);
putCoercion("ts-number", "complex", tsNumberToComplex);
/** The book's explicit cross-type operation: complex plus ordinary. */
export const addComplexToTsNumber = (
  z: ComplexNumber,
  x: bigint,
): Result<ComplexNumber, GenError> => {
  const re = realPart(z);
  if (re._tag === "Error") {
    return re;
  }
  const im = imagPart(z);
  if (im._tag === "Error") {
    return im;
  }
  return ok(makeComplexFromRealImag(re.value + Number(x), im.value));
};

/** The book's apply-generic after the coercion extension: try the
 * table; then, for two arguments, coerce the first toward the second
 * and retry, then the second toward the first; then give up. */
export const applyGeneric = (
  opName: string,
  ...args: ReadonlyArray<ArithDatum>
): Result<ArithDatum, GenError> => {
  const tags = args.map(typeTagOf);
  const proc = getOp(opName, tags);
  if (proc._tag === "Some") {
    return proc.value(args.map(contentsOf));
  }
  if (args.length === 2) {
    const a1 = args[0];
    const a2 = args[1];
    const type1 = tags[0] ?? "";
    const type2 = tags[1] ?? "";
    if (a1 !== undefined && a2 !== undefined && type1 !== type2) {
      const t1toT2 = getCoercion(type1, type2);
      if (t1toT2._tag === "Some") {
        return bindGen(t1toT2.value(a1), (coerced) => applyGeneric(opName, coerced, a2));
      }
      const t2toT1 = getCoercion(type2, type1);
      if (t2toT1._tag === "Some") {
        return bindGen(t2toT1.value(a2), (coerced) => applyGeneric(opName, a1, coerced));
      }
    }
  }
  return err(noMethod(opName, tags));
};

/** The book's add: generic over every installed package. */
export const add = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("add", x, y);

/** The book's sub. */
export const sub = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("sub", x, y);

/** The book's mul. */
export const mul = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("mul", x, y);

/** The book's div. */
export const div = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("div", x, y);

/** The book's exp, installed for ordinary numbers. */
export const exp = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("exp", x, y);

/** Exercise 2.79's generic equality predicate. */
export const equQ = (x: ArithDatum, y: ArithDatum): Result<boolean, GenError> => {
  const tags = [typeTagOf(x), typeTagOf(y)];
  const f = getPred("equ?", tags);
  if (f._tag === "None") {
    return err(noMethod("equ?", tags));
  }
  return f.value([contentsOf(x), contentsOf(y)]);
};

/** Exercise 2.80's generic =zero?. */
export const isZeroQ = (x: ArithDatum): Result<boolean, GenError> => {
  const tags = [typeTagOf(x)];
  const f = getPred("=zero?", tags);
  if (f._tag === "None") {
    return err(noMethod("=zero?", tags));
  }
  return f.value([contentsOf(x)]);
};

// ---------------------------------------------------------------------
// 2.5.3 Example: Symbolic Algebra
// ---------------------------------------------------------------------

/** The book's make-poly. */
export const makePoly = (variable: string, terms: TermList): Poly => ({ variable, terms });

/** The book's variable selector. */
export const variableOf = (p: Poly): string => p.variable;

/** The book's term-list selector. */
export const termListOf = (p: Poly): TermList => p.terms;

/** The book's same-variable?, on the variable names. */
export const sameVariableQ = (v1: string, v2: string): boolean => v1 === v2;

const isPolyContents = (c: ArithContents): c is Poly => {
  if (typeof c !== "object" || Array.isArray(c) || !("variable" in c)) {
    return false;
  }
  return typeof c.variable === "string";
};

/** The book's the-empty-termlist. */
export const theEmptyTermList = (): TermList => [];

/** The book's empty-termlist?. */
export const isEmptyTermListQ = (l: TermList): boolean => l.length === 0;

/** The book's make-term. */
export const makeTerm = (order: bigint, coeff: ArithDatum): Term => [order, coeff];

/** The book's order selector. */
export const orderOf = (t: Term): bigint => t[0];

/** The book's coeff selector. */
export const coeffOf = (t: Term): ArithDatum => t[1];

/** The book's first-term: the highest-order term. Called only on a
 * nonempty list, per the term-list discipline; an empty list is the
 * book's car of (), a loud contract failure. */
export const firstTerm = (l: TermList): Term => {
  const t = l[0];
  if (t === undefined) {
    throw new Error("first-term: empty term list");
  }
  return t;
};

/** The book's rest-terms: all but the highest-order term. */
export const restTerms = (l: TermList): TermList => l.slice(1);

/** The book's adjoin-term: skips zero coefficients, using the 2.80
 * =zero?, so a polynomial coefficient's zero is a zero polynomial. */
export const adjoinTerm = (t: Term, l: TermList): TermList => {
  const z = isZeroQ(coeffOf(t));
  return z._tag === "Ok" && z.value ? l : [t, ...l];
};

/** The term-list merge behind add-terms: the book's ordered union-set
 * of 2.62, the coefficients combined through the generic add. */
const combineTerms = (l1: TermList, l2: TermList): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l1)) {
    return ok(l2);
  }
  if (isEmptyTermListQ(l2)) {
    return ok(l1);
  }
  const t1 = firstTerm(l1);
  const t2 = firstTerm(l2);
  const o1 = orderOf(t1);
  const o2 = orderOf(t2);
  if (o1 > o2) {
    return mapGen(combineTerms(restTerms(l1), l2), (rest) => adjoinTerm(t1, rest));
  }
  if (o1 < o2) {
    return mapGen(combineTerms(l1, restTerms(l2)), (rest) => adjoinTerm(t2, rest));
  }
  return bindGen(applyGeneric("add", coeffOf(t1), coeffOf(t2)), (c) =>
    mapGen(combineTerms(restTerms(l1), restTerms(l2)), (rest) => adjoinTerm(makeTerm(o1, c), rest)),
  );
};

/** The book's add-terms: the coefficients combine through the generic
 * add, the source of the polynomial package's data-directed recursion. */
export const addTerms = (l1: TermList, l2: TermList): Result<TermList, GenError> =>
  combineTerms(l1, l2);

/** Negates every coefficient of a term list (each through the generic
 * mul by -1): the negation operation exercise 2.88 installs
 * package-wide, spelled here as the helper the long division of 2.91
 * needs. */
const negateTerms = (l: TermList): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l)) {
    return ok(theEmptyTermList());
  }
  const t = firstTerm(l);
  return bindGen(applyGeneric("mul", coeffOf(t), makeTsNumber(-1n)), (c) =>
    mapGen(negateTerms(restTerms(l)), (rest) => adjoinTerm(makeTerm(orderOf(t), c), rest)),
  );
};

/** Term-list subtraction: add-terms with the subtrahend negated, so a
 * term carried down from the subtrahend flips sign with its
 * coefficient. */
const subTerms = (l1: TermList, l2: TermList): Result<TermList, GenError> =>
  bindGen(negateTerms(l2), (negated) => addTerms(l1, negated));

/** The book's mul-term-by-all-terms. */
export const mulTermByAllTerms = (t1: Term, l: TermList): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l)) {
    return ok(theEmptyTermList());
  }
  const t2 = firstTerm(l);
  return bindGen(applyGeneric("mul", coeffOf(t1), coeffOf(t2)), (c) =>
    mapGen(mulTermByAllTerms(t1, restTerms(l)), (rest) =>
      adjoinTerm(makeTerm(orderOf(t1) + orderOf(t2), c), rest),
    ),
  );
};

/** The book's mul-terms. */
export const mulTerms = (l1: TermList, l2: TermList): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l1)) {
    return ok(theEmptyTermList());
  }
  return bindGen(mulTermByAllTerms(firstTerm(l1), l2), (product) =>
    bindGen(mulTerms(restTerms(l1), l2), (rest) => addTerms(product, rest)),
  );
};

const polyPairShow = (p: Poly, q: Poly): readonly [string, string] => [
  showArithDatum(attachTag("polynomial", p)),
  showArithDatum(attachTag("polynomial", q)),
];

const notSameVar = (proc: string, p1: Poly, p2: Poly): GenError => {
  const shown = polyPairShow(p1, p2);
  return { _tag: "NotSameVar", proc, left: shown[0], right: shown[1] };
};

/** The book's add-poly. */
export const addPoly = (p1: Poly, p2: Poly): Result<Poly, GenError> =>
  sameVariableQ(variableOf(p1), variableOf(p2))
    ? mapGen(addTerms(termListOf(p1), termListOf(p2)), (terms) => makePoly(variableOf(p1), terms))
    : err(notSameVar("add", p1, p2));

/** The book's mul-poly. */
export const mulPoly = (p1: Poly, p2: Poly): Result<Poly, GenError> =>
  sameVariableQ(variableOf(p1), variableOf(p2))
    ? mapGen(mulTerms(termListOf(p1), termListOf(p2)), (terms) => makePoly(variableOf(p1), terms))
    : err(notSameVar("mul", p1, p2));

/** The book's div-terms, the 2.91 exercise's blank filled in: long
 * division on term lists, answering the quotient and the remainder. */
export const divTerms = (
  l1: TermList,
  l2: TermList,
): Result<readonly [TermList, TermList], GenError> => {
  if (isEmptyTermListQ(l1)) {
    return ok([theEmptyTermList(), theEmptyTermList()]);
  }
  const t1 = firstTerm(l1);
  const t2 = firstTerm(l2);
  if (orderOf(t2) > orderOf(t1)) {
    return ok([theEmptyTermList(), l1]);
  }
  return bindGen(applyGeneric("div", coeffOf(t1), coeffOf(t2)), (newC) => {
    const newO = orderOf(t1) - orderOf(t2);
    const quotientTerm: TermList = [makeTerm(newO, newC)];
    return bindGen(mulTerms(l2, quotientTerm), (product) =>
      bindGen(subTerms(l1, product), (difference) =>
        bindGen(divTerms(difference, l2), (rest) => {
          const qr: readonly [TermList, TermList] = [
            adjoinTerm(makeTerm(newO, newC), rest[0]),
            rest[1],
          ];
          return ok(qr);
        }),
      ),
    );
  });
};

/** The book's div-poly on the model of add-poly: the two polys must
 * share their indeterminate; the answer is the quotient and remainder
 * polys, in that order. */
export const divPoly = (p1: Poly, p2: Poly): Result<readonly [Poly, Poly], GenError> => {
  if (!sameVariableQ(variableOf(p1), variableOf(p2))) {
    return err(notSameVar("div", p1, p2));
  }
  return bindGen(divTerms(termListOf(p1), termListOf(p2)), (qr) => {
    const pair: readonly [Poly, Poly] = [
      makePoly(variableOf(p1), qr[0]),
      makePoly(variableOf(p1), qr[1]),
    ];
    return ok(pair);
  });
};

/** The book's remainder-terms: the remainder component of div-terms. */
export const remainderTerms = (l1: TermList, l2: TermList): Result<TermList, GenError> =>
  mapGen(divTerms(l1, l2), (qr) => qr[1]);

/** The book's gcd-terms: Euclid's Algorithm over term lists. With the
 * edition's truncating integer division this is exact only when the
 * divisions are --- the situation exercise 2.95 examines. */
export const gcdTerms = (a: TermList, b: TermList): Result<TermList, GenError> =>
  isEmptyTermListQ(b) ? ok(a) : bindGen(remainderTerms(a, b), (r) => gcdTerms(b, r));

/** The book's gcd-poly: the polynomial GCD of two polys in one
 * variable. */
export const gcdPoly = (p1: Poly, p2: Poly): Result<Poly, GenError> =>
  sameVariableQ(variableOf(p1), variableOf(p2))
    ? mapGen(gcdTerms(termListOf(p1), termListOf(p2)), (terms) => makePoly(variableOf(p1), terms))
    : err(notSameVar("GCD-POLY", p1, p2));

/** A polynomial =zero?: the 2.87 exercise, all coefficients zero. */
const isZeroQTerms = (l: TermList): Result<boolean, GenError> =>
  isEmptyTermListQ(l)
    ? ok(true)
    : bindGen(isZeroQ(coeffOf(firstTerm(l))), (z) => (z ? isZeroQTerms(restTerms(l)) : ok(false)));

/** The poly argument behind a polynomial entry. */
const polyArg = (
  c: ArithContents | undefined,
  op: string,
  tags: ReadonlyArray<string>,
): Result<Poly, GenError> =>
  c !== undefined && isPolyContents(c) ? ok(c) : err(noMethod(op, tags));

/** Lifts a two-poly operation into a polynomial entry. */
const poly2 = (
  name: string,
  f: (p1: Poly, p2: Poly) => Result<ArithDatum, GenError>,
): ArithHandler =>
  op((args) => {
    const p1 = polyArg(args[0], name, ["polynomial", "polynomial"]);
    if (p1._tag === "Error") {
      return p1;
    }
    const p2 = polyArg(args[1], name, ["polynomial", "polynomial"]);
    if (p2._tag === "Error") {
      return p2;
    }
    return f(p1.value, p2.value);
  });

/** Installs the polynomial package: add-poly and mul-poly as the
 * polynomial add and mul, division and the GCD family from the
 * exercises (2.91, 2.94), the 2.87 =zero?, and the constructor. */
export const installPolynomialPackage = (): void => {
  const tag = (p: Poly): Polynomial => attachTag("polynomial", p);
  put(
    "add",
    ["polynomial", "polynomial"],
    poly2("add", (p1, p2) => mapGen(addPoly(p1, p2), tag)),
  );
  put(
    "mul",
    ["polynomial", "polynomial"],
    poly2("mul", (p1, p2) => mapGen(mulPoly(p1, p2), tag)),
  );
  put(
    "div",
    ["polynomial", "polynomial"],
    poly2("div", (p1, p2) =>
      mapGen(divPoly(p1, p2), (pair) => attachTag("quotient-remainder", pair)),
    ),
  );
  put(
    "greatest-common-divisor",
    ["polynomial", "polynomial"],
    poly2("greatest-common-divisor", (p1, p2) => mapGen(gcdPoly(p1, p2), tag)),
  );
  put(
    "greatest-common-divisor",
    ["ts-number", "ts-number"],
    bigintOp("greatest-common-divisor", gcdInteger),
  );
  put(
    "=zero?",
    ["polynomial"],
    pred((args) => {
      const p = polyArg(args[0], "=zero?", ["polynomial"]);
      if (p._tag === "Error") {
        return err(p.error);
      }
      return isZeroQTerms(p.value.terms);
    }),
  );
  put("make", ["polynomial"], {
    _tag: "MakePoly",
    fn: (variable, terms) => tag(makePoly(variable, terms)),
  });
};

/** The book's make-polynomial through the table's constructor. */
export const makePolynomial = (variable: string, terms: TermList): Polynomial => {
  const e = getIn(arithTable, "make", ["polynomial"]);
  if (e._tag === "Some" && e.value._tag === "MakePoly") {
    return e.value.fn(variable, terms);
  }
  throw new Error(showError(noMethod("make", ["polynomial"])));
};

/** The book's greatest-common-divisor, generic over polynomials and
 * ordinary numbers. */
export const greatestCommonDivisor = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("greatest-common-divisor", x, y);

// ---------------------------------------------------------------------
// The printed form: the book's list notation, one renderer for pins
// ---------------------------------------------------------------------

/** Renders a polynomial in bracket-comma datum notation. */
export const showPoly = (p: Poly): string =>
  `[${["polynomial", p.variable, ...p.terms.map((term) => `[${term[0]}, ${showArithDatum(term[1])}]`)].join(", ")}]`;

/** Renders any datum or inner tagged representation in bracket-comma notation. */
export const showArithDatum = (v: ArithDatum | Rectangular | Polar | boolean | number): string => {
  if (typeof v === "bigint") {
    return v.toString();
  }
  if (typeof v === "boolean" || typeof v === "number") {
    return String(v);
  }
  switch (v._tag) {
    case "rational": {
      const contents = v.contents;
      return isRatContents(contents)
        ? `[rational, ${contents[0]}, ${contents[1]}]`
        : `[rational, ${showArithDatum(contents.numer)}, ${showArithDatum(contents.denom)}]`;
    }
    case "real":
      return `[real, ${v.contents}]`;
    case "rectangular":
      return `[rectangular, ${v.contents[0]}, ${v.contents[1]}]`;
    case "polar":
      return `[polar, ${v.contents[0]}, ${v.contents[1]}]`;
    case "complex": {
      const inner = v.contents;
      return inner._tag === "rectangular"
        ? `[complex, rectangular, ${inner.contents[0]}, ${inner.contents[1]}]`
        : `[complex, polar, ${inner.contents[0]}, ${inner.contents[1]}]`;
    }
    case "polynomial":
      return showPoly(v.contents);
    case "quotient-remainder":
      return `[quotient-remainder, ${showPoly(v.contents[0])}, ${showPoly(v.contents[1])}]`;
  }
};

/** Renders a system answer: the value, or the book's error line. */
export const show = (r: Result<ArithDatum | boolean | number, GenError>): string =>
  r._tag === "Ok" ? showArithDatum(r.value) : showError(r.error);

// The running system: every package installed, as the section's
// interactions assume.
installGenericArithmetic();
installPolynomialPackage();
