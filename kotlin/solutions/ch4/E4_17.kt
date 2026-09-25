// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.17

package sicp.ch4.solutions

import sicp.ch4.Evaluator
import sicp.runtime.Env
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VPrimitive
import sicp.runtime.VProc
import sicp.runtime.Value

// Exercise 4.17: the extra frame. The scanned body of 4.16 lowers the
// internal defines into one `let`, and that `let`'s application pushes one
// more frame than the plain sequential rule, which defines directly in the
// parameter frame. This exercise makes the difference observable by
// counting frames live: a `frame-depth` primitive walks the environment
// chain a compound procedure captured, and the probe closes a lambda over
// the frame the body's definitions ran in. Both evaluators answer the same
// value -- the extra frame only relocates the bindings -- but the scanned
// chain is one frame longer.

/** The number of frames on the chain [procedure] captured, root included. */
private fun frameDepth(procedure: VProc): Int {
    var count = 0
    var cursor: Env? = procedure.env
    while (cursor != null) {
        count += 1
        cursor = cursor.parent
    }
    return count
}

/** The environment-inspection primitive: `(frame-depth procedure)` answers
 * the frame count of the chain the procedure captured. */
private fun frameDepthPrimitive(): Value =
    VPrimitive("frame-depth") { args ->
        val procedure =
            args.firstOrNull() as? VProc
                ?: raise(SchemeError.TypeMismatch("frame-depth of a non-procedure"))
        VInt(frameDepth(procedure).toLong())
    }

/** The probe: `f` defines `b`, closes a lambda over the frame the defines
 * ran in, and returns it; the caller checks the value and both frame
 * depths, the closure's own and a top-level lambda's anchor.
 * => 21 then 3 then 1 under the scan-out; 21 then 2 then 1 sequential */
private val PROBE: String =
    """
    (define (f x)
      (define b (+ x 1))
      (lambda () b))
    (define g (f 20))
    (g)
    (frame-depth g)
    (frame-depth (lambda () 0))
    """.trimIndent()

/** The 4.16 scan-out evaluator on the probe: the closure captures the `let`
 * frame above the parameter frame. => "21\n3\n1\n" */
public fun scannedFramesTranscript(): String =
    transcriptOn(::WithScanOut, PROBE) { env -> env.define("frame-depth", frameDepthPrimitive()) }

/** The plain evaluator on the probe: the closure captures the parameter
 * frame alone. => "21\n2\n1\n" */
public fun plainFramesTranscript(): String = transcriptOn(::Evaluator, PROBE) { env -> env.define("frame-depth", frameDepthPrimitive()) }
