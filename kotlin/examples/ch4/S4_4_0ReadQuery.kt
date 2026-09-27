// SPDX-License-Identifier: GPL-3.0-only
// Shared reader helper for the 4.4.4 listing tests: one datum under a
// typed-error scope, the edition's `read` for query input.

package sicp.ch4.examples

import arrow.core.raise.either
import sicp.ch4.QueryFault
import sicp.ch4.querySyntaxProcess
import sicp.ch4.readDatum
import sicp.runtime.Value

/** Reads one query datum from `text`; the driver's read under a typed scope. */
public fun readQuery(text: String): Value = either { readDatum(text) }.fold({ e -> throw QueryFault(e) }, { it })

/** Reads and syntax-processes one query form, the driver's input step. */
public fun patQuery(text: String): Value = querySyntaxProcess(readQuery(text))
