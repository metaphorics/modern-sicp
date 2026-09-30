// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

import arrow.core.raise.Raise
import kotlin.math.pow

/**
 * The admitted library surface of section 2.4 as runtime operations, shared
 * by every engine so direct, analyzed, machine, and compiled execution agree
 * on observable behavior. Errors are typed [GuestError] categories with
 * positions; nothing here throws host exceptions across the boundary.
 */
public object Primitives {
    context(r: Raise<GuestError>)
    public fun invoke(
        fn: GValue,
        arguments: List<GValue>,
        at: Span,
    ): GValue {
        if (fn !is GValue.VFunction) r.raise(GuestError.UnassignedRead(at))
        return fn.apply(r, arguments)
    }

    context(r: Raise<GuestError>)
    public fun call(
        name: String,
        arguments: List<GValue>,
        sink: OutputSink,
        at: Span,
    ): GValue =
        when (name) {
            "print" -> {
                writeOutput(arguments, sink, at, trailing = false)
            }

            "println" -> {
                if (arguments.isEmpty()) {
                    sink.write("\n")
                    GValue.VUnit
                } else {
                    writeOutput(arguments, sink, at, trailing = true)
                }
            }

            "listOf" -> {
                GValue.VList(arguments.toMutableList(), mutable = false)
            }

            "mutableListOf" -> {
                GValue.VList(arguments.toMutableList(), mutable = true)
            }

            "setOf" -> {
                GValue.VList(deduplicated(arguments).toMutableList(), mutable = false, asSet = true)
            }

            "emptyList" -> {
                GValue.VList(mutableListOf(), mutable = false)
            }

            "emptySet" -> {
                GValue.VList(mutableListOf(), mutable = false, asSet = true)
            }

            "mapOf" -> {
                GValue.VMap(mapEntries(arguments, at), mutable = false)
            }

            "mutableMapOf" -> {
                GValue.VMap(mapEntries(arguments, at), mutable = true)
            }

            "emptyMap" -> {
                GValue.VMap(LinkedHashMap(), mutable = false)
            }

            "abs" -> {
                absolute(arguments, at)
            }

            "min", "max" -> {
                extreme(name, arguments, at)
            }

            "sqrt", "floor", "ceil" -> {
                doubleCall(name, arguments, at)
            }

            "pow" -> {
                power(arguments, at)
            }

            "addExact" -> {
                exact(arguments, at, java.lang.Math::addExact, java.lang.Math::addExact)
            }

            "subtractExact" -> {
                exact(arguments, at, java.lang.Math::subtractExact, java.lang.Math::subtractExact)
            }

            "multiplyExact" -> {
                exact(arguments, at, java.lang.Math::multiplyExact, java.lang.Math::multiplyExact)
            }

            "negateExact" -> {
                negateExact(arguments, at)
            }

            else -> {
                r.raise(GuestError.UnassignedRead(at))
            }
        }

    context(r: Raise<GuestError>)
    private fun writeOutput(
        arguments: List<GValue>,
        sink: OutputSink,
        at: Span,
        trailing: Boolean,
    ): GValue {
        val rendered = renderPrinted(arguments[0]) ?: r.raise(GuestError.UnassignedRead(at))
        sink.write(rendered)
        if (trailing) sink.write("\n")
        return GValue.VUnit
    }

    /** `setOf` keeps the first of every group of elements that are `==`, so
     * pairs, lists, and data classes dedupe by structure (section 3.5). */
    context(r: Raise<GuestError>)
    private fun deduplicated(arguments: List<GValue>): List<GValue> {
        val out = mutableListOf<GValue>()
        for (argument in arguments) {
            if (out.none { valueEquals(it, argument) }) out.add(argument)
        }
        return out
    }

    /** Stores [key] once under structural `==`: a repeated key keeps its
     * first position and takes the last value, as `LinkedHashMap.put` does. */
    context(r: Raise<GuestError>)
    private fun putEntry(
        entries: LinkedHashMap<GValue, GValue>,
        key: GValue,
        value: GValue,
    ) {
        val existing = entries.keys.firstOrNull { valueEquals(it, key) }
        entries[existing ?: key] = value
    }

    context(r: Raise<GuestError>)
    private fun mapEntries(
        arguments: List<GValue>,
        at: Span,
    ): LinkedHashMap<GValue, GValue> {
        val out = LinkedHashMap<GValue, GValue>()
        for (argument in arguments) {
            val pair = argument as? GValue.VPair ?: r.raise(GuestError.UnassignedRead(at))
            putEntry(out, pair.first, pair.second)
        }
        return out
    }

