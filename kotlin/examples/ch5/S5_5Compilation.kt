// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.5
// Section 5.5, running the compiler: the factorial compilation is typed
// controller data -- every statement renders on the pinned one-line trace
// form the figure uses -- and compile-and-run answers the book's session,
// value 120, agreeing with direct execution on the same checked program.

package sicp.ch5.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.ch5.Compiler
import sicp.guest.Admission
import sicp.guest.CheckedProgram
import sicp.guest.GValue
import sicp.guest.Mode
import sicp.guest.RunResult
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Label
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Source.ConstSrc
import sicp.runtime.Stmt
import sicp.runtime.traceLine

private val factorialSource: String =
    """
    fun factorial(n: Long): Long = if (n < 2L) 1L else factorial(n - 1L) * n

    fun main() {
        println(factorial(5L))
    }
    """.trimIndent()

private fun checked(source: String): CheckedProgram =
    Admission.admit(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

/** The compiler's statements for [source]: compilation succeeds or the
 * test names the rejection category. */
private fun compiled(source: String): List<Stmt> =
    Compiler.compile(checked(source)).fold(
        { e -> throw AssertionError("compilation rejected the unit: ${e.category}") },
        { it },
    )

public class S5_5CompilationTest :
    FunSpec({
        test("compile-and-run answers the book's session: value 120") {
            val result: RunResult = Compiler.compileAndRun(checked(factorialSource))
            result.output shouldBe "120\n"
            result.error shouldBe null
        }

        test("the compiled machine run agrees with direct execution") {
            val compiledRun = Compiler.compileAndRun(checked(factorialSource))
            val directRun =
                Direct.run(factorialSource, Mode.CORE).fold(
                    { e -> throw AssertionError("admission rejected the unit: ${e.category}") },
                    { it },
                )
            compiledRun.output shouldBe directRun.output
            compiledRun.mainValue shouldBe directRun.mainValue
            compiledRun.error shouldBe null
        }

        test("the compilation is controller data with the recursion's stack discipline") {
            val statements = compiled(factorialSource)
            // recursion compiles through the stack: entry labels, and the
            // continuation saved and restored around the recursive call
            (statements.filterIsInstance<Label>().isNotEmpty()) shouldBe true
            (statements.count { it is Save } >= 1) shouldBe true
            (statements.count { it is Restore } >= 1) shouldBe true
        }

        test("every statement renders on the pinned one-line trace form") {
            val statements = compiled(factorialSource)
            val lines = statements.map(::traceLine)
            lines.size shouldBe statements.size
            // the trace grammar the figure's lines follow: one line per
            // instruction over the constructor fields
            traceLine(Label("entry2")) shouldBe "entry2:"
            traceLine(Assign("val", Source.RegSrc("proc"))) shouldBe "assign val <- proc"
            traceLine(Assign("val", ConstSrc(GValue.VLong(1)))) shouldBe "assign val <- const"
            traceLine(Assign("val", Source.LabelSrc("entry2"))) shouldBe "assign val <- label entry2"
            traceLine(Assign("val", Source.OpSrc("list", listOf(Source.RegSrc("val"))))) shouldBe
                "assign val <- (op list val)"
            traceLine(Save("continue")) shouldBe "save continue"
            traceLine(Restore("continue")) shouldBe "restore continue"
            traceLine(Branch("after-lambda1")) shouldBe "branch after-lambda1"
        }
    })
