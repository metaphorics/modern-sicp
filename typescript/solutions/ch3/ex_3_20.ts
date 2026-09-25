// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.20: the book draws environment diagrams to illustrate
 *
 *   (define x (cons 1 2))
 *   (define z (cons x x))
 *   (set-car! (cdr z) 17)
 *   (car x)   =>  17
 *
 * under the procedural implementation of pairs. The edition's
 * replacement for the drawing is a trace: the exercise-map row reads
 * "trace object aliasing via closures" for TypeScript, so the
 * procedural pair is spelled as a closure over two local slots with
 * the book's dispatch, and the aliasing that makes the diagram
 * interesting is recorded as an event log plus the identity and read
 * pins the diagram's sharing question asks. The punchline the drawing
 * shows: z's car and cdr slots hold the same pair object, so
 * set-car! through the cdr handle changes what the car handle reads.
 */

/** The book's procedural pair: a dispatch over two local slots, the
 * `let x` and `let y` of the book's `cons`, closed over by the
 * accessor and mutator member functions. The slots are typed
 * separately, so a pair can hold a pair in either slot, exactly the
 * book's `(cons x x)`. */
export interface MutablePair<A, B> {
  readonly getX: () => A;
  readonly getY: () => B;
  readonly setX: (value: A) => void;
  readonly setY: (value: B) => void;
}

/** Builds the book's procedural `cons`: the dispatch is the pair, its
 * two slots are local state, and mutation goes through the set
 * members the book spells `set-x!` and `set-y!`. */
export const makePair = <A, B>(x: A, y: B): MutablePair<A, B> => {
  let slotX = x;
  let slotY = y;
  return {
    getX: () => slotX,
    getY: () => slotY,
    setX: (value) => {
      slotX = value;
    },
    setY: (value) => {
      slotY = value;
    },
  };
};

/** One recorded step of the traced sequence: the pair created, the
 * aliasing noticed, the slot set through one handle, or the value
 * read back through another. */
export type PairAliasingEvent =
  | { readonly _tag: "PairCreated"; readonly label: string }
  | { readonly _tag: "Aliased"; readonly label: string }
  | { readonly _tag: "SlotSet"; readonly label: string }
  | { readonly _tag: "ReadThroughAlias"; readonly label: string };

/** What the traced sequence answers: the log the drawing is narrated
 * from, the identity facts behind the diagram's shared boxes, and the
 * final reads through each handle. */
export interface PairAliasingReport {
  readonly log: ReadonlyArray<PairAliasingEvent>;
  /** Whether z's car handle and cdr handle give the same object: the
   * book's `(cons x x)` drawing one pair reachable from two slots. */
  readonly zCarIsZCdr: boolean;
  /** Whether the tail handle of z is the original x binding: the
   * alias the set-car! travels along. */
  readonly zCdrIsX: boolean;
  /** The value `(car x)` reads after the mutation: the book's 17. */
  readonly readCarX: number;
  /** The same cell read through the z handle: the same 17. */
  readonly readThroughZ: number;
}

/** Runs the book's four-expression sequence over procedural pairs and
 * returns the trace: the log of what each step did plus the identity
 * checks and final reads the diagram's sharing question asks. */
export const tracePairAliasing = (): PairAliasingReport => {
  const log: PairAliasingEvent[] = [];

  const x = makePair<number, number>(1, 2);
  log.push({ _tag: "PairCreated", label: "x = makePair(1, 2): the book's (define x (cons 1 2))" });

  const z = makePair<MutablePair<number, number>, MutablePair<number, number>>(x, x);
  log.push({
    _tag: "PairCreated",
    label: "z = makePair(x, x): the book's (define z (cons x x))",
  });

  const zCar = z.getX();
  const zCdr = z.getY();
  const aliased = Object.is(zCar, zCdr);
  log.push({
    _tag: "Aliased",
    label: aliased
      ? "z's car and cdr slots hold the same pair object: one box, two arrows in the diagram"
      : "z's car and cdr slots hold distinct pair objects",
  });

  zCdr.setX(17);
  log.push({
    _tag: "SlotSet",
    label:
      "(set-car! (cdr z) 17): the x-slot of the shared pair is written through the tail handle",
  });

  const readCarX = x.getX();
  log.push({
    _tag: "ReadThroughAlias",
    label: `(car x) reads ${readCarX}: the original handle sees the write made through the alias`,
  });

  return {
    log,
    zCarIsZCdr: aliased,
    zCdrIsX: Object.is(zCdr, x),
    readCarX,
    readThroughZ: z.getX().getX(),
  };
};
