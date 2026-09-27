// SPDX-License-Identifier: GPL-3.0-only
// Section 5.3: the list-structured memory. The book's two vectors
// `the-cars` and `the-cdrs` are the fields of one `Memory` object beside
// the 5.2 simulator, a pointer to a pair is an index into the two
// vectors, and the typed pointers of 5.3.1 are tagged words: the type
// field rides as the tag, so the pointer to the pair with index 5 is
// `VTagged("p", VInt(5))`, drawn `p5`. The allocation path is the book's
// expansion of `cons`: store the two arguments at the free pointer, hand
// back the pointer, increment the free pointer. The operations table
// this file builds (cons, car, cdr, set-car!, set-cdr!, eq?, and the
// type predicates) installs the list-structure operations of 5.3.1 into
// a plain 5.2 machine, which is exactly the book's assumption for the
// exercises of this section: the list-structure memory operations are
// available as machine primitives.

package sicp.ch5

import arrow.core.raise.Raise
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VReal
import sicp.runtime.VSym
import sicp.runtime.VTagged
import sicp.runtime.Value

/** A `cons` reached past the end of the memory vectors. */
public data class MemoryExhausted(
    val free: Int,
    val size: Int,
) : MachineError() {
    public override fun toString(): String = "the memory is exhausted"
}

/** A `vector-ref` or `vector-set!` named an index outside the vector. */
public data class MemoryIndexOutOfRange(
    val index: Int,
    val size: Int,
) : MachineError() {
    public override fun toString(): String = "memory index out of range: $index"
}

/** `car`, `cdr`, `set-car!`, or `set-cdr!` read a word that is not a
 *  pair pointer. */
public data class NotAPair(
    val operation: String,
    val word: Value,
) : MachineError() {
    public override fun toString(): String = "$operation: not a pair: ${wordToString(word)}"
}

/** The typed pointer to the pair with index [index]: the book's `p5`.
 *  The type field is the tag, the index is the payload. */
public fun pairPointer(index: Int): Value = VTagged("p", VInt(index.toLong()))

/** True when [word] is a pair pointer: the `pair?` predicate, which
 *  checks only the type field. */
public fun isPair(word: Value): Boolean = word is VTagged && word.tag == "p"

/** The index part of a pair pointer, or null for any other word: the
 *  book's rule that numeric operations on pointers use only the index
 *  portion. */
public fun pairIndexOf(word: Value): Int? = (word as? VTagged)?.takeIf { it.tag == "p" }?.let { (it.data as VInt).n.toInt() }

/** True when [word] is the empty list pointer `e0`: the `null?`
 *  predicate. */
public fun isNull(word: Value): Boolean = word is VNil

/** True when [word] is a symbol word: the `symbol?` predicate. Two
 *  instances of a symbol are the same pointer, the obarray's interning
 *  reduced to the runtime's symbols. */
public fun isSymbol(word: Value): Boolean = word is VSym

/** True when [word] is a number word: the `number?` predicate. */
public fun isNumber(word: Value): Boolean = word is VInt || word is VReal

/** The `eq?` operation: it simply tests the equality of all fields in
 *  the words. */
public fun eqWords(
    a: Value,
    b: Value,
): Boolean = a == b

/** The word drawn the way the book writes pointers in the memory
 *  vector: `p5` for the pair with index 5, `n4` for the number 4, `e0`
 *  for the empty list, a symbol as its name. */
public fun wordToString(word: Value): String =
    when {
        isPair(word) -> "p${(word as VTagged).data}"
        word is VInt -> "n${word.n}"
        word is VNil -> "e0"
        word is VSym -> word.name
        else -> word.toString()
    }

/** The list-structured memory of 5.3.1: the two vectors and the free
 *  pointer. A fresh cell is blank, drawn `e0`; [free] starts where the
 *  exercise says (exercise 5.20 starts it at `p1`). */
public class Memory(
    /** How many cells the vectors hold. */
    public val size: Int,
    free: Int = 0,
) {
    /** The book's `the-cars` vector. */
    public val theCars: Array<Value> = Array(size) { VNil }

    /** The book's `the-cdrs` vector. */
    public val theCdrs: Array<Value> = Array(size) { VNil }

    /** A pair pointer to the next available index: `cons` allocates
     *  here. */
    public var free: Int = free
}

/** The book's `vector-ref` over `the-cars`: a bounds-checked read. */
context(r: Raise<MachineError>)
public fun Memory.readTheCars(index: Int): Value {
    if (index < 0 || index >= theCars.size) r.raise(MemoryIndexOutOfRange(index, theCars.size))
    return theCars[index]
}

/** The book's `vector-ref` over `the-cdrs`. */
context(r: Raise<MachineError>)
public fun Memory.readTheCdrs(index: Int): Value {
    if (index < 0 || index >= theCdrs.size) r.raise(MemoryIndexOutOfRange(index, theCdrs.size))
    return theCdrs[index]
}

/** The book's `vector-set!` over `the-cars`. */
context(r: Raise<MachineError>)
public fun Memory.storeTheCars(
    index: Int,
    value: Value,
) {
    if (index < 0 || index >= theCars.size) r.raise(MemoryIndexOutOfRange(index, theCars.size))
    theCars[index] = value
}

