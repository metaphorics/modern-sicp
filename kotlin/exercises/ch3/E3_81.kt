// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.81

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * The resettable random generator of 3.6 as a stream machine: a request
 * stream in, one word per request out, no assignment anywhere.
 */
public sealed interface RandRequest {
    /** Ask for the next word: `rand-update` of the current word. */
    public data object Generate : RandRequest

    /** Restart the sequence at [value]; the output word is [value]. */
    public data class Reset(
        public val value: ULong,
    ) : RandRequest
}

/**
 * The stream formulation of the 3.6 generator: [randStream] consumes the
 * request stream and produces the desired words, the current generator
 * word traveling through the consStream tails. A `generate` outputs
 * `rand-update` of the current word; a `reset` outputs the requested
 * value and threads it as the new word.
 */
public fun randStream(requests: LStream<RandRequest>): LStream<ULong> = throw PendingSolution()
