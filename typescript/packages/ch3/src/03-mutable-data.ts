// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.3

/**
 * Modeling with mutable data, spelled in the edition's idiom. The
 * book's mutators (`set-car!`, `set-cdr!`) become assignment to
 * fields an interface does not mark `readonly`, the book's
 * message-passing objects (wires, connectors, constraints) become
 * closures over local state, and the book's `error` calls become
 * typed error classes. Where the book threads one global
 * (`the-agenda`), the edition passes the agenda explicitly, so tests
 * run independent simulations. Where the book prints (probes), the
 * edition appends to a log the caller owns, so every pin a test needs
 * is a value it can read.
 */

// ---------------------------------------------------------------------
// 3.3.1 Mutable List Structure
// ---------------------------------------------------------------------

// The book introduces `set-car!` and `set-cdr!` so pairs can change
// after they are built. The chapter-2 pairs of this edition are
// immutable objects, so the section ships its own pair: the same
// two-field record with mutable fields. JavaScript object identity
// does the book's `eq?`: two variables hold the same pair exactly
// when they hold the same object.

/** The empty mutable list; the book's `nil` over mutable pairs. */
export interface MNil {
  readonly _tag: "MNil";
}

/** A mutable cons cell: the book's pair, with a changeable head (the
 * book's car pointer) and a changeable tail (the book's cdr pointer). */
export interface MCons<A> {
  readonly _tag: "MCons";
  head: A;
  tail: MList<A>;
}

export type MList<A> = MNil | MCons<A>;

/** The empty mutable list value. */
export const mnil: MList<never> = { _tag: "MNil" };

/** Whether the mutable list is empty: the book's `null?`. The tag
 * genuinely discriminates the union, so callers get narrowing. */
export const isMNil = <A>(l: MList<A>): l is MNil => l._tag === "MNil";

/** Builds a mutable pair holding `head` and `tail`: the book's `cons`
 * of this section, spelled with the mutators the section defines. */
export const mcons = <A>(head: A, tail: MList<A>): MCons<A> => ({
  _tag: "MCons",
  head,
  tail,
});

/** Builds a mutable list from a sequence of arguments: the book's
 * `list` over mutable pairs. */
export const mlist = <A>(...items: ReadonlyArray<A>): MList<A> =>
  items.reduceRight<MList<A>>((tail, head) => mcons(head, tail), mnil);

/** Replaces the head of `pair`: the book's `set-car!`. The pair is
 * returned so a chain of calls reads like the book's interactions;
 * the effect is the point. */
export const setCar = <A>(pair: MCons<A>, head: A): MCons<A> => {
  pair.head = head;
  return pair;
};

/** Replaces the tail of `pair`: the book's `set-cdr!`. */
export const setCdr = <A>(pair: MCons<A>, tail: MList<A>): MCons<A> => {
  pair.tail = tail;
  return pair;
};

/** Whether a value is a mutable list: a nested element may itself be
 * one, and the tag check is the whole test. */
const isMListValue = (value: unknown): value is MList<unknown> =>
  typeof value === "object" &&
  value !== null &&
  "_tag" in value &&
  (value._tag === "MNil" || value._tag === "MCons");

/** Renders a mutable list the way the book prints it: elements
 * between parentheses, nesting preserved. */
export const showMList = (l: MList<unknown>): string => {
  const render = (value: unknown): string =>
    isMListValue(value) ? showMList(value) : typeof value === "string" ? value : String(value);
  const items: string[] = [];
  for (let rest = l; rest._tag === "MCons"; rest = rest.tail) {
    items.push(render(rest.head));
  }
  return `(${items.join(" ")})`;
};

// ---------------------------------------------------------------------
// 3.3.2 Representing Queues
// ---------------------------------------------------------------------

// The book's queue is a pair of pointers: a front pointer to the
// first pair of the item chain and a rear pointer to the last pair,
// so insertion splices one pair at the rear and deletion advances the
// front pointer. The edition's queue is the same two-pointer object:
// mutable fields over the section's own pairs, the rear pointer null
// when the queue is empty (the book's rear pointer holds the chain's
// empty end, a pointer to nothing).

/** The book's queue: front and rear pointers over mutable pairs. */
export interface Queue<A> {
  front: MList<A>;
  rear: MCons<A> | null;
}

/** An answer requested from an empty queue: the book's `FRONT called
 * with an empty queue` / `DELETE! called with an empty queue`. */
