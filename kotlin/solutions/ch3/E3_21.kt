// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.21

package sicp.ch3.exercises

import sicp.runtime.Datum

/** Observe a queue's front-to-rear items and the datum held by its rear cell. */
public fun benView(queue: Queue): Pair<List<Datum>, Datum?> = queue.items() to queue.rearCell()?.first
