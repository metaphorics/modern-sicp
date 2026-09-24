// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.54

package sicp.ch2.exercises

import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.vlist

public fun myEqual(
    a: Value,
    b: Value,
): Boolean =
    when {
        a is VSym && b is VSym -> a == b
        a is VPair && b is VPair -> myEqual(a.car, b.car) && myEqual(a.cdr, b.cdr)
        a is VNil && b is VNil -> true
        else -> false
    }

public fun ex_2_54(): List<Boolean> {
    val listWords = vlist(VSym("this"), VSym("is"), VSym("a"), VSym("list"))
    val sameShape = vlist(VSym("this"), VSym("is"), VSym("a"), VSym("list"))
    val differentShape = vlist(VSym("this"), vlist(VSym("is"), VSym("a")), VSym("list"))
    return listOf(myEqual(listWords, sameShape), myEqual(listWords, differentShape))
}