export class EmptyQueueError extends Error {
  constructor(operation: string) {
    super(`${operation} called with an empty queue`);
    this.name = "EmptyQueueError";
  }
}

/** Builds an empty queue: both pointers empty. */
export const makeQueue = <A>(): Queue<A> => ({ front: mnil, rear: null });

/** Whether the queue is empty: the front pointer is empty. */
export const isEmptyQueue = <A>(queue: Queue<A>): boolean => queue.front._tag === "MNil";

/** The item at the front of the queue, without modifying the queue;
 * fails with `EmptyQueueError` when the queue is empty. */
export const frontQueue = <A>(queue: Queue<A>): A => {
  const front = queue.front;
  if (front._tag === "MNil") {
    throw new EmptyQueueError("FRONT");
  }
  return front.head;
};

/** Inserts `item` at the rear of the queue and returns the queue, the
 * book's `insert-queue!`: one pair is allocated and spliced on, or
 * both pointers start it when the queue was empty. */
export const insertQueue = <A>(queue: Queue<A>, item: A): Queue<A> => {
  const newPair = mcons<A>(item, mnil);
  if (queue.front._tag === "MNil") {
    queue.front = newPair;
    queue.rear = newPair;
  } else {
    const rear = queue.rear;
    if (rear === null) {
      throw new Error("corrupt queue: nonempty front with null rear");
    }
    setCdr(rear, newPair);
    queue.rear = newPair;
  }
  return queue;
};

/** Removes the item at the front of the queue and returns the queue;
 * fails with `EmptyQueueError` when the queue is empty before the
 * deletion. Deleting the last item empties the rear pointer too. */
export const deleteQueue = <A>(queue: Queue<A>): Queue<A> => {
  const front = queue.front;
  if (front._tag === "MNil") {
    throw new EmptyQueueError("DELETE!");
  }
  queue.front = front.tail;
  if (front.tail._tag === "MNil") {
    queue.rear = null;
  }
  return queue;
};

/** The queue's items as a mutable list: the book's `print-queue`
 * returns the front pointer, which the printing system renders. */
export const queueItems = <A>(queue: Queue<A>): MList<A> => queue.front;

// ---------------------------------------------------------------------
// 3.3.3 Representing Tables
// ---------------------------------------------------------------------

// The book's one-dimensional table is a pair whose car is the symbol
// `*table*` and whose cdr is the chain of records; `insert!` mutates
// that cdr, so the table object is the sentinel and the records hang
// off it. The edition's table is the same shape: a record object whose
// single mutable field holds the record chain, so inserting into an
// empty table mutates the table object itself.

/** One table record: a key with its stored value. */
export interface TableRecord<K, V> {
  readonly key: K;
  value: V;
}

/** A one-dimensional table: the book's sentinel pair, an object whose
 * mutable `records` field is the chain of records. */
export interface Table<K, V> {
  records: MList<TableRecord<K, V>>;
}

/** The book's `same-key?` default: `equal?` over the keys, spelled as
 * strict equality for the strings and numbers a key holds. */
export const sameKeyDefault = <K>(a: K, b: K): boolean => a === b;

/** Builds an empty one-dimensional table. */
export const makeTable = <K, V>(): Table<K, V> => ({ records: mnil });

/** Looks `key` up in the table, scanning the record chain and
 * comparing keys with `sameKey`; `undefined` when the key is absent
 * (the book's `false`). */
export const tableLookup = <K, V>(
  table: Table<K, V>,
  key: K,
  sameKey: (a: K, b: K) => boolean = sameKeyDefault,
): V | undefined => {
  for (let records = table.records; records._tag === "MCons"; records = records.tail) {
    if (sameKey(records.head.key, key)) {
      return records.head.value;
    }
  }
  return undefined;
};

/** Associates `key` with `value` in the table: the book's `insert!`.
 * A record for the key is replaced in place; otherwise a new record
 * is consed onto the chain, mutating the table itself when it was
 * empty. */
export const tableInsert = <K, V>(
  table: Table<K, V>,
  key: K,
  value: V,
  sameKey: (a: K, b: K) => boolean = sameKeyDefault,
): Table<K, V> => {
  for (let records = table.records; records._tag === "MCons"; records = records.tail) {
    if (sameKey(records.head.key, key)) {
      records.head.value = value;
      return table;
    }
  }
  table.records = mcons({ key, value }, table.records);
  return table;
};

