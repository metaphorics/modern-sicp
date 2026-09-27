// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.4.2, mechanisms for controlling concurrency

package sicp.ch3.examples

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.yield
import java.util.concurrent.atomic.AtomicBoolean

/**
 * The section's serializer: one kotlinx Mutex (a waiter parks in lock),
 * and wrapped suspending procedures that acquire, run, and release in
 * `finally`.
 */
public class Serializer {
    private val mutex = Mutex()

    /** Wrap a procedure of no arguments into the serializer's set. */
    public fun <A> serialized(p: suspend () -> A): suspend () -> A =
        {
            mutex.lock()
            try {
                p()
            } finally {
                mutex.unlock()
            }
        }
}

/**
 * The book's boolean mutex cell: true means the mutex is taken. The
 * hand-rolled mutex spins on it, yielding between retries; kotlinx's
 * own Mutex parks, as the footnote says.
 */
public class FlagCell(
    public var value: Boolean,
)

/** The book's test-and-set on a plain cell: true when already set. */
public fun testAndSet(cell: FlagCell): Boolean =
    if (cell.value) {
        true
    } else {
        cell.value = true
        false
    }

/** The atomic cell: one indivisible compare-and-exchange. */
public class AtomicCell(
    private val flag: AtomicBoolean = AtomicBoolean(false),
) {
    public fun testAndSet(): Boolean = !flag.compareAndSet(false, true)

    public fun clear() {
        flag.set(false)
    }
}

/** The text's serialized account (the section's `make-account`). */
public interface SerializedAccount {
    public suspend fun withdraw(amount: Long): Either<WithdrawError, Long>

    public suspend fun deposit(amount: Long): Long

    public suspend fun balance(): Long
}

public fun makeSerializedAccount(balance: Long): SerializedAccount {
    var b = balance
    val s = Serializer()
    return object : SerializedAccount {
        override suspend fun withdraw(amount: Long): Either<WithdrawError, Long> =
            s.serialized {
                if (b < amount) {
                    Either.Left(WithdrawError.InsufficientFunds)
                } else {
                    b -= amount
                    Either.Right(b)
                }
            }()

        override suspend fun deposit(amount: Long): Long =
            s.serialized {
                b += amount
                b
            }()

        override suspend fun balance(): Long = b
    }
}

public class S3_4_2ControllingConcurrencyTest :
    FunSpec({
        test("the raw race, forced five ways, yields the five possible values") {
            runTest {
                // Each schedule is forced against a fresh variable, with
                // yield() marking the points where control may switch.

                // 101: P1 runs through, then P2.
                var x = 10L
                coroutineScope {
                    launch { x = x * x }
                    launch { x = x + 1 }
                }
                x shouldBe 101L

                // 121: P2 first, then P1.
                var y = 10L
                coroutineScope {
                    launch { y = y + 1 }
                    launch { y = y * y }
                }
                y shouldBe 121L

                // 110: P2 lands between P1's two reads.
                var z = 10L
                coroutineScope {
                    launch {
                        val a = z
                        yield()
                        val b = z
                        z = a * b
                    }
                    launch { z = z + 1 }
                }
                z shouldBe 110L

                // 11: P1's write lands between P2's read and write.
                var w = 10L
                coroutineScope {
                    launch {
                        val a = w
                        val b = w
                        yield()
                        w = a * b
                    }
                    launch {
                        val v = w
                        yield()
                        w = v + 1
                    }
                }
                w shouldBe 11L

                // 100: round-robin, P1's write last.
                var q = 10L
                coroutineScope {
                    launch {
                        val a = q
                        yield()
                        val b = q
                        yield()
                        q = a * b
                    }
                    launch {
                        val v = q
                        yield()
                        q = v + 1
                    }
                }
                q shouldBe 100L
            }
        }

        test("the serialized race answers only 101 or 121") {
            runTest {
                val s = Serializer()
                var x = 10L
                coroutineScope {
                    launch { s.serialized { x = x * x }() }
                    launch { s.serialized { x = x + 1 }() }
                }
                x shouldBe 101L
                var y = 10L
                coroutineScope {
                    launch { s.serialized { y = y + 1 }() }
                    launch { s.serialized { y = y * y }() }
                }
                y shouldBe 121L
            }
        }

        test("the serialized account runs concurrent transactions one at a time") {
            runTest {
                val acc = makeSerializedAccount(100L)
                coroutineScope {
                    launch { acc.deposit(40L) }
                    launch { acc.deposit(40L) }
                    launch { acc.withdraw(25L) }
                    launch { acc.withdraw(25L) }
                }
                acc.balance() shouldBe 130L
            }
        }

        test("the book's test-and-set, plain and atomic") {
            val plain = FlagCell(false)
            testAndSet(plain) shouldBe false
            testAndSet(plain) shouldBe true
            val atomic = AtomicCell()
            atomic.testAndSet() shouldBe false
            atomic.testAndSet() shouldBe true
            atomic.clear()
            atomic.testAndSet() shouldBe false
        }
    })
