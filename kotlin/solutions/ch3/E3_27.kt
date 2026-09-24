// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.27

package sicp.ch3.exercises

/**
 * The book's memoizer: one local table captured by the returned
 * function. Each call consults the table under its argument first and
 * answers from storage; only a miss computes, through `f`, and stores
 * the fresh result.
 */
public fun memoize(f: (Long) -> Long): (Long) -> Long {
    val table = HashMap<Long, Long>()
    return { x ->
        val previouslyComputedResult = table[x]
        if (previouslyComputedResult != null) {
            previouslyComputedResult
        } else {
            val result = f(x)
            table[x] = result
            result
        }
    }
}

/**
 * The book's memo-fib: `memoize` applied to the self-recursive
 * Fibonacci lambda, so every recursive level consults the one shared
 * table. The knot is tied through a nullable `var` that `fibStep`
 * reads only once the memoized function has been built.
 */
public fun makeMemoFib(): (Long) -> Long {
    var memoFib: ((Long) -> Long)? = null

    fun fibStep(n: Long): Long =
        if (n == 0L) {
            0L
        } else if (n == 1L) {
            1L
        } else {
            val recurse =
                memoFib
                    ?: error("memoFib consulted before makeMemoFib finished building it")
            recurse(n - 1) + recurse(n - 2)
        }
    val memoized = memoize(::fibStep)
    memoFib = memoized
    return memoized
}
