// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Agenda } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.32: the agenda segment is FIFO. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_32.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.32 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's add-to-agenda! with LIFO same-time ordering. */
export function addToAgendaLifo(_time: number, _action: () => void, _agenda: Agenda): void {
  throw new PendingSolution();
}
