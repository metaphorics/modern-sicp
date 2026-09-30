// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.12: the scan abstractions beneath lookup and `set!`.
// `frameScan` is the per-frame search -- the first `(name . value)`
// pair of one frame's alist, reusing the association-list operations
// of 4.11. `envScan` walks the chain to the first frame whose per-frame
// search hits. Lookup and `set!` become clients of the two scans, and
// the same three demos pin that nothing observable changed.

/** One per-frame search and one chain walk, over the 4.11 operations. */
internal val SCANS_SOURCE: String =
    """
fun frameScan(name: String, frame: GFrame): GValue? = alistLookup(frame.cells["alist"], name)

fun envScan(name: String, env: GFrame?): GValue? {
    if (env == null) {
        return null
    }
    val found = frameScan(name, env)
    if (found != null) {
        return found
    }
    return envScan(name, env.parent)
}

fun scannedLookup(name: String, env: GFrame?): GValue? = envScan(name, env)

fun scannedSet(env: GFrame?, name: String, value: GValue): Boolean = chainSet(env, name, value)
    """.trimIndent()

/** Define, lookup, and outward `set!` through the scan abstractions.
 * => "2\n10\n10\n" */
public fun scannedTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ALIST_SOURCE + "\n" + SCANS_SOURCE + "\n" +
                """
fun main() {
    val global = alistExtend(emptyList(), emptyList(), null)
    if (global == null) {
        println("error")
    } else {
        alistDefine(global, "z", GNumV(2L))
        println(renderValue(scannedLookup("z", global)))
        val call = alistExtend(emptyList(), emptyList(), global)
        if (call == null) {
            println("error")
        } else {
            scannedSet(call, "z", GNumV(10L))
            println(renderValue(scannedLookup("z", call)))
            println(renderValue(scannedLookup("z", global)))
        }
    }
}
                """.trimIndent(),
        ),
    )

/** A call-frame binding dies with the call; the walk stops without a hit.
 * => "2\nerror\n" */
public fun scannedFreshFrameTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ALIST_SOURCE + "\n" + SCANS_SOURCE + "\n" +
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
            println(renderValue(scannedLookup("z", call)))
            println(renderValue(scannedLookup("z", global)))
        }
    }
}
                """.trimIndent(),
        ),
    )

/** Extension still refuses an arity mismatch. => "error\n" */
public fun scannedArityTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ALIST_SOURCE + "\n" + SCANS_SOURCE + "\n" +
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
