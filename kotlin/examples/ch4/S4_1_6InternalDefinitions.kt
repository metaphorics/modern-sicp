// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.6, internal definitions: mutual recursion under
// the sequential definition rule. The two names land in one frame before
// either body runs, so the procedures find each other at call time, and a
// read of a binding whose initializer has not run answers the kernel's
// null -- the premature-read fault exercise 4.16 installs in the engine.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.guest.Mode
import sicp.guest.RunResult

/** The kernel as of this section: the node family, eval, and apply. */
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
        // the sequential-definition rule: both names land in one frame
        // before either body runs, so the procedures find each other
        val frame = GFrame(mutableMapOf(), null)
        frame.cells["even?"] =
            GClosV(
                "n",
                GIf(
                    GLt(GVar("n"), GNum(1L)),
                    GBool(true),
                    GApp(GVar("odd?"), GAdd(GVar("n"), GNum(-1L))),
                ),
                frame,
            )
        frame.cells["odd?"] =
            GClosV(
                "n",
                GIf(
                    GLt(GVar("n"), GNum(1L)),
                    GBool(false),
                    GApp(GVar("even?"), GAdd(GVar("n"), GNum(-1L))),
                ),
                frame,
            )
        show("even-10", gEval(GApp(GVar("even?"), GNum(10L)), frame))
        show("even-7", gEval(GApp(GVar("even?"), GNum(7L)), frame))
        // the names live in the call frame, so repeated calls do not
        // interfere
        show("even-6", gEval(GApp(GVar("even?"), GNum(6L)), frame))
        show("even-6-again", gEval(GApp(GVar("even?"), GNum(6L)), frame))
        show("even-5", gEval(GApp(GVar("even?"), GNum(5L)), frame))
        // a read of a binding whose initializer has not run answers null
        val unassigned = GFrame(mutableMapOf("b" to GUnassigned), null)
        show("premature", gEval(GVar("b"), unassigned))
        show("unbound", gEval(GVar("a"), unassigned))
    }
    """.trimIndent()

private fun runKernel(): RunResult =
    Direct.run(KERNEL, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the kernel: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_1_6InternalDefinitionsTest :
    FunSpec({
        test("mutually recursive internal definitions work as written") {
            runKernel().output shouldBe
                "even-10 = true\n" +
                "even-7 = false\n" +
                "even-6 = true\n" +
                "even-6-again = true\n" +
                "even-5 = false\n" +
                "premature = null\n" +
                "unbound = null\n"
        }

        test("the names live in the call frame, so repeated calls do not interfere") {
            val output = runKernel().output
            output.contains("even-6 = true") shouldBe true
            output.contains("even-6-again = true") shouldBe true
            output.contains("even-5 = false") shouldBe true
        }

        test("a premature read of an uninitialized binding answers the kernel's null") {
            val output = runKernel().output
            output.contains("premature = null") shouldBe true
            output.contains("unbound = null") shouldBe true
        }
    })