/** A two-dimensional table: the record chain holds records whose
 * values are subtables, looked up one key at a time, exactly the
 * book's nested structure. */
export interface Table2<K1, K2, V> {
  records: MList<TableRecord<K1, Table<K2, V>>>;
}

/** Builds an empty two-dimensional table. */
export const makeTable2 = <K1, K2, V>(): Table2<K1, K2, V> => ({ records: mnil });

/** Looks `(key1, key2)` up in the two-dimensional table: the first
 * key finds a subtable, the second finds the value in it. */
export const tableLookup2 = <K1, K2, V>(
  table: Table2<K1, K2, V>,
  key1: K1,
  key2: K2,
): V | undefined => {
  for (let records = table.records; records._tag === "MCons"; records = records.tail) {
    if (records.head.key === key1) {
      return tableLookup(records.head.value, key2);
    }
  }
  return undefined;
};

/** Associates `(key1, key2)` with `value`: the first key finds (or
 * starts) a subtable, the value is inserted there. */
export const tableInsert2 = <K1, K2, V>(
  table: Table2<K1, K2, V>,
  key1: K1,
  key2: K2,
  value: V,
): void => {
  for (let records = table.records; records._tag === "MCons"; records = records.tail) {
    if (records.head.key === key1) {
      tableInsert(records.head.value, key2, value);
      return;
    }
  }
  const subtable = makeTable<K2, V>();
  tableInsert(subtable, key2, value);
  table.records = mcons({ key: key1, value: subtable }, table.records);
};

// ---------------------------------------------------------------------
// 3.3.4 A Simulator for Digital Circuits
// ---------------------------------------------------------------------

// A wire carries a signal and a list of action procedures; the agenda
// is a time-ordered chain of segments, each a time with a queue of
// actions. The book's message-passing wire becomes a closure over two
// local variables, and the book's one `the-agenda` becomes an agenda
// value passed to every call that schedules or runs work. The signals
// are the book's 0 and 1 spelled as `false` and `true`.

/** A wire: reads and writes its signal and accepts action procedures
 * that run whenever the signal changes. */
export interface Wire {
  readonly getSignal: () => boolean;
  readonly setSignal: (value: boolean) => void;
  readonly addAction: (action: () => void) => void;
}

/** Builds a wire with signal `false` and no actions. A newly accepted
 * action runs once immediately, so a device attached to a wire picks
 * up the wire's current value; this is the behavior exercise 3.31
 * asks about. */
export const makeWire = (): Wire => {
  let signal = false;
  let actions: Array<() => void> = [];
  const callEach = (procedures: Array<() => void>): void => {
    for (const procedure of procedures) {
      procedure();
    }
  };
  return {
    getSignal: () => signal,
    setSignal: (value) => {
      if (signal !== value) {
        signal = value;
        callEach(actions);
      }
    },
    addAction: (action) => {
      actions = [action, ...actions];
      action();
    },
  };
};

/** One scheduled segment: a simulation time with the queue of actions
 * to run at that time. */
export interface TimeSegment {
  readonly time: number;
  readonly queue: Queue<() => void>;
}

/** The agenda: the current time plus the segment chain, the book's
 * pair of current time and segments. */
export interface Agenda {
  currentTime: number;
  segments: MList<TimeSegment>;
}

/** Nothing is scheduled: the book's `Agenda is empty` error, raised
 * when the next item is demanded of an empty agenda. */
export class EmptyAgendaError extends Error {
  constructor() {
    super("Agenda is empty");
    this.name = "EmptyAgendaError";
  }
}

/** Builds an empty agenda: time 0, no segments. */
export const makeAgenda = (): Agenda => ({ currentTime: 0, segments: mnil });

/** The agenda's current time: the time of the segment being run. */
export const currentAgendaTime = (agenda: Agenda): number => agenda.currentTime;

/** Whether nothing is scheduled: the book's `empty-agenda?`. */
export const isAgendaEmpty = (agenda: Agenda): boolean => isMNil(agenda.segments);

/** The first segment cell of a nonempty agenda. */
const firstSegmentCell = (agenda: Agenda): MCons<TimeSegment> => {
  if (isMNil(agenda.segments)) {
    throw new EmptyAgendaError();
  }
  return agenda.segments;
};

/** Schedules `action` at `time`: a new segment is inserted before the
 * first later segment, or an existing segment at that time gets the
 * action at the rear of its queue, so same-time actions run in
 * insertion order (exercise 3.32's FIFO point). */
