// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.32

package sicp.ch4.solutions

import sicp.ch4.Direct
import sicp.ch4.LazyModule

// Exercise 4.32: the extra laziness. A cell carries its head directly
// and delays its tail in a thunk. The guest List accessors materialize
// the entire list, so a head-only observation walks this spine and
// never calls force on an undemanded tail. A self-reference is one
// cell whose tail closes over the shared `onesData` binding.

/** An armed tail is skipped until the spine walk forces it. */
internal val LAZY_SLOTS_PROGRAM: String =
    LAZY_DATA_SOURCE + "\n" +
        """
fun main() {
    val pair = LazyCell(LazyAtom("7"), thunk { explode() })
    println(renderAtom(pair.head))
    force(pair.tail)
}
        """.trimIndent()

/** The strict constructor evaluates every slot at construction. */
internal val EAGER_CONSTRUCTOR_PROGRAM: String =
    """
fun main() {
    val pair = listOf(7L, 1L / 0L)
    println(pair.get(0))
}
    """.trimIndent()

/** The infinite list's head answers without forcing its cyclic tail. */
internal val ONES_PROGRAM: String =
    LAZY_DATA_SOURCE + "\n" +
        """
var onesData: LazyData = LazyEnd

fun main() {
    val first = LazyCell(LazyAtom("1"), thunk { onesData })
    onesData = first
    println(renderAtom(first.head))
}
        """.trimIndent()

/** Ten heads of the infinite list, forcing tails only between heads. */
internal val ONES_HEADS_PROGRAM: String =
    LAZY_DATA_SOURCE + "\n" +
        """
var onesData: LazyData = LazyEnd

fun showHeads(node: LazyData, count: Int): String {
    var current = node
    var index = 0
    var out = "["
    while (index < count) {
        if (current !is LazyCell) {
            return out + "]"
        }
        if (index > 0) {
            out = out + ", "
        }
        out = out + renderAtom(current.head)
        index = index + 1
        if (index < count) {
            current = force(current.tail)
        }
    }
    return out + "]"
}

fun main() {
    val first = LazyCell(LazyAtom("1"), thunk { onesData })
    onesData = first
    println(showHeads(first, 10))
}
        """.trimIndent()

/** A strict self-reference is outside the admitted core subset. */
internal val STRICT_ONES_PROGRAM: String =
    """
fun main() {
    val ones: List<Long> = listOf(1L) + ones
    println(ones.get(0))
}
    """.trimIndent()

public fun lazyPairSlotsTranscript(): String = outcomeText(LazyModule.run(LAZY_SLOTS_PROGRAM).map { it.result })

public fun eagerConstructorTranscript(): String = outcomeText(Direct.run(EAGER_CONSTRUCTOR_PROGRAM))

public fun onesOneStepTranscript(): String = outcomeText(LazyModule.run(ONES_PROGRAM).map { it.result })

public fun onesHeadsTranscript(): String = outcomeText(LazyModule.run(ONES_HEADS_PROGRAM).map { it.result })

public fun strictOnesTranscript(): String = outcomeText(Direct.run(STRICT_ONES_PROGRAM))
