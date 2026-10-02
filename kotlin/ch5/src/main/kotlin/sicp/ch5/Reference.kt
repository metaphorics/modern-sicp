// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch5

import arrow.core.Either
import arrow.core.raise.either
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.Assignment
import sicp.guest.Binary
import sicp.guest.Block
import sicp.guest.Break
import sicp.guest.Call
import sicp.guest.CheckedProgram
import sicp.guest.Continue
import sicp.guest.Destructure
import sicp.guest.Elvis
import sicp.guest.Env
import sicp.guest.Expression
import sicp.guest.ExpressionStatement
import sicp.guest.For
import sicp.guest.FunctionDecl
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.GuestType
import sicp.guest.If
import sicp.guest.Index
import sicp.guest.Is
import sicp.guest.Lambda
import sicp.guest.Literal
import sicp.guest.LocalProperty
import sicp.guest.Member
import sicp.guest.Mode
import sicp.guest.NO_POSITION
import sicp.guest.Name
import sicp.guest.OutputSink
import sicp.guest.Primitives
import sicp.guest.Return
import sicp.guest.Span
import sicp.guest.Statement
import sicp.guest.StringTemplate
import sicp.guest.This
import sicp.guest.TopProperty
import sicp.guest.Unary
import sicp.guest.When
import sicp.guest.While
import sicp.guest.checkedLiteralValue
import sicp.guest.valueEquals

/**
 * Independent finite reference models for the experiment families of the
 * conformance contract. Each model evaluates the checked guest source with
 * its own semantics and never calls a teaching engine: lazy re-derives
 * memoized forcing, search re-derives choice and permanent assignment,
 * query re-derives assertion lookup over the query data, and the machine
 * model re-derives controller results and effects from the case's
 * controller description.
 */
private val QTERM_CLASSES = setOf("QSym", "QVar", "QList")
private val QQUERY_CLASSES = setOf("QPattern", "QAnd", "QOr", "QNot", "QGuard", "QUnique")

internal object ReferenceModels {
    /** The observation of one reference run: termination plus transcript. */
    class Observation(
        val termination: String,
        val transcript: String,
    )

    fun lazyCase(source: String): Either<AdmissionError, Observation> =
        either {
            val checked = Admission.admitOrRaise(source, Mode.LAZY)
            ReferenceEvaluator(checked, lazySemantics = true, searchSemantics = false).runMain()
        }

    fun searchCase(source: String): Either<AdmissionError, Observation> =
        either {
            val checked = Admission.admitOrRaise(source, Mode.SEARCH)
            ReferenceEvaluator(checked, lazySemantics = false, searchSemantics = true).runMain()
        }

    fun queryCase(source: String): Either<AdmissionError, Observation> =
        either {
            val checked = Admission.admitOrRaise(source, Mode.QUERY)
            ReferenceEvaluator(checked, lazySemantics = false, searchSemantics = false).runQuery()
        }

    /** The machine model: its own parse and execution of the controller
     * description, deriving effects and final state independently of the
     * section 4.4 machine. */
    fun machineCase(text: String): Observation = MachineModel.run(text)
}

/** One reference observation built from evaluation. */
private fun observationOf(
    sink: OutputSink,
    error: GuestError?,
): ReferenceModels.Observation =
    if (error == null) {
        ReferenceModels.Observation("value", sink.contents())
    } else {
        ReferenceModels.Observation("error", sink.contents())
    }

/**
 * A small independent evaluator over the checked syntax: its own
 * environments, its own lazy forcing and search backtracking, and the query
 * data constructors. It shares only the checked grammar and guest values
 * with the teaching engines.
 */