export const addToAgenda = (time: number, action: () => void, agenda: Agenda): void => {
  const newSegment = (): TimeSegment => ({
    time,
    queue: insertQueue<() => void>(makeQueue(), action),
  });
  // True when the chain continues past `time`: the chain is a cons
  // cell whose segment runs at or before the scheduled time, so the
  // scan moves on. False means the rest is empty or runs later, and
  // the new segment belongs right here.
  const continuesPast = (segments: MList<TimeSegment>): segments is MCons<TimeSegment> =>
    segments._tag === "MCons" && segments.head.time <= time;
  let cell = agenda.segments;
  if (!continuesPast(cell)) {
    agenda.segments = mcons(newSegment(), cell);
    return;
  }
  for (;;) {
    const segment = cell.head;
    if (segment.time === time) {
      insertQueue(segment.queue, action);
      return;
    }
    const rest: MList<TimeSegment> = cell.tail;
    if (!continuesPast(rest)) {
      setCdr(cell, mcons(newSegment(), rest));
      return;
    }
    cell = rest;
  }
};

/** The action to run next, setting the agenda's time to its segment's
 * time; fails with `EmptyAgendaError` when nothing is scheduled. */
export const firstAgendaItem = (agenda: Agenda): (() => void) => {
  const cell = firstSegmentCell(agenda);
  agenda.currentTime = cell.head.time;
  return frontQueue(cell.head.queue);
};

/** Removes the next action from the agenda, dropping its segment when
 * the queue empties. */
export const removeFirstAgendaItem = (agenda: Agenda): void => {
  const cell = firstSegmentCell(agenda);
  deleteQueue(cell.head.queue);
  if (isEmptyQueue(cell.head.queue)) {
    agenda.segments = cell.tail;
  }
};

/** The book's primitive gate delays, in the book's units: the setup
 * the section assigns before the sample simulation. */
export const inverterDelay = 2;
/** The and-gate's delay. */
export const andGateDelay = 3;
/** The or-gate's delay. */
export const orGateDelay = 5;

/** Schedules `action` after `delay` from the agenda's current time:
 * the book's `after-delay` with the agenda explicit. */
export const afterDelay = (delay: number, action: () => void, agenda: Agenda): void => {
  addToAgenda(currentAgendaTime(agenda) + delay, action, agenda);
};

/** Runs the agenda until it is empty: each action runs at its own
 * time, and actions it schedules join the agenda. */
export const propagate = (agenda: Agenda): void => {
  while (!isAgendaEmpty(agenda)) {
    firstAgendaItem(agenda)();
    removeFirstAgendaItem(agenda);
  }
};

/** The not function of the gates: the book's `logical-not`. */
export const logicalNot = (value: boolean): boolean => !value;
/** The and function of the gates. */
export const logicalAnd = (a: boolean, b: boolean): boolean => a && b;
/** The or function of the gates. */
export const logicalOr = (a: boolean, b: boolean): boolean => a || b;

/** Connects `output` to the logical negation of `input`, with the
 * inverter's delay between a change on `input` and the response. */
export const inverter = (input: Wire, output: Wire, agenda: Agenda): void => {
  const invertInput = () => {
    afterDelay(inverterDelay, () => output.setSignal(logicalNot(input.getSignal())), agenda);
  };
  input.addAction(invertInput);
};

/** Connects `output` to the and of the two inputs. */
export const andGate = (a1: Wire, a2: Wire, output: Wire, agenda: Agenda): void => {
  const andActionProcedure = () => {
    afterDelay(
      andGateDelay,
      () => output.setSignal(logicalAnd(a1.getSignal(), a2.getSignal())),
      agenda,
    );
  };
  a1.addAction(andActionProcedure);
  a2.addAction(andActionProcedure);
};

/** Connects `output` to the or of the two inputs. */
export const orGate = (a1: Wire, a2: Wire, output: Wire, agenda: Agenda): void => {
  const orActionProcedure = () => {
    afterDelay(
      orGateDelay,
      () => output.setSignal(logicalOr(a1.getSignal(), a2.getSignal())),
      agenda,
    );
  };
  a1.addAction(orActionProcedure);
  a2.addAction(orActionProcedure);
};

/** Builds a half-adder: `s` is the sum, `d and e` through the or of
 * one input's and with the other's inverse, and `c` the carry, the
 * and of the inputs. */
