// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.2.3

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The book's `map`, `filter`, and `accumulate`, hand-rolled over
 * `List<T>` exactly as the prose defines them, so 2.2.3's signal-flow
 * plans read the same way in Kotlin. Kotlin's own `List.map`,
 * `List.filter`, and `List.fold` are the everyday twins; day-to-day code
 * reaches for those directly.
 */
public fun <T, R> mapSeq(
    p: (T) -> R,
    sequence: List<T>,
): List<R> = if (sequence.isEmpty()) emptyList() else listOf(p(sequence.first())) + mapSeq(p, sequence.drop(1))

public fun <T> filterSeq(
    predicate: (T) -> Boolean,
    sequence: List<T>,
): List<T> =
    when {
        sequence.isEmpty() -> emptyList()
        predicate(sequence.first()) -> listOf(sequence.first()) + filterSeq(predicate, sequence.drop(1))
        else -> filterSeq(predicate, sequence.drop(1))
    }

public fun <T, R> accumulateSeq(
    op: (T, R) -> R,
    initial: R,
    sequence: List<T>,
): R = if (sequence.isEmpty()) initial else op(sequence.first(), accumulateSeq(op, initial, sequence.drop(1)))

/** The book's `enumerate-interval`: every integer from `low` to `high`, inclusive. */
public fun enumerateInterval(
    low: Long,
    high: Long,
): List<Long> = if (low > high) emptyList() else listOf(low) + enumerateInterval(low + 1L, high)

/** The book's `enumerate-tree`: every leaf, left to right, reusing 2.2.2's `Tree`. */
public fun enumerateTree(t: Tree): List<Long> =
    when (t) {
        is Tree.Leaf -> listOf(t.value)
        is Tree.Node -> t.subtrees.flatMap(::enumerateTree)
    }

/** `sum-odd-squares`: enumerate a tree's leaves, filter the odd ones, square, and sum. */
public fun sumOddSquares(t: Tree): Long =
    accumulateSeq(
        { x: Long, acc: Long -> x + acc },
        0L,
        mapSeq({ x -> x * x }, filterSeq({ x -> x % 2L != 0L }, enumerateTree(t))),
    )

private tailrec fun fib(
    n: Long,
    a: Long = 0L,
    b: Long = 1L,
): Long = if (n == 0L) a else fib(n - 1L, b, a + b)

/** `even-fibs`: the even numbers among the first `n + 1` Fibonacci numbers. */
public fun evenFibs(n: Long): List<Long> =
    accumulateSeq(
        { x: Long, acc: List<Long> -> listOf(x) + acc },
        emptyList(),
        filterSeq({ x -> x % 2L == 0L }, mapSeq(::fib, enumerateInterval(0L, n))),
    )

/** `list-fib-squares`: the squares of the first `n + 1` Fibonacci numbers. */
public fun listFibSquares(n: Long): List<Long> = mapSeq({ x -> x * x }, mapSeq(::fib, enumerateInterval(0L, n)))

/** `product-of-squares-of-odd-elements`: the product of the squares of a sequence's odd elements. */
public fun productOfSquaresOfOddElements(sequence: List<Long>): Long =
    accumulateSeq(
        { x: Long, acc: Long -> x * acc },
        1L,
        mapSeq({ x -> x * x }, filterSeq({ x -> x % 2L != 0L }, sequence)),
    )

/** One personnel record, for the `salary-of-highest-paid-programmer` example. */
public data class Record(
    val name: String,
    val salary: Long,
    val isProgrammer: Boolean,
)

/** `salary-of-highest-paid-programmer`: the highest salary among the programmer records. */
public fun salaryOfHighestPaidProgrammer(records: List<Record>): Long =
    accumulateSeq(
        { x: Long, acc: Long -> maxOf(x, acc) },
        0L,
        mapSeq(Record::salary, filterSeq(Record::isProgrammer, records)),
    )

/** The book's `flatmap`: map, then accumulate the results with append. */
public fun <T, R> flatMapSeq(
    proc: (T) -> List<R>,
    sequence: List<T>,
): List<R> = accumulateSeq({ x: List<R>, acc: List<R> -> x + acc }, emptyList(), mapSeq(proc, sequence))

private fun isPrime(n: Long): Boolean {
    if (n < 2L) return false
    var divisor = 2L
    while (divisor * divisor <= n) {
        if (n % divisor == 0L) return false
        divisor += 1L
    }
    return true
}

