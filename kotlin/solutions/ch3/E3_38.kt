// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.38

package sicp.ch3.exercises

/**
 * The private memory of one process of exercise 3.38: the balance the
 * process saw when its read step ran, which its write step then acts
 * on. The three processes of the exercise each read the shared balance
 * and later write a value computed from that read.
 */
public class Process38 {
    public var seen: Long = 0L
}

/** Peter's read step: deposit $10, first access the shared balance. */
public suspend fun peterDepositRead(
    balance: Cell,
    p: Process38,
) {
    p.seen = balance.value
}

/** Peter's write step: set the balance to what he saw plus 10. */
public suspend fun peterDepositWrite(
    balance: Cell,
    p: Process38,
) {
    balance.value = p.seen + 10L
}

/** Paul's read step: withdraw $20, first access the shared balance. */
public suspend fun paulWithdrawRead(
    balance: Cell,
    p: Process38,
) {
    p.seen = balance.value
}

/** Paul's write step: set the balance to what he saw minus 20. */
public suspend fun paulWithdrawWrite(
    balance: Cell,
    p: Process38,
) {
    balance.value = p.seen - 20L
}

/** Mary's read step: withdraw half, first access the shared balance. */
public suspend fun maryHalfRead(
    balance: Cell,
    p: Process38,
) {
    p.seen = balance.value
}

/** Mary's write step: set the balance to what she saw minus half of it. */
public suspend fun maryHalfWrite(
    balance: Cell,
    p: Process38,
) {
    balance.value = p.seen - p.seen / 2L
}
