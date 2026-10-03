// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.34

package sicp.ch4.solutions

import sicp.ch4.LazyModule

// Exercise 4.34: printing lazy pairs. The printer walks explicit
// `LazyCell` spines, demanding one tail between printed heads. It
// stops at its budget before forcing the next tail, so printing an
// infinite list terminates without demanding beyond the displayed
// prefix. Pair dotted rendering remains in the core kernel; this lazy
// list spine has only list-valued tails.

/** The bounded printer over an explicit lazy spine. */
internal val PRINT_SOURCE: String =
    LAZY_DATA_SOURCE + "\n" +
        """
fun renderLazy(node: LazyData, budget: Int): String {
    var current = node
    var index = 0
    var out = "("
    while (current is LazyCell && index < budget) {
        if (index > 0) {
            out = out + " "
        }
        out = out + renderData(current.head, budget)
        index = index + 1
        if (index < budget) {
            current = force(current.tail)
        }
    }
    if (current is LazyEnd) {
        return out + ")"
    }
    if (index >= budget) {
        return out + " ...)"
    }
    return out + " . " + renderAtom(current) + ")"
}

fun renderData(node: LazyData, budget: Int): String =
    if (node is LazyCell) renderLazy(node, budget) else renderAtom(node)
        """.trimIndent()

/** The shared infinite `ones` spine, without the data prelude. */
internal val ONES_SPINE_SOURCE: String =
    """
var onesData: LazyData = LazyEnd

fun shareOnes(): LazyData {
    val first = LazyCell(LazyAtom("1"), thunk { onesData })
    onesData = first
    return first
}
    """.trimIndent()

/** The spine plus the lazy-list data it names, for printer-free runs. */
internal val ONES_PRINT_SOURCE: String =
    LAZY_DATA_SOURCE + "\n" + ONES_SPINE_SOURCE

/** A finite list prints whole. */
public fun lazyProperPrintTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                PRINT_SOURCE + "\n" +
                    """
fun main() {
    fun properEnd(): LazyData = LazyEnd
    fun properSecond(): LazyData = LazyCell(LazyAtom("2"), thunk { properEnd() })
    val proper = LazyCell(LazyAtom("1"), thunk { properSecond() })
    println(renderLazy(proper, 10))
}
                    """.trimIndent(),
            ).map { it.result },
    )

/** The infinite list prints ten elements then an ellipsis, without
 * demanding the eleventh tail. */
public fun onesBudgetPrintTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                PRINT_SOURCE + "\n" + ONES_SPINE_SOURCE + "\n" +
                    """
fun main() {
    println(renderLazy(shareOnes(), 10))
}
                    """.trimIndent(),
            ).map { it.result },
    )

/** The head answers without printing or forcing the list. */
public fun carOfOnesTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                ONES_PRINT_SOURCE + "\n" +
                    """
fun main() {
    val ones = shareOnes()
    if (ones is LazyCell) {
        println(renderAtom(ones.head))
    }
}
                    """.trimIndent(),
            ).map { it.result },
    )

/** Nested lazy pairs print recursively. */
public fun nestedLazyPrintTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                PRINT_SOURCE + "\n" +
                    """
fun main() {
    fun tailEnd(): LazyData = LazyEnd
    val first = LazyCell(LazyAtom("1"), thunk { tailEnd() })
    val second = LazyCell(LazyAtom("2"), thunk { tailEnd() })
    fun tailSecond(): LazyData = LazyCell(second, thunk { tailEnd() })
    val outer = LazyCell(first, thunk { tailSecond() })
    println(renderLazy(outer, 10))
}
                    """.trimIndent(),
            ).map { it.result },
    )
