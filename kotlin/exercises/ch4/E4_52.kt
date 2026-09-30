// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.52

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.52: `ifFail`, which enters its second body when the first
 * exhausts its answers. The guard fires only when every choice of the
 * first body is exhausted -- which is why the all-odd session prints
 * `all-odd`, while the session with 8 available prints 8 first and
 * `all-odd` only after that answer is exhausted.
 *
 * Expected answers: the all-odd probe prints `all-odd`; the eight probe
 * prints 8 then `all-odd`.
 */
public fun ifFailAllOddTranscript(): String = throw PendingSolution()

/** The session whose first body has the answer 8. => "8\nall-odd\n" */
public fun ifFailEightTranscript(): String = throw PendingSolution()
