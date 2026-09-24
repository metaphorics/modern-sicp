// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3 support: the digital-circuit simulator the
// section exercises build on. This file is kept byte-identical in
// exercises/ and solutions/. The or-gate is exercise 3.28's own
// deliverable, so it lives in E3_28.kt as an extension on Simulation.

package sicp.ch3.exercises

import arrow.core.Either

/** The book's inverter-delay. */
public val inverterDelay: Long = 2L

/** The book's and-gate-delay. */
public val andGateDelay: Long = 3L

/**
 * The book's `logical-not`. The wire discipline keeps every signal at 0
 * or 1, so the `else` branch is unreachable for a well-formed circuit.
 */
private fun logicalNot(s: Long): Long =
    when (s) {
        0L -> 1L
        1L -> 0L
        else -> throw IllegalArgumentException("Invalid signal: $s")
    }

/** The book's logical-and. */
private fun logicalAnd(
    a: Long,
    b: Long,
): Long = if (a == 1L && b == 1L) 1L else 0L

/**
 * The book's wire: a signal value, initially 0, and the collection of
 * action procedures to run when the signal changes.
 */
public class Wire {
    private var signalValue: Long = 0L
    private val actions = mutableListOf<() -> Unit>()

    /** The book's `get-signal`: the current value of the signal. */
    public fun getSignal(): Long = signalValue

    /**
     * The book's `set-signal!`: store the new value and, when it differs
     * from the old, call every action in the order the actions were
     * added. The loop walks a snapshot so an action may add actions to
     * other wires while it runs.
     */
    public fun setSignal(v: Long) {
        if (signalValue == v) {
            return
        }
        signalValue = v
        for (action in actions.toList()) {
            action()
        }
    }

    /**
     * The book's `add-action!`: register the action and run it once
     * immediately, so a device attached to a wire learns the signal the
     * wire already carries (exercise 3.31).
     */
    public fun addAction(action: () -> Unit) {
        actions.add(action)
        action()
    }
}

/**
 * The agenda's segment queue: the front/rear pointer discipline of the
 * 3.3.2 queue, typed for the no-argument actions an agenda schedules.
 * `delete` answers null for an empty queue instead of a typed error,
 * because the agenda removes an item only after confirming one exists.
 */
public class ActionQueue {
    private class Cell(
        val item: () -> Unit,
        var next: Cell?,
    )

    private var front: Cell? = null
    private var rear: Cell? = null

    /** Whether the queue holds no actions. */
    public fun emptyQueue(): Boolean = front == null

    /** The front action, still queued; null exactly when empty. */
    public fun frontItem(): (() -> Unit)? = front?.item

    /** The book's `insert-queue!`: hang the new cell off the rear. */
    public fun insert(item: () -> Unit) {
        val cell = Cell(item, null)
        val tail = rear
        if (tail == null) {
            front = cell
        } else {
            tail.next = cell
        }
        rear = cell
    }

    /** The book's `delete-queue!`: advance the front pointer and answer
     * the removed action, or null when the queue is empty. */
    public fun delete(): (() -> Unit)? {
        val head = front ?: return null
        front = head.next
        if (front == null) {
            rear = null
        }
        return head.item
    }
}

/** The book's time segment: one timestamp and the queue of actions due
 * at it. */
public class TimeSegment(
    public val time: Int,
    public val queue: ActionQueue,
)

/** Where the book's agenda procedures raised, this edition answers
 * `Either.Left`. */
public sealed interface AgendaError {
    public data object EmptyAgenda : AgendaError
}

/**
 * The book's agenda: the time segments sorted by increasing time, one
 * queue of actions per time. The book kept the current time at the head
 * of its agenda list; this edition keeps it in the [Simulation] that
 * drives the agenda, where [Simulation.propagate] sets it from each
 * first item before running it.
 */
public class Agenda {
    private val segments = mutableListOf<TimeSegment>()

    /** The book's `empty-agenda?`. */
    public fun emptyAgenda(): Boolean = segments.isEmpty()

    /** The time of the first segment, 0 when the agenda is empty. */
    public fun currentTime(): Int = segments.firstOrNull()?.time ?: 0

    /**
     * The book's `add-to-agenda!`: add the action to the queue of its
     * time, creating that segment in sorted position when no segment
     * holds the time yet. Actions added to one time queue off behind
     * the actions already waiting there.
     */
    public fun addToAgenda(
        time: Int,
        action: () -> Unit,
    ) {
        val index = segments.indexOfFirst { it.time >= time }
        if (index >= 0 && segments[index].time == time) {
            segments[index].queue.insert(action)
            return
        }
        val segment = TimeSegment(time, ActionQueue())
        segment.queue.insert(action)
        if (index < 0) {
            segments.add(segment)
        } else {
            segments.add(index, segment)
        }
    }