private class ReferenceEvaluator(
    private val checked: CheckedProgram,
    private val lazySemantics: Boolean,
    private val searchSemantics: Boolean,
    private val script: List<Int> = emptyList(),
) {
    private val sink = OutputSink()
    private val globals = Env.root()
    private val effects = mutableListOf<String>()

    sealed class Signal : RuntimeException(null, null, false, false)

    class ReturnSignal(
        val value: GValue,
    ) : Signal()

    class BreakSignal : Signal()

    class ContinueSignal : Signal()

    class FailSignal : Signal()

    /** Keep the guest-error value distinct from Kotlin control-flow signals. */
    private class RefFault(
        val guestError: GuestError,
    ) : RuntimeException(null, null, false, false)

    private var choiceVector: List<Int> = emptyList()

    private var pathCounts: MutableList<Int> = mutableListOf()

    private var chooseDepth = 0

    private var lastChooseSpan: Span = NO_POSITION

    private var attempts = 0

    fun runMain(): ReferenceModels.Observation {
        var error: GuestError? = null
        try {
            if (searchSemantics) runSearchLoop() else runOnce()
        } catch (fault: RefFault) {
            error = fault.guestError
        }
        return observationOf(sink, error)
    }

    /** The query cases (grammar 4.3): a fixture declares its facts, rules,
     * query, and variables as data through the [QUERY_ENTRIES] functions, and
     * the independent [QueryModel] answers it. */
    fun runQuery(): ReferenceModels.Observation =
        try {
            installDeclarations()
            val (facts, rules, query, variables) =
                QUERY_ENTRIES.map { name ->
                    val entry = globals.lookup(name)?.value ?: throw RefFault(GuestError.UnassignedRead(NO_POSITION))
                    applyValue(entry, emptyList(), NO_POSITION)
                }
            val guard = { predicate: GValue, terms: List<GValue> ->
                val verdict = applyValue(predicate, listOf(GValue.VList(terms.toMutableList(), false)), NO_POSITION)
                (verdict as? GValue.VBool)?.value ?: throw RefFault(GuestError.UnassignedRead(NO_POSITION))
            }
            val lines = QueryModel.answers(itemsOf(facts), itemsOf(rules), query, itemsOf(variables), guard)
            ReferenceModels.Observation("value", if (lines.isEmpty()) "" else lines.joinToString("\n") + "\n")
        } catch (fault: RefFault) {
            ReferenceModels.Observation("error", "")
        }

    private fun itemsOf(value: GValue): List<GValue> =
        (value as? GValue.VList)?.items ?: throw RefFault(GuestError.UnassignedRead(NO_POSITION))

    private fun runOnce() {
        installDeclarations()
        val main = globals.lookup("main")?.value
        if (main != null && main !is GValue.VUnassigned) applyValue(main, emptyList(), NO_POSITION)
    }

    private fun runSearchLoop() {
        choiceVector = emptyList()
        attempts = 0
        while (true) {
            attempts++
            if (attempts > 100000) throw RefFault(GuestError.UnassignedRead(lastChooseSpan))
            pathCounts = mutableListOf()
            chooseDepth = 0
            try {
                runOnce()
            } catch (_: FailSignal) {
            }
            val next = nextVector() ?: return
            choiceVector = next
        }
    }

    private fun nextVector(): List<Int>? {
        for (j in pathCounts.indices.reversed()) {
            val current = choiceVector.getOrElse(j) { 0 }
            if (current + 1 < pathCounts[j]) return List(j) { choiceVector.getOrElse(it) { 0 } } + (current + 1)
        }
        return null
    }

    private fun chooseRef(
        expression: Call,
        env: Env,
    ): GValue {
        val alternatives = expression.arguments.map { it.value }
        if (alternatives.isEmpty()) throw FailSignal()
        lastChooseSpan = expression.span
        val depth = chooseDepth++
        while (pathCounts.size <= depth) pathCounts.add(0)
        val index = choiceVector.getOrElse(depth) { 0 }
        if (index >= alternatives.size) throw FailSignal()
        pathCounts[depth] = alternatives.size
        return evalExpr(alternatives[index], env)
    }

    private fun installDeclarations() {
        for (declaration in checked.syntax.declarations) {
            if (declaration is FunctionDecl) {
                globals.define(declaration.name, closureOf(declaration, globals))
            }
            if (declaration is TopProperty) {
                globals.define(declaration.property.name, GValue.VUnassigned)
            }
        }
        for (declaration in checked.syntax.declarations) {
            if (declaration is TopProperty) {
                globals.define(declaration.property.name, evalExpr(declaration.initializer, globals))
            }
        }
    }

    private fun closureOf(
        declaration: FunctionDecl,
        env: Env,
    ): GValue =
        GValue.VFunction(declaration.name, declaration.parameters.size, delaysOf(declaration)) { arguments ->
            applyFunction(declaration, arguments, env)
        }

    private fun delaysOf(declaration: FunctionDecl): List<Boolean> =
        if (lazySemantics) declaration.parameters.map { it.annotation != "Strict" } else emptyList()

    private fun applyFunction(
        declaration: FunctionDecl,
        arguments: List<GValue>,
        env: Env,
    ): GValue {
        val frame = Env.child(env)
        for ((index, parameter) in declaration.parameters.withIndex()) {
            val value = arguments.getOrElse(index) { GValue.VUnit }
            frame.define(parameter.name, value)
        }
        return runBody(declaration.body, frame)
    }

    private fun runBody(
        body: Expression,
        env: Env,
    ): GValue =
        try {
            when (body) {
                is Block -> runBlockStatements(body.statements, env)
                else -> evalExpr(body, env)
            }
        } catch (signal: ReturnSignal) {
            signal.value
        }

    private fun runBlockStatements(
        statements: List<Statement>,
        env: Env,
    ): GValue {
        var result: GValue = GValue.VUnit
        for (statement in statements) result = runStatement(statement, env)
        return result
    }

    private fun runStatement(
        statement: Statement,
        env: Env,
    ): GValue =
        when (statement) {
            is ExpressionStatement -> {
                evalExpr(statement.expression, env)
            }

            is LocalProperty -> {
                env.define(statement.name, evalExpr(statement.initializer, env))
                GValue.VUnit
            }

            is Destructure -> {
                val source = evalExpr(statement.initializer, env)
                val values = destructureValues(source, statement.names.size)
                for ((index, name) in statement.names.withIndex()) env.define(name, values[index])
                GValue.VUnit
            }

            is Assignment -> {
                val value = evalExpr(statement.value, env)
                assignTarget(statement.target, value, env)
                value
            }

            is While -> {
                runWhile(statement, env)
            }

            is For -> {
                runFor(statement, env)
            }

            is Return -> {
                throw ReturnSignal(statement.value?.let { evalExpr(it, env) } ?: GValue.VUnit)
            }

            is Break -> {
                throw BreakSignal()
            }

            is Continue -> {
                throw ContinueSignal()
            }

            is FunctionDecl -> {
                env.define(statement.name, closureOf(statement, env))
                GValue.VUnit
            }
        }

    private fun destructureValues(
        source: GValue,
        count: Int,
    ): List<GValue> {
        if (source is GValue.VObject) {
            return source.fields.values
                .toList()
                .padTo(count)
        }
        if (source is GValue.VPair) return listOf(source.first, source.second).padTo(count)
        return List(count) { GValue.VUnit }
    }

    private fun List<GValue>.padTo(count: Int): List<GValue> = this + List(count - this.size) { GValue.VUnit }

    private fun assignTarget(
        target: Expression,
        value: GValue,
        env: Env,
    ) {
        if (target is Name) {
            val cell = env.lookup(target.text) ?: globals.lookup(target.text) ?: throw RefFault(GuestError.UnassignedRead(target.span))
            cell.value = value
            return
        }
        if (target is Member) {
            val receiver = evalExpr(target.receiver, env) as? GValue.VObject ?: throw RefFault(GuestError.UnassignedRead(target.span))
            receiver.fields[target.name] = value
            return
        }
        if (target is Index) {
            val receiver = evalExpr(target.receiver, env)
            val index = evalExpr(target.index, env)
            writeIndexValue(receiver, index, value, target.span)
            return
        }
        throw RefFault(GuestError.UnassignedRead(target.span))
    }

    private fun runWhile(
        statement: While,
        env: Env,
    ): GValue {
        while (truthy(evalExpr(statement.condition, env), statement.condition.span)) {
            try {
                runBlockStatements(statement.body.statements, Env.child(env))
            } catch (signal: BreakSignal) {
                return GValue.VUnit
            } catch (signal: ContinueSignal) {
                // re-test the condition
            }
        }
        return GValue.VUnit
    }

    private fun runFor(
        statement: For,
        env: Env,
    ): GValue {
        for (item in forItems(statement, env)) {
            val frame = Env.child(env)
            frame.define(statement.name, item)
            try {
                runBlockStatements(statement.body.statements, frame)
            } catch (signal: BreakSignal) {
                return GValue.VUnit
            } catch (signal: ContinueSignal) {
                // next item
            }
        }
        return GValue.VUnit
    }

    private fun forItems(
        statement: For,
        env: Env,
    ): Iterable<GValue> {
        val endExpression = statement.end
        if (endExpression != null) {
            val start = evalExpr(statement.iterable, env)
            val end = evalExpr(endExpression, env)
            return when {
                start is GValue.VInt && end is GValue.VInt -> (start.value..end.value).asSequence().map { GValue.VInt(it) }.asIterable()
                start is GValue.VLong && end is GValue.VLong -> (start.value..end.value).asSequence().map { GValue.VLong(it) }.asIterable()
                else -> throw RefFault(GuestError.UnassignedRead(statement.span))
            }
        }
        val source = evalExpr(statement.iterable, env)
        if (source is GValue.VList) return source.items.toList()
        throw RefFault(GuestError.UnassignedRead(statement.span))
    }

    private fun evalExpr(
        expression: Expression,
        env: Env,
    ): GValue =
        when (expression) {
            is Literal -> {
                checkedLiteralValue(expression, checked.types[expression])
            }

            is Name -> {
                readName(expression.text, expression.span, env)
            }

            is StringTemplate -> {
                templateValue(expression, env)
            }

            is Binary -> {
                binaryValue(expression, env)
            }

            is Unary -> {
                unaryValue(expression, env)
            }

            is Elvis -> {
                val left = evalExpr(expression.left, env)
                if (left is GValue.VNull) evalExpr(expression.right, env) else left
            }

            is Is -> {
                GValue.VBool(typeMatches(forceIfNeeded(evalExpr(expression.value, env)), expression.type, expression.negated))
            }

            is If -> {
                ifValue(expression, env)
            }

            is When -> {
                whenValue(expression, env)
            }

            is Lambda -> {
                lambdaValue(expression, env)
            }

            is Call -> {
                callValue(expression, env)
            }

            is Member -> {
                memberValue(expression, env)
            }

            is Index -> {
                indexValue(expression, env)
            }

            is Block -> {
                runBlockStatements(expression.statements, Env.child(env))
            }

            is Return -> {
                throw ReturnSignal(expression.value?.let { evalExpr(it, env) } ?: GValue.VUnit)
            }

            is This -> {
                readName("this", expression.span, env)
            }

            is sicp.guest.CallableReference -> {
                readName(expression.name, expression.span, env)
            }
        }

    private fun readName(
        name: String,
        span: Span,
        env: Env,
    ): GValue {
        if (name == "lazyEnd") return GValue.VList(mutableListOf(), mutable = false)
        val cell = env.lookup(name) ?: globals.lookup(name) ?: throw RefFault(GuestError.UnassignedRead(span))
        return forceIfNeeded(cell.value)
    }

    private fun forceIfNeeded(value: GValue): GValue {
        if (!lazySemantics) return value
        if (value !is GValue.VThunk) return value
        return either<GuestError, GValue> { sicp.guest.forceThunk(value) }.fold({ throw RefFault(it) }, { it })
    }

    private fun templateValue(
        template: StringTemplate,
        env: Env,
    ): GValue {
        val out = StringBuilder()
        for (fragment in template.fragments) {
            val rendered =
                sicp.guest.renderPrinted(forceIfNeeded(evalExpr(fragment, env)))
                    ?: throw RefFault(GuestError.UnassignedRead(template.span))
            out.append(rendered)
        }
        return GValue.VString(out.toString())
    }

    private fun binaryValue(
        expression: Binary,
        env: Env,
    ): GValue {
        if (expression.operator == "&&") {
            val left = truthy(evalExpr(expression.left, env), expression.left.span)
            if (!left) return GValue.VBool(false)
            return GValue.VBool(truthy(evalExpr(expression.right, env), expression.right.span))
        }
        if (expression.operator == "||") {
            val left = truthy(evalExpr(expression.left, env), expression.left.span)
            if (left) return GValue.VBool(true)
            return GValue.VBool(truthy(evalExpr(expression.right, env), expression.right.span))
        }
        val left = forceIfNeeded(evalExpr(expression.left, env))
        val right = forceIfNeeded(evalExpr(expression.right, env))
        return applyArithmetic(expression.operator, left, right, expression.span)
    }

    private fun applyArithmetic(
        operator: String,
        left: GValue,
        right: GValue,
        span: Span,
    ): GValue {
        if (left is GValue.VLong && right is GValue.VLong) return longArithmetic(operator, left, right, span)
        if (left is GValue.VInt && right is GValue.VInt) return intArithmetic(operator, left, right, span)
        if (left is GValue.VDouble && right is GValue.VDouble) return doubleArithmetic(operator, left.value, right.value, span)
        if (operator == "==") return GValue.VBool(structuralEquals(left, right))
        if (operator == "+" && left is GValue.VList && right is GValue.VList) {
            return GValue.VList((left.items + right.items).toMutableList(), false)
        }
        if (operator == "!=") return GValue.VBool(!structuralEquals(left, right))
        if (operator == "+") return GValue.VString(stringify(left) + stringify(right))
        throw RefFault(GuestError.UnassignedRead(span))
    }

    private fun longArithmetic(
        operator: String,
        left: GValue.VLong,
        right: GValue.VLong,
        span: Span,
    ): GValue =
        when (operator) {
            "+" -> GValue.VLong(left.value + right.value)
            "-" -> GValue.VLong(left.value - right.value)
            "*" -> GValue.VLong(left.value * right.value)
            "/" -> GValue.VLong(left.value / right.value)
            "%" -> GValue.VLong(left.value % right.value)
            "<" -> GValue.VBool(left.value < right.value)
            ">" -> GValue.VBool(left.value > right.value)
            "<=" -> GValue.VBool(left.value <= right.value)
            ">=" -> GValue.VBool(left.value >= right.value)
            "==" -> GValue.VBool(left.value == right.value)
            "!=" -> GValue.VBool(left.value != right.value)
            else -> throw RefFault(GuestError.UnassignedRead(span))
        }

    private fun intArithmetic(
        operator: String,
        left: GValue.VInt,
        right: GValue.VInt,
        span: Span,
    ): GValue =
        when (operator) {
            "+" -> GValue.VInt(left.value + right.value)
            "-" -> GValue.VInt(left.value - right.value)
            "*" -> GValue.VInt(left.value * right.value)
            "/" -> GValue.VInt(left.value / right.value)
            "%" -> GValue.VInt(left.value % right.value)
            "<" -> GValue.VBool(left.value < right.value)
            ">" -> GValue.VBool(left.value > right.value)
            "<=" -> GValue.VBool(left.value <= right.value)
            ">=" -> GValue.VBool(left.value >= right.value)
            "==" -> GValue.VBool(left.value == right.value)
            "!=" -> GValue.VBool(left.value != right.value)
            else -> throw RefFault(GuestError.UnassignedRead(span))
        }

    private fun doubleArithmetic(
        operator: String,
        left: Double,
        right: Double,
        span: Span,
    ): GValue =
        when (operator) {
            "+" -> GValue.VDouble(left + right)
            "-" -> GValue.VDouble(left - right)
            "*" -> GValue.VDouble(left * right)
            "/" -> GValue.VDouble(left / right)
            "<" -> GValue.VBool(left < right)
            ">" -> GValue.VBool(left > right)
            "<=" -> GValue.VBool(left <= right)
            ">=" -> GValue.VBool(left >= right)
            "==" -> GValue.VBool(left == right)
            "!=" -> GValue.VBool(left != right)
            else -> throw RefFault(GuestError.UnassignedRead(span))
        }

    private fun unaryValue(
        expression: Unary,
        env: Env,
    ): GValue {
        val operand = forceIfNeeded(evalExpr(expression.operand, env))
        if (expression.operator == "!") return GValue.VBool(!truthy(operand, expression.span))
        if (expression.operator == "-") {
            if (operand is GValue.VLong) return GValue.VLong(-operand.value)
            if (operand is GValue.VInt) return GValue.VInt(-operand.value)
            if (operand is GValue.VDouble) return GValue.VDouble(-operand.value)
        }
        throw RefFault(GuestError.UnassignedRead(expression.span))
    }

    private fun ifValue(
        expression: If,
        env: Env,
    ): GValue {
        val condition = truthy(evalExpr(expression.condition, env), expression.condition.span)
        if (condition) return evalExpr(expression.yes, env)
        val alternative = expression.no ?: return GValue.VUnit
        return evalExpr(alternative, env)
    }

    private fun whenValue(
        expression: When,
        env: Env,
    ): GValue {
        val subject = expression.subject?.let { forceIfNeeded(evalExpr(it, env)) }
        for (branch in expression.branches) {
            if (branchMatches(branch, subject, env)) return evalExpr(branch.body, env)
        }
        val otherwise = expression.otherwise ?: return GValue.VUnit
        return evalExpr(otherwise, env)
    }

    private fun branchMatches(
        branch: sicp.guest.WhenBranch,
        subject: GValue?,
        env: Env,
    ): Boolean {
        val typePattern = branch.typePattern
        if (typePattern != null) return subject != null && typeMatches(subject, typePattern, negated = false)
        val pattern = branch.pattern ?: return false
        val value = forceIfNeeded(evalExpr(pattern, env))
        if (subject == null) return truthy(value, pattern.span)
        return structuralEquals(value, subject)
    }

    /** `value is type`: a nullable type also admits `null`; a class name
     * matches objects of that class, and a query family matches its variants. */
    private fun typeMatches(
        value: GValue,
        type: GuestType,
        negated: Boolean,
    ): Boolean {
        val base = (type as? GuestType.Nullable)?.base ?: type
        val name = (base as? GuestType.Named)?.name
        val matched =
            when {
                type is GuestType.Nullable && value is GValue.VNull -> true
                name == "QTerm" -> value is GValue.VObject && value.className in QTERM_CLASSES
                name == "QQuery" -> value is GValue.VObject && value.className in QQUERY_CLASSES
                name == "String" -> value is GValue.VString
                name == "Long" -> value is GValue.VLong
                name == "Int" -> value is GValue.VInt
                name == "Double" -> value is GValue.VDouble
                name == "Boolean" -> value is GValue.VBool
                name == "List" -> value is GValue.VList
                else -> value is GValue.VObject && value.className == name
            }
        return matched != negated
    }

    private fun lambdaValue(
        expression: Lambda,
        env: Env,
    ): GValue {
        val names = expression.parameters.map { it.name }
        val delays = if (lazySemantics) expression.parameters.map { true } else emptyList()
        return GValue.VFunction(null, names.size, delays) { arguments ->
            val frame = Env.child(env)
            for ((index, name) in names.withIndex()) frame.define(name, arguments.getOrElse(index) { GValue.VUnit })
            runBody(expression.body, frame)
        }
    }

    private fun callValue(
        expression: Call,
        env: Env,
    ): GValue {
        val callee = expression.callee
        if (searchSemantics && callee is Name && callee.text == "choose") return chooseRef(expression, env)
        val intrinsic = builtinCall(callee, expression, env)
        if (intrinsic != null) return intrinsic
        val built = builtInClass(callee, expression, env)
        if (built != null) return built
        if (callee is Member) {
            val receiver = forceIfNeeded(evalExpr(callee.receiver, env))
            if (callee.safe && receiver is GValue.VNull) return GValue.VNull
            val arguments = expression.arguments.map { evalExpr(it.value, env) }
            return memberCall(receiver, callee.name, arguments, expression.span)
        }
        val target = evalExpr(callee, env)
        val delays = if (lazySemantics) (forceIfNeeded(target) as? GValue.VFunction)?.delays ?: emptyList() else emptyList()
        val arguments =
            expression.arguments.mapIndexed { index, argument ->
                if (delays.getOrElse(index) { false }) {
                    GValue.VThunk(sicp.guest.ThunkState.Delayed({ _ -> evalExpr(argument.value, env) }), transparent = true)
                } else {
                    evalExpr(argument.value, env)
                }
            }
        return applyValue(target, arguments, expression.span)
    }

    /** The intrinsic call surface of the experiment corpora, evaluated by
     * this model's own semantics (never a teaching engine). */
    private fun builtinCall(
        callee: Expression,
        expression: Call,
        env: Env,
    ): GValue? {
        if (callee !is Name ||
            callee.text !in setOf("println", "print", "listOf", "emptyList", "setOf", "thunk", "force", "lazyPair", "demand")
        ) {
            return null
        }
        val arguments = expression.arguments.map { evalExpr(it.value, env) }
        return when (callee.text) {
            "println", "print" -> {
                if (arguments.isEmpty() && callee.text == "println") {
                    sink.write("\n")
                } else {
                    val rendered =
                        sicp.guest.renderPrinted(arguments.firstOrNull() ?: GValue.VUnit)
                            ?: throw RefFault(GuestError.UnassignedRead(callee.span))
                    sink.write(rendered)
                    if (callee.text == "println") sink.write("\n")
                }
                GValue.VUnit
            }

            "listOf" -> {
                GValue.VList(arguments.map { forceIfNeeded(it) }.toMutableList(), false)
            }

            "emptyList" -> {
                GValue.VList(mutableListOf(), false)
            }

            "setOf" -> {
                GValue.VList(arguments.map { forceIfNeeded(it) }.toMutableList(), false, asSet = true)
            }

            "thunk" -> {
                val body = arguments.getOrElse(0) { GValue.VUnit }
                GValue.VThunk(sicp.guest.ThunkState.Delayed({ _ -> applyValue(body, emptyList(), callee.span) }), transparent = false)
            }

            "force" -> {
                forceIfNeeded(arguments.getOrElse(0) { GValue.VUnit })
            }

            "lazyPair" -> {
                val tail = arguments.getOrElse(1) { GValue.VUnit }
                GValue.VLazyList(
                    forceIfNeeded(arguments.getOrElse(0) { GValue.VUnit }),
                    tail as? GValue.VThunk ?: GValue.VThunk(sicp.guest.ThunkState.Forced(tail)),
                )
            }

            "demand" -> {
                if (truthy(arguments.getOrElse(0) { GValue.VBool(false) }, callee.span)) GValue.VUnit else throw FailSignal()
            }

            else -> {
                null
            }
        }
    }

    private fun builtInClass(
        callee: Expression,
        expression: Call,
        env: Env,
    ): GValue? {
        if (callee !is Name) return null
        if (env.lookup(callee.text) != null || globals.lookup(callee.text) != null) return null
        val arguments = expression.arguments.map { evalExpr(it.value, env) }
        return constructQClass(callee.text, arguments, expression.span)
    }

    private fun constructQClass(
        name: String,
        arguments: List<GValue>,
        span: Span,
    ): GValue {
        val fields = linkedMapOf<String, GValue>()
        when (name) {
            "QSym", "QVar" -> {
                fields["name"] = arguments.getOrElse(0) { GValue.VNull }
            }

            "QPattern", "QFact" -> {
                fields["term"] = arguments.getOrElse(0) { GValue.VNull }
            }

            "QList" -> {
                fields["items"] = arguments.getOrElse(0) { GValue.VList(mutableListOf(), false) }
                fields["tail"] = arguments.getOrElse(1) { GValue.VNull }
            }

            "QAnd", "QOr" -> {
                fields["parts"] = arguments.getOrElse(0) { GValue.VList(mutableListOf(), false) }
            }

            "QNot", "QUnique" -> {
                fields["part"] = arguments.getOrElse(0) { GValue.VNull }
            }

            "QGuard" -> {
                fields["predicate"] = arguments.getOrElse(0) { GValue.VNull }
                fields["args"] = arguments.getOrElse(1) { GValue.VList(mutableListOf(), false) }
            }

            "QRule" -> {
                fields["conclusion"] = arguments.getOrElse(0) { GValue.VNull }
                fields["body"] = arguments.getOrElse(1) { GValue.VNull }
            }

            "QFrame" -> {
                fields["bindings"] = arguments.getOrElse(0) { GValue.VMap(linkedMapOf(), false) }
            }

            else -> {
                throw RefFault(GuestError.UnassignedRead(span))
            }
        }
        return GValue.VObject(name, structural = true, fields)
    }

    private fun applyValue(
        target: GValue,
        arguments: List<GValue>,
        span: Span,
    ): GValue {
        val function = forceIfNeeded(target) as? GValue.VFunction ?: throw RefFault(GuestError.UnassignedRead(span))
        return either<GuestError, GValue> { function.apply(this, arguments) }.fold({ throw RefFault(it) }, { it })
    }

    private fun memberValue(
        expression: Member,
        env: Env,
    ): GValue {
        val receiver = forceIfNeeded(evalExpr(expression.receiver, env))
        if (expression.safe && receiver is GValue.VNull) return GValue.VNull
        return memberCall(receiver, expression.name, emptyList(), expression.span)
    }

    private fun memberCall(
        receiver: GValue,
        name: String,
        arguments: List<GValue>,
        span: Span,
    ): GValue {
        if (receiver is GValue.VObject) {
            val field = receiver.fields[name]
            if (field != null) return forceIfNeeded(field)
        }
        if (receiver is GValue.VLazyList) return lazyMember(receiver, name, arguments, span)
        if (receiver is GValue.VList) return listMember(receiver, name, arguments, span)
        if (receiver is GValue.VString) return stringMember(receiver, name, span)
        if (receiver is GValue.VLong) return scalarMember(receiver.value.toDouble(), name)
        if (receiver is GValue.VInt) return scalarMember(receiver.value.toDouble(), name)
        throw RefFault(GuestError.UnassignedRead(span))
    }

    /** `get` walks only as many tails as the index needs; every other member
     * sees the materialized list. */
    private fun lazyMember(
        receiver: GValue.VLazyList,
        name: String,
        arguments: List<GValue>,
        span: Span,
    ): GValue {
        if (name != "get") return listMember(materializeLazy(receiver, span), name, arguments, span)
        val index = arguments.firstOrNull() as? GValue.VInt ?: throw RefFault(GuestError.UnassignedRead(span))
        var node: GValue = receiver
        var remaining = index.value
        while (node is GValue.VLazyList) {
            if (remaining == 0) return node.head
            remaining--
            node = forceTail(node.tail)
        }
        val rest = node as? GValue.VList ?: throw RefFault(GuestError.UnassignedRead(span))
        return rest.items.getOrElse(remaining) { throw RefFault(GuestError.IndexOutOfBounds(span)) }
    }

    private fun materializeLazy(
        receiver: GValue.VLazyList,
        span: Span,
    ): GValue.VList {
        val items = mutableListOf<GValue>()
        var node: GValue = receiver
        while (node is GValue.VLazyList) {
            items.add(node.head)
            node = forceTail(node.tail)
        }
        val rest = node as? GValue.VList ?: throw RefFault(GuestError.UnassignedRead(span))
        items.addAll(rest.items)
        return GValue.VList(items, false)
    }

    private fun forceTail(tail: GValue.VThunk): GValue =
        either<GuestError, GValue> { sicp.guest.forceThunk(tail) }.fold({ throw RefFault(it) }, { it })

    private fun listMember(
        receiver: GValue.VList,
        name: String,
        arguments: List<GValue>,
        span: Span,
    ): GValue =
        when (name) {
            "size" -> {
                GValue.VInt(receiver.items.size)
            }

            "get" -> {
                val index = arguments.firstOrNull() as? GValue.VInt ?: throw RefFault(GuestError.UnassignedRead(span))
                receiver.items.getOrElse(index.value) { throw RefFault(GuestError.IndexOutOfBounds(span)) }
            }

            "plus" -> {
                GValue.VList((receiver.items + arguments.flatMap { (it as? GValue.VList)?.items ?: listOf(it) }).toMutableList(), false)
            }

            "contains" -> {
                GValue.VBool(receiver.items.any { structuralEquals(it, arguments.firstOrNull() ?: GValue.VUnit) })
            }

            else -> {
                throw RefFault(GuestError.UnassignedRead(span))
            }
        }

    private fun stringMember(
        receiver: GValue.VString,
        name: String,
        span: Span,
    ): GValue =
        when (name) {
            "length" -> GValue.VInt(receiver.value.length)
            "toLong" -> GValue.VLong(receiver.value.toLong())
            else -> throw RefFault(GuestError.UnassignedRead(span))
        }

    private fun scalarMember(
        value: Double,
        name: String,
    ): GValue =
        when (name) {
            "toLong" -> GValue.VLong(value.toLong())
            "toDouble" -> GValue.VDouble(value)
            "toInt" -> GValue.VInt(value.toInt())
            else -> GValue.VDouble(value)
        }

    private fun indexValue(
        expression: Index,
        env: Env,
    ): GValue {
        val receiver = forceIfNeeded(evalExpr(expression.receiver, env))
        val index = forceIfNeeded(evalExpr(expression.index, env))
        return readIndexValue(receiver, index, expression.span)
    }

    private fun readIndexValue(
        receiver: GValue,
        index: GValue,
        span: Span,
    ): GValue {
        if (receiver is GValue.VList &&
            index is GValue.VInt
        ) {
            return receiver.items.getOrElse(index.value) { throw RefFault(GuestError.IndexOutOfBounds(span)) }
        }
        if (receiver is GValue.VMap) {
            return either<GuestError, GValue> { Primitives.readIndex(receiver, index, span) }.fold({ throw RefFault(it) }, { it })
        }
        throw RefFault(GuestError.UnassignedRead(span))
    }

    private fun writeIndexValue(
        receiver: GValue,
        index: GValue,
        value: GValue,
        span: Span,
    ) {
        if (receiver is GValue.VMap) {
            either<GuestError, Unit> { Primitives.writeIndex(receiver, index, value, span) }.fold({ throw RefFault(it) }, { it })
            return
        }
        throw RefFault(GuestError.UnassignedRead(span))
    }

    private fun truthy(
        value: GValue,
        span: Span,
    ): Boolean {
        val forced = forceIfNeeded(value)
        if (forced is GValue.VBool) return forced.value
        throw RefFault(GuestError.UnassignedRead(span))
    }

    private fun structuralEquals(
        left: GValue,
        right: GValue,
    ): Boolean = either<GuestError, Boolean> { valueEquals(left, right) }.fold({ throw RefFault(it) }, { it })

    private fun stringify(value: GValue): String =
        when (value) {
            is GValue.VString -> value.value
            is GValue.VLong -> value.value.toString()
            is GValue.VInt -> value.value.toString()
            is GValue.VDouble -> value.value.toString()
            is GValue.VBool -> value.value.toString()
            else -> ""
        }
}

