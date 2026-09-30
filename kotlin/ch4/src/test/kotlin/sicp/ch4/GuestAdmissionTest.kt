// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch4

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.guest.Admission
import sicp.guest.Mode

private fun categoryOf(source: String): String? = Admission.admit(source.trimIndent(), Mode.CORE).fold({ it.category }, { null })

private fun outputOf(source: String): Pair<String, String> {
    val checked =
        Admission.admit(source.trimIndent(), Mode.CORE).fold(
            { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
            { it },
        )
    val direct = Direct.run(checked)
    val analyzed = Analyzed.run(checked)
    direct.error shouldBe null
    analyzed.error shouldBe null
    return direct.output to analyzed.output
}

/** The declaration rules of grammar 2.2 and the two value rules the engines
 * once broke: each rejection is asserted by its category before any effect. */
public class GuestAdmissionTest :
    FunSpec({
        val rejected =
            mapOf(
                "a unit without main" to
                    ("fun helper(): Long = 1L" to "UndeclaredName"),
                "a second main" to
                    ("fun main() {\n}\nfun main() {\n}" to "Redeclaration"),
                "a main with a parameter" to
                    ("fun main(x: Long) {\n}" to "MiscKotlin"),
                "a main with a declared result" to
                    ("fun main(): Unit {\n}" to "MiscKotlin"),
                "a main with an expression body" to
                    ("fun main() = println(1L)" to "MiscKotlin"),
                "a local function named main standing in for the entry point" to
                    ("fun helper() {\n    fun main() {\n    }\n}" to "UndeclaredName"),
                "two functions of one signature" to
                    ("fun f(x: Long): Long = x\nfun f(x: Long): Long = x + 1L\nfun main() {\n}" to "Redeclaration"),
                "a user-defined overload" to
                    ("fun f(x: Long): Long = x\nfun f(x: Double): Double = x\nfun main() {\n}" to "MiscKotlin"),
                "two types of one name" to
                    ("data class A(val x: Long)\ndata class A(val y: Long)\nfun main() {\n}" to "Redeclaration"),
                "two top-level properties of one name" to
                    ("val a: Long = 1L\nval a: Long = 2L\nfun main() {\n}" to "Redeclaration"),
                "a repeated parameter name" to
                    ("fun f(a: Long, a: Long): Long = a\nfun main() {\n}" to "Redeclaration"),
                "a repeated data class property" to
                    ("data class A(val x: Long, val x: Long)\nfun main() {\n}" to "Redeclaration"),
                "inheritance from a data class" to
                    ("data class Base(val y: Long)\ndata class Child(val x: Long) : Base\nfun main() {\n}" to "MiscKotlin"),
                "inheritance from a plain class" to
                    ("class Base() {\n}\ndata class Child(val x: Long) : Base\nfun main() {\n}" to "MiscKotlin"),
                "inheritance from a data object" to
                    ("data object Base\ndata object Child : Base\nfun main() {\n}" to "MiscKotlin"),
                "a plain class that joins a sealed family" to
                    ("sealed interface S\nclass C() : S {\n}\nfun main() {\n}" to "MiscKotlin"),
            )
        for ((name, case) in rejected) {
            test("the checker rejects $name") {
                categoryOf(case.first) shouldBe case.second
            }
        }

        test("a sealed family of data classes and objects is still admitted") {
            val source =
                """
                sealed interface Shape
                data class Dot(val at: Long) : Shape
                data object Empty : Shape
                fun main() {
                }
                """
            categoryOf(source) shouldBe null
        }

        test("setOf and mapOf keep one element per structurally equal element or key") {
            val source =
                """
                data class P(val x: Long)
                fun main() {
                    println(setOf(1L to 2L, 1L to 2L).size.toLong())
                    println(setOf(P(1L), P(1L), P(2L)).size.toLong())
                    println(setOf(listOf(1L), listOf(1L)).size.toLong())
                    val m: Map<Pair<Long, Long>, String> = mapOf((1L to 2L) to "a", (1L to 2L) to "b")
                    println(m.size.toLong())
                    println(m[1L to 2L] ?: "none")
                }
                """
            val (direct, analyzed) = outputOf(source)
            direct shouldBe "1\n2\n1\n1\nb\n"
            analyzed shouldBe direct
        }

        test("a range loop is lazy: breaking out of a huge range is immediate and its last bound terminates") {
            val source =
                """
                fun main() {
                    var count: Long = 0L
                    for (n in 0L..9223372036854775807L) {
                        if (n > 3L) {
                            break
                        }
                        count = count + 1L
                    }
                    println(count)
                    var last: Long = 0L
                    for (m in 9223372036854775805L..9223372036854775807L) {
                        last = m
                    }
                    println(last)
                }
                """
            val (direct, analyzed) = outputOf(source)
            direct shouldBe "4\n9223372036854775807\n"
            analyzed shouldBe direct
        }
    })
