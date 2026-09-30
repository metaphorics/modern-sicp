// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.17

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.17: the extra frame the scan-out installs. A closure
// carries the frame its body ran in, so counting the captured chain
// tells the two strategies apart: sequential defines run in the call
// frame itself, while the scan-out moves them one frame down into a
// fresh binding frame. Both answer the same value -- the extra frame
// only relocates the bindings -- but the scanned chain is one longer.

// Exercise 4.17: the scan-out lengthens the captured chain by one.

/** Frame-chain depth over captured closure environments. */
internal val DEPTH_SOURCE: String =
    """
fun frameDepthOf(env: GFrame?): Long {
    if (env == null) {
        return 0L
    }
    return 1L + frameDepthOf(env.parent)
}

fun closureDepth(closure: GValue?): Long {
    if (closure is GClosV) {
        return frameDepthOf(closure.env)
    }
    return 0L - 1L
}
    """.trimIndent()

/** The scan-out probe: the closure captures the fresh binding frame.
 * => "21\n3\n1\n" */
public fun scannedFramesTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + DEPTH_SOURCE + "\n" +
                """
fun main() {
    val global = GFrame(mutableMapOf<String, GValue>(), null)
    val f = GLam("x", GLet("b", GAdd(GVar("x"), GNum(1L)), GLam("u", GVar("b"))))
    val g = gEval(GApp(f, GNum(20L)), global)
    println(renderValue(gApply(g, GNumV(0L))))
    println(showLong(closureDepth(g)))
    val anchor = gEval(GLam("u", GNum(0L)), global)
    println(showLong(closureDepth(anchor)))
}
                """.trimIndent(),
        ),
    )

/** The sequential probe: the closure captures the call frame alone.
 * => "21\n2\n1\n" */
public fun plainFramesTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + DEPTH_SOURCE + "\n" +
                """
fun main() {
    val global = GFrame(mutableMapOf<String, GValue>(), null)
    val body = GBlock(listOf(GVarStmt("b", GAdd(GVar("x"), GNum(1L))), GExprStmt(GLam("u", GVar("b")))))
    val f = GLam("x", body)
    val g = gEval(GApp(f, GNum(20L)), global)
    println(renderValue(gApply(g, GNumV(0L))))
    println(showLong(closureDepth(g)))
    val anchor = gEval(GLam("u", GNum(0L)), global)
    println(showLong(closureDepth(anchor)))
}
                """.trimIndent(),
        ),
    )
