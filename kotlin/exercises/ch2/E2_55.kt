// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55 (replaced)

package sicp.ch2.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 2.55 (replaced, R): SICP's Eva Lu Ator types `(car ''abracadabra)`
 * and the interpreter prints back `quote`; the question is why. Kotlin has
 * no reader, so `''abracadabra` is never something a running program
 * types at a prompt, but the reader's expansion rule is simple enough to
 * apply by hand: a single quote `'x` is sugar for the two-element list
 * `(quote x)`, so the doubled quote `''abracadabra` is sugar for
 * `(quote (quote abracadabra))`.
 *
 * Build the `Value` that `''abracadabra` would have read as, using only
 * `vlist`, `VSym`, and the pair chain (no reader involved), and return its
 * `car`. The rationale explains why that `car` is the symbol `quote`
 * rather than `abracadabra`.
 */
public fun ex_2_55(): Value = throw PendingSolution()
