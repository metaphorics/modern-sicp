// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.5, data as programs: the evaluator as a universal
// machine. The factorial program is ordinary node data -- the example
// takes it apart field by field -- then feeds the very same structure to
// the kernel and watches it compute.

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

    /** The factorial program as constructed data. */
    fun factorialProgram(n: Long): GExpr =
        GLetRec(
            "fact",
            GLam(
                "n",
                GIf(
                    GLt(GVar("n"), GNum(2L)),
                    GNum(1L),
                    GMul(GVar("n"), GApp(GVar("fact"), GAdd(GVar("n"), GNum(-1L)))),
                ),
            ),
            GApp(GVar("fact"), GNum(n)),
        )

    fun main() {
        // the program is data: its pieces are node fields
        val program = factorialProgram(5L)
        if (program is GLetRec) {
            println("name = " + program.name)
            val procedure = program.value
            if (procedure is GLam) {
                println("param = " + procedure.param)
            }
        }
        // data and programs are the same stuff, both directions: the very
        // same structure computes
        show("factorial-5", gEval(factorialProgram(5L), GFrame(mutableMapOf(), null)))
        show("factorial-10", gEval(factorialProgram(10L), GFrame(mutableMapOf(), null)))
    }
    """.trimIndent()

private fun runKernel(): RunResult =
    Direct.run(KERNEL, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the kernel: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_1_5DataAsProgramsTest :
    FunSpec({
        test("the factorial program is data: its pieces are node fields") {
            val output = runKernel().output
            output.contains("name = fact") shouldBe true
            output.contains("param = n") shouldBe true
        }

        test("feed the program to the kernel and it computes") {
            runKernel().output shouldBe
                "name = fact\n" +
                "param = n\n" +
                "factorial-5 = 120\n" +
                "factorial-10 = 3628800\n"
        }

        test("data and programs are the same structure, both directions") {
            val output = runKernel().output
            // the inspected structure and the computed one are built by
            // the same constructor call
            output.contains("factorial-5 = 120") shouldBe true
            output.contains("factorial-10 = 3628800") shouldBe true
        }
    })
