// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.14

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.14: Louis installs a fixed-operation map -- a little
// primitive table that knows `square` and nothing else -- so handing it
// the compound procedure has nowhere to go: a closure value is not a
// table entry, and the call fails. Eva defines `map` over values
// instead: her applier takes the closure itself and routes through the
// kernel application, so the same lambda maps the same list.

// Exercise 4.14: a table map cannot take a procedure; a value map can.

/** Louis's map: the procedure is a table name, never a value. */
internal val LOUIS_MAP_SOURCE: String =
    """
fun louisApply(name: String, arg: GValue): GValue? {
    if (name == "square") {
        if (arg is GNumV) {
            return GNumV(arg.n * arg.n)
        }
        return null
    }
    return null
}

fun louisMap(name: String, items: List<GValue>, env: GFrame): GValue? {
    var out: List<GValue> = emptyList()
    var index = 0
    while (index < items.size) {
        val one = louisApply(name, items.get(index)) ?: return null
        val single: List<GValue> = listOf(one)
        out = out + single
        index = index + 1
    }
    return GListV(out)
}
    """.trimIndent()

/** Eva's map: the procedure is a closure value applied by the kernel. */
internal val EVA_MAP_SOURCE: String =
    """
fun evaApply(proc: GValue?, arg: GValue?): GValue? {
    if (proc is GClosV) {
        return gApply(proc, arg)
    }
    return null
}

fun evaMap(proc: GValue?, items: List<GValue>): GValue? {
    var out: List<GValue> = emptyList()
    var index = 0
    while (index < items.size) {
        val one = evaApply(proc, items.get(index)) ?: return null
        val single: List<GValue> = listOf(one)
        out = out + single
        index = index + 1
    }
    return GListV(out)
}
    """.trimIndent()

/** Louis: the known name maps; the compound procedure has no entry.
 * => "[1, 4, 9]\nerror\n" */
public fun louisTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + LOUIS_MAP_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val items = listOf(GNumV(1L), GNumV(2L), GNumV(3L))
    println(renderValue(louisMap("square", items, env)))
    println(renderValue(louisMap("lambda", items, env)))
}
                """.trimIndent(),
        ),
    )

/** Eva: the closure value maps through the kernel application.
 * => "[1, 4, 9]\n" */
public fun evaTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + EVA_MAP_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    val square = gEval(GLam("x", GMul(GVar("x"), GVar("x"))), env)
    val items = listOf(GNumV(1L), GNumV(2L), GNumV(3L))
    println(renderValue(evaMap(square, items)))
}
                """.trimIndent(),
        ),
    )
