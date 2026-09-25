// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type {
  Agenda,
  MCons,
  MList,
  ProbeEvent,
  Queue,
  TimeSegment,
  Wire,
} from "../../packages/ch3/src/03-mutable-data.js";
import {
  insertQueue,
  isEmptyQueue,
  makeAgenda,
  makeQueue,
  makeWire,
  mcons,
  probe,
  propagate,
  setCdr,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.32: the agenda processes same-time actions in the order
 * they were added, FIFO, and the exercise asks what goes wrong if the
 * segments were LIFO stacks. The edition answers by measurement: a
 * local `addToAgendaLifo` variant inserts same-time actions at the
 * FRONT of the segment's queue, interoperating with the module's
 * segment structure, and the same labeled schedule runs through both
 * orders. The half-adder demo runs under a fully LIFO agenda too,
 * using gate constructors that schedule through the LIFO variant.
 */

/** Adds `action` to the front of a queue's item chain: the LIFO
 * insert. An empty queue takes the ordinary rear-setting insert. */
const pushFront = (queue: Queue<() => void>, action: () => void): void => {
  if (isEmptyQueue(queue)) {
    insertQueue(queue, action);
    return;
  }
  queue.front = mcons(action, queue.front);
};

/** The book's `add-to-agenda!` with LIFO same-time ordering: the
 * segment scan matches the module's, but a matching segment takes the
 * action at the FRONT of its queue. */
export const addToAgendaLifo = (time: number, action: () => void, agenda: Agenda): void => {
  const newSegment = (): TimeSegment => ({
    time,
    queue: insertQueue<() => void>(makeQueue(), action),
  });
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
      pushFront(segment.queue, action);
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

/** Schedules `action` after `delay` through the LIFO agenda. */
const afterDelayLifo = (delay: number, action: () => void, agenda: Agenda): void => {
  addToAgendaLifo(agenda.currentTime + delay, action, agenda);
};

/** The primitive gates over the LIFO agenda, same shapes as the
 * module's. */
const inverterLifo = (input: Wire, output: Wire, agenda: Agenda): void => {
  input.addAction(() => {
    afterDelayLifo(2, () => output.setSignal(!input.getSignal()), agenda);
  });
};
const andGateLifo = (a1: Wire, a2: Wire, output: Wire, agenda: Agenda): void => {
  const action = () => {
    afterDelayLifo(3, () => output.setSignal(a1.getSignal() && a2.getSignal()), agenda);
  };
  a1.addAction(action);
  a2.addAction(action);
};
const orGateLifo = (a1: Wire, a2: Wire, output: Wire, agenda: Agenda): void => {
  const action = () => {
    afterDelayLifo(5, () => output.setSignal(a1.getSignal() || a2.getSignal()), agenda);
  };
  a1.addAction(action);
  a2.addAction(action);
};

/** The book's half-adder, wired with the LIFO gates. */
const halfAdderLifo = (a: Wire, b: Wire, s: Wire, c: Wire, agenda: Agenda): void => {
  const d = makeWire();
  const e = makeWire();
  orGateLifo(a, b, d, agenda);
  andGateLifo(a, b, c, agenda);
  inverterLifo(c, e, agenda);
  andGateLifo(d, e, s, agenda);
};

/** Runs the book's half-adder demo under the LIFO agenda: probes on
 * sum and carry, build, set input-1, propagate, set input-2,
 * propagate. Returns the probe log. */
export const demonstrateLifo = (): ProbeEvent[] => {
  const agenda = makeAgenda();
  const input1 = makeWire();
  const input2 = makeWire();
  const sum = makeWire();
  const carry = makeWire();
  const events: ProbeEvent[] = [];
  probe("sum", sum, agenda, events);
  probe("carry", carry, agenda, events);
  halfAdderLifo(input1, input2, sum, carry, agenda);
  input1.setSignal(true);
  propagate(agenda);
  input2.setSignal(true);
  propagate(agenda);
  return events;
};

/** Schedules four labeled actions (A and C at time 5, B at 3, D at 2)
 * through the given add function and returns the run order. */
export const runSchedule = (
  add: (time: number, action: () => void, agenda: Agenda) => void,
): string[] => {
  const agenda = makeAgenda();
  const ran: string[] = [];
  add(5, () => ran.push("A"), agenda);
  add(3, () => ran.push("B"), agenda);
  add(5, () => ran.push("C"), agenda);
  add(2, () => ran.push("D"), agenda);
  propagate(agenda);
  return ran;
};
