// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.17

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.17: the extra frame. The scanned body of 4.16 lowers the
 * internal defines into one `GLet`, and that application pushes one
 * more frame than the plain sequential rule, which defines directly in the
 * parameter frame: at the point marked `<e3>` in the statement, the
 * environment structure differs by exactly that `let` frame. Behavior
 * cannot differ -- the extra frame only relocates the bindings, and every
 * lookup and assignment find the same values in the same order.
 *
 * Expected answers: both evaluators answer 21 for `(f 20)`; a `frame-depth`
 * inspection of the closure `f` returns counts the scanned chain at 3
 * frames (let frame, parameter frame, global) and the plain chain at 2,
 * with a top-level procedure anchoring the counter at 1.
 */
public fun scannedFramesTranscript(): String = throw PendingSolution()

public fun plainFramesTranscript(): String = throw PendingSolution()
