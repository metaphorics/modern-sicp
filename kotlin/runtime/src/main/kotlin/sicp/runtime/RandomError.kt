// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

/** The typed errors of the runtime. */
public sealed class RandomError {
    /** `seeded` rejects a zero seed. */
    public data object InvalidSeed : RandomError() {
        /** Prints the error name and its cause. */
        override fun toString(): String = "InvalidSeed: the seed of random must be nonzero"
    }
}
