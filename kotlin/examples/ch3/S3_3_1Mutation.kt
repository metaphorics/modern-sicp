// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3.1, mutation

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.setCar
import sicp.runtime.setCdr
import sicp.runtime.vlist

/**
 * The book's `cons` of 3.3.1, rebuilt from the two mutators: take a fresh
 * pair that belongs to no existing list structure, then point its two
 * slots at the designated objects.
 */
public fun cons(
    x: Value,
    y: Value,
): VPair {
    val newPair = VPair(VNil, VNil)
    newPair.setCar(x)
    newPair.setCdr(y)
    return newPair
}

/**
 * The book's `append` of 2.2.1: a fresh list built by consing the
 * elements of `x` onto `y`; no pair of `x` is touched.
 */
public fun append(
    x: Value,
    y: Value,
): Value =
    if (x !is VPair) {
        y
    } else {
        cons(x.car, append(x.cdr, y))
    }

/** The book's `last-pair`: the final pair of a nonempty chain. */
public fun lastPair(x: VPair): VPair = if (x.cdr is VNil) x else lastPair(x.cdr as VPair)

/** The book's `append!`: point the last pair of `x` at `y` and return `x`. */
public fun appendBang(
    x: VPair,
    y: VPair,
): VPair {
    lastPair(x).setCdr(y)
    return x
}

/** The book's `make-cycle` of exercise 3.13: close `x` onto itself. */
public fun makeCycle(x: VPair): VPair {
    lastPair(x).setCdr(x)
    return x
}

/**
 * The book's `mystery` of exercise 3.14: reverse a chain in place, reusing
 * the very pairs of `x`.
 */
public fun mystery(x: VPair): VPair {
    fun loop(
        x: Value,
        y: Value,
    ): VPair =
        if (x !is VPair) {
            y as VPair
        } else {
            val temp = x.cdr
            x.setCdr(y)
            loop(temp, x)
        }
    return loop(x, VNil)
}

/** The book's `set-to-wow!`: replace the car pointer of `x`'s first pair. */
public fun setToWow(x: VPair): VPair {
    (x.car as VPair).setCar(VSym("wow"))
    return x
}

/** The procedural pair of 3.3.1: four operations over captured slots. */
public interface PairProc {
    public fun car(): Value

    public fun cdr(): Value

    public fun setCar(v: Value)

    public fun setCdr(v: Value)
}

/**
 * The book's mutable `cons` as a closure: two captured `var` slots read by
 * the selectors and written by the mutators, exactly like the bank account
 * of 3.1.1 kept its balance.
 */
public fun proceduralCons(
    x: Value,
    y: Value,
): PairProc {
    var carCell = x
    var cdrCell = y
    return object : PairProc {
        override fun car(): Value = carCell

        override fun cdr(): Value = cdrCell

        override fun setCar(v: Value) {
            carCell = v
        }

        override fun setCdr(v: Value) {
            cdrCell = v
        }
    }
}

public class S3_3_1MutationTest :
    FunSpec({
        test("setCar replaces the car pointer of the pair x is bound to") {
            val x = cons(vlist(VSym("a"), VSym("b")), vlist(VSym("c"), VSym("d")))
            val y = vlist(VSym("e"), VSym("f"))

            x.setCar(y)

            x.toString() shouldBe "((e f) c d)"
        }

        test("setCdr replaces the cdr pointer and detaches (c d)") {
            val x = cons(vlist(VSym("a"), VSym("b")), vlist(VSym("c"), VSym("d")))
            val y = vlist(VSym("e"), VSym("f"))

            x.setCdr(y)

            x.toString() shouldBe "((a b) e f)"
        }

        test("append copies, appendBang splices x's last pair onto y") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val y = vlist(VSym("c"), VSym("d")) as VPair

            val z = append(x, y)
            z.toString() shouldBe "(a b c d)"
            x.cdr.toString() shouldBe "(b)"

            val w = appendBang(x, y)
            w.toString() shouldBe "(a b c d)"
            x.cdr.toString() shouldBe "(b c d)"
        }

        test("makeCycle closes the chain back onto its first pair") {
            val z = makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as VPair)
            val second = z.cdr as VPair
            val third = second.cdr as VPair

            (third.cdr === z) shouldBe true
            (second === z) shouldBe false
        }

        test("mystery reverses in place: v shrinks to (a), w is (d c b a)") {
            val v = vlist(VSym("a"), VSym("b"), VSym("c"), VSym("d")) as VPair
            val second = v.cdr as VPair
            val third = second.cdr as VPair
            val last = third.cdr as VPair

            val w = mystery(v)

            v.toString() shouldBe "(a)"
            w.toString() shouldBe "(d c b a)"
            (w === last) shouldBe true
            (w.cdr as VPair === third) shouldBe true
        }

        test("the set-to-wow session: shared z1 changes twice, unshared z2 once") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val z1 = cons(x, x)
            val z2 = cons(vlist(VSym("a"), VSym("b")), vlist(VSym("a"), VSym("b")))

            z1.toString() shouldBe "((a b) a b)"
            setToWow(z1).toString() shouldBe "((wow b) wow b)"

            z2.toString() shouldBe "((a b) a b)"
            setToWow(z2).toString() shouldBe "((wow b) a b)"
        }

        test("=== observes the sharing cons built") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val z1 = cons(x, x)
            val z2 = cons(vlist(VSym("a"), VSym("b")), vlist(VSym("a"), VSym("b")))

            (z1.car === z1.cdr) shouldBe true
            (z2.car === z2.cdr) shouldBe false
        }

        test("proceduralCons keeps its state in captured var slots") {
            val p = proceduralCons(VInt(1L), VInt(2L))
            p.car() shouldBe VInt(1L)
            p.cdr() shouldBe VInt(2L)

            val alias = p
            alias.setCar(VInt(17L))
            p.car() shouldBe VInt(17L)
        }
    })
