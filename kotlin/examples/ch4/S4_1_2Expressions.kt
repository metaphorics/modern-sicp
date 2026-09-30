// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.2, representing expressions: the typed node family
// the kernel evaluates, `when` to `GIf` nests as the derived-expression
// rewrite, constructor data where the old language quoted it, and the D19
// operation table -- the immutable registry keyed by the operation plus the
// ordered tag list -- that exercise 4.3 builds the data-directed eval on.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.guest.Admission
import sicp.guest.Mode
import sicp.guest.RunResult
import sicp.runtime.Key
import sicp.runtime.OpTable

/** The section's syntax specification: the sealed node family, the
 * derived rewrite, and the constructor demonstration. */
private val SYNTAX_UNIT: String =
    """
    sealed interface GExpr

    // literals
    data class GNum(val n: Long) : GExpr
    data class GStr(val s: String) : GExpr
    data class GDouble(val d: Double) : GExpr
    data class GBool(val b: Boolean) : GExpr
    data object GNull : GExpr

    // names, bindings, and mutation
    data class GVar(val name: String) : GExpr
    data class GLam(val param: String, val body: GExpr) : GExpr
    data class GApp(val fn: GExpr, val arg: GExpr) : GExpr
    data class GLet(val name: String, val value: GExpr, val body: GExpr) : GExpr
    data class GLetRec(val name: String, val value: GExpr, val body: GExpr) : GExpr
    data class GSet(val name: String, val value: GExpr) : GExpr

    // conditionals and type tests
    data class GIf(val test: GExpr, val onTrue: GExpr, val onFalse: GExpr) : GExpr
    data class GWhen(
        val subject: GExpr?,
        val cases: List<GCase>,
        val otherwise: GExpr?,
    ) : GExpr
    data class GCase(val pattern: GExpr?, val body: GExpr)
    data class GIs(val value: GExpr, val typeName: String) : GExpr

    // primitive operations
    data class GAdd(val left: GExpr, val right: GExpr) : GExpr
    data class GSub(val left: GExpr, val right: GExpr) : GExpr
    data class GMul(val left: GExpr, val right: GExpr) : GExpr
    data class GDiv(val left: GExpr, val right: GExpr) : GExpr
    data class GMod(val left: GExpr, val right: GExpr) : GExpr
    data class GLt(val left: GExpr, val right: GExpr) : GExpr
    data class GEq(val left: GExpr, val right: GExpr) : GExpr

    // structured data and access
    data class GConstruct(val className: String, val args: List<GExpr>) : GExpr
    data class GMember(val target: GExpr, val name: String) : GExpr
    data class GIndex(val target: GExpr, val index: GExpr) : GExpr

    // statements and blocks
    data class GBlock(val statements: List<GStmt>) : GExpr

    sealed interface GStmt
    data class GExprStmt(val expr: GExpr) : GStmt
    data class GVarStmt(val name: String, val value: GExpr) : GStmt
    data class GAssignStmt(val name: String, val value: GExpr) : GStmt
    data class GWhileStmt(val cond: GExpr, val body: GBlock) : GStmt
    data class GReturnStmt(val value: GExpr) : GStmt
    data object GBreakStmt : GStmt
    data object GContinueStmt : GStmt

    fun whenToIfChain(expr: GWhen): GExpr {
        val cases = expr.cases
        val subject = expr.subject
        if (cases.isEmpty()) return expr.otherwise ?: GBool(false)
        val first = cases.get(0)
        val rest = cases.drop(1)
        val test =
            if (subject == null) first.pattern ?: GBool(true)
            else GEq(subject, first.pattern ?: GNull)
        val alternative =
            if (rest.isEmpty()) expr.otherwise ?: GBool(false)
            else whenToIfChain(GWhen(subject, rest, expr.otherwise))
        return GIf(test, first.body, alternative)
    }

    fun main() {
        // the derived rewrite answers a nest of GIf nodes
        val rewritten = whenToIfChain(GWhen(null, listOf(GCase(GBool(true), GNum(1L))), GNum(0L)))
        if (rewritten is GIf) {
            println("rewritten = if")
        }
        // a subject-carrying analysis tests its clauses with GEq
        val subjectForm =
            whenToIfChain(
                GWhen(
                    GVar("x"),
                    listOf(GCase(GNum(3L), GNum(30L)), GCase(GNum(4L), GNum(40L))),
                    GNum(0L),
                ),
            )
        if (subjectForm is GIf) {
            println("outer = if")
            val test = subjectForm.test
            if (test is GEq) {
                println("test = eq")
            }
            val alternative = subjectForm.onFalse
            if (alternative is GIf) {
                println("nested = if")
            } else {
                println("nested = fallback")
            }
        }
        // constructed data denotes itself: no quotation step exists
        val built = GConstruct("Pair", listOf(GNum(1L), GNum(2L)))
        if (built is GConstruct) {
            println("built = " + built.className)
        }
    }
    """.trimIndent()

