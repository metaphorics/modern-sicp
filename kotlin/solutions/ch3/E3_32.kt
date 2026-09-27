// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.32

package sicp.ch3.exercises

/**
 * Exercise 3.32's pinned order: two actions added to one agenda segment
 * at the same time, then one `propagate`. The segment's `ActionQueue`
 * is first in, first out, so the actions run in insertion order and the
 * answer is `["first", "second"]`.
 */
public fun sameSegmentRunOrder(): List<String> {
    val sim = Simulation()
    val ran = mutableListOf<String>()
    sim.agenda.addToAgenda(5) { ran.add("first") }
    sim.agenda.addToAgenda(5) { ran.add("second") }
    sim.propagate()
    return ran
}

/**
 * The exercise's comparison simulator: the section's circuit rules with
 * one difference -- within a time segment, the most recently scheduled
 * action runs first: last in, first out. It exists only to run the
 * comparison the exercise asks for; the section's own [Agenda] is
 * untouched and stays first in, first out.
 */
private class LifoSimulation {
    private class Segment(
        val time: Int,
    ) {
        val stack = ArrayDeque<() -> Unit>()
    }

    /** Segments in increasing time order, one stack per time. */
    private val segments = mutableListOf<Segment>()
    private var currentTime = 0

    /** The segment for [time], created and slotted in when new. */
    private fun segmentAt(time: Int): Segment {
        val index = segments.indexOfFirst { it.time >= time }
        if (index >= 0 && segments[index].time == time) {
            return segments[index]
        }
        val segment = Segment(time)
        if (index < 0) segments.add(segment) else segments.add(index, segment)
        return segment
    }

    /** The book's `after-delay`, pushing onto the time's stack. */
    fun afterDelay(
        delay: Long,
        action: () -> Unit,
    ) {
        segmentAt(currentTime + delay.toInt()).stack.addLast(action)
    }

    /** The book's `propagate`, popping each segment's stack. */
    fun propagate() {
        while (segments.isNotEmpty()) {
            val first = segments.first()
            currentTime = first.time
            val action = first.stack.removeLast()
            action()
            if (first.stack.isEmpty()) {
                segments.removeAt(0)
            }
        }
    }

    /**
     * The book's and-gate wired against the comparison agenda: the same
     * rule as [Simulation.andGate] -- read both inputs whenever either
     * changes, schedule the output [andGateDelay] later -- so the only
     * difference between the two runs is which agenda receives the
     * scheduled actions.
     */
    fun andGate(
        a1: Wire,
        a2: Wire,
        output: Wire,
    ) {
        val andAction = {
            val newValue = if (a1.getSignal() == 1L && a2.getSignal() == 1L) 1L else 0L
            afterDelay(andGateDelay) { output.setSignal(newValue) }
        }
        a1.addAction(andAction)
        a2.addAction(andAction)
    }
}

/**
 * The exercise's and-gate trace over the section's own agenda: settle
 * the inputs at (0, 1), then move both to (1, 0) inside one time
 * segment. Each change runs the gate action the moment it lands, so two
 * output updates enter the same later segment: `and(1, 1) = 1` first,
 * then `and(1, 0) = 0`. The queue applies them in schedule order and
 * the output settles at the gate's actual final inputs: 0.
 */
public fun fifoResult(): Long {
    val sim = Simulation()
    val a1 = Wire()
    val a2 = Wire()
    val out = Wire()
    sim.andGate(a1, a2, out)
    a2.setSignal(1L)
    sim.propagate()
    a1.setSignal(1L)
    a2.setSignal(0L)
    sim.propagate()
    return out.getSignal()
}

/**
 * The comparison run: the same scenario over [LifoSimulation], where
 * the two same-segment updates run backwards. The update scheduled
 * second -- from the correct reading (1, 0) -- pops first, and the
 * update scheduled first -- from the stale reading (1, 1) -- lands
 * last, so the output is left at the gate's superseded value: 1.
 */
public fun lifoResult(): Long {
    val sim = LifoSimulation()
    val a1 = Wire()
    val a2 = Wire()
    val out = Wire()
    sim.andGate(a1, a2, out)
    a2.setSignal(1L)
    sim.propagate()
    a1.setSignal(1L)
    a2.setSignal(0L)
    sim.propagate()
    return out.getSignal()
}
