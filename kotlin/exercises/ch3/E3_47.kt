// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.47

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Part (a) of the exercise: a semaphore of size n in terms of mutexes.
 * The count is guarded by a mutex; because a mutex cannot park a
 * waiter by itself, waiters take a ticket from a channel when the count
 * goes negative and are woken by a release.
 */
public class MutexSemaphore(
    n: Int,
) {
    public suspend fun acquire(): Unit = throw PendingSolution()

    public suspend fun release(): Unit = throw PendingSolution()
}

/**
 * Part (b) of the exercise: a semaphore of size n in terms of an atomic
 * test-and-set. The count is a CAS target; a waiter retries, yielding
 * between attempts so it hands the thread back.
 */
public class TestAndSetSemaphore(
    n: Int,
) {
    public suspend fun acquire(): Unit = throw PendingSolution()

    public suspend fun release(): Unit = throw PendingSolution()
}

/**
 * The edition's token semaphore: n tokens in a channel, receive to
 * acquire, send in finally to release, so a waiter parks in receive.
 */
public class TokenSemaphore(
    n: Int,
) {
    public suspend fun acquire(): Unit = throw PendingSolution()

    public suspend fun release(): Unit = throw PendingSolution()

    public suspend fun <A> use(f: suspend () -> A): A = throw PendingSolution()
}

/**
 * Run `users` users through one semaphore: each acquires, records its
 * occupancy in the probe, then releases. The probe answers how many
 * users completed and the highest occupancy the semaphore allowed.
 */
public suspend fun probeOccupancy(
    users: Int,
    acquire: suspend () -> Unit,
    release: suspend () -> Unit,
    probe: PermitProbe,
): Unit = throw PendingSolution()

/** Records completed users and the highest simultaneous occupancy. */
public class PermitProbe {
    public var completed: Int = 0

    public var maxInside: Int = 0
}
