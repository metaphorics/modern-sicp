// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

/** Type constructors and type algebra for the checker. */
internal val T_INT: GuestType.Named = GuestType.Named("Int")
internal val T_LONG: GuestType.Named = GuestType.Named("Long")
internal val T_DOUBLE: GuestType.Named = GuestType.Named("Double")
internal val T_BOOL: GuestType.Named = GuestType.Named("Boolean")
internal val T_STRING: GuestType.Named = GuestType.Named("String")
internal val T_UNIT: GuestType.Named = GuestType.Named("Unit")

internal fun tList(element: GuestType): GuestType.Named = GuestType.Named("List", listOf(element))

internal fun tMutableList(element: GuestType): GuestType.Named = GuestType.Named("MutableList", listOf(element))

internal fun tSet(element: GuestType): GuestType.Named = GuestType.Named("Set", listOf(element))

internal fun tCollection(element: GuestType): GuestType.Named = GuestType.Named("Collection", listOf(element))

internal fun tMap(
    key: GuestType,
    value: GuestType,
): GuestType.Named = GuestType.Named("Map", listOf(key, value))

internal fun tMutableMap(
    key: GuestType,
    value: GuestType,
): GuestType.Named = GuestType.Named("MutableMap", listOf(key, value))

internal fun tPair(
    first: GuestType,
    second: GuestType,
): GuestType.Named = GuestType.Named("Pair", listOf(first, second))

internal fun tThunk(element: GuestType): GuestType.Named = GuestType.Named("Thunk", listOf(element))

/** The unifier's free element type: `lazyEnd` and empty builders fill it
 * from the expected type, and it unifies with anything. */
internal val T_FREE: GuestType.Named = GuestType.Named("Free")

/** True for the modes that admit the query DSL of grammar section 4.3. */
public val Mode.admitsQuery: Boolean
    get() = this == Mode.QUERY || this == Mode.QUERY_LAZY || this == Mode.QUERY_SEARCH

/** The query DSL vocabulary of grammar section 4.3 as host-supplied
 * declarations: two sealed families with data variants plus the rule, fact,
 * and frame records. The checker admits them exactly like user-declared
 * data, and every engine installs them as class shapes, so constructor
 * calls, members, and `is` tests behave as for program declarations. */
public val QUERY_VOCABULARY: List<Declaration> =
    run {
        fun property(
            name: String,
            type: GuestType,
        ): Property = Property(name, type, mutable = false, NO_POSITION)

        fun data(
            name: String,
            parent: String?,
            vararg properties: Property,
        ): DataClass = DataClass(name, properties.toList(), parent, NO_POSITION)
        val qTerm = GuestType.Named("QTerm")
        val qQuery = GuestType.Named("QQuery")
        listOf(
            SealedInterface("QTerm", NO_POSITION),
            SealedInterface("QQuery", NO_POSITION),
            data("QSym", "QTerm", property("name", T_STRING)),
            data("QVar", "QTerm", property("name", T_STRING)),
            data("QList", "QTerm", property("items", tList(qTerm)), property("tail", nullable(qTerm))),
            data("QPattern", "QQuery", property("term", qTerm)),
            data("QAnd", "QQuery", property("parts", tList(qQuery))),
            data("QOr", "QQuery", property("parts", tList(qQuery))),
            data("QNot", "QQuery", property("part", qQuery)),
            data(
                "QGuard",
                "QQuery",
                property("predicate", GuestType.Function(listOf(tList(qTerm)), T_BOOL)),
                property("args", tList(qTerm)),
            ),
            data("QUnique", "QQuery", property("part", qQuery)),
            data("QRule", null, property("conclusion", qTerm), property("body", qQuery)),
            data("QFact", null, property("term", qTerm)),
            data("QFrame", null, property("bindings", tMap(GuestType.Named("QVar"), qTerm))),
        )
    }

/** Variant-to-family registrations for sealed hierarchies: user families are
 * registered by the checker, the query DSL family is builtin. Registrations
 * are per-thread and cleared at the start of each admission, so one check
 * cannot leak families into another. Type conformance admits a variant
 * wherever its family is expected. */
internal object TypeFamilies {
    private val registrations = ThreadLocal.withInitial { mutableMapOf<String, String>() }

    val variantToFamily: MutableMap<String, String>
        get() = registrations.get()

    fun clear() {
        registrations.get().clear()
    }

