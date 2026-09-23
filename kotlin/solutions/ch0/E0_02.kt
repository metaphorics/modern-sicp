// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

/** `compose(f, g)` applies `g`, then `f`: the book's composition of 1.3.4. */
public fun compose(
    f: (Long) -> Long,
    g: (Long) -> Long,
): (Long) -> Long = { x -> f(g(x)) }

/** The self-tail-call of [repeatN] is what `tailrec` compiles into a loop. */
private tailrec fun repeatN(
    f: (Long) -> Long,
    k: Int,
    acc: Long,
): Long = if (k == 0) acc else repeatN(f, k - 1, f(acc))

/** `repeated(f, n)` applies `f` exactly `n` times; `n = 0` is the identity. */
public fun repeated(
    f: (Long) -> Long,
    n: Int,
): (Long) -> Long = { x -> repeatN(f, n, x) }