/** The independent machine model: parse the controller description and
 * derive effects and final state with its own execution loop. */
internal object MachineModel {
    private class State(
        val registers: LinkedHashMap<String, Long>,
        val effects: MutableList<String>,
        val stack: ArrayDeque<Long>,
    ) {
        var testFlag: Boolean = false
    }

    fun run(text: String): ReferenceModels.Observation {
        val registers = linkedMapOf<String, Long>()
        val instructions = mutableListOf<List<String>>()
        val labels = linkedMapOf<String, Int>()
        for (rawLine in text.lineSequence()) {
            val line = rawLine.trim()
            if (line.isEmpty() || line.startsWith("#")) continue
            if (line.startsWith("registers")) {
                for (name in line.removePrefix("registers").trim().split(Regex("\\s+"))) registers[name] = 0L
                continue
            }
            if (line.startsWith("const")) {
                val parts = line.removePrefix("const").trim().split(Regex("\\s+"))
                registers[parts[0]] = parts[1].toLong()
                continue
            }
            if (line.endsWith(":")) {
                labels[line.dropLast(1)] = instructions.size
                continue
            }
            instructions.add(line.split(Regex("\\s+")))
        }
        val state = State(registers, mutableListOf(), ArrayDeque())
        val halted = executeLoop(instructions, labels, state)
        val finals = registers.entries.joinToString(" ") { "${it.key}=${it.value}" }
        val transcript = (state.effects + "final: $finals").joinToString("\n") + "\n"
        if (!halted) return ReferenceModels.Observation("error", transcript)
        return ReferenceModels.Observation("value", transcript)
    }

