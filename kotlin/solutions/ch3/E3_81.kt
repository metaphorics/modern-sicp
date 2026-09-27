// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.81

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * The resettable random generator of 3.6 as a stream machine: a request
 * stream of [Generate] and [Reset] requests in, one word per request
 * out. No assignment anywhere; the current generator word travels
 * through the consStream tails.
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
 * request stream and produces the desired words, seeded with the 3.1.2
 * initial word. A `generate` outputs `rand-update` of the current word
 * and threads that word on; a `reset` outputs the requested value and
 * threads it as the new word. An exhausted request stream ends the
 * output.
 */
public fun randStream(requests: LStream<RandRequest>): LStream<ULong> = randLoop(requests, RANDOM_INIT)

/** The recursion carrying [x], the current generator word. */
private fun randLoop(
    requests: LStream<RandRequest>,
    x: ULong,
): LStream<ULong> {
    val request = requests.streamHead() ?: return LStream.Empty
    return when (request) {
        is RandRequest.Generate -> {
            val next = randNext(x)
            consStream(next) { randLoop(requests.streamTail(), next) }
        }

        is RandRequest.Reset -> {
            consStream(request.value) { randLoop(requests.streamTail(), request.value) }
        }
    }
}
