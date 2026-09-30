// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.3, evaluator data structures: procedures and
// environments. Frames are the kernel's capture-shared cell maps: a
// definition adds to the first frame, assignment rebinds where the name
// lives, shadowing leaves the outer binding alone, and a compound
// procedure carries the environment it was defined in.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.guest.Mode
import sicp.guest.RunResult

/** The kernel as of this section: the node family, the value and frame
 * data structures, eval, and apply. */
private val KERNEL: String =
    """
    sealed interface GExpr

    data class GNum(val n: Long) : GExpr
    data class GBool(val b: Boolean) : GExpr
    data class GVar(val name: String) : GExpr
    data class GLam(val param: String, val body: GExpr) : GExpr
    data class GApp(val fn: GExpr, val arg: GExpr) : GExpr
    data class GLet(val name: String, val value: GExpr, val body: GExpr) : GExpr
    data class GLetRec(val name: String, val value: GExpr, val body: GExpr) : GExpr
    data class GIf(val test: GExpr, val onTrue: GExpr, val onFalse: GExpr) : GExpr
    data class GAdd(val left: GExpr, val right: GExpr) : GExpr
    data class GMul(val left: GExpr, val right: GExpr) : GExpr
    data class GLt(val left: GExpr, val right: GExpr) : GExpr
    data class GSet(val name: String, val value: GExpr) : GExpr

    sealed interface GValue

    data class GNumV(val n: Long) : GValue
    data class GBoolV(val b: Boolean) : GValue
    data class GClosV(val param: String, val body: GExpr, val env: GFrame) : GValue
    data object GUnassigned : GValue

    class GFrame(val cells: MutableMap<String, GValue>, val parent: GFrame?) {
        fun lookup(name: String): GValue? {
            var here: GFrame? = this
            while (here is GFrame) {
                val current: GFrame = here
                val hit = current.cells[name]
                if (hit != null) {
                    return hit
                }
                here = current.parent
            }
            return null
        }

        fun assign(name: String, value: GValue): Boolean {
            var here: GFrame? = this
            while (here is GFrame) {
                val current: GFrame = here
                if (current.cells.containsKey(name)) {
                    current.cells[name] = value
                    return true
                }
                here = current.parent
            }
            return false
        }
    }

    fun gEval(expr: GExpr, env: GFrame): GValue? =
        when (expr) {
            is GNum -> GNumV(expr.n)
            is GBool -> GBoolV(expr.b)
            is GVar -> {
                val found = env.lookup(expr.name)
                if (found == null || found is GUnassigned) {
                    null
                } else {
                    found
                }
            }
            is GLam -> GClosV(expr.param, expr.body, env)
            is GApp -> {
                val fn = gEval(expr.fn, env) ?: return null
                val arg = gEval(expr.arg, env) ?: return null
                gApply(fn, listOf(arg))
            }
            is GLet -> {
                val value = gEval(expr.value, env) ?: return null
                val frame = GFrame(mutableMapOf(expr.name to value), env)
                gEval(expr.body, frame)
            }
            is GLetRec -> {
                val frame = GFrame(mutableMapOf(expr.name to GUnassigned), env)
                val value = gEval(expr.value, frame) ?: return null
                frame.cells[expr.name] = value
                gEval(expr.body, frame)
            }
            is GIf -> {
                val test = gEval(expr.test, env) ?: return null
                if (test !is GBoolV) {
                    null
                } else if (test.b) {
                    gEval(expr.onTrue, env)
                } else {
                    gEval(expr.onFalse, env)
                }
            }
            is GAdd -> {
                val left = gEval(expr.left, env) ?: return null
                val right = gEval(expr.right, env) ?: return null
                if (left is GNumV && right is GNumV) {
                    GNumV(left.n + right.n)
                } else {
                    null
                }
            }
            is GMul -> {
                val left = gEval(expr.left, env) ?: return null
                val right = gEval(expr.right, env) ?: return null
                if (left is GNumV && right is GNumV) {
                    GNumV(left.n * right.n)
                } else {
                    null
                }
            }
            is GLt -> {
                val left = gEval(expr.left, env) ?: return null
                val right = gEval(expr.right, env) ?: return null
                if (left is GNumV && right is GNumV) {
                    GBoolV(left.n < right.n)
                } else {
                    null
                }
            }
            is GSet -> {
                val value = gEval(expr.value, env) ?: return null
                if (env.assign(expr.name, value)) {
                    GBoolV(true)
                } else {
                    null
                }
            }
        }

    fun gApply(fn: GValue, args: List<GValue>): GValue? {
        if (fn !is GClosV) return null
        if (args.size != 1) return null
        val frame = GFrame(mutableMapOf(fn.param to args.get(0)), fn.env)
        return gEval(fn.body, frame)
    }

    fun render(value: GValue): String =
        when (value) {
            is GNumV -> "${'$'}{value.n}"
            is GBoolV -> "${'$'}{value.b}"
            is GClosV -> "closure"
            is GUnassigned -> "unassigned"
        }

    fun show(label: String, value: GValue?): Unit {
        if (value == null) {
            println(label + " = null")
        } else {
            println(label + " = " + render(value))
        }
    }

    fun main() {
        // a definition adds to the first frame, shadowing leaves the
        // outer binding alone
        val outer = GFrame(mutableMapOf("x" to GNumV(10L)), null)
        val inner = GFrame(mutableMapOf(), outer)
        show("outer", outer.lookup("x"))
        inner.cells["x"] = GNumV(20L)
        show("inner", inner.lookup("x"))
        show("outer-shadowed", outer.lookup("x"))
        // assignment rebinds where the name lives
        inner.assign("x", GNumV(30L))
        show("inner-assigned", inner.lookup("x"))
        show("outer-untouched", outer.lookup("x"))
        // a frame without the binding walks outward
        val deepest = GFrame(mutableMapOf(), inner)
        deepest.assign("x", GNumV(40L))
        show("walked-out", deepest.lookup("x"))
        show("owning-frame", inner.lookup("x"))
        // a lookup that walks off the chain answers null
        show("missing", outer.lookup("no-such-name"))
        // applying a procedure binds its parameter over the environment
        // it carries: the captured n is still 5 wherever the call starts
        val global = GFrame(mutableMapOf("n" to GNumV(5L)), null)
        val closure = GClosV("x", GAdd(GVar("x"), GVar("n")), global)
        show("applied", gApply(closure, listOf(GNumV(37L))))
        // the closure's own environment wins over the caller's frame
        val deep = GFrame(mutableMapOf("adder" to closure), GFrame(mutableMapOf("n" to GNumV(99L)), null))
        show("from-deep-frame", gEval(GApp(GVar("adder"), GNum(37L)), deep))
    }
    """.trimIndent()

private fun runKernel(): RunResult =
    Direct.run(KERNEL, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the kernel: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_1_3FramesTest :
    FunSpec({
        test("define adds to the first frame and assignment rebinds where the name lives") {
            runKernel().output shouldBe
                "outer = 10\n" +
                "inner = 20\n" +
                "outer-shadowed = 10\n" +
                "inner-assigned = 30\n" +
                "outer-untouched = 10\n" +
                "walked-out = 40\n" +
                "owning-frame = 40\n" +
                "missing = null\n" +
                "applied = 42\n" +
                "from-deep-frame = 42\n"
        }

        test("a lookup that walks off the chain answers the kernel's null") {
            runKernel().output.contains("missing = null") shouldBe true
        }

        test("a procedure carries its defining environment to every call") {
            val output = runKernel().output
            output.contains("applied = 42") shouldBe true
            output.contains("from-deep-frame = 42") shouldBe true
        }

        test("shadowing leaves the outer binding alone while assignment walks outward") {
            val output = runKernel().output
            output.contains("outer-shadowed = 10") shouldBe true
            output.contains("outer-untouched = 10") shouldBe true
            output.contains("owning-frame = 40") shouldBe true
        }
    })
