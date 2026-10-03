// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.27

package sicp.ch4.solutions

import sicp.ch4.LazyModule

// Exercise 4.27: the lazy identity observed. The delay rules give the
// sequence before any run. `outer` applies at once -- a compound
// procedure's body is entered, its unannotated parameter arriving as a
// transparent thunk -- so `count` climbs to 1 and `w` is bound to the
// delayed argument itself, still unforced, because returning a parameter
// hands back the thunk the delay built. Reading `w` through `force` runs
// the inner computation: `count` climbs to 2 and the memoized cell stores
// 10. Re-reading `w` forces the memo, which answers from the stored value:
// still 10, and `count` never climbs again. The book's `w (id (id 10))`
// maps onto `outer(thunk { id(10) })` here: the outer application enters
// immediately while the inner application stays behind the thunk.

/** The statement's interaction, memoized: `count`, the forced value,
 * `count`, then the re-force that proves the memo.
 * => "1\n10\n2\n10\n2\n" */
internal val LAZY_IDENTITY_PROGRAM: String =
    """
var count: Long = 0L

fun id(x: Long): Long {
    count = count + 1L
    return x
}

fun outer(t: Thunk<Long>): Thunk<Long> {
    count = count + 1L
    return t
}

fun main() {
    val w = outer(thunk { id(10L) })
    println(count)
    println(force(w))
    println(count)
    println(force(w))
    println(count)
}
    """.trimIndent()

/** The lazy identity session under the forcing instrument.
 * => "1\n10\n2\n10\n2\n" */
public fun lazyIdentityTranscript(): String = outcomeText(LazyModule.run(LAZY_IDENTITY_PROGRAM).map { it.result })