    private fun executeLoop(
        instructions: List<List<String>>,
        labels: Map<String, Int>,
        state: State,
    ): Boolean {
        var pc = 0
        var guard = 0
        while (pc < instructions.size && guard < 1000000) {
            guard++
            val words = instructions[pc]
            pc = step(words, labels, state, pc)
        }
        return guard < 1000000
    }

    private fun step(
        words: List<String>,
        labels: Map<String, Int>,
        state: State,
        pc: Int,
    ): Int {
        if (words[0] == "assign") return assignStep(words, state, pc)
        if (words[0] == "test") return testStep(words, state, pc)
        if (words[0] == "goto") return labels[words[1]] ?: pc + 1
        if (words[0] == "branch") {
            if (!state.testFlag) return pc + 1
            return labels[words[1]] ?: pc + 1
        }
        if (words[0] == "save") {
            state.stack.addLast(state.registers[words[1]] ?: 0L)
            return pc + 1
        }
        if (words[0] == "restore") {
            state.registers[words[1]] = state.stack.removeLastOrNull() ?: 0L
            return pc + 1
        }
        if (words[0] == "perform") return performStep(words, state, pc)
        return pc + 1
    }

    private fun assignStep(
        words: List<String>,
        state: State,
        pc: Int,
    ): Int {
        val target = words[1]
        val source = words.subList(3, words.size)
        state.registers[target] = sourceValue(source, state)
        return pc + 1
    }

