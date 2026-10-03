// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.17: environments under scanned-out internal declarations. The
 * statement asks for the frame structure at e3 under sequential and scanned
 * interpretation, the reason the transformed program has an extra frame, why
 * that difference can never change the behavior of a correct program, and a
 * design that implements simultaneous scope without constructing the extra
 * frame.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.17 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Definition-order data for the body: two bindings and a final read. */
export type BodyStep =
  | { readonly kind: "define"; readonly name: string; readonly valueName: string }
  | { readonly kind: "read"; readonly name: string };
export const bodySteps: readonly BodyStep[] = [
  { kind: "define", name: "u", valueName: "e1" },
  { kind: "define", name: "v", valueName: "e2" },
  { kind: "read", name: "e3" },
];

/** The scanned-out shape: two unassigned cells, two writes, then the read. */
export type ScanStep =
  | { readonly kind: "declare-unassigned"; readonly name: string }
  | { readonly kind: "write"; readonly name: string; readonly valueName: string }
  | { readonly kind: "read"; readonly name: string };
export const scannedSteps: readonly ScanStep[] = [
  { kind: "declare-unassigned", name: "u" },
  { kind: "declare-unassigned", name: "v" },
  { kind: "write", name: "u", valueName: "e1" },
  { kind: "write", name: "v", valueName: "e2" },
  { kind: "read", name: "e3" },
];

export function ex_4_17(): string {
  throw new PendingSolution();
}
