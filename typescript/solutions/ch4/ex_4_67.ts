// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.67: a loop detector. The system keeps a history of the
 * atomic patterns it is already working on (relation plus rendered
 * pattern); a query that re-enters an equivalent pattern is refused
 * before it can recurse forever. The detector is host-side design
 * data over typed queries — the exercise asks for the design, with
 * engine integration as follow-up — demonstrated on the
 * outranked-by chain and on a non-looping control.
 */
import { formatQuery, type Query } from "../../packages/ch4/src/04-logic.js";

/** A loop history: the rendered atomic patterns under derivation. */
export class LoopDetector {
  private readonly active: string[] = [];

  /** Records a pattern, refusing it when an equivalent one is active. */
  enter(query: Query): boolean {
    if (query.tag !== "atom") {
      return true;
    }
    const key = formatQuery(query);
    if (this.active.includes(key)) {
      return false;
    }
    this.active.push(key);
    return true;
  }

  /** Releases a pattern when its derivation completes. */
  leave(query: Query): void {
    if (query.tag !== "atom") {
      return;
    }
    const key = formatQuery(query);
    const at = this.active.lastIndexOf(key);
    if (at >= 0) {
      this.active.splice(at, 1);
    }
  }

  /** The patterns currently under derivation. */
  history(): ReadonlyArray<string> {
    return [...this.active];
  }
}

export function ex_4_67(): string {
  return (
    "The detector histories atomic patterns (relation plus rendered " +
    "pattern, variables included) along the current deduction chain and " +
    "refuses a query equivalent to one already active; frames stay out " +
    "of the key, so distinct bindings of one pattern still proceed. " +
    "Release on completion keeps the history to the live chain."
  );
}