    fun familyOf(name: String): String? {
        var here = variantToFamily[name]
        while (here != null) {
            val up = variantToFamily[here]
            if (up == null) return here
            here = up
        }
        return null
    }
}

internal fun coreType(type: GuestType): GuestType = (type as? GuestType.Nullable)?.base ?: type

internal fun nullable(type: GuestType): GuestType = if (type is GuestType.Nullable) type else GuestType.Nullable(type)

internal fun isNumeric(type: GuestType): Boolean {
    val core = coreType(type)
    return core is GuestType.Named && core.name in setOf("Int", "Long", "Double")
}

internal fun isPrimitiveScalar(type: GuestType): Boolean {
    val core = coreType(type)
    return core is GuestType.Named && core.name in setOf("Int", "Long", "Double", "Boolean", "String")
}

internal fun sameType(
    a: GuestType,
    b: GuestType,
): Boolean {
    if (a == T_FREE || b == T_FREE) return true
    return when {
        a is GuestType.Nullable && b is GuestType.Nullable -> {
            sameType(a.base, b.base)
        }

        a is GuestType.Nullable || b is GuestType.Nullable -> {
            false
        }

        a is GuestType.Function && b is GuestType.Function -> {
            a.parameters.size == b.parameters.size &&
                a.parameters.zip(b.parameters).all { (x, y) -> sameType(x, y) } &&
                sameType(a.result, b.result)
        }

        a is GuestType.Named && b is GuestType.Named -> {
            a.name == b.name &&
                a.arguments.size == b.arguments.size &&
                a.arguments.zip(b.arguments).all { (x, y) -> sameType(x, y) }
        }

        a is GuestType.Nothing && b is GuestType.Nothing -> {
            true
        }

        a is GuestType.Null && b is GuestType.Null -> {
            true
        }

        else -> {
            a == b
        }
    }
}

/** Assignment and argument conformance follows native direction: a variant
 * may flow to its sealed family, and a non-null value may flow to a nullable
 * slot, but neither conversion is reversible. */
internal fun conforms(
    actual: GuestType,
    expected: GuestType,
): Boolean {
    if (actual is GuestType.Nothing) return true
    if (expected is GuestType.Nullable) {
        if (actual is GuestType.Null) return true
        return conforms(if (actual is GuestType.Nullable) actual.base else actual, expected.base)
    }
    if (actual is GuestType.Nullable || actual is GuestType.Null) return false
    if (sameType(actual, expected)) return true
    val variant = actual as? GuestType.Named ?: return false
    val family = expected as? GuestType.Named ?: return false
    if (variant.name == family.name && variant.arguments.size == family.arguments.size) {
        return when (variant.name) {
            "Pair", "List", "Set", "Collection" -> {
                variant.arguments.zip(family.arguments).all { (element, target) -> conforms(element, target) }
            }

            "Map" -> {
                variant.arguments.size == 2 && sameType(variant.arguments[0], family.arguments[0]) &&
                    conforms(variant.arguments[1], family.arguments[1])
            }

            else -> {
                false
            }
        }
    }
    if (variant.arguments.size == 1 && family.arguments.size == 1 &&
        (
            (variant.name == "MutableList" && family.name == "List") ||
                (
                    family.name == "Collection" &&
                        (variant.name == "List" || variant.name == "MutableList" || variant.name == "Set")
                )
        )
    ) {
        return conforms(variant.arguments[0], family.arguments[0])
    }
    if (variant.name == "MutableMap" && family.name == "Map" &&
        variant.arguments.size == 2 && family.arguments.size == 2
    ) {
        return sameType(variant.arguments[0], family.arguments[0]) &&
            conforms(variant.arguments[1], family.arguments[1])
    }
    return variant.arguments.isEmpty() && family.arguments.isEmpty() &&
        TypeFamilies.familyOf(variant.name) == family.name
}

/** The verdict of consulting the admitted library surface of section 2.4. */
internal sealed interface SurfaceResult {
    data class Ok(
        val type: GuestType,
    ) : SurfaceResult

    /** The call is outside every declaration: host-invalid. */
    data class Bad(
        val message: String,
    ) : SurfaceResult

    /** Valid Kotlin, outside the subset surface: host-valid but unsupported. */
    data class Outside(
        val category: String,
        val message: String,
    ) : SurfaceResult

