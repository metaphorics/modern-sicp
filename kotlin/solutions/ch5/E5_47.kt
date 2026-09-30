// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.47: compiled code calls an interpreted
// procedure. Direct builds a closure capturing a lexical binding; the
// compiler's mixed-call interface substitutes that closure for the
// checked program's same-signature declaration before the machine runs.

package sicp.ch5.solutions

import sicp.ch4.Direct
import sicp.ch5.Compiler
import sicp.guest.GValue

/** A compiled caller reaches an interpreter-created closure, rather
 * than the compiled declaration it would ordinarily resolve. */
public fun mixedCallsRun(): List<String> {
    val interpreterSource =
        """
        fun incrementer(): (Long) -> Long {
            val increment: Long = 1L
            return { x: Long -> x + increment }
        }

        fun main() {
        }
        """.trimIndent()
    val closure =
        Direct
            .values(admitProgram(interpreterSource), listOf("incrementer"))
            .fold({ error("the direct evaluator did not run: $it") }, { it[0] }) as? GValue.VFunction
            ?: error("the direct evaluator did not return an interpreted closure")
    val compiledSource =
        """
        fun interpreted(x: Long): Long {
            return x + 100L
        }

        fun compiledCaller(x: Long): Long {
            return interpreted(x)
        }

        fun main() {
            println(compiledCaller(41L))
        }
        """.trimIndent()
    val checked = admitProgram(compiledSource)
    val compiledOnly = outputLines(Compiler.compileAndRun(checked))
    val mixed = outputLines(Compiler.compileAndRun(checked, interpretedBindings = mapOf("interpreted" to closure)))
    return listOf(
        "the compiled declaration answers: ${compiledOnly.joinToString(" ")}",
        "the interpreted binding answers: ${mixed.joinToString(" ")}",
        "the mixed call reached the interpreted closure: ${compiledOnly == listOf("141") && mixed == listOf("42")}",
    )
}
