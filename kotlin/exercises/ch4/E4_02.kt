// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.2

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.2, part (a): the kernel's dispatch order is the order of its
 * `gEval` clause chain. The applications-first kernel runs the application
 * arm before the binding arms, so the definition probe reaches that arm --
 * its head is the unbound operator `define`, and the kernel answers its
 * typed fault convention, null.
 *
 * Expected: the probe prints `define-as-application = null`.
 */
public fun applicationsFirstTranscript(): String = throw PendingSolution()

/**
 * Exercise 4.2, part (b): application syntax grows a distinguished `call`
 * head -- `call(fn, arg)` applies, every other node dispatches as before
 * -- so one strip-the-sugar clause keeps special forms and bare
 * applications working together. The probe applies the square both ways.
 *
 * Expected: `call = 49` and `bare = 49`.
 */
public fun callSyntaxTranscript(): String = throw PendingSolution()
