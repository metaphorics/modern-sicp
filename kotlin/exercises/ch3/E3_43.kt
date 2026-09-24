// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.43

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/** Three accounts starting at $10, $20, and $30. */
public fun makeThreeAccounts(): Triple<RawAccount, RawAccount, RawAccount> = throw PendingSolution()

/**
 * Run `rounds` exchanges sequentially, cycling the three pairs, and
 * answer the final balances.
 */
public suspend fun sequentialExchanges(rounds: Int): Triple<Long, Long, Long> = throw PendingSolution()

/**
 * Two unserialized exchanges of the section's first kind share account
 * a1 concurrently, on the section's serialized accounts. The schedule
 * is forced: both exchanges read their differences first, then Paul's
 * writes land, then Peter's.
 */
public suspend fun brokenConcurrentExchanges(): Triple<Long, Long, Long> = throw PendingSolution()

/**
 * The same two exchanges, each wrapped in both accounts' serializers as
 * the section's serialized-exchange does; they contend on a1's
 * serializer, so one waits for the other.
 */
public suspend fun serializedConcurrentExchanges(): Triple<Long, Long, Long> = throw PendingSolution()
