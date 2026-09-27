// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 4_73: Explain why flatten-stream delays its recursive tail. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4_73 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_4_73(): string {
  throw new PendingSolution();
}
