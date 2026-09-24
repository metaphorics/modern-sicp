// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.46

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * The book's test-and-set with the atomicity left out: the test and the
 * set are two separate steps, and the yield marks the point where the
 * scheduler may switch between them.
 */
public suspend fun testAndSetRacy(cell: FlagCell): Boolean = throw PendingSolution()

/**
 * The book's mutex acquire loop built on the racy test-and-set: retry,
 * yielding between attempts.
 */
public suspend fun racyMutexAcquire(cell: FlagCell): Unit = throw PendingSolution()
