// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.49

package sicp.ch3.exercises

import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withTimeout

/**
 * A named shared record whose contents are locked for the duration of a
 * piece of work.
 */
public class SharedRecord(
    public val name: String,
) {
    private val mutex = Mutex()

    /** Run `work` holding this record's lock. */
    public suspend fun <A> withRecord(work: suspend () -> A): A = mutex.withLock { work() }
}

/**
 * The directory of records. A process must hold the directory while it
 * discovers which record it needs next, which is exactly the situation
 * where acquisition order cannot be fixed in advance: the second
 * resource is unknown until the first lock is already held.
 */
public class ResourceDirectory {
    private val mutex = Mutex()
    private val records = mutableMapOf<String, SharedRecord>()

    /** Install a record so later lookups can find it. */
    public fun install(record: SharedRecord) {
        records[record.name] = record
    }

    /** Run `work` holding the directory, the one lock every lookup needs. */
    public suspend fun <A> withDirectory(work: suspend () -> A): A = mutex.withLock { work() }

    /** Look up a record by name; the caller must hold the directory. */
    public fun find(name: String): SharedRecord? = records[name]
}

/**
 * The scenario of the exercise: one process holds the directory and
 * waits for a record; the other holds that record and waits for the
 * directory to name its next resource. Each process acquires its first
 * resource before it can know the second, so no numbering fixed in
 * advance can order the acquisitions, and the two park forever.
 */
public suspend fun directoryDeadlockDemo(): String {
    val directory = ResourceDirectory()
    val record = SharedRecord("checking")
    directory.install(record)
    val results = mutableListOf<String>()
    return try {
        withTimeout(2_000) {
            coroutineScope {
                launch {
                    val moved =
                        directory.withDirectory {
                            record.withRecord {
                                "moved the record"
                            }
                        }
                    results.add(moved)
                }
                launch {
                    val next =
                        record.withRecord {
                            directory.withDirectory {
                                directory.find("savings")
                            }
                            "learned the next resource"
                        }
                    results.add(next)
                }
            }
            "no deadlock after ${results.size} completions"
        }
    } catch (e: TimeoutCancellationException) {
        "deadlock: the directory holder waits for the record, the record holder waits for the directory"
    }
}
