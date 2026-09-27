// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4a

package sicp.ch3.exercises

import arrow.core.Either
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf

// AccountError and Account are exercise 3.3's public declarations, reused here from the same package.

/** One logged access attempt: the operation tried and its outcome, in the order they happened. */
public data class AuditEntry(
    public val seq: Int,
    public val operation: String,
    public val outcome: String,
)

/** Exercise 3.4's `Account`, extended with a readable trail of every request it has ever answered. */
public interface AuditedAccount : Account {
    public fun auditLog(): List<AuditEntry>
}

private fun outcomeOf(error: AccountError): String =
    when (error) {
        AccountError.WrongPassword -> "wrong-password"
        AccountError.InsufficientFunds -> "insufficient-funds"
        AccountError.CallTheCops -> "call-the-cops"
    }

/**
 * A third captured cell, `log`, rebound to a new `PersistentList` on every
 * request: `record` appends one entry with a fresh sequence number, and
 * every branch of `withdraw` and `deposit` calls it exactly once, whether
 * the request succeeded or failed.
 */
public fun makeAccountWithAuditLog(
    balance: Long,
    correctPassword: String,
): AuditedAccount {
    var b = balance
    var consecutiveWrong = 0
    var seq = 0
    var log: PersistentList<AuditEntry> = persistentListOf()

    fun record(
        operation: String,
        outcome: String,
    ) {
        seq += 1
        log = log.adding(AuditEntry(seq, operation, outcome))
    }

    fun checkPassword(password: String): AccountError? {
        if (password == correctPassword) {
            consecutiveWrong = 0
            return null
        }
        consecutiveWrong += 1
        return if (consecutiveWrong > 7) AccountError.CallTheCops else AccountError.WrongPassword
    }

    return object : AuditedAccount {
        override fun withdraw(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> {
            val failure = checkPassword(password)
            if (failure != null) {
                record("withdraw", outcomeOf(failure))
                return Either.Left(failure)
            }
            if (amount > b) {
                record("withdraw", "insufficient-funds")
                return Either.Left(AccountError.InsufficientFunds)
            }
            b -= amount
            record("withdraw", "ok")
            return Either.Right(b)
        }

        override fun deposit(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> {
            val failure = checkPassword(password)
            if (failure != null) {
                record("deposit", outcomeOf(failure))
                return Either.Left(failure)
            }
            b += amount
            record("deposit", "ok")
            return Either.Right(b)
        }

        override fun auditLog(): List<AuditEntry> = log
    }
}
