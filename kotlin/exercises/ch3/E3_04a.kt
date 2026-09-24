// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4a

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

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

/**
 * Exercise 3.4a is added by this edition and extends exercise 3.4; SICP
 * numbers stop at 3.4. Exercise 3.4 locks an account out after seven
 * consecutive wrong passwords, but a locked-out account keeps no record
 * of what happened to it. Extend `makeAccountWithLockout` into
 * `makeAccountWithAuditLog`, whose `auditLog()` returns every access
 * attempt in the order it happened, each entry naming the operation
 * (`"withdraw"` or `"deposit"`) and its outcome (`"ok"`,
 * `"wrong-password"`, `"insufficient-funds"`, or `"call-the-cops"`).
 *
 * The scaffold reports no attempts, ever.
 */
public fun makeAccountWithAuditLog(
    balance: Long,
    correctPassword: String,
): AuditedAccount = throw PendingSolution()
