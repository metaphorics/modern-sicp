// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 0.2: implement [compose] and [repeated] over `(Long) -> Long`
 * procedures and property-test them with `checkAll`.
 *
 * Section 0.2.
 *
 * `compose(f, g)(x)` is `f(g(x))`. `repeated(f, n)(x)` applies `f` exactly
 * `n` times, so `repeated(f, 0)(x)` is `x`. The statement lives in the
 * section 0.2 chapter text.
 */
public fun compose(
    f: (Long) -> Long,
    g: (Long) -> Long,
): (Long) -> Long = throw PendingSolution()

public fun repeated(
    f: (Long) -> Long,
    n: Int,
): (Long) -> Long = throw PendingSolution()
