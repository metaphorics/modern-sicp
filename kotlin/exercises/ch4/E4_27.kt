// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.27

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.27: the lazy identity observed. The delay rules give the
 * sequence: binding `w` enters the outer `id` (count 1) and stores the
 * inner thunk; reading `w` at the driver forces it (count 2) into the
 * memoized cell; re-reading `w` answers from the cell without counting.
 *
 * Expected answer: the transcript is `1`, `10`, `2`, then the re-display
 * `10` with `count` still 2.
 */
public fun lazyIdentityTranscript(): String = throw PendingSolution()