    /** Not a surface entry at all; the checker consults user declarations. */
    data object Absent : SurfaceResult
}

/** `print`/`println` and the experimental and query constructors: the
 * top-level call surface by mode. */
internal fun topLevelCall(
    name: String,
    arguments: List<GuestType>,
    mode: Mode,
): SurfaceResult =
    when (name) {
        "print", "println" -> {
            outputCall(name, arguments)
        }

        "listOf" -> {
            homogeneousBuilder(arguments, ::tList)
        }

        "mutableListOf" -> {
            homogeneousBuilder(arguments, ::tMutableList)
        }

        "setOf" -> {
            homogeneousBuilder(arguments, ::tSet)
        }

        "emptyList" -> {
            SurfaceResult.Ok(tList(T_FREE))
        }

        "emptySet" -> {
            SurfaceResult.Ok(tSet(T_FREE))
        }

        "mutableMapOf" -> {
            mapBuilder(arguments, ::tMutableMap)
        }

        "mapOf" -> {
            mapBuilder(arguments, ::tMap)
        }

        "emptyMap" -> {
            SurfaceResult.Ok(tMap(T_FREE, T_FREE))
        }

        "abs" -> {
            unaryNumeric(arguments)
        }

        "min", "max" -> {
            binaryNumeric(arguments)
        }

        "sqrt", "floor", "ceil" -> {
            doubleCall(arguments)
        }

        "pow" -> {
            doubleDoubleCall(arguments)
        }

        "addExact", "subtractExact", "multiplyExact" -> {
            exactBinaryCall(arguments)
        }

        "negateExact" -> {
            exactUnaryCall(arguments)
        }

        "thunk", "force", "lazyPair", "lazyEnd" -> {
            lazyCall(name, arguments, mode)
        }

        "choose", "chooseRandom", "demand", "setPermanent", "ifFail", "seededRandom" -> {
            searchCall(name, arguments, mode)
        }

        else -> {
            SurfaceResult.Absent
        }
    }

/** Mode-only property values (`lazyEnd`). */
internal fun topLevelValue(
    name: String,
    mode: Mode,
): SurfaceResult =
    if (name == "lazyEnd" && mode == Mode.LAZY) {
        SurfaceResult.Ok(tList(T_FREE))
    } else if (name == "lazyEnd") {
        SurfaceResult.Absent
    } else {
        SurfaceResult.Absent
    }

private fun outputCall(
    name: String,
    arguments: List<GuestType>,
): SurfaceResult {
    if (name == "println" && arguments.isEmpty()) return SurfaceResult.Ok(T_UNIT)
    if (arguments.size != 1) return SurfaceResult.Bad("$name takes one argument")
    // Section 3.7: printable values are exactly String, Long, Double, Boolean.
    val core = coreType(arguments[0])
    val printable = core is GuestType.Named && core.name in setOf("String", "Long", "Double", "Boolean")
    if (!printable) return SurfaceResult.Outside("StructuredOutput", "printing a value section 3.7 does not admit")
    return SurfaceResult.Ok(T_UNIT)
}

private fun homogeneousBuilder(
    arguments: List<GuestType>,
    wrap: (GuestType) -> GuestType,
): SurfaceResult {
    val first = arguments.firstOrNull() ?: return SurfaceResult.Ok(wrap(T_FREE))
    var element = first
    for (argument in arguments) {
        if (sameType(argument, element)) {
            if (element == T_FREE) element = argument
            continue
        }
        val common = commonFamily(argument, element)
        if (common != null) {
            element = common
            continue
        }
        return if (arguments.all { isNumeric(it) }) {
            SurfaceResult.Outside("MixedWidthArithmetic", "builder elements must share one width")
        } else {
            SurfaceResult.Outside("MiscKotlin", "builder elements must share one type")
        }
    }
    return SurfaceResult.Ok(wrap(element))
}

/** The family type two variant values share, if they are one family. */
private fun commonFamily(
    a: GuestType,
    b: GuestType,
): GuestType? {
    val first = coreType(a) as? GuestType.Named ?: return null
    val second = coreType(b) as? GuestType.Named ?: return null
    val firstFamily = TypeFamilies.familyOf(first.name)
    val secondFamily = TypeFamilies.familyOf(second.name)
    if (firstFamily != null && firstFamily == second.name) return b
    if (secondFamily != null && secondFamily == first.name) return a
    if (firstFamily != null && firstFamily == secondFamily) return GuestType.Named(firstFamily)
    return null
}

