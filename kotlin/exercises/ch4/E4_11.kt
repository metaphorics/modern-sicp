// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.11

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.11: represent frames as association lists instead of the
// runtime's name->value maps. The frame the four environment operations
// see is one `(name . value)` pair list: `define` conses a fresh pair
// onto the front, `set!` scans outwards and rewrites the value slot of
// the nearest pair, lookup returns the first pair's value, and
// extend-environment builds a fresh frame from the parameters -- with the
// runtime's arity contract. The solution's `AlistFrames` evaluator
// subclasses `Evaluator` and overrides exactly the env-op seam, so every
// procedure call and variable touch runs over the alist representation.
// Pins: define z 2 then lookup => 2; `set!` z 10 through an inner frame
// then lookup => 10; a `z` that lived only in a returned call frame reads
// `"Error: unbound variable: z\n"`; an arity mismatch still faults with
// the `extend` message.

/** The define/lookup/set! demo on alist frames. => "2\n10\n10\n" */
public fun alistTranscript(): String = throw PendingSolution()

/** `z` died with its call frame. => "2\nError: unbound variable: z\n" */
public fun alistFreshFrameTranscript(): String = throw PendingSolution()

/** extend-environment refuses an arity mismatch.
 * => "Error: extend: wrong number of arguments, expected 2, got 1\n" */
public fun alistArityTranscript(): String = throw PendingSolution()
