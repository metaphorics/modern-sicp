// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.3

import { describe, expect, it } from "vitest";

import {
  adder,
  addToAgenda,
  type ConnectorProbeEvent,
  ContradictionError,
  celsiusFahrenheitConverter,
  connectorProbe,
  constant,
  currentAgendaTime,
  deleteQueue,
  EmptyAgendaError,
  EmptyQueueError,
  firstAgendaItem,
  frontQueue,
  fullAdder,
  halfAdder,
  insertQueue,
  isAgendaEmpty,
  isEmptyQueue,
  type MCons,
  type MList,
  makeAgenda,
  makeConnector,
  makeQueue,
  makeTable,
  makeTable2,
  makeWire,
  mcons,
  mlist,
  mnil,
  multiplier,
  type ProbeEvent,
  probe,
  propagate,
  queueItems,
  setCar,
  setCdr,
  showMList,
  tableInsert,
  tableInsert2,
  tableLookup,
  tableLookup2,
} from "./03-mutable-data.js";

describe("section 3.3.1: mutable list structure", () => {
  it("set-car! replaces the head pointer, detaching the old branch", () => {
    const ab = mlist("a", "b");
    const x = mcons<MList<string> | string>(ab, mlist("c", "d"));
    const y = mlist("e", "f");
    expect(showMList(x)).toBe("((a b) c d)");
    setCar(x, y);
    expect(showMList(x)).toBe("((e f) c d)");
    // The replaced pointer leaves the old branch intact but detached.
    expect(showMList(ab)).toBe("(a b)");
  });

  it("set-cdr! splices structure, and every alias sees the change", () => {
    const x = mcons("a", mlist("b"));
    const y = mlist("c", "d");
    const secondPair = x.tail;
    if (secondPair._tag === "MNil") {
      throw new Error("expected a second pair");
    }
    setCdr(secondPair, y);
    expect(showMList(x)).toBe("(a b c d)");
    // y itself is now the tail of x's second pair: one structure, shared.
    expect(secondPair.tail).toBe(y);
  });

  it("mlist and showMList render nesting the way the book prints it", () => {
    expect(showMList(mlist(1, 2, 3))).toBe("(1 2 3)");
    expect(showMList(mnil)).toBe("()");
    const innerList = mcons<number>(2, mnil);
    const first = mcons<number | MCons<number>>(1, mcons(innerList, mnil));
    const outer = mcons<MCons<number> | number>(first, mcons(3, mnil));
    expect(showMList(outer)).toBe("((1 (2)) 3)");
  });
});

describe("section 3.3.2: queues", () => {
  it("runs the figure 3.18 sequence with constant-time ends", () => {
    const q = makeQueue<string>();
    insertQueue(q, "a");
    insertQueue(q, "b");
    expect(showMList(queueItems(q))).toBe("(a b)");
    deleteQueue(q);
    expect(showMList(queueItems(q))).toBe("(b)");
    insertQueue(q, "c");
    insertQueue(q, "d");
    expect(showMList(queueItems(q))).toBe("(b c d)");
    deleteQueue(q);
    expect(showMList(queueItems(q))).toBe("(c d)");
    expect(frontQueue(q)).toBe("c");
  });

  it("resets the rear pointer when the last item is deleted", () => {
    const q = makeQueue<string>();
    insertQueue(q, "only");
    deleteQueue(q);
    expect(isEmptyQueue(q)).toBe(true);
    // A queue emptied by deletion must accept insertions again.
    insertQueue(q, "next");
    expect(showMList(queueItems(q))).toBe("(next)");
  });

  it("fails on front or delete of an empty queue", () => {
    const q = makeQueue<string>();
    expect(() => frontQueue(q)).toThrow(EmptyQueueError);
    expect(() => deleteQueue(q)).toThrow(EmptyQueueError);
  });
});

describe("section 3.3.3: tables", () => {
  it("inserts into an empty table by mutating the table object", () => {
    const table = makeTable<string, number>();
    expect(tableLookup(table, "a")).toBeUndefined();
    tableInsert(table, "a", 1);
    expect(tableLookup(table, "a")).toBe(1);
    // A repeated key replaces the value in place.
    tableInsert(table, "a", 2);
    expect(tableLookup(table, "a")).toBe(2);
    expect(tableLookup(table, "b")).toBeUndefined();
  });

  it("honors a same-key? predicate supplied by the caller", () => {
    const table = makeTable<string, number>();
    const sameKey = (a: string, b: string): boolean => a.toLowerCase() === b.toLowerCase();
    tableInsert(table, "Foo", 1);
    tableInsert(table, "foo", 9, sameKey);
    expect(tableLookup(table, "FOO", sameKey)).toBe(9);
    // The default same-key? distinguishes the two spellings.
    const strict = makeTable<string, number>();
    tableInsert(strict, "Foo", 1);
    tableInsert(strict, "foo", 9);
    expect(tableLookup(strict, "Foo")).toBe(1);
    expect(tableLookup(strict, "foo")).toBe(9);
  });

  it("indexes a two-dimensional table one key at a time", () => {
    const table = makeTable2<string, string, number>();
    tableInsert2(table, "rectangular", "a", 1);
    tableInsert2(table, "rectangular", "b", 2);
    tableInsert2(table, "polar", "a", 3);
    expect(tableLookup2(table, "rectangular", "a")).toBe(1);
    expect(tableLookup2(table, "rectangular", "b")).toBe(2);
    expect(tableLookup2(table, "polar", "a")).toBe(3);
    expect(tableLookup2(table, "polar", "b")).toBeUndefined();
  });
});

