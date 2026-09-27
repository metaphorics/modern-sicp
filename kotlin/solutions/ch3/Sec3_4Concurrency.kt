// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.4, shared concurrency code for the 3.4 exercises

package sicp.ch3.exercises

import arrow.core.Either
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.yield
import java.util.concurrent.atomic.AtomicBoolean

/**
 * The 3.4.1 shared balance: one field, and every process closes over
 * this holder, which is the point.
 */
public class Cell(
    public var value: Long,
)

/**
 * The book's boolean mutex cell of "Implementing serializers": true
 * means the mutex is taken.
 */
public class FlagCell(
    public var value: Boolean,
)

/**
 * The book's test-and-set on a plain cell: true when the cell was
 * already set, otherwise set it and answer false. Deliberately not
 * atomic -- exercise 3.46's subject; [AtomicCell.testAndSet] is the
 * atomic version the section says the operation must have.
 */
public fun testAndSet(cell: FlagCell): Boolean =
    if (cell.value) {
        true
    } else {
        cell.value = true
        false
    }

/** The book's mutex shape: acquire may wait, release never blocks. */
public interface BookMutex {
    public suspend fun acquire()

    public fun release()
}

/**
 * The book's hand-rolled mutex: spin on the cell, yielding between
 * retries so a waiting coroutine hands the thread back instead of
 * burning it. Real code parks in kotlinx's Mutex, as the section's
 * footnote says; this one keeps the book's plainer shape for the
 * serializer experiments.
 */
public fun makeMutex(): BookMutex {
    val cell = FlagCell(false)
    return object : BookMutex {
        override suspend fun acquire() {
            while (testAndSet(cell)) {
                yield() // retry
            }
        }

        override fun release() {
            cell.value = false
        }
    }
}

/**
 * The section's atomic cell: one indivisible compare-and-exchange, the
 * hardware route the text takes for test-and-set. `testAndSet` answers
 * true exactly when the cell was already set, the book's contract.
 */
public class AtomicCell(
    private val flag: AtomicBoolean = AtomicBoolean(false),
) {
    public fun testAndSet(): Boolean = !flag.compareAndSet(false, true)

    public fun clear() {
        flag.set(false)
    }
}

/**
 * The section's serializer: one kotlinx Mutex (a waiter parks in lock,
 * per the footnote), and wrapped suspending procedures that acquire,
 * run, and release in `finally`.
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

    /**
     * Wrap a one-argument procedure: the book's make-serializer is
     * indifferent to the wrapped procedure's shape, and exercise 3.42's
     * once-created procedures need this arity. A separate name, not an
     * overload: a brace lambda matches both arities through its
     * implicit `it`.
     */
    public fun <P, A> serializedArg(p: suspend (P) -> A): suspend (P) -> A =
        {
            mutex.lock()
            try {
                p(it)
            } finally {
                mutex.unlock()
            }
        }
}

/**
 * The text's serialized account (the book's `make-account` of this
 * section): deposits and withdrawals share one serializer, so two
 * processes cannot be inside a single account's transaction at once.
 * Named `SerializedAccount` here because E3_03's password protocol
 * already owns `Account` in this shared package.
 */
public interface SerializedAccount {
    public suspend fun withdraw(amount: Long): Either<WithdrawError, Long>

    public suspend fun deposit(amount: Long): Long

    public suspend fun balance(): Long
}

/** The text's serialized `make-account`. */
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

/**
 * The exported-serializer account: the raw procedures the book's
 * dispatch returned, plus the serializer itself. Serialization is the
 * caller's job now.
 */
public interface RawAccount {
    public suspend fun withdraw(amount: Long): Either<WithdrawError, Long>

    public suspend fun deposit(amount: Long): Long

    public suspend fun balance(): Long

    /** The book's `serializer` message: the account's own serializer. */
    public fun serializer(): Serializer
}

/** The text's `make-account-and-serializer`. */
public fun makeAccountAndSerializer(balance: Long): RawAccount {
    var b = balance
    val s = Serializer()
    return object : RawAccount {
        override suspend fun withdraw(amount: Long): Either<WithdrawError, Long> =
            if (b < amount) {
                Either.Left(WithdrawError.InsufficientFunds)
            } else {
                b -= amount
                Either.Right(b)
            }

        override suspend fun deposit(amount: Long): Long {
            b += amount
            return b
        }

        override suspend fun balance(): Long = b

        override fun serializer(): Serializer = s
    }
}

/**
 * The book's exchange: read both balances, withdraw the difference from
 * one, deposit it into the other. Correct only when nothing else
 * touches the accounts between the read and the writes.
 */
public suspend fun exchange(
    account1: RawAccount,
    account2: RawAccount,
): Long {
    val difference = account1.balance() - account2.balance()
    account1.withdraw(difference)
    account2.deposit(difference)
    return difference
}

/** The text's serialized deposit over an exported-serializer account. */
public suspend fun deposit(
    account: RawAccount,
    amount: Long,
): Long =
    account.serializer().serialized {
        account.deposit(amount)
    }()

/**
 * The text's serialized-exchange: one serializer wrapped around the
 * other around the exchange. On Louis's account (exercise 3.45) the
 * inner wrap re-enters the outer mutex and deadlocks.
 */
public suspend fun serializedExchange(
    account1: RawAccount,
    account2: RawAccount,
): Long =
    account1.serializer().serialized {
        account2.serializer().serialized {
            exchange(account1, account2)
        }()
    }()

/**
 * All interleavings of some processes with fixed per-process step
 * counts, as lane indices: `order[t]` is the lane whose step runs at
 * time `t`. Within a lane the steps keep their order, which is exactly
 * the constraint the section's interleavings obey. The test scheduler
 * is nondeterministic on the JVM, so the enumerations of exercises 3.38
 * to 3.42 drive this generator and force the schedules instead.
 */
public fun allInterleavings(stepCounts: List<Int>): List<List<Int>> {
    val next = IntArray(stepCounts.size)
    val done = mutableSetOf<Int>()
    val acc = mutableListOf<Int>()
    val out = mutableListOf<List<Int>>()

    fun go() {
        if (done.size == stepCounts.size) {
            out.add(acc.toList())
            return
        }
        for (lane in stepCounts.indices) {
            if (lane in done) {
                continue
            }
            val isLast = next[lane] == stepCounts[lane] - 1
            next[lane] += 1
            if (isLast) {
                done.add(lane)
            }
            acc.add(lane)
            go()
            acc.removeAt(acc.lastIndex)
            if (isLast) {
                done.remove(lane)
            }
            next[lane] -= 1
        }
    }
    go()
    return out
}
