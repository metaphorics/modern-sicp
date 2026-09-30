// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch4

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.guest.Admission
import sicp.guest.Mode

/** Executable regressions for numeric admission and call parsing: these
 * assert guest effects, not parser implementation details or source copies. */
public class GuestCallsTest :
    FunSpec({
        test("ordinary and typed calls execute with Long operands in source order") {
            val source =
                """
                fun twice(n: Long): Long = n * 2L
                fun main() {
                    println(twice(12L))
                    val items: List<Long> = listOf<Long>(1L, 2L)
                    println(items.get(1))
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val direct = Direct.run(checked)
            val analyzed = Analyzed.run(checked)
            direct.error shouldBe null
            analyzed.error shouldBe null
            direct.output shouldBe "24\n2\n"
            analyzed.output shouldBe direct.output
        }

        test("contextual unsuffixed Long and signed width minima execute") {
            val source =
                """
                fun addOne(n: Long): Long = n + 1L
                fun main() {
                    println(addOne(2))
                    val intMinimum: Int = -2147483648
                    if (intMinimum < 0) println("int-min")
                    println(-9223372036854775808L)
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val result = Direct.run(checked)
            result.error shouldBe null
            result.output shouldBe "3\nint-min\n-9223372036854775808\n"
        }

        test("multiline sealed when branches keep their own type patterns") {
            val source =
                """
                sealed interface Shape
                data class Dot(val radius: Long) : Shape
                data object None : Shape
                fun describe(shape: Shape): String =
                    when (shape) {
                        is Dot -> "dot"
                        is None -> "none"
                    }
                fun main() {
                    println(describe(Dot(2L)))
                    println(describe(None))
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val direct = Direct.run(checked)
            val analyzed = Analyzed.run(checked)
            direct.error shouldBe null
            analyzed.error shouldBe null
            direct.output shouldBe "dot\nnone\n"
            analyzed.output shouldBe direct.output
        }

        test("ordinary variable named out is not mistaken for generic variance") {
            val source =
                """
                fun main() {
                    var out = ""
                    out = out + "ok"
                    println(out)
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val run = Direct.run(checked)
            run.error shouldBe null
            run.output shouldBe "ok\n"
        }

        test("variance remains unsupported only in type argument position") {
            val source =
                """
                fun main() {
                    val numbers: List<out Long> = listOf(1L)
                    println(numbers.get(0))
                }
                """.trimIndent()
            Admission.admit(source, Mode.CORE).fold(
                { rejection -> rejection.category shouldBe "UserGenerics" },
                { kotlin.error("type variance was admitted") },
            )
        }

        test("family and nullable values cannot flow into narrower parameters") {
            val invalidPrograms =
                listOf(
                    """
                    sealed interface Outcome
                    data class Good(val n: Long) : Outcome
                    data object Bad : Outcome
                    fun read(value: Good): Long = value.n
                    fun main() {
                        val outcome: Outcome = Bad
                        println(read(outcome))
                    }
                    """.trimIndent(),
                    """
                    fun read(value: Long): Long = value
                    fun main() {
                        val nullable: Long? = null
                        println(read(nullable))
                    }
                    """.trimIndent(),
                )
            for (source in invalidPrograms) {
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> rejection.category shouldBe "TypeMismatch" },
                    { kotlin.error("a wider type was admitted as a narrower parameter") },
                )
            }
        }

        test("collection builders honor contextual and explicit element types") {
            val source =
                """
                fun main() {
                    val contextual: List<Long> = listOf(1, 2)
                    val explicit = listOf<Long>(3, 4)
                    println(contextual.get(0) + explicit.get(1))
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("builder rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val run = Direct.run(checked)
            run.error shouldBe null
            run.output shouldBe "5\n"
        }

        test("empty builder without type evidence rejects before effects") {
            val source =
                """
                fun main() {
                    val unknown = emptyList()
                    println("must not execute")
                }
                """.trimIndent()
            Admission.admit(source, Mode.CORE).fold(
                { rejection -> rejection.category shouldBe "TypeInference" },
                { kotlin.error("untyped empty builder was admitted") },
            )
        }

        test("null guards narrow stable parameters through disjunction and return") {
            val source =
                """
                fun add(a: Long?, b: Long?): Long {
                    if (a == null || b == null) return -1L
                    return a + b
                }
                fun main() {
                    println(add(null, 2L))
                    println(add(3L, 4L))
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("null flow rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val direct = Direct.run(checked)
            val analyzed = Analyzed.run(checked)
            direct.error shouldBe null
            analyzed.error shouldBe null
            direct.output shouldBe "-1\n7\n"
            analyzed.output shouldBe direct.output
        }

        test("shadowed bindings do not inherit an outer smart cast") {
            val source =
                """
                sealed interface Value
                data class Number(val n: Long) : Value
                data object Other : Value
                fun main() {
                    val value: Value = Number(1L)
                    if (value is Number) {
                        val value: Value = Other
                        println(value.n)
                    }
                }
                """.trimIndent()
            Admission.admit(source, Mode.CORE).fold(
                { rejection -> rejection.category shouldBe "UndeclaredName" },
                { kotlin.error("inner binding inherited outer smart cast") },
            )
        }

        test("a braceless if statement branch may assign; an if expression branch may not") {
            val source =
                """
                fun main() {
                    var out = "a"
                    var i = 0L
                    while (i < 3L) {
                        if (i > 0L) out = out + ","
                        else if (i == 0L) out = out + ":"
                        out = out + "${'$'}{i}"
                        i = i + 1L
                    }
                    println(out)
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val direct = Direct.run(checked)
            val analyzed = Analyzed.run(checked)
            direct.error shouldBe null
            direct.output shouldBe "a:0,1,2\n"
            analyzed.output shouldBe direct.output
            val expressionAssignment =
                """
                fun main() {
                    var x = 0L
                    val y: Long = if (x == 0L) x = 1L else 2L
                    println(y)
                }
                """.trimIndent()
            Admission.admit(expressionAssignment, Mode.CORE).isLeft() shouldBe true
        }

        test("branch values join to the family, nullable when a branch is null") {
            val source =
                """
                sealed interface Term
                data class Sym(val name: String) : Term
                data class Var(val name: String) : Term
                fun pick(a: Term, b: Term): Term? =
                    when {
                        a is Var -> b
                        a is Sym && b is Sym -> if (a.name == b.name) a else null
                        else -> null
                    }
                fun show(t: Term?): String =
                    when {
                        t is Sym -> t.name
                        t is Var -> "?" + t.name
                        else -> "none"
                    }
                fun main() {
                    println(show(pick(Sym("a"), Sym("a"))))
                    println(show(pick(Sym("a"), Sym("b"))))
                    println(show(pick(Var("x"), Sym("b"))))
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.CORE).fold(
                    { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val direct = Direct.run(checked)
            direct.error shouldBe null
            direct.output shouldBe "a\nnone\nb\n"
            val nullableIntoNarrower =
                """
                sealed interface Term
                data class Sym(val name: String) : Term
                fun pick(a: Term): Term = if (a is Sym) a else null
                fun main() {
                    println(pick(Sym("a")) is Sym)
                }
                """.trimIndent()
            Admission.admit(nullableIntoNarrower, Mode.CORE).fold(
                { rejection -> rejection.category shouldBe "TypeMismatch" },
                { kotlin.error("a nullable branch join was admitted as a non-null result") },
            )
        }

        test("query modes construct and test the query vocabulary in every engine") {
            val source =
                """
                fun render(t: QTerm): String =
                    when {
                        t is QSym -> t.name
                        t is QVar -> "?" + t.name
                        t is QList -> "[" + render(t.items.get(0)) + " | " + render(t.items.get(1)) + "]"
                        else -> "other"
                    }
                fun main() {
                    val term: QTerm = QList(listOf(QSym("a"), QVar("x")), null)
                    println(render(term))
                    println(render(QVar("y")))
                }
                """.trimIndent()
            val checked =
                Admission.admit(source, Mode.QUERY).fold(
                    { rejection -> kotlin.error("admission rejected ${rejection.category}: ${rejection.message}") },
                    { it },
                )
            val direct = Direct.run(checked)
            val analyzed = Analyzed.run(checked)
            direct.error shouldBe null
            direct.output shouldBe "[a | ?x]\n?y\n"
            analyzed.output shouldBe direct.output
        }
    })