/** The least upper bound of two branch types: the join of their
 * non-null cores (one core conforming to the other, or two variants of
 * one sealed family), made nullable when either side admits `null`. */
internal fun joinedBranchType(
    first: GuestType,
    second: GuestType,
): GuestType? {
    if (first is GuestType.Nothing) return second
    if (second is GuestType.Nothing) return first
    if (first is GuestType.Null && second !is GuestType.Null) return nullable(second)
    if (second is GuestType.Null && first !is GuestType.Null) return nullable(first)
    val firstCore = coreType(first)
    val secondCore = coreType(second)
    val family = (firstCore as? GuestType.Named)?.name?.let(TypeFamilies::familyOf)
    val joined =
        when {
            conforms(firstCore, secondCore) -> secondCore
            conforms(secondCore, firstCore) -> firstCore
            family != null && family == (secondCore as? GuestType.Named)?.name?.let(TypeFamilies::familyOf) -> GuestType.Named(family)
            else -> return null
        }
    return if (first is GuestType.Nullable || second is GuestType.Nullable) nullable(joined) else joined
}

private fun mapBuilder(
    arguments: List<GuestType>,
    wrap: (GuestType, GuestType) -> GuestType,
): SurfaceResult {
    var key: GuestType = T_FREE
    var value: GuestType = T_FREE
    for (argument in arguments) {
        val pair = coreType(argument)
        if (pair !is GuestType.Named || pair.name != "Pair" || pair.arguments.size != 2) {
            return SurfaceResult.Bad("map entries must be pairs")
        }
        key = sharedType(key, pair.arguments[0]) ?: return SurfaceResult.Outside("MiscKotlin", "map keys must share one type")
        value = sharedType(value, pair.arguments[1]) ?: return SurfaceResult.Outside("MiscKotlin", "map values must share one type")
    }
    return SurfaceResult.Ok(wrap(key, value))
}

private fun sharedType(
    known: GuestType,
    candidate: GuestType,
): GuestType? {
    if (known == T_FREE || sameType(known, candidate)) return if (known == T_FREE) candidate else known
    return commonFamily(known, candidate)
}

private fun unaryNumeric(arguments: List<GuestType>): SurfaceResult {
    if (arguments.size != 1 || !isNumeric(arguments[0])) return SurfaceResult.Bad("abs takes one number")
    return SurfaceResult.Ok(arguments[0])
}

private fun binaryNumeric(arguments: List<GuestType>): SurfaceResult {
    if (arguments.size != 2 || !isNumeric(arguments[0]) || !sameType(arguments[0], arguments[1])) {
        return SurfaceResult.Bad("min and max take two numbers of one width")
    }
    return SurfaceResult.Ok(arguments[0])
}

private fun doubleCall(arguments: List<GuestType>): SurfaceResult {
    if (arguments.size != 1 || !sameType(arguments[0], T_DOUBLE)) return SurfaceResult.Bad("expected one Double")
    return SurfaceResult.Ok(T_DOUBLE)
}

private fun doubleDoubleCall(arguments: List<GuestType>): SurfaceResult {
    if (arguments.size != 2 || !sameType(arguments[0], T_DOUBLE) || !sameType(arguments[1], T_DOUBLE)) {
        return SurfaceResult.Bad("pow takes two Doubles")
    }
    return SurfaceResult.Ok(T_DOUBLE)
}

private fun exactBinaryCall(arguments: List<GuestType>): SurfaceResult {
    if (arguments.size != 2 || !sameType(arguments[0], arguments[1])) return SurfaceResult.Bad("operand widths must match")
    val operand = coreType(arguments[0])
    if (operand !is GuestType.Named || operand.name !in setOf("Int", "Long")) {
        return SurfaceResult.Bad("checked arithmetic takes Int or Long")
    }
    return SurfaceResult.Ok(operand)
}

private fun exactUnaryCall(arguments: List<GuestType>): SurfaceResult {
    if (arguments.size != 1) return SurfaceResult.Bad("negateExact takes one operand")
    val operand = coreType(arguments[0])
    if (operand !is GuestType.Named || operand.name !in setOf("Int", "Long")) {
        return SurfaceResult.Bad("checked arithmetic takes Int or Long")
    }
    return SurfaceResult.Ok(operand)
}

