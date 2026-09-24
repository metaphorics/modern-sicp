// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.41a

package sicp.ch3.exercises

/**
 * What a purchaser decides from the balance it observed. The decision
 * is `Approved` exactly when the observed balance covers the price.
 */
public sealed interface PurchaseDecision {
    public data object Approved : PurchaseDecision

    public data object Declined : PurchaseDecision
}

/**
 * One purchase attempt: what the purchaser saw, and what it decided
 * from that sight.
 */
public class Purchase(
    public val price: Long,
) {
    public var observed: Long = 0L

    public var decision: PurchaseDecision = PurchaseDecision.Declined
}

/** The purchaser's read-and-decide step: observe, then decide. */
public suspend fun authorizeRead(
    account: SerializedAccount,
    purchase: Purchase,
) {
    purchase.observed = account.balance()
    purchase.decision =
        if (purchase.observed >= purchase.price) {
            PurchaseDecision.Approved
        } else {
            PurchaseDecision.Declined
        }
}

/** The other party's withdraw, which drains the same account. */
public suspend fun spendBehindTheRead(
    account: SerializedAccount,
    amount: Long,
) {
    account.withdraw(amount)
}
