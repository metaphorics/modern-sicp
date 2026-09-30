// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3, constraints support shared by exercises 3.33-3.37

package sicp.ch3.exercises

import arrow.core.Either

/**
 * The book's constraint protocol: the two messages a connector sends to
 * the constraints it serves. `newValue` folds the book's
 * `inform-about-value` (the `I-have-a-value` arm of the `me` dispatch)
 * and `forgetValue` its `inform-about-no-value`.
 */
public interface Constraint {
    /** A connector this constraint serves has been given a value. */
    public fun newValue()

    /** A connector this constraint serves has lost its value. */
    public fun forgetValue()
}

/** Domain errors of the connector layer. */
public sealed interface ConstraintError {
    /** The book's "Contradiction": [old] is recorded and [attempted] arrived. */
    public data class Contradiction(
        val old: Long,
        val attempted: Long,
    ) : ConstraintError
}

/**
 * The book's `'user`: the outside world driving a connector through
 * `setValue` and `forgetValue`. It sits in no connector's constraint
 * list, so nothing ever dispatches to its two protocol methods.
 */
public data object User : Constraint {
    override fun newValue() {}

    override fun forgetValue() {}
}

/**
 * The book's connector of 3.3.5: one object whose local state is
 * `value`, `informant`, and `constraints`, with the five operations
 * `has-value?`, `get-value`, `set-value!`, `forget-value!`, and
 * `connect` as methods.
 */
public class Connector {
    private var value: Long? = null
    private var informant: Constraint? = null
    private val constraints = mutableListOf<Constraint>()

    /** The book's `has-value?`: whether anyone currently owns the value. */
    public fun hasValue(): Boolean = informant != null

    /** The book's `get-value`: the current value; null while valueless. */
    public fun value(): Long? = value

    /**
     * The book's `set-value!`: the first setter wins. A conflicting
     * value comes back as [ConstraintError.Contradiction]; an agreeing
     * one is ignored; both are `Either` values, never an exception.
     */
    public fun setValue(
        newValue: Long,
        setter: Constraint,
    ): Either<ConstraintError, Unit> {
        val current = value
        if (current == null) {
            value = newValue
            informant = setter
            informExcept(setter, Constraint::newValue)
            return Either.Right(Unit)
        }
        if (current == newValue) {
            return Either.Right(Unit)
        }
        return Either.Left(ConstraintError.Contradiction(current, newValue))
    }

    /**
     * The book's `forget-value!`: only the constraint that set the value
     * may retract it; any other retractor is ignored. The value is
     * dropped with the informant, the book's `has-value?` false, so no
     * constraint can poll a stale number.
     */
    public fun forgetValue(retractor: Constraint) {
        if (retractor !== informant) {
            return
        }
        value = null
        informant = null
        informExcept(retractor, Constraint::forgetValue)
    }

    /**
     * The book's `connect`: join [source] once (membership, like the
     * exception test below, is object identity), then bring it up to
     * date. New members join at index 0, so the newest constraint is
     * consulted first.
     */
    public fun connect(source: Constraint) {
        if (source !in constraints) {
            constraints.add(0, source)
        }
        if (hasValue()) {
            source.newValue()
        }
    }

    /**
     * The book's `for-each-except`: applies [inform] to every constraint
     * in the list except [exception], the one that just spoke. The test
     * is physical: `!==` checks reference identity, so lookalike
     * constraints remain distinct.
     */
    private fun informExcept(
        exception: Constraint,
        inform: (Constraint) -> Unit,
    ) {
        for (constraint in constraints) {
            if (constraint !== exception) {
                inform(constraint)
            }
        }
    }
}

/**
 * The book's `adder`: constrains [sum] to the sum of [a1] and [a2],
 * wires itself to all three connectors, and returns the constraint it
 * installed.
 */
