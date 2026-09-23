// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

/** Thrown by an exercise body that is still an unsolved scaffold (D28). */
public class PendingSolution(
    message: String = "pending solution: this exercise scaffold has no solution yet",
) : IllegalStateException(message)
