// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55 (replaced)

package sicp.ch2.exercises

import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.vlist

/** The value `''abracadabra` would read as: `(quote (quote abracadabra))`. */
public val doubledQuoteAbracadabra: Value = vlist(VSym("quote"), vlist(VSym("quote"), VSym("abracadabra")))

/** `car` of [doubledQuoteAbracadabra]. */
public fun ex_2_55(): Value = (doubledQuoteAbracadabra as VPair).car