private fun lazyCall(
    name: String,
    arguments: List<GuestType>,
    mode: Mode,
): SurfaceResult {
    if (mode != Mode.LAZY && mode != Mode.QUERY_LAZY) return SurfaceResult.Absent
    return when (name) {
        "thunk" -> {
            if (arguments.size != 1) return SurfaceResult.Bad("thunk takes one body")
            val body = coreType(arguments[0])
            if (body !is GuestType.Function || body.parameters.isNotEmpty()) {
                return SurfaceResult.Bad("thunk takes a zero-argument body")
            }
            SurfaceResult.Ok(tThunk(body.result))
        }

        "force" -> {
            if (arguments.size != 1) return SurfaceResult.Bad("force takes one thunk")
            val thunk = coreType(arguments[0])
            if (thunk !is GuestType.Named || thunk.name != "Thunk" || thunk.arguments.size != 1) {
                return SurfaceResult.Bad("force takes a Thunk")
            }
            SurfaceResult.Ok(thunk.arguments[0])
        }

        "lazyPair" -> {
            if (arguments.size != 2) return SurfaceResult.Bad("lazyPair takes a head and a tail")
            val tail = coreType(arguments[1])
            if (tail !is GuestType.Named || tail.name != "Thunk") return SurfaceResult.Bad("lazyPair tails are thunks")
            SurfaceResult.Ok(tList(arguments[0]))
        }

        else -> {
            SurfaceResult.Ok(tList(T_FREE))
        }
    }
}

private fun searchCall(
    name: String,
    arguments: List<GuestType>,
    mode: Mode,
): SurfaceResult {
    if (mode != Mode.SEARCH && mode != Mode.QUERY_SEARCH) return SurfaceResult.Absent
    return when (name) {
        "choose", "chooseRandom" -> {
            var element = arguments.firstOrNull() ?: return SurfaceResult.Ok(T_FREE)
            for (argument in arguments) {
                if (sameType(argument, element)) {
                    if (element == T_FREE) element = argument
                    continue
                }
                val common = commonFamily(argument, element)
                if (common != null) {
                    element = common
                    continue
                }
                return if (arguments.all { isNumeric(it) }) {
                    SurfaceResult.Outside("MixedWidthArithmetic", "alternatives share one width")
                } else {
                    SurfaceResult.Outside("MiscKotlin", "alternatives share one type")
                }
            }
            SurfaceResult.Ok(element)
        }

        "demand" -> {
            if (arguments.size != 1 || !sameType(arguments[0], T_BOOL)) return SurfaceResult.Bad("demand takes one Boolean")
            SurfaceResult.Ok(T_UNIT)
        }

        "setPermanent" -> {
            zeroArgBody(arguments, T_UNIT)
        }

        "ifFail" -> {
            if (arguments.size != 2) return SurfaceResult.Bad("ifFail takes two bodies")
            val primary = zeroArgBody(listOf(arguments[0]), T_FREE)
            if (primary !is SurfaceResult.Ok) return primary
            val fallback = zeroArgBody(listOf(arguments[1]), T_FREE)
            if (fallback !is SurfaceResult.Ok) return fallback
            val result =
                joinedBranchType(primary.type, fallback.type)
                    ?: return SurfaceResult.Outside("MiscKotlin", "ifFail bodies share one result type")
            SurfaceResult.Ok(result)
        }

        "seededRandom" -> {
            if (arguments.size != 1 || !sameType(arguments[0], T_LONG)) return SurfaceResult.Bad("seededRandom takes one Long")
            SurfaceResult.Ok(GuestType.Named("Random"))
        }

        else -> {
            SurfaceResult.Absent
        }
    }
}

private fun zeroArgBody(
    arguments: List<GuestType>,
    result: GuestType,
): SurfaceResult {
    if (arguments.size != 1) return SurfaceResult.Bad("expected one body")
    val body = coreType(arguments[0])
    if (body !is GuestType.Function || body.parameters.isNotEmpty()) {
        return SurfaceResult.Bad("expected a zero-argument body")
    }
    return SurfaceResult.Ok(if (result == T_FREE) body.result else result)
}

