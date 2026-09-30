// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.33

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.33: constructor data under the lazy regime. With the procedural pairs
 * installed, the head of `[a, b, c]` fails because the data is not a procedural pair;
 * the fix lifts every constructor pair into the procedural-pair expression, so
 * constructed lists are true lazy lists and the section's list operations run
 * on them.
 *
 * Expected answers: `null` plain; `a` lifted;
 * `listAt([a, b, c, d], 3)` answers `d` lifted.
 */
public fun plainQuoteCarTranscript(): String = throw PendingSolution()

public fun lazyQuoteCarTranscript(): String = throw PendingSolution()

public fun lazyQuoteListRefTranscript(): String = throw PendingSolution()