export const halfAdder = (a: Wire, b: Wire, s: Wire, c: Wire, agenda: Agenda): void => {
  const d = makeWire();
  const e = makeWire();
  orGate(a, b, d, agenda);
  andGate(a, b, c, agenda);
  inverter(c, e, agenda);
  andGate(d, e, s, agenda);
};

/** Builds a full-adder from two half-adders and an or-gate: `sum` is
 * the parity of the three inputs, `cOut` carries when two or more are
 * 1. */
export const fullAdder = (
  a: Wire,
  b: Wire,
  cIn: Wire,
  sum: Wire,
  cOut: Wire,
  agenda: Agenda,
): void => {
  const s = makeWire();
  const c1 = makeWire();
  const c2 = makeWire();
  halfAdder(b, cIn, s, c1, agenda);
  halfAdder(a, s, sum, c2, agenda);
  orGate(c1, c2, cOut, agenda);
};

/** One probe reading: the wire's name, the agenda time, and the new
 * signal value, standing for one line of the book's printed output. */
export interface ProbeEvent {
  readonly time: number;
  readonly name: string;
  readonly value: boolean;
}

/** Places a probe on the wire: every signal change appends the time,
 * name, and new value to `events`, the book's printed probe lines
 * made readable. */
export const probe = (name: string, wire: Wire, agenda: Agenda, events: ProbeEvent[]): void => {
  wire.addAction(() => {
    events.push({ time: currentAgendaTime(agenda), name, value: wire.getSignal() });
  });
};

// ---------------------------------------------------------------------
// 3.3.5 Propagation of Constraints
// ---------------------------------------------------------------------

// A connector holds a value, the informant that set it, and the list
// of constraints attached to it. Setting and forgetting values
// notifies the constraints except the requester; a constraint asked
// to set a second, different value is a contradiction, the book's
// `error`. The constraints are message-passing objects: each answers
// a value message by rechecking its arithmetic and a lost message by
// withdrawing whatever it had claimed.

/** The two messages a constraint answers: a value arrived, a value
 * was lost. The book's `I-have-a-value` / `I-lost-my-value`. */
export type ConstraintMessage = "informAboutValue" | "informAboutNoValue";

/** A constraint: the book's dispatch procedure, answering the two
 * messages. */
export type Constraint = (message: ConstraintMessage) => void;

/** The object requesting a value be set or forgotten: a constraint,
 * or the user label the book's examples pass. */
export type Informant = Constraint | string;

/** A connector: value, informant, and constraints, behind the book's
 * five operations. */
export interface Connector {
  readonly hasValue: () => boolean;
  readonly getValue: () => number;
  readonly setValue: (value: number, informant: Informant) => void;
  readonly forgetValue: (retractor: Informant) => void;
  readonly connect: (constraint: Constraint) => void;
}

/** Setting a connector to a different value than the one already set:
 * the book's `Contradiction` error, carrying the old and new values. */
export class ContradictionError extends Error {
  readonly old: number;
  readonly requested: number;
  constructor(oldValue: number, requestedValue: number) {
    super(`Contradiction: (${String(oldValue)} ${String(requestedValue)})`);
    this.name = "ContradictionError";
    this.old = oldValue;
    this.requested = requestedValue;
  }
}

/** Builds a connector with no value and no constraints. A new
 * constraint is added once, and is told about an existing value at
 * connect time; otherwise it hears only changes. */
export const makeConnector = (): Connector => {
  let value: number | undefined;
  let informant: Informant | undefined;
  let constraints: Constraint[] = [];
  const forEachExcept = (exception: unknown, message: ConstraintMessage): void => {
    for (const constraint of constraints) {
      if (constraint !== exception) {
        constraint(message);
      }
    }
  };
  return {
    hasValue: () => value !== undefined,
    getValue: () => {
      if (value === undefined) {
        throw new Error("connector has no value");
      }
      return value;
    },
    setValue: (newValue, setter) => {
      if (value === undefined) {
        value = newValue;
        informant = setter;
        forEachExcept(setter, "informAboutValue");
      } else if (value !== newValue) {
        throw new ContradictionError(value, newValue);
      }
    },
    forgetValue: (retractor) => {
      if (retractor === informant) {
        informant = undefined;
        value = undefined;
        forEachExcept(retractor, "informAboutNoValue");
      }
    },
    connect: (newConstraint) => {
      if (!constraints.includes(newConstraint)) {
        constraints = [newConstraint, ...constraints];
      }
      if (value !== undefined) {
        newConstraint("informAboutValue");
      }
    },
  };
};

