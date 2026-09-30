// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.13

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.13: `unbind(name)` removes a binding from the kernel's
// frame. The rewrite takes the name unevaluated -- the variable may be
// about to stop existing -- and drops the binding from the FIRST frame
// only, the reading that keeps unbind a frame operation, symmetric with
// definition. A name absent from the first frame is left alone and the
// form answers the kernel's success value, so unbind is idempotent.
// Pins: unbind the global `x` and the next read answers the kernel's
// null; unbinding a shadowing `x` inside a body makes the outer `x`
// visible again, the global untouched; unbinding an absent name answers
// the success value.

/** Unbinding removes the first frame's binding; the next read faults.
 * => "3\ntrue\nnull\n" */
public fun unboundTranscript(): String = throw PendingSolution()

/** After the shadow is unbound, the outer `x` is visible again.
 * => "1\n1\n" */
public fun unboundShadowTranscript(): String = throw PendingSolution()

/** Unbinding an absent name is a no-op answering the success value.
 * => "true\n" */
public fun unboundAbsentTranscript(): String = throw PendingSolution()
