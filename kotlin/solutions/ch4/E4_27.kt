// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.27

package sicp.ch4.solutions

import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn

// Exercise 4.27: the lazy identity observed. The delay rules give the
// sequence before any run. `(define w (id (id 10)))` applies the outer
// `id`, whose body runs at once -- compound procedures are non-strict in
// their arguments, but their bodies are entered -- so `count` climbs to 1
// and `w` is bound to the thunk of the inner `(id 10)`. Reading `count`
// answers 1. Reading `w` at the driver forces the thunk: the inner
// application runs, `count` climbs to 2, and the memoized cell stores 10.
// Reading `count` answers 2. Re-reading `w` forces the `evaluated-thunk`,
// which answers from the memoized cell: still 10, and `count` never
// climbs again.

/** The statement's interaction, memoized: `count`, `w`, `count`, then the
 * re-display that proves the memo. => "1\n10\n2\n10\n2\n" */
public fun lazyIdentityTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        (define count 0)
        (define (id x) (set! count (+ count 1)) x)
        (define w (id (id 10)))
        count
        w
        count
        w
        count
        """.trimIndent(),
    )
