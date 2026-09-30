// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, section 5.3: the list-structured memory the section's
// machines treat as primitives. The two vectors, the typed pointer
// words (`p5`), the number (`n4`) and empty (`e0`) words, the
// allocation path the book's expansion of `cons` describes, and the
// operations table of cons, car, cdr, and the mutators live here, one
// home for the memory model the exercises of the section share.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.NO_POSITION
import sicp.runtime.MachineOp

// ---------------------------------------------------------------------------
// Memory words
// ---------------------------------------------------------------------------

/** The typed pointer to the pair with [index]: the book's `p5`. */
public fun pairPointer(index: Int): GValue =
    GValue.VObject("pair", structural = false, fields = mutableMapOf("index" to GValue.VInt(index)))

/** The index part of a pair pointer, or null for any other word: the
 *  book's rule that numeric operations on pointers use only the index
 *  portion. */
public fun pairIndexOf(word: GValue): Int? =
    (word as? GValue.VObject)
        ?.takeIf { it.className == "pair" }
        ?.let { (it.fields["index"] as GValue.VInt).value }

/** True when [word] is a pair pointer: the `pair?` predicate, which
 *  checks only the type field. */
public fun isPair(word: GValue): Boolean = pairIndexOf(word) != null

/** A number word: the book's `n4`. */
public fun numberWord(n: Long): GValue = GValue.VLong(n)

/** The word drawn the way the book writes pointers in the memory
 *  vector: `p5` for the pair with index 5, `n4` for the number 4, `e0`
 *  for the empty list. */
public fun wordToString(word: GValue): String =
    when {
        isPair(word) -> "p${pairIndexOf(word)}"
        word is GValue.VLong -> "n${word.value}"
        word is GValue.VNull -> "e0"
        else -> render(word)
    }

// ---------------------------------------------------------------------------
// The memory
// ---------------------------------------------------------------------------

/** The list-structured memory of 5.3.1: the two vectors and the free
 *  pointer. A fresh cell is blank, drawn `e0`; [free] starts where the
 *  exercise says (exercise 5.20 starts it at `p1`). */
public class Memory(
    /** How many cells the vectors hold. */
    public val size: Int,
    free: Int = 0,
) {
    /** The book's `the-cars` vector. */
    public val theCars: Array<GValue> = Array(size) { GValue.VNull }

    /** The book's `the-cdrs` vector. */
    public val theCdrs: Array<GValue> = Array(size) { GValue.VNull }

    /** A pair pointer to the next available index: `cons` allocates
     *  here. */
    public var free: Int = free

    /** The book's expansion of `cons`: store the two arguments at the
     *  free pointer, hand back the pointer, increment the free pointer. */
    context(r: Raise<GuestError>)
    public fun cons(
        car: GValue,
        cdr: GValue,
    ): GValue {
        if (free >= size) r.raise(GuestError.IndexOutOfBounds(NO_POSITION))
        theCars[free] = car
        theCdrs[free] = cdr
        return pairPointer(free++)
    }

    /** `car` over the memory: the pair's `the-cars` slot. */
    context(r: Raise<GuestError>)
    public fun car(word: GValue): GValue = theCars[pairSlot(word, "car")]

    /** `cdr` over the memory: the pair's `the-cdrs` slot. */
    context(r: Raise<GuestError>)
    public fun cdr(word: GValue): GValue = theCdrs[pairSlot(word, "cdr")]

    /** `set-car!` over the memory. */
    context(r: Raise<GuestError>)
    public fun setCar(
        word: GValue,
        value: GValue,
    ) {
        theCars[pairSlot(word, "set-car!")] = value
    }

    /** `set-cdr!` over the memory. */
    context(r: Raise<GuestError>)
    public fun setCdr(
        word: GValue,
        value: GValue,
    ) {
        theCdrs[pairSlot(word, "set-cdr!")] = value
    }

    context(r: Raise<GuestError>)
    private fun pairSlot(
        word: GValue,
        operation: String,
    ): Int = pairIndexOf(word) ?: r.raise(GuestError.UnassignedRead(NO_POSITION))

    /** The memory-vector drawing: one row per vector, one column per
     *  cell, the book's table. */
    public fun dump(): String =
        listOf(
            dumpRow("index", (0 until size).map { it.toString() }),
            dumpRow("the-cars", theCars.map { wordToString(it) }),
            dumpRow("the-cdrs", theCdrs.map { wordToString(it) }),
        ).joinToString("\n")

    private fun dumpRow(
        label: String,
        cells: List<String>,
    ): String = (label.padEnd(9) + cells.joinToString(separator = "") { it.padEnd(4) }).trimEnd()

    /** A structure printed the way the book reads it: `(1 2 3 4 5)` for
     *  the proper list of numbers, nested lists inside their parens. */
    context(r: Raise<GuestError>)
    public fun write(word: GValue): String =
        when {
            isPair(word) -> {
                val items = mutableListOf<String>()
                var cursor = word
                while (isPair(cursor)) {
                    items.add(write(car(cursor)))
                    cursor = cdr(cursor)
                }
                "(${items.joinToString(separator = " ")})"
            }

            word is GValue.VLong -> {
                word.value.toString()
            }

            word is GValue.VNull -> {
                "()"
            }

            else -> {
                wordToString(word)
            }
        }
}

// ---------------------------------------------------------------------------
// The list-structure operations of 5.3.1
// ---------------------------------------------------------------------------

context(r: Raise<GuestError>)
private fun argAt(
    args: List<GValue>,
    index: Int,
): GValue = args.getOrNull(index) ?: r.raise(GuestError.UnassignedRead(NO_POSITION))

/** The operations table the section's machines assume: cons, car, cdr,
 *  and the two mutators over [memory], plus the type predicates. */
public fun listOperations(memory: Memory): Map<String, MachineOp> =
    mapOf(
        "null?" to { args -> GValue.VBool(argAt(args, 0) is GValue.VNull) },
        "pair?" to { args -> GValue.VBool(isPair(argAt(args, 0))) },
        "number?" to { args -> GValue.VBool(argAt(args, 0) is GValue.VLong) },
        "car" to { args -> memory.car(argAt(args, 0)) },
        "cdr" to { args -> memory.cdr(argAt(args, 0)) },
        "cons" to { args -> memory.cons(argAt(args, 0), argAt(args, 1)) },
        "set-car!" to { args -> GValue.VUnit.also { memory.setCar(argAt(args, 0), argAt(args, 1)) } },
        "set-cdr!" to { args -> GValue.VUnit.also { memory.setCdr(argAt(args, 0), argAt(args, 1)) } },
    )