private fun primeSumPredicate(pair: List<Long>): Boolean = isPrime(pair[0] + pair[1])

private fun makePairSum(pair: List<Long>): List<Long> = listOf(pair[0], pair[1], pair[0] + pair[1])

/** `prime-sum-pairs`: every ordered pair (i, j), 1 <= j < i <= n, with i + j prime, as (i, j, i + j) triples. */
public fun primeSumPairs(n: Long): List<List<Long>> =
    mapSeq(
        ::makePairSum,
        filterSeq(
            ::primeSumPredicate,
            flatMapSeq(
                { i: Long -> mapSeq({ j: Long -> listOf(i, j) }, enumerateInterval(1L, i - 1L)) },
                enumerateInterval(1L, n),
            ),
        ),
    )

private fun removeItem(
    item: Long,
    sequence: List<Long>,
): List<Long> = filterSeq({ x -> x != item }, sequence)

/** `permutations`: every ordering of the elements of `s`. */
public fun permutations(s: List<Long>): List<List<Long>> =
    if (s.isEmpty()) {
        listOf(emptyList())
    } else {
        flatMapSeq({ x: Long -> mapSeq({ p: List<Long> -> listOf(x) + p }, permutations(removeItem(x, s))) }, s)
    }

public class S2_2_3SequencesAsInterfacesTest :
    FunSpec({
        test("mapSeq squares a list, filterSeq keeps the odd ones, accumulateSeq sums with +") {
            mapSeq({ x: Long -> x * x }, listOf(1L, 2L, 3L)) shouldBe listOf(1L, 4L, 9L)
            filterSeq({ x: Long -> x % 2L != 0L }, listOf(1L, 2L, 3L, 4L, 5L)) shouldBe listOf(1L, 3L, 5L)
            accumulateSeq({ x: Long, acc: Long -> x + acc }, 0L, listOf(1L, 2L, 3L, 4L, 5L)) shouldBe 15L
        }
        test("enumerateInterval lists every integer in range, enumerateTree lists every leaf") {
            enumerateInterval(2L, 7L) shouldBe listOf(2L, 3L, 4L, 5L, 6L, 7L)
            enumerateTree(tree(leaf(1L), tree(leaf(2L), leaf(3L)), leaf(4L))) shouldBe listOf(1L, 2L, 3L, 4L)
        }
        test("sumOddSquares sums the squares of a tree's odd leaves") {
            sumOddSquares(tree(leaf(1L), tree(leaf(2L), leaf(3L)), leaf(4L), leaf(5L))) shouldBe (1L + 9L + 25L)
        }
        test("evenFibs collects the even Fibonacci numbers up to index n") {
            evenFibs(10L) shouldBe listOf(0L, 2L, 8L, 34L)
        }
        test("listFibSquares squares the Fibonacci numbers up to index n") {
            listFibSquares(5L) shouldBe listOf(0L, 1L, 1L, 4L, 9L, 25L)
        }
        test("productOfSquaresOfOddElements multiplies the squares of the odd elements") {
            productOfSquaresOfOddElements(listOf(1L, 2L, 3L, 4L, 5L)) shouldBe (1L * 9L * 25L)
        }
        test("salaryOfHighestPaidProgrammer finds the maximum salary among programmer records") {
            val records =
                listOf(
                    Record("Ben", 90000L, true),
                    Record("Alyssa", 95000L, true),
                    Record("Cy", 70000L, false),
                )
            salaryOfHighestPaidProgrammer(records) shouldBe 95000L
        }
        test("primeSumPairs finds the pairs summing to a prime, the book's n = 6 example") {
            primeSumPairs(6L).map { it[2] } shouldBe listOf(3L, 5L, 5L, 7L, 7L, 7L, 11L)
        }
        test("permutations of {1, 2, 3} enumerates all six orderings") {
            permutations(listOf(1L, 2L, 3L)).toSet().size shouldBe 6
            permutations(listOf(1L, 2L, 3L)) shouldBe
                listOf(
                    listOf(1L, 2L, 3L),
                    listOf(1L, 3L, 2L),
                    listOf(2L, 1L, 3L),
                    listOf(2L, 3L, 1L),
                    listOf(3L, 1L, 2L),
                    listOf(3L, 2L, 1L),
                )
        }
    })
