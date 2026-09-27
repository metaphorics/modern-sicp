// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12

package sicp.ch4.exercises

import sicp.runtime.Env
import sicp.runtime.PendingSolution
import sicp.runtime.VPair

// Exercise 4.12: the book asks for abstractions that capture the pattern
// beneath `lookup-variable-value` and `set-variable-value!`: one
// per-frame search and one frame-chain walk, with the environment
// operations redefined on top. [frameScan] is the `assoc` over one
// frame's alist representation (Exercise 4.11's), [envScan] walks the
// chain to the first frame whose [frameScan] hits, and the solution's
// `ScannedFrames` evaluator defines lookup and `set!` purely on the two
// scans; `define` conses without scanning. Same pins as 4.11: 2, then 10
// through an inner frame's `set!`, and `"Error: unbound variable: z\n"`
// for the binding that died with its call frame.

/** The per-frame search: the first `(name . value)` pair of [env]'s
 * alist frame, or null. */
public fun frameScan(
    name: String,
    env: Env,
): VPair? = throw PendingSolution()

/** The frame-chain walk: the nearest `(name . value)` pair on the chain,
 * or null when no frame holds [name]. */
public fun envScan(
    name: String,
    env: Env,
): VPair? = throw PendingSolution()

/** The define/lookup/set! demo through the scan abstractions.
 * => "2\n10\n10\n" */
public fun scannedTranscript(): String = throw PendingSolution()

/** `z` died with its call frame. => "2\nError: unbound variable: z\n" */
public fun scannedFreshFrameTranscript(): String = throw PendingSolution()

/** extend-environment refuses an arity mismatch.
 * => "Error: extend: wrong number of arguments, expected 2, got 1\n" */
public fun scannedArityTranscript(): String = throw PendingSolution()
