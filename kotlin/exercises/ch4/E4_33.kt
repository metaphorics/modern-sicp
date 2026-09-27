// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.33

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.33: quotes under the lazy regime. With the procedural pairs
 * installed, `(car '(a b c))` fails because the quote is ordinary data;
 * the fix lifts every quoted pair into the procedural-pair expression, so
 * quoted lists are true lazy lists and the section's list operations run
 * on them.
 *
 * Expected answers: `Error: not a procedure: (a b c)` plain; `a` lifted;
 * `(list-ref '(a b c d) 3)` answers `d` lifted.
 */
public fun plainQuoteCarTranscript(): String = throw PendingSolution()

public fun lazyQuoteCarTranscript(): String = throw PendingSolution()

public fun lazyQuoteListRefTranscript(): String = throw PendingSolution()