/** Top-level entries that own call syntax, for reference and dispatch. */
internal val surfaceFunctionNames: Set<String> =
    setOf(
        "print",
        "println",
        "listOf",
        "emptyList",
        "setOf",
        "emptySet",
        "mutableListOf",
        "mapOf",
        "emptyMap",
        "mutableMapOf",
        "abs",
        "min",
        "max",
        "sqrt",
        "floor",
        "ceil",
        "pow",
        "addExact",
        "subtractExact",
        "multiplyExact",
        "negateExact",
        "thunk",
        "force",
        "lazyPair",
        "lazyEnd",
        "choose",
        "chooseRandom",
        "demand",
        "setPermanent",
        "ifFail",
        "seededRandom",
    )

/** The contextual expected type of top-level argument [index]; `T_FREE`
 * types the argument from itself. */
internal fun topLevelParameterTypes(
    name: String,
    argCount: Int,
    prior: List<GuestType>,
    index: Int,
): GuestType {
    val fixed =
        when (name) {
            "thunk" -> listOf(GuestType.Function(emptyList(), T_FREE))
            "setPermanent" -> listOf(GuestType.Function(emptyList(), T_UNIT))
            "force" -> listOf(tThunk(T_FREE))
            "lazyPair" -> listOf(T_FREE, tThunk(tList(T_FREE)))
            "ifFail" -> listOf(GuestType.Function(emptyList(), T_FREE), GuestType.Function(emptyList(), T_FREE))
            "demand" -> listOf(T_BOOL)
            "seededRandom" -> listOf(T_LONG)
            "sqrt", "floor", "ceil" -> listOf(T_DOUBLE)
            "pow" -> listOf(T_DOUBLE, T_DOUBLE)
            else -> return T_FREE
        }
    return fixed.getOrElse(index) { T_FREE }
}

/** The contextual expected type of member-call argument [index]. */
internal fun memberParameterTypes(
    receiver: GuestType.Named,
    name: String,
    argCount: Int,
    prior: List<GuestType>,
    index: Int,
): GuestType {
    val element = receiver.arguments.getOrElse(0) { T_FREE }
    if (name == "fold") {
        val initial = prior.getOrElse(0) { T_FREE }
        return when (index) {
            1 -> GuestType.Function(listOf(initial, element), initial)
            else -> T_FREE
        }
    }
    val fixed =
        when (name) {
            "map" -> {
                listOf(GuestType.Function(listOf(element), T_FREE))
            }

            "filter", "any", "all" -> {
                listOf(GuestType.Function(listOf(element), T_BOOL))
            }

            "get" -> {
                listOf(
                    if (receiver.name == "Map" ||
                        receiver.name == "MutableMap"
                    ) {
                        receiver.arguments.getOrElse(0) { T_FREE }
                    } else {
                        T_INT
                    },
                )
            }

            "set" -> {
                listOf(T_INT, element)
            }

            "add", "contains", "minus" -> {
                listOf(element)
            }

            "take", "drop" -> {
                listOf(T_INT)
            }

            "plus" -> {
                if (receiver.name == "List") listOf(tList(element)) else listOf(element)
            }

            "put" -> {
                listOf(receiver.arguments.getOrElse(0) { T_FREE }, receiver.arguments.getOrElse(1) { T_FREE })
            }

            "remove", "containsKey" -> {
                listOf(receiver.arguments.getOrElse(0) { T_FREE })
            }

            else -> {
                return T_FREE
            }
        }
    return fixed.getOrElse(index) { T_FREE }
}

/** Member selection on a library type; user-declared members are the
 * checker's own lookup, this covers the surface of section 2.4. */
internal fun memberSelect(
    receiver: GuestType,
    name: String,
): SurfaceResult {
    val core = coreType(receiver)
    if (core !is GuestType.Named) return SurfaceResult.Absent
    return when (core.name) {
        "String" -> {
            when (name) {
                "length" -> SurfaceResult.Ok(T_INT)
                "get" -> SurfaceResult.Outside("CharSurface", "String.get")
                "toString" -> SurfaceResult.Outside("MiscKotlin", "render strings in templates")
                else -> SurfaceResult.Outside("MiscKotlin", "String.$name")
            }
        }

        "List", "Collection", "Set", "MutableList" -> {
            if (name == "size") SurfaceResult.Ok(T_INT) else SurfaceResult.Absent
        }

        "Map", "MutableMap" -> {
            when (name) {
                "size" -> SurfaceResult.Ok(T_INT)
                "keys" -> SurfaceResult.Ok(tSet(core.arguments[0]))
                "values" -> SurfaceResult.Ok(tCollection(core.arguments[1]))
                else -> SurfaceResult.Absent
            }
        }

        "Pair" -> {
            when (name) {
                "first" -> SurfaceResult.Ok(core.arguments[0])
                "second" -> SurfaceResult.Ok(core.arguments[1])
                else -> SurfaceResult.Absent
            }
        }

        "Int", "Long", "Double", "Boolean" -> {
            if (name == "toString") {
                SurfaceResult.Outside("MiscKotlin", "render values in templates")
            } else {
                SurfaceResult.Outside("MiscKotlin", "${core.name}.$name")
            }
        }

        else -> {
            SurfaceResult.Absent
        }
    }
}