    private fun testStep(
        words: List<String>,
        state: State,
        pc: Int,
    ): Int {
        val name = words[1]
        val operands = words.subList(2, words.size)
        state.testFlag =
            when (name) {
                "zero" -> operand(operands[0], state) == 0L
                "eq" -> operand(operands[0], state) == operand(operands[1], state)
                "lt" -> operand(operands[0], state) < operand(operands[1], state)
                else -> false
            }
        return pc + 1
    }

    private fun performStep(
        words: List<String>,
        state: State,
        pc: Int,
    ): Int {
        val name = words[1]
        val operands = words.subList(2, words.size)
        if (name == "print") state.effects.add(operand(operands[0], state).toString())
        return pc + 1
    }

    private fun sourceValue(
        source: List<String>,
        state: State,
    ): Long {
        if (source[0] == "const") return source[1].toLong()
        if (source[0] == "reg") return state.registers[source[1]] ?: 0L
        if (source[0] != "op") return 0L
        val left = operand(source[2], state)
        val right = if (source.size > 3) operand(source[3], state) else 0L
        return applyOp(source[1], left, right)
    }

    private fun operand(
        word: String,
        state: State,
    ): Long {
        if (word.startsWith("#")) return word.drop(1).toLong()
        return state.registers[word] ?: 0L
    }

