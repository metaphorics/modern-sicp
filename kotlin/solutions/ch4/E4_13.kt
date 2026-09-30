// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.13

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.13: `make-unbound!` removes a binding from the environment.
// The removal touches the first frame only -- the reading that keeps
// unbind a frame operation, symmetric with `define`, which also binds
// one frame. An absent name removes nothing and still answers `ok`, so
// unbind is idempotent; the shadow probe shows the outer binding visible
// again after the shadow is removed.

/** First-frame-only removal over the kernel frames. */
internal val UNBOUND_SOURCE: String =
    """
fun makeUnbound(env: GFrame, name: String): GValue? = env.cells.remove(name)
    """.trimIndent()

/** Unbind the global `x`, then fail to find it. => "3\nok\nerror\n" */
public fun unboundTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + UNBOUND_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("x" to GNumV(3L)), null)
    println(renderValue(gEval(GVar("x"), env)))
    makeUnbound(env, "x")
    println("ok")
    println(renderValue(gEval(GVar("x"), env)))
}
                """.trimIndent(),
        ),
    )

/** Unbind a shadowing `x`: the outer binding shows through again, and
 * the global was never touched. => "1\n1\n" */
public fun unboundShadowTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + UNBOUND_SOURCE + "\n" +
                """
fun main() {
    val global = GFrame(mutableMapOf<String, GValue>("x" to GNumV(1L)), null)
    val call = GFrame(mutableMapOf<String, GValue>("x" to GNumV(2L)), global)
    makeUnbound(call, "x")
    println(renderValue(gEval(GVar("x"), call)))
    println(renderValue(gEval(GVar("x"), global)))
}
                """.trimIndent(),
        ),
    )

/** An absent name unbinds to `ok`; the frame is unchanged. => "ok\n" */
public fun unboundAbsentTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + UNBOUND_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>(), null)
    makeUnbound(env, "nowhere")
    println("ok")
}
                """.trimIndent(),
        ),
    )
