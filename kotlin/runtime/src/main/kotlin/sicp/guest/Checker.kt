// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

import arrow.core.raise.Raise

private class ClassInfo(
    val name: String,
    val data: Boolean,
    val singleton: Boolean,
    val properties: List<Property>,
    val methods: Map<String, FunctionDecl>,
    val parent: String?,
    val variants: MutableList<String>,
    val sealed: Boolean = false,
)

private enum class BindingKind { LOCAL, PARAM, FUNCTION, PROPERTY, INITIALIZING }

private class Binding(
    val name: String,
    var type: GuestType,
    val mutable: Boolean,
    val kind: BindingKind,
    var captured: Boolean = false,
)

private class Scope(
    val parent: Scope?,
    val lambdaBoundary: Boolean,
) {
    val bindings: MutableMap<String, Binding> = mutableMapOf()
}

/**
 * The type checker and admission authority of sections 2-3 and 6: every
 * expression gets a type with contextual expected types, every rejection is
 * classified into `HostInvalid` or `Unsupported` with its category and
 * position, and only a [CheckedProgram] reaches an evaluator.
 */
public class Checker(
    private val mode: Mode,
) {
    private val types: MutableMap<Expression, GuestType> = java.util.IdentityHashMap()
    private val lambdaCoercions: MutableSet<Expression> = java.util.Collections.newSetFromMap(java.util.IdentityHashMap())
    private val aliases = mutableMapOf<String, GuestType>()
    private val classes = mutableMapOf<String, ClassInfo>()
    private val functions = mutableMapOf<String, FunctionDecl>()
    private val properties = mutableMapOf<String, TopProperty>()
    private var narrowings: Map<Binding, GuestType> = emptyMap()
    private var loopDepth = 0
    private var lambdaDepth = 0
    private var currentClass: ClassInfo? = null
    private var currentReturn: GuestType = T_UNIT
    private var inCondition = false

    context(r: Raise<AdmissionError>)
    public fun check(
        program: Program,
        requireEntryPoint: Boolean = true,
    ): CheckedProgram {
        collectHeaders(if (mode.admitsQuery) QUERY_VOCABULARY + program.declarations else program.declarations)
        if (requireEntryPoint) requireEntryPoint(program)
        for (declaration in program.declarations) checkDeclaration(declaration)
        return CheckedProgram(program, types, mode, lambdaCoercions)
    }

    context(r: Raise<AdmissionError>)
    private fun fail(
        category: String,
        node: Node,
        message: String,
    ): Nothing = r.raise(AdmissionError.HostInvalid(category, node.span, message))

    context(r: Raise<AdmissionError>)
    private fun failUnsupported(
        category: String,
        node: Node,
        message: String,
    ): Nothing = r.raise(AdmissionError.Unsupported(category, node.span, message))

    context(r: Raise<AdmissionError>)
    private fun conformOrFail(
        actual: GuestType,
        expected: GuestType,
        at: Node,
        what: String,
    ) {
        if (conforms(actual, expected)) return
        val widthOnly =
            actual !is GuestType.Nullable && isNumeric(actual) && isNumeric(expected) && coreType(actual) != coreType(expected)
        fail(if (widthOnly) "ImplicitWidthChange" else "TypeMismatch", at, what)
    }

    // ---------- declarations ----------

    /** Section 2.2: a unit that runs contains exactly one top-level
     * `fun main()` with no parameters, a block body, and no declared result.
     * A second `main` is already a redeclaration by [collectHeaders]. */
    context(r: Raise<AdmissionError>)
    private fun requireEntryPoint(program: Program) {
        val main =
            program.declarations.filterIsInstance<FunctionDecl>().singleOrNull { it.name == "main" }
                ?: fail("UndeclaredName", program, "a compilation unit declares exactly one `fun main()`")
        if (main.parameters.isNotEmpty()) failUnsupported("MiscKotlin", main, "`main` takes no parameters")
        if (main.result != null) failUnsupported("MiscKotlin", main, "`main` declares no return type")
        if (main.body !is Block) failUnsupported("MiscKotlin", main, "`main` has a block body")
    }

    /** Rejects a second declaration of one name in one namespace: the same
     * signature is a host-invalid redeclaration, a different one is a
     * user-defined overload, which the grammar never admits (section 2.2). */
    context(r: Raise<AdmissionError>)
    private fun rejectDuplicate(
        previous: FunctionDecl,
        duplicate: FunctionDecl,
    ): Nothing =
        if (previous.parameters.map { it.type } == duplicate.parameters.map { it.type }) {
            fail("Redeclaration", duplicate, "conflicting declarations of `${duplicate.name}`")
        } else {
            failUnsupported("MiscKotlin", duplicate, "user-defined overload of `${duplicate.name}`")
        }

    context(r: Raise<AdmissionError>)
    private fun requireFreshType(
        declaration: Declaration,
        name: String,
    ) {
        if (name in classes || name in aliases) fail("Redeclaration", declaration, "type `$name` is declared twice")
    }

    context(r: Raise<AdmissionError>)
    private fun requireDistinct(
        names: List<String>,
        at: Node,
        what: String,
    ) {
        val seen = mutableSetOf<String>()
        for (name in names) if (!seen.add(name)) fail("Redeclaration", at, "$what `$name` is declared twice")
    }

    context(r: Raise<AdmissionError>)
    private fun methodTable(declaration: PlainClass): Map<String, FunctionDecl> {
        val table = LinkedHashMap<String, FunctionDecl>()
        for (method in declaration.methods) {
            table[method.name]?.let { rejectDuplicate(it, method) }
            table[method.name] = method
        }
        return table
    }

    /** Registers [declarations]: the query modes prepend the query DSL
     * vocabulary, admitted exactly like user-declared data. */
    context(r: Raise<AdmissionError>)
    private fun collectHeaders(declarations: List<Declaration>) {
        TypeFamilies.clear()
        for (declaration in declarations) {
            when (declaration) {
                is FunctionDecl -> {
                    functions[declaration.name]?.let { rejectDuplicate(it, declaration) }
                    functions[declaration.name] = declaration
                }

                is TopProperty -> {
                    if (declaration.property.name in properties) {
                        fail("Redeclaration", declaration, "property `${declaration.property.name}` is declared twice")
                    }
                    properties[declaration.property.name] = declaration
                }

                is TypeAlias -> {
                    requireFreshType(declaration, declaration.name)
                    aliases[declaration.name] = resolveType(declaration.target, declaration)
                }

                is DataClass -> {
                    requireFreshType(declaration, declaration.name)
                    requireDistinct(declaration.properties.map { it.name }, declaration, "property")
                    classes[declaration.name] =
                        ClassInfo(declaration.name, true, false, declaration.properties, emptyMap(), declaration.parent, mutableListOf())
                }

                is PlainClass -> {
                    requireFreshType(declaration, declaration.name)
                    requireDistinct(declaration.properties.map { it.name }, declaration, "property")
                    classes[declaration.name] =
                        ClassInfo(
                            declaration.name,
                            false,
                            false,
                            declaration.properties,
                            methodTable(declaration),
                            declaration.parent,
                            mutableListOf(),
                        )
                }

                is SealedInterface -> {
                    requireFreshType(declaration, declaration.name)
                    classes[declaration.name] =
                        ClassInfo(declaration.name, false, false, emptyList(), emptyMap(), null, mutableListOf(), sealed = true)
                }

                is DataObject -> {
                    requireFreshType(declaration, declaration.name)
                    classes[declaration.name] =
                        ClassInfo(declaration.name, true, true, emptyList(), emptyMap(), declaration.parent, mutableListOf())
                }
            }
        }
        for (declaration in declarations) {
            val parentName =
                when (declaration) {
                    is DataClass -> declaration.parent
                    is PlainClass -> declaration.parent
                    is DataObject -> declaration.parent
                    else -> null
                }
            if (parentName != null) linkVariant(declaration, parentName)
        }
    }

    /** Section 3.5: only a `data class` or `data object` may name a parent,
     * and the parent must be a `sealed interface`; every other supertype is
     * inheritance outside a sealed hierarchy. */
    context(r: Raise<AdmissionError>)
    private fun linkVariant(
        declaration: Declaration,
        parentName: String,
    ) {
        val parent = classes[parentName] ?: fail("UndeclaredName", declaration, "undeclared parent `$parentName`")
        if (declaration is PlainClass) failUnsupported("MiscKotlin", declaration, "a plain class has no inheritance")
        if (!parent.sealed) {
            failUnsupported("MiscKotlin", declaration, "inheritance outside a sealed hierarchy: `$parentName` is not a sealed interface")
        }
        val name = declarationName(declaration)
        if (name !in parent.variants) parent.variants.add(name)
        TypeFamilies.variantToFamily[name] = parentName
    }

    private fun declarationName(declaration: Declaration): String =
        when (declaration) {
            is FunctionDecl -> declaration.name
            is TopProperty -> declaration.property.name
            is TypeAlias -> declaration.name
            is DataClass -> declaration.name
            is PlainClass -> declaration.name
            is SealedInterface -> declaration.name
            is DataObject -> declaration.name
        }

    context(r: Raise<AdmissionError>)
    private fun checkDeclaration(declaration: Declaration) {
        when (declaration) {
            is FunctionDecl -> {
                checkFunction(declaration, entry = declaration.name == "main")
            }

            is TopProperty -> {
                checkTopProperty(declaration)
            }

            is DataClass, is SealedInterface, is DataObject, is TypeAlias -> {}

            is PlainClass -> {
                val previous = currentClass
                currentClass = classes[declaration.name]
                for (method in declaration.methods) checkFunction(method)
                currentClass = previous
            }
        }
    }

    context(r: Raise<AdmissionError>)
    private fun checkTopProperty(declaration: TopProperty) {
        val type = resolveType(declaration.property.type, declaration)
        conformOrFail(
            checkExpression(declaration.initializer, Scope(null, false), type),
            type,
            declaration.initializer,
            "initializer does not conform",
        )
    }

    context(r: Raise<AdmissionError>)
    private fun checkFunction(
        declaration: FunctionDecl,
        enclosing: Scope? = null,
        entry: Boolean = false,
    ) {
        val result = declaredResult(declaration, entry)
        requireDistinct(declaration.parameters.map { it.name }, declaration, "parameter")
        val scope = Scope(enclosing, lambdaBoundary = enclosing != null)
        for (parameter in declaration.parameters) {
            val type = resolveType(parameter.type, parameter)
            scope.bindings[parameter.name] = Binding(parameter.name, type, mutable = false, BindingKind.PARAM)
        }
        val previousReturn = currentReturn
        val previousNarrowings = narrowings
        val previousLoopDepth = loopDepth
        val previousLambdaDepth = lambdaDepth
        loopDepth = 0
        lambdaDepth = 0
        narrowings = emptyMap()
        currentReturn = result
        val bodyType =
            if (declaration.body is Block) {
                checkStatements(declaration.body.statements, scope)
                if (result != T_UNIT && !guaranteesReturn(declaration.body)) {
                    fail("TypeMismatch", declaration.body, "a block body with a result must return on every path")
                }
                result
            } else {
                checkExpression(declaration.body, scope, result)
            }
        conformOrFail(bodyType, result, declaration.body, "function body does not conform to its declared result")
        narrowings = previousNarrowings
        currentReturn = previousReturn
        loopDepth = previousLoopDepth
        lambdaDepth = previousLambdaDepth
        if (declaration.tailrec) checkTailrec(declaration)
    }

    context(r: Raise<AdmissionError>)
    private fun declaredResult(
        declaration: FunctionDecl,
        entry: Boolean = false,
    ): GuestType {
        if (declaration.result != null) return resolveType(declaration.result, declaration)
        if (entry) return T_UNIT
        failUnsupported("MiscKotlin", declaration, "every named function declares its result type")
    }

    private fun guaranteesReturn(block: Block): Boolean = block.statements.lastOrNull()?.let(::guaranteesReturn) ?: false

    private fun guaranteesReturn(statement: Statement): Boolean =
        when (statement) {
            is Return -> true
            is ExpressionStatement -> guaranteesReturn(statement.expression)
            else -> false
        }

    private fun guaranteesReturn(expression: Expression): Boolean =
        when (expression) {
            is Block -> guaranteesReturn(expression)
            is If -> expression.no != null && guaranteesReturn(expression.yes) && guaranteesReturn(expression.no)
            is When -> expression.otherwise != null && expression.branches.all { guaranteesReturn(it.body) }
            is Return -> true
            else -> false
        }

    context(r: Raise<AdmissionError>)
    private fun checkTailrec(declaration: FunctionDecl) {
        var misplaced = false
        walkTailExpression(declaration.body, tail = true) { call, isTail ->
            if ((call.callee as? Name)?.text == declaration.name && !isTail) misplaced = true
        }
        if (misplaced) failUnsupported("MiscKotlin", declaration, "tailrec self-calls must be in tail position")
    }

    private fun walkTailCalls(
        statement: Statement,
        tail: Boolean,
        visit: (Call, Boolean) -> Unit,
    ) {
        when (statement) {
            is ExpressionStatement -> {
                walkTailExpression(statement.expression, tail, visit)
            }

            is Return -> {
                statement.value?.let { walkTailExpression(it, tail, visit) }
            }

            is LocalProperty -> {
                walkTailExpression(statement.initializer, tail = false, visit)
            }

            is Assignment -> {
                walkTailExpression(statement.target, tail = false, visit)
                walkTailExpression(statement.value, tail = false, visit)
            }

            is While -> {
                walkTailExpression(statement.condition, tail = false, visit)
                statement.body.statements.forEach { walkTailCalls(it, tail = false, visit) }
            }

            is For -> {
                walkTailExpression(statement.iterable, tail = false, visit)
                statement.end?.let { walkTailExpression(it, tail = false, visit) }
                statement.body.statements.forEach { walkTailCalls(it, tail = false, visit) }
            }

            is Destructure -> {
                walkTailExpression(statement.initializer, tail = false, visit)
            }

            is FunctionDecl, is Break, is Continue -> {}
        }
    }

    private fun walkTailBlock(
        block: Block,
        tail: Boolean,
        visit: (Call, Boolean) -> Unit,
    ) {
        for (statement in block.statements.dropLast(1)) walkTailCalls(statement, tail = false, visit)
        block.statements.lastOrNull()?.let { walkTailCalls(it, tail, visit) }
    }

    private fun walkTailExpression(
        expression: Expression,
        tail: Boolean,
        visit: (Call, Boolean) -> Unit,
    ) {
        when (expression) {
            is Call -> {
                visit(expression, tail)
                walkTailExpression(expression.callee, tail = false, visit)
                for (argument in expression.arguments) walkTailExpression(argument.value, tail = false, visit)
            }

            is If -> {
                walkTailExpression(expression.condition, tail = false, visit)
                walkTailExpression(expression.yes, tail, visit)
                expression.no?.let { walkTailExpression(it, tail, visit) }
            }

            is When -> {
                expression.subject?.let { walkTailExpression(it, tail = false, visit) }
                for (branch in expression.branches) walkTailExpression(branch.body, tail, visit)
                expression.otherwise?.let { walkTailExpression(it, tail, visit) }
            }

            is Block -> {
                walkTailBlock(expression, tail, visit)
            }

            is Return -> {
                expression.value?.let { walkTailExpression(it, tail, visit) }
            }

            is Binary -> {
                walkTailExpression(expression.left, tail = false, visit)
                walkTailExpression(expression.right, tail = false, visit)
            }

            is Elvis -> {
                walkTailExpression(expression.left, tail = false, visit)
                walkTailExpression(expression.right, tail, visit)
            }

            is Unary -> {
                walkTailExpression(expression.operand, tail = false, visit)
            }

            else -> {}
        }
    }

    // ---------- types ----------

    context(r: Raise<AdmissionError>)
    private fun resolveType(
        type: GuestType,
        at: Node,
    ): GuestType =
        when (type) {
            is GuestType.Nullable -> {
                GuestType.Nullable(resolveType(type.base, at))
            }

            is GuestType.Function -> {
                GuestType.Function(type.parameters.map { resolveType(it, at) }, resolveType(type.result, at))
            }

            is GuestType.Nothing -> {
                type
            }

            is GuestType.Null -> {
                type
            }

            is GuestType.Named -> {
                resolveNamedType(type, at)
            }
        }

    context(r: Raise<AdmissionError>)
    private fun resolveNamedType(
        type: GuestType.Named,
        at: Node,
    ): GuestType {
        if (type.name == "Free") return type
        aliases[type.name]?.let { return it }
        val arity = type.arguments.size
        val known =
            when (type.name) {
                "Int", "Long", "Double", "Boolean", "String", "Unit" -> arity == 0
                "List", "MutableList", "Set", "Collection" -> arity == 1
                "Map", "MutableMap", "Pair" -> arity == 2
                "Thunk" -> arity == 1 && (mode == Mode.LAZY || mode == Mode.QUERY_LAZY)
                "Random" -> arity == 0 && (mode == Mode.SEARCH || mode == Mode.QUERY_SEARCH)
                else -> false
            }
        if (known) return GuestType.Named(type.name, type.arguments.map { resolveType(it, at) })
        rejectTypeName(type.name, at)
        val info = classes[type.name] ?: fail("UndeclaredName", at, "undeclared type `${type.name}`")
        if (arity != 0) fail("TypeMismatch", at, "declared types take no type arguments")
        return GuestType.Named(info.name)
    }

    context(r: Raise<AdmissionError>)
    private fun rejectTypeName(
        name: String,
        at: Node,
    ) {
        when (name) {
            "Char" -> {
                failUnsupported("CharSurface", at, "no Char type in the subset")
            }

            "Byte", "Short", "Float", "UByte", "UShort", "UInt", "ULong", "Any", "KClass" -> {
                failUnsupported("MiscKotlin", at, "`$name` is outside the type surface")
            }

            "BigInteger", "BigDecimal" -> {
                failUnsupported("MiscKotlin", at, "arbitrary-precision types are rejected")
            }

            "Sequence" -> {
                failUnsupported("Sequence", at, "lazy data belongs to the lazy module")
            }
        }
    }

    // ---------- statements and blocks ----------

    context(r: Raise<AdmissionError>)
    private fun checkStatements(
        statements: List<Statement>,
        scope: Scope,
    ) {
        for (statement in statements.filterIsInstance<FunctionDecl>()) declareLocalFunction(statement, scope)
        for (statement in statements) {
            checkStatement(statement, scope)
            val conditional = (statement as? ExpressionStatement)?.expression as? If ?: continue
            if (guaranteesReturn(conditional.yes)) {
                narrowCondition(conditional.condition, succeeds = false, scope)
            } else if (conditional.no != null && guaranteesReturn(conditional.no)) {
                narrowCondition(conditional.condition, succeeds = true, scope)
            }
        }
    }

    context(r: Raise<AdmissionError>)
    private fun declareLocalFunction(
        declaration: FunctionDecl,
        scope: Scope,
    ) {
        val result = declaredResult(declaration)
        val parameters = declaration.parameters.map { resolveType(it.type, it) }
        scope.bindings[declaration.name] =
            Binding(declaration.name, GuestType.Function(parameters, result), mutable = false, BindingKind.FUNCTION)
    }

    context(r: Raise<AdmissionError>)
    private fun checkStatement(
        statement: Statement,
        scope: Scope,
    ) {
        when (statement) {
            is FunctionDecl -> checkFunction(statement, scope)
            is LocalProperty -> checkLocalProperty(statement, scope)
            is Destructure -> checkDestructure(statement, scope)
            is Assignment -> checkAssignment(statement, scope)
            is While -> checkWhile(statement, scope)
            is For -> checkFor(statement, scope)
            is Return -> checkReturn(statement, scope)
            is Break -> if (loopDepth == 0) fail("Syntax", statement, "break outside a loop")
            is Continue -> if (loopDepth == 0) fail("Syntax", statement, "continue outside a loop")
            is ExpressionStatement -> checkExpression(statement.expression, scope, null)
        }
    }

    context(r: Raise<AdmissionError>)
    private fun checkBlock(
        block: Block,
        scope: Scope,
    ): GuestType {
        val previous = narrowings
        val inner = Scope(scope, lambdaBoundary = false)
        checkStatements(block.statements, inner)
        narrowings = previous
        return blockValue(block)
    }

    private fun blockValue(block: Block): GuestType {
        val last = block.statements.lastOrNull()
        return if (last is ExpressionStatement) types[last.expression] ?: T_UNIT else T_UNIT
    }

    context(r: Raise<AdmissionError>)
    private fun checkLocalProperty(
        statement: LocalProperty,
        scope: Scope,
    ) {
        val declared = statement.annotation?.let { resolveType(it, statement) }
        val isLambdaValue = declared == null && (statement.initializer is Lambda || statement.initializer is Block)
        if (isLambdaValue) {
            scope.bindings[statement.name] = Binding(statement.name, T_FREE, statement.mutable, BindingKind.INITIALIZING)
            val type = checkExpression(statement.initializer, scope, null)
            scope.bindings[statement.name] = Binding(statement.name, type, statement.mutable, BindingKind.LOCAL)
            return
        }
        val type = checkExpression(statement.initializer, scope, declared)
        val final = declared ?: type
        if (declared != null) conformOrFail(type, declared, statement.initializer, "initializer does not conform")
        scope.bindings[statement.name] = Binding(statement.name, final, statement.mutable, BindingKind.LOCAL)
    }

    context(r: Raise<AdmissionError>)
    private fun checkDestructure(
        statement: Destructure,
        scope: Scope,
    ) {
        val source = checkExpression(statement.initializer, scope, null)
        val components = destructureComponents(source, statement)
        if (components.size != statement.names.size) {
            fail("TypeMismatch", statement, "destructuring expects ${components.size} names")
        }
        for ((name, type) in statement.names.zip(components)) {
            scope.bindings[name] = Binding(name, type, mutable = false, BindingKind.LOCAL)
        }
    }

    context(r: Raise<AdmissionError>)
    private fun destructureComponents(
        source: GuestType,
        at: Node,
    ): List<GuestType> {
        val core = coreType(source)
        if (core is GuestType.Named && core.name == "Pair" && core.arguments.size == 2) return core.arguments
        val info = classInfoOrNull(core) ?: fail("TypeMismatch", at, "destructuring needs a Pair or a data class")
        if (!info.data) fail("TypeMismatch", at, "destructuring needs a Pair or a data class")
        return info.properties.map { resolveType(it.type, it) }
    }

    private fun classInfoOrNull(type: GuestType): ClassInfo? {
        val core = type as? GuestType.Named ?: return null
        return classes[core.name]
    }

    context(r: Raise<AdmissionError>)
    private fun checkAssignment(
        statement: Assignment,
        scope: Scope,
    ) {
        when (val target = statement.target) {
            is Name -> checkNameAssignment(statement, target, scope)
            is Member -> checkMemberAssignment(statement, target, scope)
            is Index -> checkIndexAssignment(statement, target, scope)
            else -> fail("Syntax", statement, "assignment target must be a name, member, or index")
        }
    }

    context(r: Raise<AdmissionError>)
    private fun checkNameAssignment(
        statement: Assignment,
        target: Name,
        scope: Scope,
    ) {
        val inScope = resolveBindingOrNull(target, scope)
        if (inScope != null) {
            if (inScope.kind == BindingKind.INITIALIZING) raiseRecursiveValLambda(target)
            if (!inScope.mutable) fail("ValReassignment", statement, "reassignment of `val` binding `${target.text}`")
            val valueType = checkExpression(statement.value, scope, inScope.type)
            checkCompound(statement.operator, inScope.type, valueType, statement)
            narrowings = narrowings - inScope
            return
        }
        val property =
            properties[target.text]
                ?: fail("UndeclaredName", target, "undeclared name `${target.text}`")
        if (!property.property.mutable) fail("ValReassignment", statement, "property `${target.text}` is a `val`")
        val propertyType = resolveType(property.property.type, property)
        val valueType = checkExpression(statement.value, scope, propertyType)
        checkCompound(statement.operator, propertyType, valueType, statement)
    }

    context(r: Raise<AdmissionError>)
    private fun checkMemberAssignment(
        statement: Assignment,
        target: Member,
        scope: Scope,
    ) {
        val receiver = checkExpression(target.receiver, scope, null)
        val info = classInfoOrNull(receiver) ?: fail("TypeMismatch", target, "member assignment needs a class instance")
        val property =
            info.properties.firstOrNull { it.name == target.name }
                ?: fail("UndeclaredName", target, "undeclared property `${target.name}`")
        if (!property.mutable) fail("ValReassignment", statement, "property `${target.name}` is a `val`")
        val propertyType = resolveType(property.type, property)
        val valueType = checkExpression(statement.value, scope, propertyType)
        checkCompound(statement.operator, propertyType, valueType, statement)
    }

    context(r: Raise<AdmissionError>)
    private fun checkIndexAssignment(
        statement: Assignment,
        target: Index,
        scope: Scope,
    ) {
        val receiver = checkExpression(target.receiver, scope, null)
        if (receiver is GuestType.Nullable) fail("TypeMismatch", target.receiver, "nullable receivers need `?.` or `?:`")
        val core = coreType(receiver)
        val keyType = checkExpression(target.index, scope, null)
        if (core !is GuestType.Named || core.name !in setOf("MutableList", "MutableMap")) {
            fail("TypeMismatch", target, "indexed writes need a mutable collection")
        }
        val valueType = checkExpression(statement.value, scope, null)
        if (core.name == "MutableList") {
            conformOrFail(keyType, T_INT, target.index, "list indices are Int")
            conformOrFail(valueType, core.arguments[0], statement.value, "element type mismatch")
        } else {
            conformOrFail(keyType, core.arguments[0], target.index, "key type mismatch")
            conformOrFail(valueType, core.arguments[1], statement.value, "value type mismatch")
        }
        checkCompound(statement.operator, valueType, valueType, statement)
    }

    context(r: Raise<AdmissionError>)
    private fun checkCompound(
        operator: String,
        target: GuestType,
        value: GuestType,
        at: Node,
    ) {
        if (operator == "=") {
            conformOrFail(value, target, at, "assigned value does not conform")
            return
        }
        if (operator == "+=" && sameType(coreType(target), T_STRING)) {
            conformOrFail(value, T_STRING, at, "string concatenation needs a String")
            return
        }
        if (isNumeric(target) && sameType(target, value)) return
        if (isNumeric(target) && isNumeric(value)) failUnsupported("MixedWidthArithmetic", at, "compound assignment widths differ")
        fail("TypeMismatch", at, "compound assignment on non-numeric operands")
    }

    context(r: Raise<AdmissionError>)
    private fun checkWhile(
        statement: While,
        scope: Scope,
    ) {
        checkCondition(statement.condition, scope)
        val previous = narrowings
        narrowCondition(statement.condition, succeeds = true, scope)
        loopDepth++
        checkBlock(statement.body, scope)
        loopDepth--
        narrowings = previous
    }

    context(r: Raise<AdmissionError>)
    private fun checkFor(
        statement: For,
        scope: Scope,
    ) {
        val element = forElementType(statement, scope)
        val inner = Scope(scope, lambdaBoundary = false)
        inner.bindings[statement.name] = Binding(statement.name, element, mutable = false, BindingKind.LOCAL)
        loopDepth++
        checkBlock(statement.body, inner)
        loopDepth--
    }

    context(r: Raise<AdmissionError>)
    private fun forElementType(
        statement: For,
        scope: Scope,
    ): GuestType {
        if (statement.end != null) {
            val start = checkExpression(statement.iterable, scope, null)
            val end = checkExpression(statement.end, scope, start)
            if (!sameType(start, end) || coreType(start) == T_DOUBLE) {
                fail("TypeMismatch", statement, "ranges use one Int or Long width")
            }
            return start
        }
        val iterable = coreType(checkExpression(statement.iterable, scope, null))
        if (iterable !is GuestType.Named) fail("TypeMismatch", statement.iterable, "for needs a collection or a range")
        return when (iterable.name) {
            "List", "MutableList", "Set", "Collection" -> iterable.arguments[0]
            "String" -> failUnsupported("CharSurface", statement.iterable, "no Char type in the subset")
            else -> fail("TypeMismatch", statement.iterable, "for needs a collection or a range")
        }
    }

    context(r: Raise<AdmissionError>)
    private fun checkReturn(
        statement: Return,
        scope: Scope,
    ) {
        if (lambdaDepth > 0) fail("Syntax", statement, "return is only allowed in a named function")
        val value = statement.value?.let { checkExpression(it, scope, currentReturn) }
        if (value != null) {
            conformOrFail(value, currentReturn, statement, "returned value does not conform")
        } else if (!sameType(currentReturn, T_UNIT)) {
            fail("TypeMismatch", statement, "a bare `return` belongs to a Unit function")
        }
    }

    context(r: Raise<AdmissionError>)
    private fun checkCondition(
        expression: Expression,
        scope: Scope,
    ) {
        val previous = inCondition
        inCondition = true
        val type = checkExpression(expression, scope, T_BOOL)
        inCondition = previous
        requireBoolean(type, expression)
    }

    context(r: Raise<AdmissionError>)
    private fun requireBoolean(
        type: GuestType,
        at: Node,
    ) {
        if (type is GuestType.Nullable) fail("NullableCondition", at, "a nullable Boolean is not a condition")
        if (!sameType(type, T_BOOL)) fail("NonBooleanCondition", at, "conditions are Boolean-only")
    }

    // ---------- expressions ----------

    context(r: Raise<AdmissionError>)
    private fun checkExpression(
        expression: Expression,
        scope: Scope,
        expected: GuestType?,
    ): GuestType {
        val previous = narrowings
        val type = dispatchExpression(expression, scope, expected)
        narrowings = previous
        types[expression] = type
        return type
    }

    context(r: Raise<AdmissionError>)
    private fun dispatchExpression(
        expression: Expression,
        scope: Scope,
        expected: GuestType?,
    ): GuestType =
        when (expression) {
            is Literal -> {
                literalType(expression, expected)
            }

            is Name -> {
                nameType(expression, scope)
            }

            is StringTemplate -> {
                templateType(expression, scope)
            }

            is Binary -> {
                binaryType(expression, scope)
            }

            is Unary -> {
                unaryType(expression, scope)
            }

            is Elvis -> {
                elvisType(expression, scope)
            }

            is Is -> {
                isType(expression, scope)
            }

            is If -> {
                ifType(expression, scope, expected)
            }

            is When -> {
                whenType(expression, scope, expected, WhenUse.EXPRESSION)
            }

            is Lambda -> {
                lambdaType(expression, scope, expected)
            }

            is Call -> {
                callType(expression, scope, expected)
            }

            is Member -> {
                memberValueType(expression, scope)
            }

            is Index -> {
                indexType(expression, scope)
            }

            is Block -> {
                blockOrZeroLambda(expression, scope, expected)
            }

            is Return -> {
                checkReturn(expression, scope)
                GuestType.Nothing
            }

            is This -> {
                thisType(expression)
            }

            is CallableReference -> {
                callableType(expression, scope)
            }
        }

    /** "An omitted arrow denotes a zero-parameter lambda, as determined by
     * the expected function type." */
    context(r: Raise<AdmissionError>)
    private fun blockOrZeroLambda(
        block: Block,
        scope: Scope,
        expected: GuestType?,
    ): GuestType {
        val function = coreType(expected ?: T_FREE)
        if (function !is GuestType.Function || function.parameters.isNotEmpty()) return checkBlock(block, scope)
        lambdaCoercions.add(block)
        return zeroArgLambdaType(block, scope, function)
    }

    context(r: Raise<AdmissionError>)
    private fun zeroArgLambdaType(
        body: Block,
        scope: Scope,
        expected: GuestType.Function,
    ): GuestType {
        val inner = Scope(scope, lambdaBoundary = true)
        lambdaDepth++
        val previousReturn = currentReturn
        val previousLoopDepth = loopDepth
        loopDepth = 0
        currentReturn = expected.result
        checkStatements(body.statements, inner)
        val bodyType = blockValue(body)
        currentReturn = previousReturn
        loopDepth = previousLoopDepth
        lambdaDepth--
        if (expected.result != T_FREE) {
            conformOrFail(bodyType, expected.result, body, "lambda body does not conform")
            return expected
        }
        return GuestType.Function(emptyList(), bodyType)
    }

    context(r: Raise<AdmissionError>)
    private fun literalType(
        literal: Literal,
        expected: GuestType?,
    ): GuestType =
        when (literal.kind) {
            LiteralKind.BOOLEAN -> T_BOOL
            LiteralKind.STRING -> T_STRING
            LiteralKind.NULL -> GuestType.Null
            LiteralKind.DOUBLE -> T_DOUBLE
            LiteralKind.INT, LiteralKind.LONG -> numericLiteralType(literal, expected)
        }

    context(r: Raise<AdmissionError>)
    private fun numericLiteralType(
        literal: Literal,
        expected: GuestType?,
    ): GuestType {
        val signed = literal.text.startsWith("-")
        val magnitude = literal.text.removePrefix("-")
        val target = coreType(expected ?: T_FREE)
        if (literal.kind == LiteralKind.LONG) {
            if (!fitsLong(magnitude, signed)) fail("LiteralRange", literal, "literal out of range")
            return T_LONG
        }
        if (target == T_INT) {
            if (!fitsInt(magnitude, signed)) fail("LiteralRange", literal, "literal out of range")
            return T_INT
        }
        if (target == T_LONG) {
            if (!fitsLong(magnitude, signed)) fail("LiteralRange", literal, "literal out of range")
            return T_LONG
        }
        if (fitsInt(magnitude, signed)) return T_INT
        if (fitsLong(magnitude, signed)) return T_LONG
        return fail("LiteralRange", literal, "literal out of range")
    }

    private fun fitsInt(
        magnitude: String,
        signed: Boolean,
    ): Boolean {
        if (magnitudeExceeds(magnitude, INT_MAX)) return signed && magnitude.trimStart('0') == "2147483648"
        return true
    }

    private fun fitsLong(
        magnitude: String,
        signed: Boolean,
    ): Boolean = !magnitudeExceeds(magnitude, LONG_MAX) || (signed && magnitude.trimStart('0') == LONG_MIN_ABS)

    private fun magnitudeExceeds(
        magnitude: String,
        limit: String,
    ): Boolean {
        val clean = magnitude.trimStart('0').ifEmpty { "0" }
        if (clean.length != limit.length) return clean.length > limit.length
        return clean > limit
    }

    context(r: Raise<AdmissionError>)
    private fun templateType(
        template: StringTemplate,
        scope: Scope,
    ): GuestType {
        for (fragment in template.fragments) {
            val type = checkExpression(fragment, scope, null)
            val core = coreType(type)
            val printable = core is GuestType.Named && core.name in setOf("String", "Long", "Double", "Boolean")
            if (!printable) failUnsupported("StructuredOutput", fragment, "templates admit String, Long, Double, Boolean")
        }
        return T_STRING
    }

    context(r: Raise<AdmissionError>)
    private fun binaryType(
        expression: Binary,
        scope: Scope,
    ): GuestType {
        val left = checkExpression(expression.left, scope, null)
        val previous = narrowings
        if (expression.operator == "&&") narrowCondition(expression.left, succeeds = true, scope)
        if (expression.operator == "||") narrowCondition(expression.left, succeeds = false, scope)
        val right = checkExpression(expression.right, scope, null)
        narrowings = previous
        return binaryResult(expression, left, right)
    }

    context(r: Raise<AdmissionError>)
    private fun binaryResult(
        expression: Binary,
        left: GuestType,
        right: GuestType,
    ): GuestType =
        when (expression.operator) {
            "&&", "||" -> {
                requireBoolean(left, expression.left)
                requireBoolean(right, expression.right)
                T_BOOL
            }

            "+", "-", "*", "/", "%" -> {
                arithmeticResult(expression, left, right)
            }

            "<", "<=", ">", ">=" -> {
                if (!isNumeric(left) || !isNumeric(right)) fail("TypeMismatch", expression, "comparisons need numbers")
                if (!sameType(left, right)) failUnsupported("MixedWidthArithmetic", expression, "comparison widths differ")
                T_BOOL
            }

            "==", "!=", "===", "!==" -> {
                if (coreType(left) == GuestType.Null || coreType(right) == GuestType.Null) return T_BOOL
                if (!sameType(left, right)) fail("MixedEquality", expression, "`==` operands must share one type")
                T_BOOL
            }

            "to" -> {
                tPair(left, right)
            }

            else -> {
                fail("Syntax", expression, "unknown operator `${expression.operator}`")
            }
        }

    context(r: Raise<AdmissionError>)
    private fun arithmeticResult(
        expression: Binary,
        left: GuestType,
        right: GuestType,
    ): GuestType {
        val op = expression.operator
        if (op == "+" && sameType(left, T_STRING) && sameType(right, T_STRING)) return T_STRING
        if (op == "+" && sameType(left, right) && coreType(left) is GuestType.Named) {
            if ((coreType(left) as GuestType.Named).name == "List") return left
        }
        val leftCore = coreType(left) as? GuestType.Named
        val rightCore = coreType(right) as? GuestType.Named
        if (leftCore != null && leftCore.name == "Set" && (op == "+" || op == "-") && !sameType(left, right)) return left
        if (op == "+" && leftCore != null && leftCore.name == "Map" && rightCore != null && rightCore.name == "Pair") return left
        if (!isNumeric(left) || !isNumeric(right)) fail("TypeMismatch", expression, "arithmetic needs numbers")
        if (op == "%" && sameType(coreType(left), T_DOUBLE)) {
            failUnsupported("MiscKotlin", expression, "`%` on Double is rejected")
        }
        if (!sameType(left, right)) failUnsupported("MixedWidthArithmetic", expression, "arithmetic widths differ")
        return left
    }

    context(r: Raise<AdmissionError>)
    private fun unaryType(
        expression: Unary,
        scope: Scope,
    ): GuestType {
        val operand = checkExpression(expression.operand, scope, null)
        if (expression.operator == "-") {
            if (!isNumeric(operand)) fail("TypeMismatch", expression, "unary minus needs a number")
            return operand
        }
        requireBoolean(operand, expression.operand)
        return T_BOOL
    }

    context(r: Raise<AdmissionError>)
    private fun elvisType(
        expression: Elvis,
        scope: Scope,
    ): GuestType {
        val left = checkExpression(expression.left, scope, null)
        val right = checkExpression(expression.right, scope, coreType(left))
        return unifyBranches(coreType(left), right, expression)
    }

    /** The least upper bound of two branch types: the join of their
     * non-null cores (one core conforming to the other, or two variants of
     * one sealed family), made nullable when either side admits `null`. */
    context(r: Raise<AdmissionError>)
    private fun unifyBranches(
        first: GuestType,
        second: GuestType,
        at: Node,
    ): GuestType = joinedBranchType(first, second) ?: fail("TypeMismatch", at, "branch values disagree on type")

    context(r: Raise<AdmissionError>)
    private fun isType(
        expression: Is,
        scope: Scope,
    ): GuestType {
        checkExpression(expression.value, scope, null)
        val tested = resolveType(expression.type, expression)
        if (inCondition) enforceNarrowable(expression.value, scope)
        if (!expression.negated) narrow(expression.value, tested, scope)
        return T_BOOL
    }

    context(r: Raise<AdmissionError>)
    private fun enforceNarrowable(
        subject: Expression,
        scope: Scope,
    ) {
        if (subject is Member) failUnsupported("PropertySmartCast", subject, "smart casts never apply to properties")
        if (subject is Name && resolveBindingOrNull(subject, scope)?.captured == true) {
            failUnsupported("PropertySmartCast", subject, "smart casts never apply to captured `var` bindings")
        }
    }

    private fun narrow(
        subject: Expression,
        tested: GuestType,
        scope: Scope,
    ) {
        if (subject !is Name) return
        val binding = resolveBindingOrNull(subject, scope) ?: return
        if (!binding.mutable || !binding.captured) narrowings = narrowings + (binding to tested)
    }

    context(r: Raise<AdmissionError>)
    private fun raiseRecursiveValLambda(name: Name): Nothing =
        failUnsupported("RecursiveValLambda", name, "a `val` lambda that references its own name is rejected")

    context(r: Raise<AdmissionError>)
    private fun ifType(
        expression: If,
        scope: Scope,
        expected: GuestType?,
    ): GuestType {
        checkCondition(expression.condition, scope)
        val previous = narrowings
        narrowCondition(expression.condition, succeeds = true, scope)
        val yes = checkExpression(expression.yes, scope, expected)
        narrowings = previous
        narrowCondition(expression.condition, succeeds = false, scope)
        val no = expression.no?.let { checkExpression(it, scope, expected) }
        narrowings = previous
        return if (no == null) T_UNIT else unifyBranches(yes, no, expression)
    }

    /** `is` tests and stable null comparisons refine their known branch.
     * Conjunction composes true paths; disjunction composes false paths. */
    context(r: Raise<AdmissionError>)
    private fun narrowCondition(
        expression: Expression,
        succeeds: Boolean,
        scope: Scope,
    ) {
        when (expression) {
            is Is -> {
                if (expression.negated != succeeds) {
                    narrow(expression.value, resolveType(expression.type, expression), scope)
                }
            }

            is Unary -> {
                if (expression.operator == "!") narrowCondition(expression.operand, !succeeds, scope)
            }

            is Binary -> {
                if (expression.operator == "&&" && succeeds) {
                    narrowCondition(expression.left, succeeds = true, scope)
                    narrowCondition(expression.right, succeeds = true, scope)
                }
                if (expression.operator == "||" && !succeeds) {
                    narrowCondition(expression.left, succeeds = false, scope)
                    narrowCondition(expression.right, succeeds = false, scope)
                }
                if ((expression.operator == "==" || expression.operator == "!=") &&
                    (expression.operator == "!=") == succeeds
                ) {
                    val subject =
                        if ((expression.left as? Literal)?.kind == LiteralKind.NULL) {
                            expression.right
                        } else if ((expression.right as? Literal)?.kind == LiteralKind.NULL) {
                            expression.left
                        } else {
                            null
                        }
                    if (subject is Name) {
                        val binding = resolveBindingOrNull(subject, scope)
                        val current = binding?.let { narrowings[it] ?: it.type }
                        if (current is GuestType.Nullable) narrow(subject, current.base, scope)
                    }
                }
            }

            else -> {}
        }
    }

    private enum class WhenUse { EXPRESSION, STATEMENT }

    context(r: Raise<AdmissionError>)
    private fun whenType(
        expression: When,
        scope: Scope,
        expected: GuestType?,
        use: WhenUse,
    ): GuestType {
        val subjectType = expression.subject?.let { checkExpression(it, scope, null) }
        var result: GuestType = T_UNIT
        val covered = mutableSetOf<String>()
        for (branch in expression.branches) {
            val branchType = checkWhenBranch(branch, expression, subjectType, scope, expected, covered)
            result = if (result == T_UNIT) branchType else unifyBranches(result, branchType, branch)
        }
        expression.otherwise?.let { otherwise ->
            val elseType = checkExpression(otherwise, scope, expected)
            result = if (result == T_UNIT) elseType else unifyBranches(result, elseType, otherwise)
        }
        enforceWhenTotality(expression, subjectType, covered, use)
        return result
    }

    context(r: Raise<AdmissionError>)
    private fun checkWhenBranch(
        branch: WhenBranch,
        expression: When,
        subjectType: GuestType?,
        scope: Scope,
        expected: GuestType?,
        covered: MutableSet<String>,
    ): GuestType {
        val previous = narrowings
        val pattern = branch.typePattern
        if (pattern != null) {
            val tested = resolveType(pattern, branch)
            narrowWhenSubject(expression, tested, scope)
            val variantName = (coreType(tested) as? GuestType.Named)?.name
            if (variantName != null && variantName in (classInfoOrNull(subjectType ?: T_FREE)?.variants ?: emptyList())) {
                covered.add(variantName)
            }
        } else if (branch.pattern != null) {
            checkValuePattern(branch.pattern, expression, subjectType, scope, covered)
            if (expression.subject == null) narrowCondition(branch.pattern, succeeds = true, scope)
        }
        val body = checkExpression(branch.body, scope, expected)
        narrowings = previous
        return body
    }

    context(r: Raise<AdmissionError>)
    private fun narrowWhenSubject(
        expression: When,
        tested: GuestType,
        scope: Scope,
    ) {
        val subject = expression.subject ?: return
        enforceNarrowable(subject, scope)
        narrow(subject, tested, scope)
    }

    context(r: Raise<AdmissionError>)
    private fun checkValuePattern(
        pattern: Expression,
        expression: When,
        subjectType: GuestType?,
        scope: Scope,
        covered: MutableSet<String>,
    ) {
        if (pattern is Name && subjectType != null && classInfoOrNull(subjectType)?.singleton == true) {
            val info = classInfoOrNull(subjectType)
            if (info != null && pattern.text in info.variants) {
                covered.add(pattern.text)
                return
            }
        }
        if (expression.subject == null) {
            requireBoolean(checkExpression(pattern, scope, T_BOOL), pattern)
            return
        }
        val patternType = checkExpression(pattern, scope, null)
        if (subjectType != null && patternType != GuestType.Null && !sameType(patternType, subjectType)) {
            fail("MixedEquality", pattern, "a `when` branch value shares the subject type")
        }
    }

    context(r: Raise<AdmissionError>)
    private fun enforceWhenTotality(
        expression: When,
        subjectType: GuestType?,
        covered: Set<String>,
        use: WhenUse,
    ) {
        val info = subjectType?.let { classInfoOrNull(it) }
        val sealedDispatch = info != null && info.variants.isNotEmpty()
        if (sealedDispatch && expression.otherwise != null) {
            failUnsupported("SealedWhenElse", expression, "sealed dispatch is closed; no `else`")
        }
        val total =
            when {
                expression.otherwise != null -> true
                sealedDispatch -> info.variants.all { it in covered }
                else -> false
            }
        if (total) return
        if (use == WhenUse.STATEMENT) {
            failUnsupported("NonExhaustiveWhen", expression, "every admitted `when` is exhaustive in its form")
        }
        fail("NonExhaustiveWhenExpression", expression, "a `when` expression must cover every case")
    }

    context(r: Raise<AdmissionError>)
    private fun lambdaType(
        expression: Lambda,
        scope: Scope,
        expected: GuestType?,
    ): GuestType {
        val expectedFunction = coreType(expected ?: T_FREE) as? GuestType.Function
        val inner = Scope(scope, lambdaBoundary = true)
        val parameters = mutableListOf<GuestType>()
        for ((index, parameter) in expression.parameters.withIndex()) {
            val type = resolveLambdaParameter(parameter, expectedFunction, index)
            parameters.add(type)
            inner.bindings[parameter.name] = Binding(parameter.name, type, mutable = false, BindingKind.PARAM)
        }
        if (expectedFunction == null && expression.parameters.any { it.annotation == null }) {
            fail("TypeMismatch", expression, "lambda parameters need annotations or an expected type")
        }
        lambdaDepth++
        val previousReturn = currentReturn
        val previousLoopDepth = loopDepth
        loopDepth = 0
        currentReturn = expectedFunction?.result ?: T_FREE
        checkStatements(expression.body.statements, inner)
        val bodyType = blockValue(expression.body)
        currentReturn = previousReturn
        loopDepth = previousLoopDepth
        lambdaDepth--
        val pinned = expectedFunction != null && expectedFunction.result != T_FREE
        if (pinned) conformOrFail(bodyType, expectedFunction.result, expression.body, "lambda body does not conform")
        val result = if (pinned) expectedFunction.result else bodyType
        return GuestType.Function(parameters, result)
    }

    context(r: Raise<AdmissionError>)
    private fun resolveLambdaParameter(
        parameter: LambdaParameter,
        expected: GuestType.Function?,
        index: Int,
    ): GuestType {
        if (parameter.annotation != null) return resolveType(parameter.annotation, parameter)
        return expected?.parameters?.getOrNull(index)
            ?: fail("TypeMismatch", parameter, "lambda parameters need annotations or an expected type")
    }

    context(r: Raise<AdmissionError>)
    private fun nameType(
        expression: Name,
        scope: Scope,
    ): GuestType {
        resolveBindingOrNull(expression, scope)?.let { binding ->
            return narrowings[binding] ?: checkedBinding(expression, binding)
        }
        functions[expression.text]?.let { return functionType(it) }
        properties[expression.text]?.let { return resolveType(it.property.type, it) }
        classes[expression.text]?.let {
            if (it.singleton) return GuestType.Named(it.name)
            return fail("TypeMismatch", expression, "class names are constructor calls, not values")
        }
        topLevelValue(expression.text, mode).let { result -> if (result is SurfaceResult.Ok) return result.type }
        when (expression.text) {
            "Sequence" -> failUnsupported("Sequence", expression, "lazy data belongs to the lazy module")
            "javaClass", "KClass" -> failUnsupported("Reflection", expression, "reflection is outside the grammar")
        }
        if (expression.text == "it" && lambdaDepth > 0) {
            failUnsupported("ImplicitIt", expression, "lambda parameters are named")
        }
        return fail("UndeclaredName", expression, "undeclared name `${expression.text}`")
    }

    private fun resolveBindingOrNull(
        name: Name,
        scope: Scope,
    ): Binding? {
        var here: Scope? = scope
        var crossedLambda = false
        while (here != null) {
            val hit = here.bindings[name.text]
            if (hit != null) {
                if (crossedLambda && hit.mutable) hit.captured = true
                return hit
            }
            if (here.lambdaBoundary) crossedLambda = true
            here = here.parent
        }
        return null
    }

    context(r: Raise<AdmissionError>)
    private fun checkedBinding(
        name: Name,
        binding: Binding,
    ): GuestType {
        if (binding.kind == BindingKind.INITIALIZING) raiseRecursiveValLambda(name)
        return binding.type
    }

    context(r: Raise<AdmissionError>)
    private fun thisType(expression: This): GuestType {
        val owner = currentClass ?: failUnsupported("MiscKotlin", expression, "`this` outside member functions")
        return GuestType.Named(owner.name)
    }

    context(r: Raise<AdmissionError>)
    private fun callableType(
        expression: CallableReference,
        scope: Scope,
    ): GuestType {
        resolveBindingOrNull(expression.let { Name(it.name, it.span) }, scope)?.let { return it.type }
        functions[expression.name]?.let { return functionType(it) }
        if (expression.name in surfaceFunctionNames) {
            failUnsupported("MiscKotlin", expression, "callable references to library functions are outside the surface")
        }
        return fail("UndeclaredName", expression, "undeclared function `${expression.name}`")
    }

    context(r: Raise<AdmissionError>)
    private fun functionType(declaration: FunctionDecl): GuestType =
        GuestType.Function(
            declaration.parameters.map { resolveType(it.type, it) },
            declaration.result?.let { resolveType(it, declaration) } ?: T_UNIT,
        )

    // ---------- calls and members ----------

    context(r: Raise<AdmissionError>)
    private fun callType(
        expression: Call,
        scope: Scope,
        expected: GuestType?,
    ): GuestType {
        val callee = expression.callee
        if (expression.typeArguments.isNotEmpty()) {
            val builder = callee as? Name
            if (builder == null || builder.text !in BUILDER_TYPES) {
                failUnsupported("UserGenerics", expression, "type arguments are admitted only on collection builders")
            }
        }
        if (callee is Name) {
            resolveBindingOrNull(callee, scope)?.let { return applyValue(expression, it.type, scope) }
            properties[callee.text]?.let { return applyValue(expression, resolveType(it.property.type, it.property), scope) }
            functions[callee.text]?.let { return applyDeclared(expression, it, scope) }
            classes[callee.text]?.let { return constructorCall(expression, it, scope) }
            return surfaceTopLevel(expression, scope, expected)
        }
        if (callee is Member) return memberCallType(expression, callee, scope)
        return applyValue(expression, checkExpression(callee, scope, null), scope)
    }

    context(r: Raise<AdmissionError>)
    private fun constructorCall(
        expression: Call,
        info: ClassInfo,
        scope: Scope,
    ): GuestType {
        if (info.singleton) fail("TypeMismatch", expression, "data objects have no constructor call")
        if (expression.arguments.any { it.name != null }) {
            failUnsupported("NamedDefaultVarargs", expression, "named arguments are admitted only for data-class `copy`")
        }
        if (expression.arguments.size != info.properties.size) {
            fail("WrongArity", expression, "constructor takes ${info.properties.size} arguments")
        }
        for ((property, argument) in info.properties.zip(expression.arguments)) {
            val parameterType = resolveType(property.type, property)
            conformOrFail(checkExpression(argument.value, scope, parameterType), parameterType, argument, "argument type mismatch")
        }
        return GuestType.Named(info.name)
    }

    context(r: Raise<AdmissionError>)
    private fun applyDeclared(
        expression: Call,
        declaration: FunctionDecl,
        scope: Scope,
    ): GuestType {
        if (declaration.parameters.size != expression.arguments.size) {
            fail("WrongArity", expression, "`${declaration.name}` takes ${declaration.parameters.size} arguments")
        }
        for ((parameter, argument) in declaration.parameters.zip(expression.arguments)) {
            if (argument.name != null) {
                failUnsupported("NamedDefaultVarargs", argument, "named arguments are admitted only for data-class `copy`")
            }
            val parameterType = resolveType(parameter.type, parameter)
            conformOrFail(checkExpression(argument.value, scope, parameterType), parameterType, argument, "argument type mismatch")
        }
        return declaration.result?.let { resolveType(it, declaration) } ?: T_UNIT
    }

    context(r: Raise<AdmissionError>)
    private fun applyValue(
        expression: Call,
        calleeType: GuestType,
        scope: Scope,
    ): GuestType {
        val function = coreType(calleeType)
        if (function !is GuestType.Function) fail("TypeMismatch", expression.callee, "callee is not a function")
        if (function.parameters.size != expression.arguments.size) {
            fail("WrongArity", expression, "call takes ${function.parameters.size} arguments")
        }
        for ((parameter, argument) in function.parameters.zip(expression.arguments)) {
            if (argument.name != null) {
                failUnsupported("NamedDefaultVarargs", argument, "named arguments are admitted only for data-class `copy`")
            }
            conformOrFail(checkExpression(argument.value, scope, parameter), parameter, argument, "argument type mismatch")
        }
        return function.result
    }

    context(r: Raise<AdmissionError>)
    private fun surfaceTopLevel(
        expression: Call,
        scope: Scope,
        expected: GuestType?,
    ): GuestType {
        val callee = expression.callee as Name
        val target = builderTarget(expression, expected)
        if (target == null && callee.text in BUILDER_TYPES && expression.arguments.isEmpty()) {
            fail("TypeInference", expression, "empty collection builder needs an expected type or explicit type arguments")
        }
        val element =
            when (target?.name) {
                "Map", "MutableMap" -> tPair(target.arguments[0], target.arguments[1])
                null -> null
                else -> target.arguments[0]
            }
        val checked =
            checkArgumentsWithExpected(expression, scope) { prior, index ->
                element ?: topLevelParameterTypes(callee.text, expression.arguments.size, prior, index)
            }
        if (element != null) {
            for ((argument, actual) in expression.arguments.zip(checked)) {
                conformOrFail(actual, element, argument, "builder element does not conform")
            }
        }
        return when (val result = topLevelCall(callee.text, checked, mode)) {
            is SurfaceResult.Ok -> target ?: result.type
            is SurfaceResult.Bad -> fail("TypeMismatch", expression, result.message)
            is SurfaceResult.Outside -> failUnsupported(result.category, expression, result.message)
            SurfaceResult.Absent -> fail("UndeclaredName", expression, "undeclared name `${callee.text}`")
        }
    }

    context(r: Raise<AdmissionError>)
    private fun builderTarget(
        expression: Call,
        expected: GuestType?,
    ): GuestType.Named? {
        val name = (expression.callee as Name).text
        val resultName = BUILDER_TYPES[name] ?: return null
        val count = if (resultName == "Map" || resultName == "MutableMap") 2 else 1
        val explicit = expression.typeArguments.map { resolveType(it, expression) }
        if (explicit.isNotEmpty() && explicit.size != count) {
            fail("WrongArity", expression, "builder `$name` takes $count type arguments")
        }
        val expectedType = expected?.let(::coreType) as? GuestType.Named
        val contextual =
            expectedType != null && expectedType.arguments.size == count &&
                (
                    expectedType.name == resultName ||
                        (resultName == "MutableList" && (expectedType.name == "List" || expectedType.name == "Collection")) ||
                        ((resultName == "List" || resultName == "Set") && expectedType.name == "Collection") ||
                        (resultName == "MutableMap" && expectedType.name == "Map")
                )
        val arguments =
            when {
                explicit.isNotEmpty() -> explicit
                contextual -> expectedType.arguments
                else -> return null
            }
        return GuestType.Named(resultName, arguments)
    }

    context(r: Raise<AdmissionError>)
    private fun memberCallType(
        expression: Call,
        callee: Member,
        scope: Scope,
    ): GuestType {
        val receiver = checkExpression(callee.receiver, scope, null)
        val receiverCore = coreType(receiver)
        if (!callee.safe && receiver is GuestType.Nullable) fail("TypeMismatch", callee, "nullable receivers need `?.`")
        val info = classInfoOrNull(receiverCore)
        val result =
            if (info != null) {
                classMethodCall(expression, callee, receiverCore, info, scope)
            } else {
                surfaceMemberCall(expression, callee, receiverCore, scope)
            }
        return if (callee.safe) nullable(result) else result
    }

    context(r: Raise<AdmissionError>)
    private fun classMethodCall(
        expression: Call,
        callee: Member,
        receiver: GuestType,
        info: ClassInfo,
        scope: Scope,
    ): GuestType {
        if (info.data && callee.name == "copy") return copyCall(expression, info, scope)
        val method = info.methods[callee.name] ?: fail("UndeclaredName", callee, "undeclared member `${callee.name}`")
        return applyDeclared(expression, method, scope)
    }

    context(r: Raise<AdmissionError>)
    private fun copyCall(
        expression: Call,
        info: ClassInfo,
        scope: Scope,
    ): GuestType {
        val seen = mutableSetOf<String>()
        for (argument in expression.arguments) {
            val name = argument.name ?: failUnsupported("NamedDefaultVarargs", argument, "`copy` takes named arguments")
            if (!seen.add(name)) fail("DuplicateArgument", argument, "duplicate `copy` property `$name`")
            val property =
                info.properties.firstOrNull { it.name == name }
                    ?: fail("WrongArgumentType", argument, "`copy` has no property `$name`")
            val parameterType = resolveType(property.type, property)
            conformOrFail(checkExpression(argument.value, scope, parameterType), parameterType, argument, "argument type mismatch")
        }
        return GuestType.Named(info.name)
    }

    context(r: Raise<AdmissionError>)
    private fun surfaceMemberCall(
        expression: Call,
        callee: Member,
        receiver: GuestType,
        scope: Scope,
    ): GuestType {
        if (receiver !is GuestType.Named) fail("TypeMismatch", callee.receiver, "member call on a non-surface type")
        val checked =
            checkArgumentsWithExpected(expression, scope) { prior, index ->
                memberParameterTypes(receiver, callee.name, expression.arguments.size, prior, index)
            }
        return when (val result = memberCall(receiver, callee.name, checked)) {
            is SurfaceResult.Ok -> result.type
            is SurfaceResult.Bad -> fail("TypeMismatch", expression, result.message)
            is SurfaceResult.Outside -> failUnsupported(result.category, expression, result.message)
            SurfaceResult.Absent -> fail("UndeclaredName", callee, "undeclared member `${callee.name}`")
        }
    }

    /** Checks call arguments left to right, giving each its contextual
     * expected type; arguments are checked exactly once. */
    context(r: Raise<AdmissionError>)
    private fun checkArgumentsWithExpected(
        expression: Call,
        scope: Scope,
        expectedFor: (prior: List<GuestType>, index: Int) -> GuestType,
    ): List<GuestType> {
        val prior = mutableListOf<GuestType>()
        for ((index, argument) in expression.arguments.withIndex()) {
            if (argument.name != null) {
                failUnsupported("NamedDefaultVarargs", argument, "named arguments are admitted only for data-class `copy`")
            }
            prior.add(checkExpression(argument.value, scope, expectedFor(prior, index)))
        }
        return prior
    }

    context(r: Raise<AdmissionError>)
    private fun memberValueType(
        expression: Member,
        scope: Scope,
    ): GuestType {
        val receiver = checkExpression(expression.receiver, scope, null)
        if (receiver is GuestType.Nullable) {
            if (!expression.safe) fail("TypeMismatch", expression, "nullable receivers need `?.` or `?:`")
            return nullable(memberSelectType(coreType(receiver), expression))
        }
        val selected = memberSelectType(receiver, expression)
        return if (expression.safe) nullable(selected) else selected
    }

    context(r: Raise<AdmissionError>)
    private fun memberSelectType(
        receiver: GuestType,
        expression: Member,
    ): GuestType {
        val info = classInfoOrNull(receiver)
        if (info != null) {
            info.properties.firstOrNull { it.name == expression.name }?.let { return resolveType(it.type, it) }
            info.methods[expression.name]?.let { return functionType(it) }
            if (info.data && expression.name.startsWith("component")) {
                fail("TypeMismatch", expression, "component selectors are admitted behind destructuring")
            }
            return fail("UndeclaredName", expression, "undeclared member `${expression.name}`")
        }
        return when (val selected = memberSelect(receiver, expression.name)) {
            is SurfaceResult.Ok -> selected.type
            is SurfaceResult.Bad -> fail("TypeMismatch", expression, selected.message)
            is SurfaceResult.Outside -> failUnsupported(selected.category, expression, selected.message)
            SurfaceResult.Absent -> fail("UndeclaredName", expression, "undeclared member `${expression.name}`")
        }
    }

    context(r: Raise<AdmissionError>)
    private fun indexType(
        expression: Index,
        scope: Scope,
    ): GuestType {
        val receiver = checkExpression(expression.receiver, scope, null)
        if (receiver is GuestType.Nullable) fail("TypeMismatch", expression.receiver, "nullable receivers need `?.` or `?:`")
        val core = coreType(receiver)
        val index = checkExpression(expression.index, scope, null)
        if (core !is GuestType.Named) fail("TypeMismatch", expression.receiver, "indexing needs a collection")
        return when (core.name) {
            "List", "MutableList" -> {
                conformOrFail(index, T_INT, expression.index, "list indices are Int")
                core.arguments[0]
            }

            "Map", "MutableMap" -> {
                conformOrFail(index, core.arguments[0], expression.index, "key type mismatch")
                nullable(core.arguments[1])
            }

            "String" -> {
                failUnsupported("CharSurface", expression, "String.get")
            }

            else -> {
                fail("TypeMismatch", expression.receiver, "indexing needs a collection")
            }
        }
    }

    private companion object {
        private val BUILDER_TYPES =
            mapOf(
                "listOf" to "List",
                "emptyList" to "List",
                "mutableListOf" to "MutableList",
                "setOf" to "Set",
                "emptySet" to "Set",
                "mapOf" to "Map",
                "emptyMap" to "Map",
                "mutableMapOf" to "MutableMap",
            )
        private const val INT_MAX: String = "2147483647"
        private const val LONG_MAX: String = "9223372036854775807"
        private const val LONG_MIN_ABS: String = "9223372036854775808"
    }
}