/** The book's `vector-set!` over `the-cdrs`. */
context(r: Raise<MachineError>)
public fun Memory.storeTheCdrs(
    index: Int,
    value: Value,
) {
    if (index < 0 || index >= theCdrs.size) r.raise(MemoryIndexOutOfRange(index, theCdrs.size))
    theCdrs[index] = value
}

/** The allocation path: store the two arguments at the free pointer's
 *  index, hand back the pointer, increment the free pointer. Exhaustion
 *  is the typed [MemoryExhausted] fault. */
context(r: Raise<MachineError>)
public fun Memory.cons(
    carValue: Value,
    cdrValue: Value,
): Value {
    if (free >= size) r.raise(MemoryExhausted(free, size))
    val index = free
    theCars[index] = carValue
    theCdrs[index] = cdrValue
    free = index + 1
    return pairPointer(index)
}

/** The book's `car`: the entry in `the-cars` at the pointer's index. */
context(r: Raise<MachineError>)
public fun Memory.car(word: Value): Value = pairIndexOf(word)?.let { readTheCars(it) } ?: r.raise(NotAPair("car", word))

/** The book's `cdr`: the entry in `the-cdrs` at the pointer's index. */
context(r: Raise<MachineError>)
public fun Memory.cdr(word: Value): Value = pairIndexOf(word)?.let { readTheCdrs(it) } ?: r.raise(NotAPair("cdr", word))

/** The book's `set-car!`, answering the unspecified value, `VNil`. */
context(r: Raise<MachineError>)
public fun Memory.setCar(
    word: Value,
    value: Value,
): Value {
    val index = pairIndexOf(word) ?: r.raise(NotAPair("set-car!", word))
    storeTheCars(index, value)
    return VNil
}

/** The book's `set-cdr!`, answering the unspecified value, `VNil`. */
context(r: Raise<MachineError>)
public fun Memory.setCdr(
    word: Value,
    value: Value,
): Value {
    val index = pairIndexOf(word) ?: r.raise(NotAPair("set-cdr!", word))
    storeTheCdrs(index, value)
    return VNil
}

/** The book's `write`: the surface form of the structure [word]
 *  designates, walking `car` and `cdr` through the memory, so a planted
 *  list reads back as `(1 2 3)`. */
context(r: Raise<MachineError>)
public fun Memory.write(word: Value): String {
    if (!isPair(word)) {
        return when {
            isNull(word) -> "()"
            word is VInt -> word.n.toString()
            word is VSym -> word.name
            else -> wordToString(word)
        }
    }
    val next = cdr(word)
    if (!isPair(next) && !isNull(next)) {
        return "(${write(car(word))} . ${write(next)})"
    }
    val parts = ArrayList<String>()
    var cursor = word
    while (isPair(cursor)) {
        parts.add(write(car(cursor)))
        cursor = cdr(cursor)
    }
    if (isNull(cursor)) return parts.joinToString(" ", "(", ")")
    return "(${parts.joinToString(" ")} . ${write(cursor)})"
}

/** The memory-vector drawing, the lower half of the book's Figure 5.14:
 *  one column per cell, the index, then `the-cars`, then `the-cdrs`.
 *  Blank cells draw as `e0`. */
public fun Memory.dump(): String {
    val rows =
        listOf(
            "index" to Array(size) { i -> i.toString() },
            "the-cars" to Array(size) { i -> wordToString(theCars[i]) },
            "the-cdrs" to Array(size) { i -> wordToString(theCdrs[i]) },
        )
    val labelWidth = "the-cars".length + 1
    val widths = IntArray(size) { i -> rows.maxOf { (_, cells) -> cells[i].length } + 2 }
    return rows.joinToString("\n") { (label, cells) ->
        buildString {
            append(label.padEnd(labelWidth))
            cells.indices.forEach { i -> append(cells[i].padEnd(widths[i])) }
        }.trimEnd()
    }
}

/** The list-structure operations of 5.3.1 as a machine operations
 *  table: the selectors and mutators go through the index part of a
 *  pair pointer, `cons` through the free pointer, and the predicates
 *  check only the type field. Combine with `arithOperations` for the
 *  arithmetic a machine also names. */
public fun listOperations(memory: Memory): Map<String, Op> =
    buildMap {
        put("cons") { args -> memory.cons(args[0], args[1]) }
        put("car") { args -> memory.car(args[0]) }
        put("cdr") { args -> memory.cdr(args[0]) }
        put("set-car!") { args -> memory.setCar(args[0], args[1]) }
        put("set-cdr!") { args -> memory.setCdr(args[0], args[1]) }
        put("eq?") { args -> VBool(eqWords(args[0], args[1])) }
        put("pair?") { args -> VBool(isPair(args[0])) }
        put("null?") { args -> VBool(isNull(args[0])) }
        put("symbol?") { args -> VBool(isSymbol(args[0])) }
        put("number?") { args -> VBool(isNumber(args[0])) }
    }