/** Member calls on a library type: conversions and the collection surface. */
internal fun memberCall(
    receiver: GuestType,
    name: String,
    arguments: List<GuestType>,
): SurfaceResult {
    val core = coreType(receiver)
    if (core !is GuestType.Named) return SurfaceResult.Absent
    if (name in setOf("toInt", "toLong", "toDouble")) return conversionCall(core, name, arguments)
    return when (core.name) {
        "String" -> stringCall(name, arguments)
        "List", "Collection", "Set", "MutableList" -> collectionCall(core, name, arguments)
        "Map", "MutableMap" -> mapCall(core, name, arguments)
        else -> SurfaceResult.Absent
    }
}

private fun conversionCall(
    receiver: GuestType.Named,
    name: String,
    arguments: List<GuestType>,
): SurfaceResult {
    if (arguments.isNotEmpty()) return SurfaceResult.Bad("conversions take no arguments")
    if (receiver.name !in setOf("Int", "Long", "Double")) return SurfaceResult.Absent
    val result =
        when (name) {
            "toInt" -> T_INT
            "toLong" -> T_LONG
            else -> T_DOUBLE
        }
    return SurfaceResult.Ok(result)
}

private fun stringCall(
    name: String,
    arguments: List<GuestType>,
): SurfaceResult {
    if (arguments.isNotEmpty()) return SurfaceResult.Bad("string parsing takes no arguments")
    return when (name) {
        "toLongOrNull" -> SurfaceResult.Ok(nullable(T_LONG))
        "toDoubleOrNull" -> SurfaceResult.Ok(nullable(T_DOUBLE))
        "get" -> SurfaceResult.Outside("CharSurface", "String.get")
        else -> SurfaceResult.Outside("MiscKotlin", "String.$name")
    }
}

private fun collectionCall(
    receiver: GuestType.Named,
    name: String,
    arguments: List<GuestType>,
): SurfaceResult {
    val element = receiver.arguments.getOrElse(0) { T_FREE }
    return when (name) {
        "isEmpty" -> {
            zeroArgs(arguments)?.let { return it }.let { SurfaceResult.Ok(T_BOOL) }
        }

        "contains" -> {
            oneArg(arguments, element, T_BOOL)
        }

        "firstOrNull" -> {
            zeroArgs(arguments)?.let { return it }.let { SurfaceResult.Ok(nullable(element)) }
        }

        "toList" -> {
            zeroArgs(arguments)?.let { return it }.let { SurfaceResult.Ok(tList(element)) }
        }

        "get" -> {
            oneArg(arguments, T_INT, element)
        }

        "add" -> {
            if (receiver.name != "MutableList") return SurfaceResult.Bad("`add` is a MutableList member")
            oneArg(arguments, element, T_BOOL)
        }

        "set" -> {
            if (receiver.name != "MutableList") return SurfaceResult.Bad("`set` is a MutableList member")
            if (arguments.size != 2 || !sameType(arguments[0], T_INT) || !sameType(arguments[1], element)) {
                return SurfaceResult.Bad("set takes an index and an element")
            }
            SurfaceResult.Ok(element)
        }

        "plus" -> {
            plusCall(receiver, arguments, element)
        }

        "minus" -> {
            oneArg(arguments, element, tSet(element))
        }

        "take", "drop" -> {
            oneArg(arguments, T_INT, tList(element))
        }

        "sorted" -> {
            if (arguments.isNotEmpty()) return SurfaceResult.Bad("sorted takes no arguments")
            val core = coreType(element)
            if (core is GuestType.Named && core.name !in setOf("Int", "Long", "Double", "String", "Free")) {
                return SurfaceResult.Bad("sorted orders Int, Long, Double, or String elements")
            }
            SurfaceResult.Ok(tList(element))
        }

        "map" -> {
            val transform = singleFunction(arguments) ?: return SurfaceResult.Bad("map takes one function")
            if (transform.parameters.size != 1 || !sameType(transform.parameters[0], element)) {
                return SurfaceResult.Bad("map transforms one element")
            }
            SurfaceResult.Ok(tList(transform.result))
        }

        "filter" -> {
            val predicate = singleFunction(arguments) ?: return SurfaceResult.Bad("filter takes one function")
            if (predicate.parameters.size != 1 || !sameType(predicate.parameters[0], element) || !sameType(predicate.result, T_BOOL)) {
                return SurfaceResult.Bad("filter predicates take one element and answer Boolean")
            }
            SurfaceResult.Ok(tList(element))
        }

        "fold" -> {
            if (arguments.size != 2) return SurfaceResult.Bad("fold takes an initial value and a step")
            val step = coreType(arguments[1])
            if (step !is GuestType.Function || step.parameters.size != 2) return SurfaceResult.Bad("fold steps take two arguments")
            if (!sameType(step.parameters[0], arguments[0]) || !sameType(step.parameters[1], element)) {
                return SurfaceResult.Bad("fold steps take the accumulator and the element")
            }
            SurfaceResult.Ok(arguments[0])
        }

        "any", "all" -> {
            val predicate = singleFunction(arguments) ?: return SurfaceResult.Bad("$name takes one function")
            if (predicate.parameters.size != 1 || !sameType(predicate.parameters[0], element) || !sameType(predicate.result, T_BOOL)) {
                return SurfaceResult.Bad("$name predicates take one element and answer Boolean")
            }
            SurfaceResult.Ok(T_BOOL)
        }

        else -> {
            SurfaceResult.Outside("MiscKotlin", "${receiver.name}.$name")
        }
    }
}