public fun adder(
    a1: Connector,
    a2: Connector,
    sum: Connector,
): Constraint {
    val me = Adder(a1, a2, sum)
    a1.connect(me)
    a2.connect(me)
    sum.connect(me)
    return me
}

/** The adder's local state is the three connectors it constrains. */
private class Adder(
    private val a1: Connector,
    private val a2: Connector,
    private val sum: Connector,
) : Constraint {
    override fun newValue() {
        val x = a1.value()
        val y = a2.value()
        val total = sum.value()
        when {
            x != null && y != null -> sum.setValue(x + y, this)
            x != null && total != null -> a2.setValue(total - x, this)
            y != null && total != null -> a1.setValue(total - y, this)
        }
    }

    override fun forgetValue() {
        sum.forgetValue(this)
        a1.forgetValue(this)
        a2.forgetValue(this)
        newValue()
    }
}

/**
 * The book's `multiplier`: constrains [product] to the product of [m1]
 * and [m2]; a zero factor forces the product even when the other factor
 * is unknown. Backward inference divides with Long division, so a
 * network that infers a factor must feed the box quotients it can
 * divide exactly.
 */
public fun multiplier(
    m1: Connector,
    m2: Connector,
    product: Connector,
): Constraint {
    val me = Multiplier(m1, m2, product)
    m1.connect(me)
    m2.connect(me)
    product.connect(me)
    return me
}

/** The multiplier's local state is the three connectors it constrains. */
private class Multiplier(
    private val m1: Connector,
    private val m2: Connector,
    private val product: Connector,
) : Constraint {
    override fun newValue() {
        val x = m1.value()
        val y = m2.value()
        val total = product.value()
        when {
            x == 0L || y == 0L -> product.setValue(0L, this)
            x != null && y != null -> product.setValue(x * y, this)
            total != null && x != null -> m2.setValue(total / x, this)
            total != null && y != null -> m1.setValue(total / y, this)
        }
    }

    override fun forgetValue() {
        product.forgetValue(this)
        m1.forgetValue(this)
        m2.forgetValue(this)
        newValue()
    }
}

/**
 * The book's `constant`: sets [connector] to [v] once, at construction.
 * A well-formed network never notifies a constant box -- only the box
 * itself may set its connector -- so the book's error for a stray
 * message has nothing to answer and the two protocol methods stay empty.
 */
public fun constant(
    v: Long,
    connector: Connector,
): Constraint {
    val box = ConstantBox()
    connector.connect(box)
    connector.setValue(v, box)
    return box
}

/** The book's constant box: a constraint with no response of its own. */
private class ConstantBox : Constraint {
    override fun newValue() {}

    override fun forgetValue() {}
}

/**
 * The book's `probe`: appends `"Probe: <name> = <value>"` to [log] when
 * the connector is given a value and `"Probe: <name> = ?"` when it loses
 * one. Pass one list to several probes to read the book's transcript.
 */
public fun probe(
    name: String,
    connector: Connector,
    log: MutableList<String> = mutableListOf(),
): Constraint {
    val me = Probe(name, connector, log)
    connector.connect(me)
    return me
}

/** The probe's local state: its name, its connector, and the shared log. */
private class Probe(
    private val name: String,
    private val connector: Connector,
    private val log: MutableList<String>,
) : Constraint {
    override fun newValue() {
        log.add("Probe: $name = ${connector.value()}")
    }

    override fun forgetValue() {
        log.add("Probe: $name = ?")
    }
}

/**
 * The book's `celsius-fahrenheit-converter`: the 9/5/32 network of
 * figure 3.28 between [c] and [f].
 */
public fun celsiusFahrenheitConverter(
    c: Connector,
    f: Connector,
) {
    val u = Connector()
    val v = Connector()
    val w = Connector()
    val x = Connector()
    val y = Connector()
    multiplier(c, w, u)
    multiplier(v, x, u)
    adder(v, y, f)
    constant(9L, w)
    constant(5L, x)
    constant(32L, y)
}
