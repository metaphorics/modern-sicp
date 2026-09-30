// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either

/** The experimental modules of section 4, composed into a [Mode]. */
public enum class Module { LAZY, SEARCH, QUERY }

/** The admission gate of section 1: every engine entry point runs checked
 * source only, and the check precedes every guest effect. */
public object Admission {
    /** Composes requested modules into an execution mode. An unlisted
     * combination raises `UnsupportedComposition` (section 4.5). */
    public fun modeOf(modules: Set<Module>): Either<AdmissionError, Mode> = either { compose(modules) }

    /** Lexes, parses, and type-checks [source] in [mode]; only a checked
     * program may reach an evaluator. */
    public fun admit(
        source: String,
        mode: Mode,
        requireEntryPoint: Boolean = true,
    ): Either<AdmissionError, CheckedProgram> = either { checkSource(source, mode, requireEntryPoint) }

    /** Admission with explicit module composition. */
    public fun admit(
        source: String,
        modules: Set<Module>,
    ): Either<AdmissionError, CheckedProgram> = either { checkSource(source, compose(modules)) }

    /** Admission for engine pipelines already carrying an error channel. */
    context(r: Raise<AdmissionError>)
    public fun admitOrRaise(
        source: String,
        mode: Mode,
    ): CheckedProgram = checkSource(source, mode)

    context(r: Raise<AdmissionError>)
    private fun compose(modules: Set<Module>): Mode {
        val lazy = Module.LAZY in modules
        val search = Module.SEARCH in modules
        val query = Module.QUERY in modules
        if (lazy && search) {
            r.raise(
                AdmissionError.Unsupported(
                    "UnsupportedComposition",
                    Span(Place(0, 1, 1), Place(0, 1, 1)),
                    "Lazy with Search has no lesson-pinned interaction",
                ),
            )
        }
        return when {
            query && lazy -> Mode.QUERY_LAZY
            query && search -> Mode.QUERY_SEARCH
            query -> Mode.QUERY
            lazy -> Mode.LAZY
            search -> Mode.SEARCH
            else -> Mode.CORE
        }
    }

    context(r: Raise<AdmissionError>)
    private fun checkSource(
        source: String,
        mode: Mode,
        requireEntryPoint: Boolean = true,
    ): CheckedProgram {
        val tokens = Lexer(source).tokens()
        val syntax = Parser(tokens).parseProgram()
        return Checker(mode).check(syntax, requireEntryPoint)
    }
}
