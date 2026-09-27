// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.13

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.13: `make-unbound!` removes a binding from the environment.
// The solution's `WithUnbound` evaluator checks for the form in `step`
// (it arrives as an application whose operator names `make-unbound!`),
// takes the operand's name unevaluated -- the variable may be about to
// stop existing -- and drops the binding from the FIRST frame only, the
// reading that keeps `make-unbound!` a frame operation, symmetric with
// `define`. A name absent from the first frame is left alone and the form
// answers `ok`, so unbind is idempotent. Pins: unbind the global `x` and
// the next lookup faults `"Error: unbound variable: x\n"`; unbinding a
// shadowing `x` inside a body makes the outer `x` visible again, the
// global untouched; unbinding an absent name answers `"ok\n"`.

/** Unbinding removes the first frame's binding; the next lookup faults.
 * => "3\nok\nError: unbound variable: x\n" */
public fun unboundTranscript(): String = throw PendingSolution()

/** After the shadow is unbound, the outer `x` is visible again.
 * => "1\n1\n" */
public fun unboundShadowTranscript(): String = throw PendingSolution()

/** Unbinding an absent name is a no-op answering `ok`. => "ok\n" */
public fun unboundAbsentTranscript(): String = throw PendingSolution()