/** Tells `constraint` a value arrived: the book's
 * `inform-about-value`. */
export const informAboutValue = (constraint: Constraint): void => {
  constraint("informAboutValue");
};

/** Tells `constraint` its value left: the book's
 * `inform-about-no-value`. */
export const informAboutNoValue = (constraint: Constraint): void => {
  constraint("informAboutNoValue");
};

/** Constrains `sum` to be `a1 + a2`. When one input and the sum are
 * known, the adder claims the other input; when a value is lost, the
 * adder withdraws all three claims (only ones it set are lost) and
 * rechecks, because values it never set may remain. */
export const adder = (a1: Connector, a2: Connector, sum: Connector): Constraint => {
  const processNewValue = (): void => {
    if (a1.hasValue() && a2.hasValue()) {
      sum.setValue(a1.getValue() + a2.getValue(), me);
    } else if (a1.hasValue() && sum.hasValue()) {
      a2.setValue(sum.getValue() - a1.getValue(), me);
    } else if (a2.hasValue() && sum.hasValue()) {
      a1.setValue(sum.getValue() - a2.getValue(), me);
    }
  };
  const processForgetValue = (): void => {
    sum.forgetValue(me);
    a1.forgetValue(me);
    a2.forgetValue(me);
    processNewValue();
  };
  const me: Constraint = (message) => {
    if (message === "informAboutValue") {
      processNewValue();
    } else {
      processForgetValue();
    }
  };
  a1.connect(me);
  a2.connect(me);
  sum.connect(me);
  return me;
};

/** Constrains `product` to be `m1 * m2`. A zero factor sets the
 * product to 0 even when the other factor is unknown. */
export const multiplier = (m1: Connector, m2: Connector, product: Connector): Constraint => {
  const processNewValue = (): void => {
    if ((m1.hasValue() && m1.getValue() === 0) || (m2.hasValue() && m2.getValue() === 0)) {
      product.setValue(0, me);
    } else if (m1.hasValue() && m2.hasValue()) {
      product.setValue(m1.getValue() * m2.getValue(), me);
    } else if (product.hasValue() && m1.hasValue()) {
      m2.setValue(product.getValue() / m1.getValue(), me);
    } else if (product.hasValue() && m2.hasValue()) {
      m1.setValue(product.getValue() / m2.getValue(), me);
    }
  };
  const processForgetValue = (): void => {
    product.forgetValue(me);
    m1.forgetValue(me);
    m2.forgetValue(me);
    processNewValue();
  };
  const me: Constraint = (message) => {
    if (message === "informAboutValue") {
      processNewValue();
    } else {
      processForgetValue();
    }
  };
  m1.connect(me);
  m2.connect(me);
  product.connect(me);
  return me;
};

/** Fixes `connector` to `value` permanently. Any message sent to the
 * constant box is an error, as in the book. */
export const constant = (value: number, connector: Connector): Constraint => {
  const me: Constraint = () => {
    throw new Error("Unknown request: CONSTANT");
  };
  connector.connect(me);
  connector.setValue(value, me);
  return me;
};

/** One connector probe reading: the name and the value, or the `?`
 * the book prints when the connector loses its value. */
export interface ConnectorProbeEvent {
  readonly name: string;
  readonly value: number | "?";
}

/** Watches the connector: every value gained or lost appends one
 * event to `events`, the book's printed probe lines made readable. */
export const connectorProbe = (
  name: string,
  connector: Connector,
  events: ConnectorProbeEvent[],
): Constraint => {
  const me: Constraint = (message) => {
    if (message === "informAboutValue") {
      events.push({ name, value: connector.getValue() });
    } else {
      events.push({ name, value: "?" });
    }
  };
  connector.connect(me);
  return me;
};

/** Builds the Celsius-to-Fahrenheit network: `f = 9c/5 + 32`, wired
 * through the book's multipliers, adder, and constants over the
 * auxiliary connectors `u`, `v`, `w`. */
export const celsiusFahrenheitConverter = (c: Connector, f: Connector): void => {
  const u = makeConnector();
  const v = makeConnector();
  const w = makeConnector();
  const x = makeConnector();
  const y = makeConnector();
  multiplier(c, w, u);
  multiplier(v, x, u);
  adder(v, y, f);
  constant(9, w);
  constant(5, x);
  constant(32, y);
};
