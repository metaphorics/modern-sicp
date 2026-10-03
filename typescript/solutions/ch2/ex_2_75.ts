// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import type { MessageError, MessageObject } from "../../packages/ch2/src/04-data-directed.js";

/**
 * Exercise 2.75: the constructor `makeFromMagAng` in message-passing
 * style, analogous to the section's `makeFromRealImagMessage`. The
 * object is one closure that answers the four selector messages from
 * its captured magnitude and angle, deriving the rectangular parts by
 * the same trigonometry Ben's package uses in reverse. A message the
 * object does not answer carries the book's "Unknown op" error, naming
 * the constructor and the message.
 */

/** The book's message-passing make-from-mag-ang. */
export const makeFromMagAngMessage = (r: number, a: number): MessageObject => {
  const dispatch = (op: string): Result<number, MessageError> => {
    switch (op) {
      case "real-part":
        return ok(r * Math.cos(a));
      case "imag-part":
        return ok(r * Math.sin(a));
      case "magnitude":
        return ok(r);
      case "angle":
        return ok(a);
      default:
        return err({ _tag: "UnknownMessage", op, source: "makeFromMagAng" });
    }
  };
  return dispatch;
};
