// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.30 is not solved yet");
    this.name = "PendingSolution";
  }
}
export const ex_5_30 = (): never => {
  throw new PendingSolution();
};
