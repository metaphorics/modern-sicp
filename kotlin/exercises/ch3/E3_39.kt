// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.39

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * P1 of exercise 3.39: the square is serialized, but the assignment of
 * its result to the shared variable is bare, so a switch may happen
 * between the protected block and the write. Run straight through.
 */
public suspend fun squareThenAssign(
    x: Cell,
    s: Serializer,
): Unit = throw PendingSolution()

/**
 * The same P1 with one injected yield between the protected square and
 * the bare assignment: exactly the preemption point whose existence the
 * exercise turns on.
 */
public suspend fun squareThenAssignWithPause(
    x: Cell,
    s: Serializer,
): Unit = throw PendingSolution()

/** P2 of the exercise: the whole increment inside one serialized block. */
public suspend fun serializedIncrement(
    x: Cell,
    s: Serializer,
): Unit = throw PendingSolution()

/** P1 fully serialized (the section's earlier contrast), for the check. */
public suspend fun fullySerializedSquare(
    x: Cell,
    s: Serializer,
): Unit = throw PendingSolution()
