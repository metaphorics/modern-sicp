// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.46

package sicp.ch3.exercises

import kotlinx.coroutines.yield

/**
 * The book's test-and-set with the atomicity left out: the test and the
 * set are two separate steps, and the yield marks the point where the
 * scheduler may switch between them. Two processes that both pass the
 * test before either sets the cell both come away thinking they hold
 * the mutex.
 */
public suspend fun testAndSetRacy(cell: FlagCell): Boolean {
    val wasSet = cell.value
    yield()
    if (wasSet) {
        return true
    }
    cell.value = true
    return false
}

/**
 * The book's mutex acquire loop built on the racy test-and-set: retry,
 * yielding between attempts.
 */
public suspend fun racyMutexAcquire(cell: FlagCell) {
    while (testAndSetRacy(cell)) {
        yield()
    }
}