    /** The book's `remove-first-agenda-item!`: delete the first action
     * of the first segment, and drop the segment when its queue
     * empties. */
    public fun removeFirstAgendaItem() {
        val first = segments.first()
        first.queue.delete()
        if (first.queue.emptyQueue()) {
            segments.removeAt(0)
        }
    }

    /** The book's `first-agenda-item`: the first segment's time paired
     * with the action at its queue's head; `Either.Left(EmptyAgenda)`
     * where the book raised. */
    public fun firstAgendaItem(): Either<AgendaError, Pair<Int, () -> Unit>> {
        val first = segments.firstOrNull()
        val item = first?.queue?.frontItem()
        return if (first == null || item == null) {
            Either.Left(AgendaError.EmptyAgenda)
        } else {
            Either.Right(first.time to item)
        }
    }
}

/**
 * The book's `the-agenda` and its procedures, gathered into one object:
 * the agenda and the probe log are constructor properties, and
 * `afterDelay`, `propagate`, `probe`, the gate constructors, and the
 * compound circuit constructors are members.
 */
public class Simulation(
    public val agenda: Agenda = Agenda(),
    public val probeLog: MutableList<String> = mutableListOf(),
) {
    /**
     * The book's current time: the time of the most recently processed
     * action. Keeping it here (rather than at the head of the agenda)
     * means it survives the deletion of the segment it came from, and
     * scheduling still works between two `propagate` runs.
     */
    private var currentTime: Int = 0

    /** The book's `after-delay`: run the action `delay` from now. */
    public fun afterDelay(
        delay: Long,
        action: () -> Unit,
    ) {
        agenda.addToAgenda(currentTime + delay.toInt(), action)
    }

    /**
     * The book's `propagate`: while the agenda is nonempty, take the
     * first item's time as the current time, run it, and remove it. The
     * agenda empties when every scheduled action has run; new items the
     * actions schedule keep the loop going.
     */
    public fun propagate(): Either<AgendaError, Unit> {
        while (!agenda.emptyAgenda()) {
            val (time, action) =
                when (val item = agenda.firstAgendaItem()) {
                    is Either.Left -> return Either.Left(item.value)
                    is Either.Right -> item.value
                }
            currentTime = time
            action()
            agenda.removeFirstAgendaItem()
        }
        return Either.Right(Unit)
    }

    /**
     * The book's `probe`: whenever the wire's signal changes, record the
     * name, the new value, and the current time. The registration itself
     * runs the action once (exercise 3.31), so a fresh probe prints the
     * wire's initial state.
     */
    public fun probe(
        name: String,
        wire: Wire,
    ) {
        wire.addAction {
            probeLog.add("-- $name = ${wire.getSignal()}, current-time = $currentTime")
        }
    }

    /**
     * The book's inverter: whenever `input` changes, drive `output` to
     * the logical not of the input one inverter-delay later.
     */
    public fun inverter(
        input: Wire,
        output: Wire,
    ) {
        val invertInput = {
            val newValue = logicalNot(input.getSignal())
            afterDelay(inverterDelay) { output.setSignal(newValue) }
        }
        input.addAction(invertInput)
    }

    /** The book's and-gate: one and-gate-delay after either input
     * changes, drive `output` to the logical and of the inputs. */
    public fun andGate(
        a1: Wire,
        a2: Wire,
        output: Wire,
    ) {
        val andAction = {
            val newValue = logicalAnd(a1.getSignal(), a2.getSignal())
            afterDelay(andGateDelay) { output.setSignal(newValue) }
        }
        a1.addAction(andAction)
        a2.addAction(andAction)
    }

    /** The book's half-adder: an or-gate, two and-gates, and an
     * inverter over the internal wires `d` and `e`; the or-gate itself
     * is exercise 3.28's `Simulation.orGate`. */
    public fun halfAdder(
        a: Wire,
        b: Wire,
        s: Wire,
        c: Wire,
    ) {
        val d = Wire()
        val e = Wire()
        orGate(a, b, d)
        andGate(a, b, c)
        inverter(c, e)
        andGate(d, e, s)
    }

    /** The book's full-adder: two half-adders and an or-gate over the
     * internal wires `c1`, `c2`, and `s`. */
    public fun fullAdder(
        a: Wire,
        b: Wire,
        cIn: Wire,
        sum: Wire,
        cOut: Wire,
    ) {
        val c1 = Wire()
        val c2 = Wire()
        val s = Wire()
        halfAdder(b, cIn, s, c1)
        halfAdder(a, s, sum, c2)
        orGate(c1, c2, cOut)
    }
}