/** The same classification written as a `when` analysis and as its `if`
 * nest: the two spellings the derived rewrite relates. */
private val WHEN_FORM: String =
    """
    fun classify(x: Long): Long = when {
        x > 0L -> x
        x == 0L -> 100L
        else -> -x
    }

    fun main() {
        println(classify(5L))
        println(classify(0L))
        println(classify(-7L))
    }
    """.trimIndent()

private val IF_NEST_FORM: String =
    """
    fun classify(x: Long): Long =
        if (x > 0L) {
            x
        } else if (x == 0L) {
            100L
        } else {
            -x
        }

    fun main() {
        println(classify(5L))
        println(classify(0L))
        println(classify(-7L))
    }
    """.trimIndent()

private fun runCore(source: String): RunResult =
    Direct.run(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

/** The tag-list key the D19 registry orders: `("t1" "t2")` as a chain. */
private fun tagKey(tags: List<String>): Key = tags.foldRight(Key.Nil as Key) { tag, acc -> Key.Pair(Key.Sym(tag), acc) }

public class S4_1_2ExpressionsTest :
    FunSpec({
        test("the node family and the derived rewrite run as guest source") {
            val result = runCore(SYNTAX_UNIT)
            result.error shouldBe null
            result.output shouldBe
                "rewritten = if\n" +
                "outer = if\n" +
                "test = eq\n" +
                "nested = if\n" +
                "built = Pair\n"
        }

        test("the derived rewrite answers exactly what its source form answers") {
            runCore(WHEN_FORM).output shouldBe "5\n100\n7\n"
            runCore(IF_NEST_FORM).output shouldBe runCore(WHEN_FORM).output
        }

        test("every clause of the analysis is driven") {
            val output = runCore(WHEN_FORM).output
            output.contains("5") shouldBe true
            output.contains("100") shouldBe true
            output.contains("7") shouldBe true
        }

        test("the D19 registry keys on the operation plus the ordered tag list") {
            val table = OpTable()
            val realPart: sicp.runtime.Op = { args -> args[0] }
            table.put(Key.Sym("real-part"), tagKey(listOf("rectangular")), realPart)
            table.get(Key.Sym("real-part"), tagKey(listOf("rectangular"))) shouldBe realPart
            // get on a missing key is the absent option, never a false-ish sentinel
            table.get(Key.Sym("imag-part"), tagKey(listOf("rectangular"))) shouldBe null
            table.get(Key.Sym("real-part"), tagKey(listOf("polar"))) shouldBe null
            // put on an existing key overwrites
            val replacement: sicp.runtime.Op = { args -> args[1] }
            table.put(Key.Sym("real-part"), tagKey(listOf("rectangular")), replacement)
            table.get(Key.Sym("real-part"), tagKey(listOf("rectangular"))) shouldBe replacement
        }

        test("admission rejects a malformed form before any guest effect") {
            Admission.admit("fun main() {\n    println(1L\n}\n", Mode.CORE).isLeft() shouldBe true
        }
    })
