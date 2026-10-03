// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1
// Chapter 4, section 4.1.1, the core of the evaluator: eval as the clause
// chain over the typed node family, apply over compound procedures, and
// the operand order the kernel's code fixes. The book's unknown-expression
// fault cannot fire in this edition -- a malformed form never becomes a
// typed node -- so the equivalent failure is the kernel's null answer for
// an ill-shaped or unbound operand.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.guest.Admission
import sicp.guest.GValue
import sicp.guest.Mode
import sicp.guest.RunResult

/** The kernel unit of the section: the node family of 4.1.2, the eval
 * clause chain, apply, and the explicit operand walk. */
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

    fun listOfValues(operands: List<GExpr>, env: GFrame): List<GValue>? {
        if (operands.isEmpty()) return emptyList()
        val first = gEval(operands.get(0), env) ?: return null
        val rest = listOfValues(operands.drop(1), env) ?: return null
        return listOf(first) + rest
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
        val global = GFrame(mutableMapOf(), null)
        // primitives and compound procedures compose
        show("composed", gEval(GAdd(GNum(2L), GMul(GNum(3L), GNum(4L))), global))
        // let binds, and assignment rebinds where the name lives
        show(
            "assigned",
            gEval(
                GLet("x", GNum(10L), GLet("step", GSet("x", GAdd(GVar("x"), GNum(5L))), GVar("x"))),
                global,
            ),
        )
        // let is a derived expression over lambda: both spellings agree
        show("let", gEval(GLet("x", GNum(3L), GLet("y", GNum(4L), GAdd(GVar("x"), GVar("y")))), global))
        show(
            "lambda",
            gEval(
                GApp(GLam("x", GApp(GLam("y", GAdd(GVar("x"), GVar("y"))), GNum(4L))), GNum(3L)),
                global,
            ),
        )
        // if skips the untaken branch, so an unbound arm never reads
        show("taken", gEval(GIf(GBool(true), GNum(1L), GVar("missing")), global))
        show("untaken", gEval(GIf(GBool(false), GVar("missing"), GNum(2L)), global))
        // an ill-shaped or unbound operand answers null, the kernel's fault
        show("unbound", gEval(GVar("missing"), global))
        // operand order is the kernel's statement order: the note records
        // 1 then 2, so the recorded order reads 12
        val frame = GFrame(mutableMapOf("order" to GNumV(0L)), null)
        val note =
            GClosV(
                "x",
                GLet("saved", GSet("order", GAdd(GMul(GVar("order"), GNum(10L)), GVar("x"))), GVar("x")),
                frame,
            )
        frame.cells["note"] = note
        val operands = listOf(GApp(GVar("note"), GNum(1L)), GApp(GVar("note"), GNum(2L)))
        val values = listOfValues(operands, frame) ?: emptyList()
        for (value in values) {
            show("operand", value)
        }
        show("order", frame.lookup("order"))
    }
    """.trimIndent()

/** Runs the kernel unit as a core program, asserting admission. */
private fun runKernel(): RunResult =
    Direct.run(KERNEL, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the kernel: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_1_1EvalApplyTest :
    FunSpec({
        test("the kernel unit admits as guest source") {
            val checked =
                Admission.admit(KERNEL, Mode.CORE).fold(
                    { e -> throw AssertionError("rejected: ${e.category}: ${e.message}") },
                    { it },
                )
            checked.mode shouldBe Mode.CORE
        }

        test("primitives and compound procedures compose") {
            runKernel().output shouldBe
                "composed = 14\n" +
                "assigned = 15\n" +
                "let = 7\n" +
                "lambda = 7\n" +
                "taken = 1\n" +
                "untaken = 2\n" +
                "unbound = null\n" +
                "operand = 1\n" +
                "operand = 2\n" +
                "order = 12\n"
        }

        test("the untaken branch never evaluates its arm") {
            val output = runKernel().output
            output.contains("missing") shouldBe false
            output.contains("taken = 1") shouldBe true
            output.contains("untaken = 2") shouldBe true
        }

        test("let and its lambda rewriting answer the same value") {
            val output = runKernel().output
            output.contains("let = 7") shouldBe true
            output.contains("lambda = 7") shouldBe true
        }

        test("the operand walk records the kernel's statement order") {
            val output = runKernel().output
            output.contains("operand = 1\noperand = 2") shouldBe true
            output.contains("order = 12") shouldBe true
        }

        test("an unbound name answers the kernel's null, not a value") {
            runKernel().output.contains("unbound = null") shouldBe true
        }
    })
