// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The domain's errors: a closed set, one variant per failure mode. */
public sealed interface PortError {
    public data object Blank : PortError

    public data class NotDigits(
        val input: String,
    ) : PortError

    public data class OutOfRange(
        val input: String,
    ) : PortError
}

/**
 * The parser raises instead of throwing: [r] arrives as a context
 * parameter, and [Raise.raise] aborts the surrounding raise scope with the
 * error value.
 */
context(r: Raise<PortError>)
public fun parsePort(s: String): Int {
    val t = s.trim()
    if (t.isEmpty()) r.raise(PortError.Blank)
    if (t.any { c -> c !in '0'..'9' }) r.raise(PortError.NotDigits(s))
    val n = t.toIntOrNull() ?: r.raise(PortError.OutOfRange(t))
    if (n !in 1..65535) r.raise(PortError.OutOfRange(t))
    return n
}

/** The driver turns every `raise` into an `Either` value. */
public fun parsePortEither(s: String): Either<PortError, Int> = either { parsePort(s) }

public class S0_6ErrorsTest :
    FunSpec({
        test("a good parse is Right") {
            parsePortEither(" 8080 ") shouldBe Either.Right(8080)
        }
        test("each failure mode is its own Left value") {
            parsePortEither("   ") shouldBe Either.Left(PortError.Blank)
            parsePortEither("80x0") shouldBe Either.Left(PortError.NotDigits("80x0"))
            parsePortEither("99999") shouldBe Either.Left(PortError.OutOfRange("99999"))
            parsePortEither("2147483648") shouldBe Either.Left(PortError.OutOfRange("2147483648"))
        }
    })