describe("section 3.3.4: the digital-circuit simulator", () => {
  it("reproduces the book's half-adder sample simulation", () => {
    const agenda = makeAgenda();
    const input1 = makeWire();
    const input2 = makeWire();
    const sum = makeWire();
    const carry = makeWire();
    const events: ProbeEvent[] = [];
    probe("sum", sum, agenda, events);
    probe("carry", carry, agenda, events);
    // A probe prints the wire's current value the moment it is
    // attached, as the book's do.
    expect(events).toEqual([
      { time: 0, name: "sum", value: false },
      { time: 0, name: "carry", value: false },
    ]);
    halfAdder(input1, input2, sum, carry, agenda);
    input1.setSignal(true);
    propagate(agenda);
    // The e := 1 initialization item (exercise 3.31) fires at time 2
    // and schedules a sum response into the same time-5 segment where
    // d := 1 lands, so the and-gate fires with d already 1 and the
    // probe reports the change at time 5, not the hand trace's 8.
    expect(events).toEqual([
      { time: 0, name: "sum", value: false },
      { time: 0, name: "carry", value: false },
      { time: 5, name: "sum", value: true },
    ]);
    input2.setSignal(true);
    propagate(agenda);
    // From here the literal run and the book's printed trace agree.
    expect(events).toEqual([
      { time: 0, name: "sum", value: false },
      { time: 0, name: "carry", value: false },
      { time: 5, name: "sum", value: true },
      { time: 11, name: "carry", value: true },
      { time: 16, name: "sum", value: false },
    ]);
    // The second propagate ended in the time-16 segment.
    expect(currentAgendaTime(agenda)).toBe(16);
  });

  it("a full-adder computes parity and majority after propagation", () => {
    const agenda = makeAgenda();
    const a = makeWire();
    const b = makeWire();
    const cIn = makeWire();
    const sum = makeWire();
    const cOut = makeWire();
    fullAdder(a, b, cIn, sum, cOut, agenda);
    a.setSignal(true);
    b.setSignal(true);
    cIn.setSignal(true);
    propagate(agenda);
    expect(sum.getSignal()).toBe(true);
    expect(cOut.getSignal()).toBe(true);
  });

  it("runs same-time actions in insertion order and times in order", () => {
    const agenda = makeAgenda();
    const ran: string[] = [];
    addToAgenda(5, () => ran.push("A"), agenda);
    addToAgenda(3, () => ran.push("B"), agenda);
    addToAgenda(5, () => ran.push("C"), agenda);
    addToAgenda(2, () => ran.push("D"), agenda);
    propagate(agenda);
    expect(ran).toEqual(["D", "B", "A", "C"]);
    expect(isAgendaEmpty(agenda)).toBe(true);
    expect(currentAgendaTime(agenda)).toBe(5);
    expect(() => firstAgendaItem(agenda)).toThrow(EmptyAgendaError);
  });
});

describe("section 3.3.5: propagation of constraints", () => {
  it("converts between Celsius and Fahrenheit in both directions", () => {
    const c = makeConnector();
    const f = makeConnector();
    const events: ConnectorProbeEvent[] = [];
    connectorProbe("C", c, events);
    connectorProbe("F", f, events);
    celsiusFahrenheitConverter(c, f);
    c.setValue(25, "user");
    expect(c.getValue()).toBe(25);
    expect(f.getValue()).toBe(77);
    c.forgetValue("user");
    // The user's value is gone; the network relaxes, and the probes
    // record the loss as the book's printed ?.
    expect(events.filter((event) => event.value === "?").length).toBeGreaterThan(0);
    expect(c.hasValue()).toBe(false);
    f.setValue(212, "user");
    expect(c.getValue()).toBe(100);
    expect(f.getValue()).toBe(212);
  });

  it("an adder computes forward and backward", () => {
    const a = makeConnector();
    const b = makeConnector();
    const sum = makeConnector();
    adder(a, b, sum);
    a.setValue(2, "user");
    b.setValue(3, "user");
    expect(sum.getValue()).toBe(5);
    const a2 = makeConnector();
    const b2 = makeConnector();
    const sum2 = makeConnector();
    adder(a2, b2, sum2);
    a2.setValue(10, "user");
    sum2.setValue(17, "user");
    expect(b2.getValue()).toBe(7);
  });

  it("a zero factor sets the product even with an unknown factor", () => {
    const m1 = makeConnector();
    const m2 = makeConnector();
    const product = makeConnector();
    multiplier(m1, m2, product);
    m1.setValue(0, "user");
    expect(product.hasValue()).toBe(true);
    expect(product.getValue()).toBe(0);
  });

  it("accepts a constant and rejects a contradicting second value", () => {
    const c = makeConnector();
    constant(42, c);
    expect(c.getValue()).toBe(42);
    expect(() => c.setValue(30, "user")).toThrow(ContradictionError);
    // The same value from another informant is not a contradiction.
    c.setValue(42, "user");
  });
});