    private fun applyOp(
        name: String,
        left: Long,
        right: Long,
    ): Long =
        when (name) {
            "add" -> left + right
            "sub" -> left - right
            "mul" -> left * right
            "rem" -> left % right
            else -> left
        }
}

/** The functions a query case declares its data through, in the order the
 * models read them: facts, rules, the query, and the variables to report. */
internal val QUERY_ENTRIES: List<String> = listOf("facts", "rules", "query", "variables")

/**
 * The independent query model: its own bindings, unifier with an occurs
 * check, rule renaming, and answer order, over the guest values of the
 * section 4.3 constructors. It answers exactly what the teaching driver is
 * pinned to answer -- facts before rules in database order, conjunction by
 * successive extension, disjunction taking one answer from each branch in
 * turn, `not` and `unique` as filters -- and never calls it.
 */
internal object QueryModel {
    private typealias Bindings = Map<String, GValue>

    fun answers(
        facts: List<GValue>,
        rules: List<GValue>,
        query: GValue,
        variables: List<GValue>,
        guard: (GValue, List<GValue>) -> Boolean,
    ): List<String> {
        val model = Model(facts.map { field(it, "term") }, rules.map { field(it, "conclusion") to field(it, "body") }, guard)
        return model
            .solve(query, emptyMap())
            .flatMap { bindings -> variables.map { "?${name(it)} = ${render(resolve(it, bindings))}" } }
            .toList()
    }

