// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Agenda, Wire } from "../../packages/ch3/src/03-mutable-data.js";
import { fullAdder, makeWire } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.30: the ripple-carry adder. The book composes n
 * full-adders so each stage's carry-out feeds the next stage's
 * carry-in, with `C` as the initial carry-in. This edition takes the
 * two operand wires and the sum wires as arrays ordered least
 * significant bit first, exactly the order the book's `Ak`, `Bk`,
 * `Sk` index from, and returns the most significant carry-out so the
 * arithmetic can be checked without reaching inside the circuit.
 */

/** Builds the ripple-carry adder and returns the final carry-out:
 * stage 0 adds `listA[0] + listB[0]` with `cIn` as its carry-in, and
 * each later stage adds the next bits with the previous carry-out.
 * The sum wires are driven in place. Fails when the operand and sum
 * lists do not all have the same length. */
export const rippleCarryAdder = (
  listA: Wire[],
  listB: Wire[],
  listS: Wire[],
  cIn: Wire,
  agenda: Agenda,
): Wire => {
  if (listA.length !== listB.length || listA.length !== listS.length) {
    throw new Error("ripple-carry adder: operand and sum lists must have equal length");
  }
  let carry = cIn;
  let index = 0;
  for (const a of listA) {
    const b = listB[index];
    const s = listS[index];
    if (b === undefined || s === undefined) {
      throw new Error("ripple-carry adder: operand and sum lists must have equal length");
    }
    const nextCarry = makeWire();
    fullAdder(a, b, carry, s, nextCarry, agenda);
    carry = nextCarry;
    index += 1;
  }
  return carry;
};

/** Reads a list of wires as an unsigned number, least significant
 * bit first: `true` wires are 1 bits. */
export const bitsToNumber = (bits: Wire[]): number => {
  let value = 0;
  let index = 0;
  for (const bit of bits) {
    if (bit.getSignal()) {
      value += 2 ** index;
    }
    index += 1;
  }
  return value;
};

/** Builds a number's bit wires, least significant bit first, padded
 * to `width` bits. */
export const numberToBits = (value: number, width: number): Wire[] => {
  const bits: Wire[] = [];
  for (let i = 0; i < width; i += 1) {
    const wire = makeWire();
    wire.setSignal((value & (2 ** i)) !== 0);
    bits.push(wire);
  }
  return bits;
};

/** The adder's full answer: the sum bits as a number plus the final
 * carry-out as its own bit, to compare against integer addition. */
export const answer = (listS: Wire[], carryOut: Wire): number =>
  bitsToNumber(listS) + (carryOut.getSignal() ? 2 ** listS.length : 0);