    context(r: Raise<GuestError>)
    private fun absolute(
        arguments: List<GValue>,
        at: Span,
    ): GValue =
        when (val value = arguments[0]) {
            is GValue.VInt -> GValue.VInt(kotlin.math.abs(value.value))
            is GValue.VLong -> GValue.VLong(kotlin.math.abs(value.value))
            is GValue.VDouble -> GValue.VDouble(kotlin.math.abs(value.value))
            else -> r.raise(GuestError.UnassignedRead(at))
        }

    context(r: Raise<GuestError>)
    private fun extreme(
        name: String,
        arguments: List<GValue>,
        at: Span,
    ): GValue {
        val pickMin = name == "min"
        return when {
            arguments[0] is GValue.VInt && arguments[1] is GValue.VInt -> {
                val a = (arguments[0] as GValue.VInt).value
                val b = (arguments[1] as GValue.VInt).value
                GValue.VInt(if (pickMin) kotlin.math.min(a, b) else kotlin.math.max(a, b))
            }

            arguments[0] is GValue.VLong && arguments[1] is GValue.VLong -> {
                val a = (arguments[0] as GValue.VLong).value
                val b = (arguments[1] as GValue.VLong).value
                GValue.VLong(if (pickMin) kotlin.math.min(a, b) else kotlin.math.max(a, b))
            }

            arguments[0] is GValue.VDouble && arguments[1] is GValue.VDouble -> {
                val a = (arguments[0] as GValue.VDouble).value
                val b = (arguments[1] as GValue.VDouble).value
                GValue.VDouble(if (pickMin) kotlin.math.min(a, b) else kotlin.math.max(a, b))
            }

            else -> {
                r.raise(GuestError.UnassignedRead(at))
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun doubleCall(
        name: String,
        arguments: List<GValue>,
        at: Span,
    ): GValue {
        val x = (arguments[0] as? GValue.VDouble ?: r.raise(GuestError.UnassignedRead(at))).value
        val result =
            when (name) {
                "sqrt" -> kotlin.math.sqrt(x)
                "floor" -> kotlin.math.floor(x)
                else -> kotlin.math.ceil(x)
            }
        return GValue.VDouble(result)
    }

    context(r: Raise<GuestError>)
    private fun power(
        arguments: List<GValue>,
        at: Span,
    ): GValue {
        val base = (arguments[0] as? GValue.VDouble ?: r.raise(GuestError.UnassignedRead(at))).value
        val exponent = (arguments[1] as? GValue.VDouble ?: r.raise(GuestError.UnassignedRead(at))).value
        return GValue.VDouble(base.pow(exponent))
    }

    context(r: Raise<GuestError>)
    private fun exact(
        arguments: List<GValue>,
        at: Span,
        intOp: (Int, Int) -> Int,
        longOp: (Long, Long) -> Long,
    ): GValue =
        try {
            when {
                arguments[0] is GValue.VInt -> {
                    GValue.VInt(intOp((arguments[0] as GValue.VInt).value, (arguments[1] as GValue.VInt).value))
                }

                arguments[0] is GValue.VLong -> {
                    GValue.VLong(
                        longOp((arguments[0] as GValue.VLong).value, (arguments[1] as GValue.VLong).value),
                    )
                }

                else -> {
                    r.raise(GuestError.UnassignedRead(at))
                }
            }
        } catch (_: ArithmeticException) {
            r.raise(GuestError.Overflow(at))
        }

    context(r: Raise<GuestError>)
    private fun negateExact(
        arguments: List<GValue>,
        at: Span,
    ): GValue =
        try {
            when (val value = arguments[0]) {
                is GValue.VInt -> GValue.VInt(java.lang.Math.negateExact(value.value))
                is GValue.VLong -> GValue.VLong(java.lang.Math.negateExact(value.value))
                else -> r.raise(GuestError.UnassignedRead(at))
            }
        } catch (_: ArithmeticException) {
            r.raise(GuestError.Overflow(at))
        }

    // ---------- member access ----------

    context(r: Raise<GuestError>)
    public fun property(
        receiver: GValue,
        name: String,
        at: Span,
    ): GValue {
        val list = receiver.asList()
        if (list != null && name == "size") return GValue.VInt(list.items.size)
        if (receiver is GValue.VString && name == "length") return GValue.VInt(receiver.value.length)
        if (receiver is GValue.VPair) {
            return when (name) {
                "first" -> receiver.first
                "second" -> receiver.second
                else -> r.raise(GuestError.UnassignedRead(at))
            }
        }
        if (receiver is GValue.VMap) {
            return when (name) {
                "size" -> GValue.VInt(receiver.entries.size)
                "keys" -> GValue.VList(receiver.entries.keys.toMutableList(), mutable = false, asSet = true)
                "values" -> GValue.VList(receiver.entries.values.toMutableList(), mutable = false)
                else -> r.raise(GuestError.UnassignedRead(at))
            }
        }
        if (receiver is GValue.VObject) {
            return receiver.fields[name] ?: r.raise(GuestError.UnassignedRead(at))
        }
        return r.raise(GuestError.UnassignedRead(at))
    }

    context(r: Raise<GuestError>)
    public fun member(
        receiver: GValue,
        name: String,
        arguments: List<GValue>,
        at: Span,
    ): GValue {
        if (name in setOf("toInt", "toLong", "toDouble")) return convert(receiver, name, at)
        if (receiver is GValue.VString) return stringMember(receiver, name, arguments, at)
        if (receiver is GValue.VLazyList) return lazyListMember(receiver, name, arguments, at)
        val list = receiver.asList()
        if (list != null) return collectionMember(list, receiver, name, arguments, at)
        if (receiver is GValue.VMap) return mapMember(receiver, name, arguments, at)
        return r.raise(GuestError.UnassignedRead(at))
    }

    context(r: Raise<GuestError>)
    private fun convert(
        receiver: GValue,
        name: String,
        at: Span,
    ): GValue =
        when (name) {
            "toInt" -> {
                when (receiver) {
                    is GValue.VInt -> receiver
                    is GValue.VLong -> GValue.VInt(receiver.value.toInt())
                    is GValue.VDouble -> GValue.VInt(receiver.value.toInt())
                    else -> r.raise(GuestError.UnassignedRead(at))
                }
            }

            "toLong" -> {
                when (receiver) {
                    is GValue.VLong -> receiver
                    is GValue.VInt -> GValue.VLong(receiver.value.toLong())
                    is GValue.VDouble -> GValue.VLong(receiver.value.toLong())
                    else -> r.raise(GuestError.UnassignedRead(at))
                }
            }

            else -> {
                when (receiver) {
                    is GValue.VDouble -> receiver
                    is GValue.VInt -> GValue.VDouble(receiver.value.toDouble())
                    is GValue.VLong -> GValue.VDouble(receiver.value.toDouble())
                    else -> r.raise(GuestError.UnassignedRead(at))
                }
            }
        }

    context(r: Raise<GuestError>)
    private fun stringMember(
        receiver: GValue.VString,
        name: String,
        arguments: List<GValue>,
        at: Span,
    ): GValue =
        when (name) {
            "toLongOrNull" -> receiver.value.toLongOrNull()?.let { GValue.VLong(it) } ?: GValue.VNull
            "toDoubleOrNull" -> receiver.value.toDoubleOrNull()?.let { GValue.VDouble(it) } ?: GValue.VNull
            else -> r.raise(GuestError.UnassignedRead(at))
        }

    context(r: Raise<GuestError>)
    private fun collectionMember(
        list: GValue.VList,
        receiver: GValue,
        name: String,
        arguments: List<GValue>,
        at: Span,
    ): GValue =
        when (name) {
            "isEmpty" -> {
                GValue.VBool(list.items.isEmpty())
            }

            "contains" -> {
                GValue.VBool(list.items.any { valueEquals(it, arguments[0]) })
            }

            "firstOrNull" -> {
                list.items.firstOrNull() ?: GValue.VNull
            }

            "toList" -> {
                GValue.VList(list.items.toMutableList(), mutable = false)
            }

            "get" -> {
                elementAt(list, arguments[0], at)
            }

            "add" -> {
                if (!list.mutable) r.raise(GuestError.UnassignedRead(at))
                list.items.add(arguments[0])
                GValue.VBool(true)
            }

            "set" -> {
                if (!list.mutable) r.raise(GuestError.UnassignedRead(at))
                list.items[intIndex(arguments[0], at)] = arguments[1]
                GValue.VUnit
            }

            "plus" -> {
                plusMember(list, arguments[0])
            }

            "minus" -> {
                GValue.VList(list.items.filter { !valueEquals(it, arguments[0]) }.toMutableList(), mutable = false, asSet = true)
            }

            "take" -> {
                GValue.VList(list.items.take(intIndex(arguments[0], at)).toMutableList(), mutable = false, asSet = list.asSet)
            }

            "drop" -> {
                GValue.VList(list.items.drop(intIndex(arguments[0], at)).toMutableList(), mutable = false, asSet = list.asSet)
            }

            "sorted" -> {
                GValue.VList(list.items.sortedWith(::compareValues).toMutableList(), mutable = false, asSet = list.asSet)
            }

            "map" -> {
                GValue.VList(list.items.map { invoke(arguments[0], listOf(it), at) }.toMutableList(), mutable = false)
            }

            "filter" -> {
                GValue.VList(
                    list.items.filter { truth(invoke(arguments[0], listOf(it), at), at) }.toMutableList(),
                    mutable = false,
                    asSet = list.asSet,
                )
            }

            "fold" -> {
                foldMember(list, arguments, at)
            }

            "any" -> {
                GValue.VBool(list.items.any { truth(invoke(arguments[0], listOf(it), at), at) })
            }

            "all" -> {
                GValue.VBool(list.items.all { truth(invoke(arguments[0], listOf(it), at), at) })
            }

            else -> {
                r.raise(GuestError.UnassignedRead(at))
            }
        }

    /** Observe a lazy spine one node at a time; an infinite tail is never
     * traversed for head, tail, map, indexing, or a finite prefix. */
    context(r: Raise<GuestError>)
    private fun lazyListMember(
        receiver: GValue.VLazyList,
        name: String,
        arguments: List<GValue>,
        at: Span,
    ): GValue =
        when (name) {
            "isEmpty" -> GValue.VBool(false)
            "firstOrNull" -> receiver.head
            "get" -> lazyGet(receiver, intIndex(arguments[0], at), at)
            "drop" -> lazyDrop(receiver, intIndex(arguments[0], at))
            "take" -> lazyTake(receiver, intIndex(arguments[0], at))
            "map" -> lazyMap(receiver, arguments[0], at)
            else -> collectionMember(materializeNoRaise(receiver), receiver, name, arguments, at)
        }

    context(r: Raise<GuestError>)
    private fun lazyGet(
        source: GValue,
        position: Int,
        at: Span,
    ): GValue {
        if (position < 0) return r.raise(GuestError.IndexOutOfBounds(at))
        var cursor = source
        var remaining = position
        while (cursor is GValue.VLazyList) {
            if (remaining == 0) return cursor.head
            cursor = forceThunk(cursor.tail)
            remaining--
        }
        val list = cursor as? GValue.VList ?: return r.raise(GuestError.UnassignedRead(at))
        return list.items.getOrNull(remaining) ?: r.raise(GuestError.IndexOutOfBounds(at))
    }

    context(r: Raise<GuestError>)
    private fun lazyDrop(
        source: GValue.VLazyList,
        count: Int,
    ): GValue {
        var cursor: GValue = source
        var remaining = count.coerceAtLeast(0)
        while (remaining > 0 && cursor is GValue.VLazyList) {
            cursor = forceThunk(cursor.tail)
            remaining--
        }
        return if (cursor is GValue.VList) GValue.VList(cursor.items.drop(remaining).toMutableList(), false) else cursor
    }

    context(r: Raise<GuestError>)
    private fun lazyTake(
        source: GValue.VLazyList,
        count: Int,
    ): GValue {
        val items = mutableListOf<GValue>()
        var cursor: GValue = source
        while (items.size < count && cursor is GValue.VLazyList) {
            items.add(cursor.head)
            if (items.size < count) cursor = forceThunk(cursor.tail)
        }
        if (items.size < count && cursor is GValue.VList) items.addAll(cursor.items.take(count - items.size))
        return GValue.VList(items, false)
    }

    context(r: Raise<GuestError>)
    private fun lazyMap(
        source: GValue,
        fn: GValue,
        at: Span,
    ): GValue =
        when (source) {
            is GValue.VLazyList -> {
                GValue.VLazyList(
                    invoke(fn, listOf(source.head), at),
                    GValue.VThunk(ThunkState.Delayed { _ -> lazyMap(forceThunk(source.tail), fn, at) }, counters = source.tail.counters),
                )
            }

            is GValue.VList -> {
                GValue.VList(source.items.map { invoke(fn, listOf(it), at) }.toMutableList(), false)
            }

            else -> {
                r.raise(GuestError.UnassignedRead(at))
            }
        }

    context(r: Raise<GuestError>)
    private fun plusMember(
        list: GValue.VList,
        argument: GValue,
    ): GValue {
        if (list.asSet) {
            val items = list.items.toMutableList()
            if (items.none { valueEquals(it, argument) }) items.add(argument)
            return GValue.VList(items, mutable = false, asSet = true)
        }
        val extra = argument.asList() ?: return GValue.VList((list.items + argument).toMutableList(), mutable = false)
        return GValue.VList((list.items + extra.items).toMutableList(), mutable = false)
    }

    context(r: Raise<GuestError>)
    private fun foldMember(
        list: GValue.VList,
        arguments: List<GValue>,
        at: Span,
    ): GValue {
        var acc = arguments[0]
        for (item in list.items) acc = invoke(arguments[1], listOf(acc, item), at)
        return acc
    }

    context(r: Raise<GuestError>)
    private fun mapMember(
        receiver: GValue.VMap,
        name: String,
        arguments: List<GValue>,
        at: Span,
    ): GValue =
        when (name) {
            "isEmpty" -> {
                GValue.VBool(receiver.entries.isEmpty())
            }

            "containsKey" -> {
                GValue.VBool(mapLookup(receiver, arguments[0]) != null)
            }

            "get" -> {
                mapLookup(receiver, arguments[0]) ?: GValue.VNull
            }

            "remove" -> {
                val found = mapLookup(receiver, arguments[0]) ?: return GValue.VNull
                if (!receiver.mutable) r.raise(GuestError.UnassignedRead(at))
                receiver.entries.entries.removeIf { valueEquals(it.key, arguments[0]) }
                found
            }

            "put" -> {
                if (!receiver.mutable) r.raise(GuestError.UnassignedRead(at))
                val previous = mapLookup(receiver, arguments[0])
                receiver.entries.entries.removeIf { valueEquals(it.key, arguments[0]) }
                receiver.entries[arguments[0]] = arguments[1]
                previous ?: GValue.VNull
            }

            "plus" -> {
                val pair = arguments[0] as? GValue.VPair ?: r.raise(GuestError.UnassignedRead(at))
                val entries = LinkedHashMap(receiver.entries)
                putEntry(entries, pair.first, pair.second)
                GValue.VMap(entries, mutable = false)
            }

            else -> {
                r.raise(GuestError.UnassignedRead(at))
            }
        }

    context(r: Raise<GuestError>)
    private fun mapLookup(
        receiver: GValue.VMap,
        key: GValue,
    ): GValue? =
        receiver.entries.entries
            .firstOrNull { valueEquals(it.key, key) }
            ?.value

    // ---------- ranges ----------

    /** The values of `start..end` in order, produced one at a time: a `for`
     * that breaks early never pays for the rest of the range, and a bound of
     * `Long.MAX_VALUE` terminates like the native loop instead of wrapping. */
    context(r: Raise<GuestError>)
    public fun rangeValues(
        start: GValue,
        end: GValue,
        at: Span,
    ): Sequence<GValue> =
        when {
            start is GValue.VInt && end is GValue.VInt -> (start.value..end.value).asSequence().map { GValue.VInt(it) }
            start is GValue.VLong && end is GValue.VLong -> (start.value..end.value).asSequence().map { GValue.VLong(it) }
            else -> r.raise(GuestError.UnassignedRead(at))
        }

    // ---------- indexing ----------

    context(r: Raise<GuestError>)
    public fun readIndex(
        receiver: GValue,
        index: GValue,
        at: Span,
    ): GValue {
        if (receiver is GValue.VLazyList) return lazyGet(receiver, intIndex(index, at), at)
        val list = receiver.asList()
        if (list != null) return elementAt(list, index, at)
        return mapLookupIndex(receiver, index, at)
    }

    context(r: Raise<GuestError>)
    public fun writeIndex(
        receiver: GValue,
        index: GValue,
        value: GValue,
        at: Span,
    ) {
        if (receiver is GValue.VLazyList) r.raise(GuestError.UnassignedRead(at))
        val list = receiver.asList()
        if (list != null) {
            if (!list.mutable) r.raise(GuestError.UnassignedRead(at))
            list.items[intIndex(index, at)] = value
            return
        }
        val map = receiver as? GValue.VMap ?: r.raise(GuestError.UnassignedRead(at))
        if (!map.mutable) r.raise(GuestError.UnassignedRead(at))
        map.entries.entries.removeIf { valueEquals(it.key, index) }
        map.entries[index] = value
    }

    context(r: Raise<GuestError>)
    private fun mapLookupIndex(
        receiver: GValue,
        index: GValue,
        at: Span,
    ): GValue {
        val map = receiver as? GValue.VMap ?: r.raise(GuestError.UnassignedRead(at))
        return mapLookup(map, index) ?: GValue.VNull
    }

    context(r: Raise<GuestError>)
    private fun elementAt(
        list: GValue.VList,
        index: GValue,
        at: Span,
    ): GValue {
        val position = intIndex(index, at)
        if (position < 0 || position >= list.items.size) r.raise(GuestError.IndexOutOfBounds(at))
        return list.items[position]
    }

    context(r: Raise<GuestError>)
    private fun intIndex(
        index: GValue,
        at: Span,
    ): Int =
        when (index) {
            is GValue.VInt -> index.value
            is GValue.VLong -> index.value.toInt()
            else -> r.raise(GuestError.UnassignedRead(at))
        }

    // ---------- operators ----------

    context(r: Raise<GuestError>)
    public fun binary(
        operator: String,
        left: GValue,
        right: GValue,
        at: Span,
    ): GValue =
        when (operator) {
            "to" -> GValue.VPair(left, right)
            "<", "<=", ">", ">=" -> GValue.VBool(compareOrdered(operator, left, right, at))
            "==", "!=" -> GValue.VBool(valueEquals(left, right) == (operator == "=="))
            "===", "!==" -> GValue.VBool(valueIdentical(left, right) == (operator == "==="))
            "+" -> addition(left, right, at)
            "-", "*", "/" -> arithmetic(operator, left, right, at)
            "%" -> remainder(left, right, at)
            else -> r.raise(GuestError.UnassignedRead(at))
        }

    context(r: Raise<GuestError>)
    public fun unary(
        operator: String,
        operand: GValue,
        at: Span,
    ): GValue =
        when {
            operator == "!" -> GValue.VBool(!truth(operand, at))
            operand is GValue.VInt -> GValue.VInt(-operand.value)
            operand is GValue.VLong -> GValue.VLong(-operand.value)
            operand is GValue.VDouble -> GValue.VDouble(-operand.value)
            else -> r.raise(GuestError.UnassignedRead(at))
        }

    context(r: Raise<GuestError>)
    private fun addition(
        left: GValue,
        right: GValue,
        at: Span,
    ): GValue {
        if (left is GValue.VString && right is GValue.VString) return GValue.VString(left.value + right.value)
        if (left is GValue.VList) return plusMember(left, right)
        if (left is GValue.VMap && right is GValue.VPair) return mapMember(left, "plus", listOf(right), at)
        return arithmetic("+", left, right, at)
    }

    context(r: Raise<GuestError>)
    private fun arithmetic(
        operator: String,
        left: GValue,
        right: GValue,
        at: Span,
    ): GValue =
        when {
            left is GValue.VInt && right is GValue.VInt -> {
                GValue.VInt(applyInt(operator, left.value, right.value, at))
            }

            left is GValue.VLong && right is GValue.VLong -> {
                GValue.VLong(applyLong(operator, left.value, right.value, at))
            }

            left is GValue.VDouble && right is GValue.VDouble -> {
                GValue.VDouble(applyDouble(operator, left.value, right.value, at))
            }

            else -> {
                r.raise(GuestError.UnassignedRead(at))
            }
        }

    context(r: Raise<GuestError>)
    private fun applyInt(
        operator: String,
        a: Int,
        b: Int,
        at: Span,
    ): Int =
        when (operator) {
            "+" -> a + b
            "-" -> a - b
            "*" -> a * b
            else -> divideInt(a, b, at)
        }

    context(r: Raise<GuestError>)
    private fun applyLong(
        operator: String,
        a: Long,
        b: Long,
        at: Span,
    ): Long =
        when (operator) {
            "+" -> a + b
            "-" -> a - b
            "*" -> a * b
            else -> divideLong(a, b, at)
        }

    private fun applyDouble(
        operator: String,
        a: Double,
        b: Double,
        at: Span,
    ): Double =
        when (operator) {
            "+" -> a + b
            "-" -> a - b
            "*" -> a * b
            else -> a / b
        }

    context(r: Raise<GuestError>)
    private fun divideInt(
        a: Int,
        b: Int,
        at: Span,
    ): Int {
        if (b == 0) r.raise(GuestError.DivisionByZero(at))
        return a / b
    }

    context(r: Raise<GuestError>)
    private fun divideLong(
        a: Long,
        b: Long,
        at: Span,
    ): Long {
        if (b == 0L) r.raise(GuestError.DivisionByZero(at))
        return a / b
    }

    context(r: Raise<GuestError>)
    private fun remainder(
        left: GValue,
        right: GValue,
        at: Span,
    ): GValue =
        when {
            left is GValue.VInt && right is GValue.VInt -> {
                if (right.value == 0) r.raise(GuestError.DivisionByZero(at))
                GValue.VInt(left.value % right.value)
            }

            left is GValue.VLong && right is GValue.VLong -> {
                if (right.value == 0L) r.raise(GuestError.DivisionByZero(at))
                GValue.VLong(left.value % right.value)
            }

            else -> {
                r.raise(GuestError.UnassignedRead(at))
            }
        }

    context(r: Raise<GuestError>)
    private fun compareOrdered(
        operator: String,
        left: GValue,
        right: GValue,
        at: Span,
    ): Boolean {
        val ordering = compareValues(left, right)
        return when (operator) {
            "<" -> ordering < 0
            "<=" -> ordering <= 0
            ">" -> ordering > 0
            else -> ordering >= 0
        }
    }

    /** Native ordering for Int, Long, Double, and String elements (2.4). */
    private fun compareValues(
        left: GValue,
        right: GValue,
    ): Int =
        when {
            left is GValue.VInt && right is GValue.VInt -> left.value.compareTo(right.value)
            left is GValue.VLong && right is GValue.VLong -> left.value.compareTo(right.value)
            left is GValue.VDouble && right is GValue.VDouble -> left.value.compareTo(right.value)
            left is GValue.VString && right is GValue.VString -> left.value.compareTo(right.value)
            else -> 0
        }

    context(r: Raise<GuestError>)
    public fun truth(
        value: GValue,
        at: Span,
    ): Boolean {
        if (value is GValue.VBool) return value.value
        return r.raise(GuestError.UnassignedRead(at))
    }

    /** Runtime `is`/`!is` tests: scalars by shape, instances by class name. */
    context(r: Raise<GuestError>)
    public fun isTypeValue(
        value: GValue,
        type: GuestType,
    ): Boolean {
        if (value is GValue.VNull && type is GuestType.Nullable) return true
        val core = coreType(type)
        val name = (core as? GuestType.Named)?.name ?: return value is GValue.VNull
        return when (name) {
            "Int" -> value is GValue.VInt
            "Long" -> value is GValue.VLong
            "Double" -> value is GValue.VDouble
            "Boolean" -> value is GValue.VBool
            "String" -> value is GValue.VString
            "Unit" -> value is GValue.VUnit
            "Pair" -> value is GValue.VPair
            "List", "Collection" -> value is GValue.VLazyList || value is GValue.VList
            "MutableList" -> value is GValue.VList && value.mutable
            "Set" -> value is GValue.VList && value.asSet
            "Map", "MutableMap" -> value is GValue.VMap
            "Thunk" -> value is GValue.VThunk
            "Random" -> value is GValue.VRandom
            "Nothing" -> false
            else -> value is GValue.VObject && value.className == name
        }
    }

    context(r: Raise<GuestError>)
    private fun GValue.asList(): GValue.VList? =
        when (this) {
            is GValue.VList -> this
            is GValue.VLazyList -> materializeNoRaise(this)
            else -> null
        }

    /** Lazy lists materialize through the instrumented force path so every
     * tail thunk counts toward the forcing instrument. */
    context(r: Raise<GuestError>)
    private fun materializeNoRaise(value: GValue.VLazyList): GValue.VList {
        val items = mutableListOf<GValue>(value.head)
        var cursor = value.tail
        while (true) {
            when (val next = forceThunk(cursor)) {
                is GValue.VLazyList -> {
                    items.add(next.head)
                    cursor = next.tail
                }

                is GValue.VList -> {
                    items.addAll(next.items)
                    return GValue.VList(items, mutable = false)
                }

                else -> {
                    return GValue.VList(items, mutable = false)
                }
            }
        }
    }
}