    private class Model(
        val facts: List<GValue>,
        val rules: List<Pair<GValue, GValue>>,
        val guard: (GValue, List<GValue>) -> Boolean,
    ) {
        private var renamings = 0

        fun solve(
            query: GValue,
            bindings: Bindings,
        ): Sequence<Bindings> =
            when (kind(query)) {
                "QPattern" -> pattern(field(query, "term"), bindings)
                "QAnd" -> items(query, "parts").fold(sequenceOf(bindings)) { frames, part -> frames.flatMap { solve(part, it) } }
                "QOr" -> turns(items(query, "parts").map { solve(it, bindings).iterator() })
                "QNot" -> sequence { if (solve(field(query, "part"), bindings).none()) yield(bindings) }
                "QUnique" -> only(solve(field(query, "part"), bindings))
                "QGuard" -> sequence { if (holds(query, bindings)) yield(bindings) }
                else -> throw IllegalStateException("not a query: ${kind(query)}")
            }

        private fun pattern(
            term: GValue,
            bindings: Bindings,
        ): Sequence<Bindings> =
            sequence {
                for (fact in facts) unify(term, fact, bindings)?.let { yield(it) }
                for ((conclusion, body) in rules) {
                    val suffix = "#${renamings++}"
                    val entered = unify(term, rename(conclusion, suffix), bindings) ?: continue
                    yieldAll(solve(renameQuery(body, suffix), entered))
                }
            }

        private fun holds(
            guardQuery: GValue,
            bindings: Bindings,
        ): Boolean {
            val arguments = items(guardQuery, "args").map { resolve(it, bindings) }
            return arguments.none { hasVariable(it) } && guard(field(guardQuery, "predicate"), arguments)
        }

        private fun only(frames: Sequence<Bindings>): Sequence<Bindings> =
            sequence {
                val matches = frames.iterator()
                if (!matches.hasNext()) return@sequence
                val one = matches.next()
                if (!matches.hasNext()) yield(one)
            }

        private fun turns(streams: List<Iterator<Bindings>>): Sequence<Bindings> =
            sequence {
                val waiting = ArrayDeque(streams)
                while (waiting.isNotEmpty()) {
                    val stream = waiting.removeFirst()
                    if (!stream.hasNext()) continue
                    yield(stream.next())
                    waiting.addLast(stream)
                }
            }

        private fun rename(
            term: GValue,
            suffix: String,
        ): GValue =
            when (kind(term)) {
                "QVar" -> variable(name(term) + suffix)
                "QList" -> list(items(term, "items").map { rename(it, suffix) }, tailOf(term)?.let { rename(it, suffix) })
                else -> term
            }

        private fun renameQuery(
            query: GValue,
            suffix: String,
        ): GValue =
            when (kind(query)) {
                "QPattern" -> {
                    node("QPattern", "term" to rename(field(query, "term"), suffix))
                }

                "QAnd", "QOr" -> {
                    node(kind(query), "parts" to listValue(items(query, "parts").map { renameQuery(it, suffix) }))
                }

                "QNot", "QUnique" -> {
                    node(kind(query), "part" to renameQuery(field(query, "part"), suffix))
                }

                else -> {
                    val args = listValue(items(query, "args").map { rename(it, suffix) })
                    node("QGuard", "predicate" to field(query, "predicate"), "args" to args)
                }
            }
    }

