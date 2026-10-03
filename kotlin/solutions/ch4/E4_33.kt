// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.33

package sicp.ch4.solutions

import sicp.ch4.LazyModule

// Exercise 4.33: quotes under the lazy regime. The ordinary data
// literal evaluates every slot at construction, so an armed slot dies
// there; the lifted quote builds the same list from explicit lazy
// cells, so each tail waits behind its thunk and list operations run
// one demand at a time. Quoted lists are lazy lists because their quote
// recursively builds the section's `LazyCell` spine.

/** The lifted quote and list operations, over the explicit lazy spine. */
internal val QUOTED_SOURCE: String =
    LAZY_DATA_SOURCE + "\n" +
        """
fun tailE(): LazyData = LazyEnd

fun tailD(): LazyData = LazyCell(LazyAtom("d"), thunk { tailE() })

fun tailC(): LazyData = LazyCell(LazyAtom("c"), thunk { tailD() })

fun tailB(): LazyData = LazyCell(LazyAtom("b"), thunk { tailC() })

fun quoted(): LazyData =
    LazyCell(LazyAtom("a"), thunk { tailB() })

fun car(node: LazyData): String = if (node is LazyCell) renderAtom(node.head) else "error"

fun listRef(node: LazyData, index: Int): String {
    var current = node
    var at = 0
    while (at < index) {
        if (current !is LazyCell) {
            return "error"
        }
        current = force(current.tail)
        at = at + 1
    }
    if (current is LazyCell) {
        return renderAtom(current.head)
    }
    return "error"
}
        """.trimIndent()

/** An ordinary strict literal is rejected where a lazy-spine value is required. */
public fun plainArmedQuoteTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                """
fun main() {
    val literal = listOf(1L, 1L / 0L)
    println(literal.get(0))
}
                """.trimIndent(),
            ).map { it.result },
    )

/** The lifted quote answers its head without forcing a tail. */
public fun lazyQuoteCarTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                QUOTED_SOURCE + "\n" +
                    """
fun main() {
    println(car(quoted()))
}
                    """.trimIndent(),
            ).map { it.result },
    )

/** The list operation reaches `d` by forcing exactly three tails. */
public fun lazyQuoteListRefTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                QUOTED_SOURCE + "\n" +
                    """
fun main() {
    println(listRef(quoted(), 3))
}
                    """.trimIndent(),
            ).map { it.result },
    )

/** An armed lazy tail survives construction and answers its head. */
public fun lazyArmedQuoteTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                LAZY_DATA_SOURCE + "\n" +
                    """
fun main() {
    val armed = LazyCell(LazyAtom("1"), thunk { explode() })
    println(renderAtom(armed.head))
}
                    """.trimIndent(),
            ).map { it.result },
    )
