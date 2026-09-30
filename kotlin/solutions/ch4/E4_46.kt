// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.46

package sicp.ch4.solutions

// Exercise 4.46: left-to-right operands in the search. The parser relies
// on it: `parse-verb-phrase` must consume the verb before it tries to
// extend, so the operands of an application are evaluated left to right.
// The probe is the property itself: two choice points in one attempt
// enumerate the pairs with the first operand outermost.

/** Two nested choices enumerate in operand order. */
internal val OPERAND_ORDER_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun operandOrder(): Unit {
    val x = choose(1L, 2L)
    val y = choose(3L, 4L)
    println(showPair(x, y))
}

fun main() {
    budgetCap = 1000000L
    operandOrder()
}
        """.trimIndent()

/** The pairs in the search's operand order.
 * => [(1 3), (1 4), (2 3), (2 4)] */
public fun operandOrderEnumeration(): List<String> = searchLines(OPERAND_ORDER_PROGRAM)