    private fun kind(value: GValue): String =
        (value as? GValue.VObject)?.className ?: throw IllegalStateException("not a query datum: $value")

    private fun field(
        value: GValue,
        name: String,
    ): GValue = (value as GValue.VObject).fields.getValue(name)

    private fun items(
        value: GValue,
        name: String,
    ): List<GValue> = (field(value, name) as GValue.VList).items

    private fun name(value: GValue): String = (field(value, "name") as GValue.VString).value

    private fun tailOf(term: GValue): GValue? = field(term, "tail").takeUnless { it is GValue.VNull }

    private fun listValue(values: List<GValue>): GValue = GValue.VList(values.toMutableList(), false)

    private fun node(
        className: String,
        vararg fields: Pair<String, GValue>,
    ): GValue = GValue.VObject(className, true, linkedMapOf(*fields))

    private fun variable(name: String): GValue = node("QVar", "name" to GValue.VString(name))

    private fun list(
        items: List<GValue>,
        tail: GValue?,
    ): GValue = node("QList", "items" to listValue(items), "tail" to (tail ?: GValue.VNull))

    private fun walk(
        term: GValue,
        bindings: Bindings,
    ): GValue {
        var current = term
        while (kind(current) == "QVar") current = bindings[name(current)] ?: return current
        return current
    }

    private fun bind(
        variable: GValue,
        term: GValue,
        bindings: Bindings,
    ): Bindings? = if (occurs(name(variable), term, bindings)) null else bindings + (name(variable) to term)

    private fun occurs(
        variable: String,
        term: GValue,
        bindings: Bindings,
    ): Boolean {
        val here = walk(term, bindings)
        return when (kind(here)) {
            "QVar" -> {
                name(here) == variable
            }

            "QList" -> {
                items(here, "items").any { occurs(variable, it, bindings) } ||
                    tailOf(here)?.let { occurs(variable, it, bindings) } == true
            }

            else -> {
                false
            }
        }
    }

    private fun unify(
        left: GValue,
        right: GValue,
        bindings: Bindings,
    ): Bindings? {
        val a = walk(left, bindings)
        val b = walk(right, bindings)
        return when {
            kind(a) == "QVar" && kind(b) == "QVar" && name(a) == name(b) -> bindings
            kind(a) == "QVar" -> bind(a, b, bindings)
            kind(b) == "QVar" -> bind(b, a, bindings)
            kind(a) == "QSym" && kind(b) == "QSym" -> bindings.takeIf { name(a) == name(b) }
            kind(a) == "QList" && kind(b) == "QList" -> unifyLists(a, b, bindings)
            else -> null
        }
    }

    private fun unifyLists(
        a: GValue,
        b: GValue,
        bindings: Bindings,
    ): Bindings? {
        val left = items(a, "items")
        val right = items(b, "items")
        val common = minOf(left.size, right.size)
        var current = bindings
        for (index in 0 until common) current = unify(left[index], right[index], current) ?: return null
        val leftTail = tailOf(a)
        val rightTail = tailOf(b)
        return when {
            left.size > common -> rightTail?.let { unify(it, list(left.drop(common), leftTail), current) }
            right.size > common -> leftTail?.let { unify(it, list(right.drop(common), rightTail), current) }
            leftTail == null && rightTail == null -> current
            else -> unify(leftTail ?: list(emptyList(), null), rightTail ?: list(emptyList(), null), current)
        }
    }

    /** The term with every bound variable replaced, a list whose tail
     * resolves to a list spliced into one list. */
    private fun resolve(
        term: GValue,
        bindings: Bindings,
    ): GValue {
        val here = walk(term, bindings)
        if (kind(here) != "QList") return here
        val head = items(here, "items").map { resolve(it, bindings) }
        val tail = tailOf(here)?.let { resolve(it, bindings) }
        return when {
            tail == null -> list(head, null)
            kind(tail) == "QList" -> list(head + items(tail, "items"), tailOf(tail))
            head.isEmpty() -> tail
            else -> list(head, tail)
        }
    }

    private fun hasVariable(term: GValue): Boolean =
        when (kind(term)) {
            "QVar" -> true
            "QList" -> items(term, "items").any(::hasVariable) || tailOf(term)?.let(::hasVariable) == true
            else -> false
        }

    private fun render(term: GValue): String =
        when (kind(term)) {
            "QSym" -> {
                name(term)
            }

            "QVar" -> {
                "?" + name(term)
            }

            else -> {
                val tail = tailOf(term)
                val shown = items(term, "items").joinToString(", ") { render(it) }
                if (tail == null) "[$shown]" else "[$shown | ${render(tail)}]"
            }
        }
}
