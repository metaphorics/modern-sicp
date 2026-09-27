// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.47

package sicp.ch3.exercises

import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.Semaphore
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.yield
import java.util.concurrent.atomic.AtomicLong

/**
 * Part (a) of the exercise: a semaphore of size n in terms of mutexes.
 * The count is guarded by a mutex; because a mutex cannot park a
 * waiter by itself, waiters take a ticket from a channel when the count
 * goes negative and are woken by a release. This is Dijkstra's negative
 * count: |count| waiters are parked.
 */
public class MutexSemaphore(
    n: Int,
) {
    private val mutex = Mutex()
    private var count = n
    private val wake = Channel<Unit>(Channel.UNLIMITED)

    public suspend fun acquire() {
        var parked = false
        mutex.withLock {
            count -= 1
            parked = count < 0
        }
        if (parked) {
            wake.receive()
        }
    }

    public suspend fun release() {
        var wakeOne = false
        mutex.withLock {
            count += 1
            wakeOne = count <= 0
        }
        if (wakeOne) {
            wake.send(Unit)
        }
    }
}

/**
 * Part (b) of the exercise: a semaphore of size n in terms of an atomic
 * test-and-set. The count is a CAS target: acquire succeeds exactly
 * when a compare-and-exchange moves it from c to c - 1 for some c > 0;
 * a waiter retries, yielding between attempts so it hands the thread
 * back.
 */
public class TestAndSetSemaphore(
    n: Int,
) {
    private val count = AtomicLong(n.toLong())

    public suspend fun acquire() {
        while (true) {
            val seen = count.get()
            if (seen > 0 && count.compareAndSet(seen, seen - 1)) {
                return
            }
            yield()
        }
    }

    public suspend fun release() {
        count.incrementAndGet()
    }
}

/**
 * The edition's token semaphore: n tokens in a channel, receive to
 * acquire, send in finally to release, so a waiter parks in receive.
 */
public class TokenSemaphore(
    n: Int,
) {
    private val tokens = Channel<Unit>(n).apply { repeat(n) { trySend(Unit) } }

    public suspend fun acquire() {
        tokens.receive()
    }

    public suspend fun release() {
        tokens.send(Unit)
    }

    public suspend fun <A> use(f: suspend () -> A): A =
        try {
            acquire()
            f()
        } finally {
            release()
        }
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
) {
    var inside = 0
    coroutineScope {
        repeat(users) {
            launch {
                acquire()
                inside += 1
                if (inside > probe.maxInside) {
                    probe.maxInside = inside
                }
                yield()
                inside -= 1
                release()
                probe.completed += 1
            }
        }
    }
}

/**
 * Records completed users and the highest simultaneous occupancy the
 * semaphore under test allowed.
 */
public class PermitProbe {
    public var completed: Int = 0

    public var maxInside: Int = 0
}
