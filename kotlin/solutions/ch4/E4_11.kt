// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.11

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.11: frames as association lists. The kernel threads
// `GFrame` maps through every call; this exercise replays the same
// four environment operations over frames built as data -- one
// `(name . value)` pair list per frame, pairs as kernel `Pair` objects.
// `define` conses a fresh pair onto the front, `set!` scans outward
// and rewrites the nearest pair's value slot, lookup answers the
// first hit, and extension checks arity before building the frame.

/** The alist frame operations over kernel pair objects. */
internal val ALIST_SOURCE: String =
    """
fun alistLookup(alist: GValue?, name: String): GValue? {
    if (alist is GListV && !alist.items.isEmpty()) {
        val binding = alist.items.get(0)
        if (binding is GObjectV && binding.className == "Pair") {
            val key = binding.fields["first"]
            if (key is GStrV && key.text == name) {
                return binding.fields["second"]
            }
        }
        return alistLookup(GListV(alist.items.drop(1)), name)
    }
    return null
}

fun chainLookup(env: GFrame?, name: String): GValue? {
    if (env == null) {
        return null
    }
    val found = alistLookup(env.cells["alist"], name)
    if (found != null) {
        return found
    }
    return chainLookup(env.parent, name)
}

fun alistDefine(env: GFrame, name: String, value: GValue): GValue {
    val old = env.cells["alist"]
    if (old is GListV) {
        val pair = GObjectV("Pair", mutableMapOf("first" to GStrV(name), "second" to value))
        val one: List<GValue> = listOf(pair)
        env.cells["alist"] = GListV(one + old.items)
        return value
    }
    return GUnitV
}

fun alistSet(alist: GValue?, name: String, value: GValue): Boolean {
    if (alist is GListV && !alist.items.isEmpty()) {
        val binding = alist.items.get(0)
        if (binding is GObjectV && binding.className == "Pair") {
            val key = binding.fields["first"]
            if (key is GStrV && key.text == name) {
                binding.fields["second"] = value
                return true
            }
        }
        return alistSet(GListV(alist.items.drop(1)), name, value)
    }
    return false
}

fun chainSet(env: GFrame?, name: String, value: GValue): Boolean {
    if (env == null) {
        return false
    }
    if (alistSet(env.cells["alist"], name, value)) {
        return true
    }
    return chainSet(env.parent, name, value)
}

fun alistExtend(names: List<String>, values: List<GValue>, parent: GFrame?): GFrame? {
    if (names.size != values.size) {
        return null
    }
    var alist: List<GValue> = emptyList()
    var index = names.size - 1
    while (index >= 0) {
        val pair = GObjectV("Pair", mutableMapOf("first" to GStrV(names.get(index)), "second" to values.get(index)))
        val one: List<GValue> = listOf(pair)
        alist = one + alist
        index = index - 1
    }
    return GFrame(mutableMapOf<String, GValue>("alist" to GListV(alist)), parent)
}
    """.trimIndent()

/** Define, lookup, and outward `set!` over alist frames. => "2\n10\n10\n" */
public fun alistTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ALIST_SOURCE + "\n" +
                """
fun main() {
    val global = alistExtend(emptyList(), emptyList(), null)
    if (global == null) {
        println("error")
    } else {
        alistDefine(global, "z", GNumV(2L))
        println(renderValue(chainLookup(global, "z")))
        val call = alistExtend(emptyList(), emptyList(), global)
        if (call == null) {
            println("error")
        } else {
            chainSet(call, "z", GNumV(10L))
            println(renderValue(chainLookup(call, "z")))
            println(renderValue(chainLookup(global, "z")))
        }
    }
}
                """.trimIndent(),
        ),
    )

/** A call-frame binding dies with the call: the global chain misses.
 * => "2\nerror\n" */
public fun alistFreshFrameTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ALIST_SOURCE + "\n" +
                """
fun main() {
    val global = alistExtend(emptyList(), emptyList(), null)
    if (global == null) {
        println("error")
    } else {
        val call = alistExtend(emptyList(), emptyList(), global)
        if (call == null) {
            println("error")
        } else {
            alistDefine(call, "z", GNumV(2L))
            println(renderValue(chainLookup(call, "z")))
            println(renderValue(chainLookup(global, "z")))
        }
    }
}
                """.trimIndent(),
        ),
    )

/** Extension still refuses an arity mismatch. => "error\n" */
public fun alistArityTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ALIST_SOURCE + "\n" +
                """
fun main() {
    val global = alistExtend(emptyList(), emptyList(), null)
    val bad = alistExtend(listOf("x", "y"), listOf(GNumV(1L)), global)
    if (bad == null) {
        println("error")
    } else {
        println("accepted")
    }
}
                """.trimIndent(),
        ),
    )
