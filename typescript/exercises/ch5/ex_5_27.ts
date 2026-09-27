// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.27 is not solved yet");
    this.name = "PendingSolution";
  }
}
export const ex_5_27 = (): never => {
  throw new PendingSolution();
};