private fun plusCall(
    receiver: GuestType.Named,
    arguments: List<GuestType>,
    element: GuestType,
): SurfaceResult {
    if (arguments.size != 1) return SurfaceResult.Bad("plus takes one argument")
    return when (receiver.name) {
        "List" -> oneArg(arguments, tList(element), tList(element))
        "Set" -> oneArg(arguments, element, tSet(element))
        else -> SurfaceResult.Bad("plus is admitted for List and Set")
    }
}

private fun mapCall(
    receiver: GuestType.Named,
    name: String,
    arguments: List<GuestType>,
): SurfaceResult {
    val key = receiver.arguments.getOrElse(0) { T_FREE }
    val value = receiver.arguments.getOrElse(1) { T_FREE }
    return when (name) {
        "isEmpty" -> {
            zeroArgs(arguments)?.let { return it }.let { SurfaceResult.Ok(T_BOOL) }
        }

        "containsKey" -> {
            oneArg(arguments, key, T_BOOL)
        }

        "get" -> {
            oneArg(arguments, key, nullable(value))
        }

        "remove" -> {
            if (receiver.name != "MutableMap") return SurfaceResult.Bad("`remove` is a MutableMap member")
            oneArg(arguments, key, nullable(value))
        }

        "put" -> {
            if (receiver.name != "MutableMap") return SurfaceResult.Bad("`put` is a MutableMap member")
            if (arguments.size != 2 || !sameType(arguments[0], key) || !sameType(arguments[1], value)) {
                return SurfaceResult.Bad("put takes a key and a value")
            }
            SurfaceResult.Ok(nullable(value))
        }

        "plus" -> {
            oneArg(arguments, tPair(key, value), receiver)
        }

        else -> {
            SurfaceResult.Outside("MiscKotlin", "${receiver.name}.$name")
        }
    }
}

private fun zeroArgs(arguments: List<GuestType>): SurfaceResult? =
    if (arguments.isEmpty()) null else SurfaceResult.Bad("call takes no arguments")

private fun oneArg(
    arguments: List<GuestType>,
    parameter: GuestType,
    result: GuestType,
): SurfaceResult {
    if (arguments.size != 1 || !conforms(arguments[0], parameter)) {
        return SurfaceResult.Bad("argument types do not match the surface signature")
    }
    return SurfaceResult.Ok(result)
}

private fun singleFunction(arguments: List<GuestType>): GuestType.Function? {
    if (arguments.size != 1) return null
    return coreType(arguments[0]) as? GuestType.Function
}
