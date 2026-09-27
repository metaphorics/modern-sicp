// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.52

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.52: `if-fail`, which catches the failure of its first
 * expression and answers its second instead. The guard is a frame on
 * the choice stack, so it fires only when every choice of the first
 * expression is exhausted -- which is why the all-odd session answers
 * `all-odd` and then finds no more values, while the session with 8
 * available answers 8, then `all-odd` on the next try-again, then
 * finds no more values.
 */
public fun ifFailAllOddTranscript(): String = throw PendingSolution()

public fun ifFailEightTranscript(): String = throw PendingSolution()
