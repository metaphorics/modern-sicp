// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class DatumOpTableTest :
    FunSpec({
        fun constant(value: Long): DatumOp = { Whole(value) }

        test("put then get returns the installed handler") {
            val table = DatumOpTable()
            table.put(DatumKey.Symbol("real-part"), DatumKey.Symbol("rectangular"), constant(1))

            val handler = table.get(DatumKey.Symbol("real-part"), DatumKey.Symbol("rectangular"))
            either { handler?.let { operation -> operation(emptyList()) } }.getOrNull() shouldBe Whole(1)
        }

        test("a missing key returns null") {
            val table = DatumOpTable()
            table.get(DatumKey.Symbol("missing"), DatumKey.Symbol("rectangular")) shouldBe null
        }

        test("a later install replaces the same operation and tag") {
            val table = DatumOpTable()
            val operation = DatumKey.Symbol("add")
            val tag = DatumKey.Symbol("number")
            table.put(operation, tag, constant(1))
            table.put(operation, tag, constant(2))

            val handler = table.get(operation, tag)
            either { handler?.let { operation -> operation(emptyList()) } }.getOrNull() shouldBe Whole(2)
        }

        test("operations and tags dispatch independently") {
            val table = DatumOpTable()
            val operation = DatumKey.Symbol("imag-part")
            table.put(operation, DatumKey.Symbol("rectangular"), constant(1))
            table.put(operation, DatumKey.Symbol("polar"), constant(2))

            val rectangular = table.get(operation, DatumKey.Symbol("rectangular"))
            val polar = table.get(operation, DatumKey.Symbol("polar"))
            either { rectangular?.let { operation -> operation(emptyList()) } }.getOrNull() shouldBe Whole(1)
            either { polar?.let { operation -> operation(emptyList()) } }.getOrNull() shouldBe Whole(2)
        }

        test("deep structural keys remain usable by operation dispatch") {
            val depth = 4_096
            var first: Datum = Whole(0)
            var second: Datum = Whole(0)
            repeat(depth) {
                first = pair(first, Empty)
                second = pair(second, Empty)
            }

            val result =
                either {
                    val table = DatumOpTable()
                    table.put(keyOf(first), DatumKey.Empty, constant(7))
                    val handler = table.get(keyOf(second), DatumKey.Empty)
                    handler?.let { operation -> operation(emptyList()) }
                }

            result.getOrNull() shouldBe Whole(7)
        }

        test("handlers receive the argument list and raise typed errors") {
            val table = DatumOpTable()
            val add = DatumKey.Symbol("add")
            val number = DatumKey.Symbol("number")
            table.put(add, number) { arguments ->
                if (arguments.size != 2) raise(DatumError.BadDatum("add expects two operands"))
                val left = arguments[0] as? Whole ?: raise(DatumError.TypeMismatch("add expects whole numbers"))
                val right = arguments[1] as? Whole ?: raise(DatumError.TypeMismatch("add expects whole numbers"))
                Whole(Math.addExact(left.value, right.value))
            }

            val handler = table.get(add, number)
            either { handler?.let { operation -> operation(listOf(Whole(2), Whole(3))) } }.getOrNull() shouldBe Whole(5)
            either { handler?.let { operation -> operation(listOf(Whole(2))) } } shouldBe
                Either.Left(DatumError.BadDatum("add expects two operands"))
        }
    })
