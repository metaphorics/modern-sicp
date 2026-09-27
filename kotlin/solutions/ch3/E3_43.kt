// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.43

package sicp.ch3.exercises

import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.yield

/** Three accounts starting at $10, $20, and $30. */
public fun makeThreeAccounts(): Triple<RawAccount, RawAccount, RawAccount> =
    Triple(
        makeAccountAndSerializer(10L),
        makeAccountAndSerializer(20L),
        makeAccountAndSerializer(30L),
    )

/**
 * Run `rounds` exchanges sequentially, cycling the three pairs, and
 * answer the final balances. Each exchange preserves the multiset, so
 * any number of them still leaves $10, $20, and $30 in some order.
 */
public suspend fun sequentialExchanges(rounds: Int): Triple<Long, Long, Long> {
    val (a1, a2, a3) = makeThreeAccounts()
    val pairs =
        listOf(
            a1 to a2,
            a2 to a3,
            a1 to a3,
        )
    repeat(rounds) { i ->
        val (x, y) = pairs[i % pairs.size]
        serializedExchange(x, y)
    }
    return Triple(a1.balance(), a2.balance(), a3.balance())
}

/**
 * Two unserialized exchanges of the section's first kind share account
 * a1 concurrently, on the section's serialized accounts. The schedule
 * is forced: both exchanges read their differences first, then Paul's
 * writes land, then Peter's -- so Peter acts on a difference computed
 * from a balance Paul has since destroyed, and the multiset breaks.
 */
public suspend fun brokenConcurrentExchanges(): Triple<Long, Long, Long> {
    val a1 = makeSerializedAccount(10L)
    val a2 = makeSerializedAccount(20L)
    val a3 = makeSerializedAccount(30L)
    coroutineScope {
        launch {
            val petersDifference = a1.balance() - a2.balance()
            yield()
            a1.withdraw(petersDifference)
            a2.deposit(petersDifference)
        }
        launch {
            val paulsDifference = a1.balance() - a3.balance()
            yield()
            a1.withdraw(paulsDifference)
            a3.deposit(paulsDifference)
        }
    }
    return Triple(a1.balance(), a2.balance(), a3.balance())
}

/**
 * The same two exchanges, each wrapped in both accounts' serializers as
 * the section's serialized-exchange does; they contend on a1's
 * serializer, so one waits for the other and the pair is sequential in
 * effect.
 */
public suspend fun serializedConcurrentExchanges(): Triple<Long, Long, Long> {
    val (a1, a2, a3) = makeThreeAccounts()
    coroutineScope {
        launch { serializedExchange(a1, a2) }
        launch { serializedExchange(a1, a3) }
    }
    return Triple(a1.balance(), a2.balance(), a3.balance())
}
