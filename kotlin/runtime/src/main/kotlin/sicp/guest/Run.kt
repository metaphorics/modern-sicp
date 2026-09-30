// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

/** The ordered output stream of one guest evaluation. */
public class OutputSink {
    private val buffer = StringBuilder()

    /** Writes one rendered value or line exactly as the program produced it. */
    public fun write(text: String) {
        buffer.append(text)
    }

    /** An output cursor for one compiled-machine turn. */
    public fun mark(): Int = buffer.length

    /** Output emitted since [mark], without copying the older turns. */
    public fun since(mark: Int): String = buffer.substring(mark)

    /** The stream so far. */
    public fun contents(): String = buffer.toString()
}

/** What a guest evaluation observes (section 1): the ordered output stream,
 * the program's typed guest error if it raised one, and the value of main's
 * completion. Engines report categories, never host exceptions. */
public class RunResult(
    /** The ordered `print`/`println` stream, section 3.7 rendering. */
    public val output: String,
    /** The first typed guest error, category and position; null when none. */
    public val error: GuestError?,
    /** The value `main` completed with; `VUnit` for a block-bodied main. */
    public val mainValue: GValue?,
)
