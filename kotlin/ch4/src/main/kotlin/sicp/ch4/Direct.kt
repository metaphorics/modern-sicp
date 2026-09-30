// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch4

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.Assignment
import sicp.guest.Binary
import sicp.guest.Block
import sicp.guest.Break
import sicp.guest.Call
import sicp.guest.CallableReference
import sicp.guest.CheckedProgram
import sicp.guest.Continue
import sicp.guest.DataClass
import sicp.guest.DataObject
import sicp.guest.Destructure
import sicp.guest.Elvis
import sicp.guest.Env
import sicp.guest.Expression
import sicp.guest.ExpressionStatement
import sicp.guest.For
import sicp.guest.FunctionDecl
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.If
import sicp.guest.Index
import sicp.guest.Is
import sicp.guest.Lambda
import sicp.guest.LambdaParameter
import sicp.guest.Literal
import sicp.guest.LocalProperty
import sicp.guest.Member
import sicp.guest.Mode
import sicp.guest.NO_POSITION
import sicp.guest.Name
import sicp.guest.OutputSink
import sicp.guest.Parameter
import sicp.guest.PlainClass
import sicp.guest.Primitives
import sicp.guest.Program
import sicp.guest.Property
import sicp.guest.Return
import sicp.guest.RunResult
import sicp.guest.SealedInterface
import sicp.guest.Statement
import sicp.guest.StringTemplate
import sicp.guest.This
import sicp.guest.TopProperty
import sicp.guest.Unary
import sicp.guest.When
import sicp.guest.WhenBranch
import sicp.guest.While
import sicp.guest.checkedLiteralValue
import sicp.guest.valueEquals

/** Internal control-flow signals of the guest language: `return`, `break`,
 * and `continue` (sections 2.3 and 3.6). They are control flow, never errors,
 * and never cross the engine boundary. */
internal class ReturnSignal(
    val value: GValue,
) : RuntimeException(null, null, false, false)

internal class BreakSignal : RuntimeException(null, null, false, false)

internal class ContinueSignal : RuntimeException(null, null, false, false)

/** The direct evaluator of section 4.1 over the shared checked syntax. */
public object Direct {
    /** Admits and runs [source]; admission failures never execute code. */
    public fun run(
        source: String,
        mode: Mode = Mode.CORE,
    ): Either<AdmissionError, RunResult> = either { execute(Admission.admitOrRaise(source, mode)) }

    /** Runs checked source; supplying a sink lets an interpreted closure
     * retain the same effect stream across a compiled caller (5.47). */
    public fun run(
        checked: CheckedProgram,
        sink: OutputSink = OutputSink(),
    ): RunResult = execute(checked, sink)

    /** Evaluates the named zero-parameter top-level functions of [checked]
     * in order and answers their values. The query cases declare their data
     * (facts, rules, query) as such functions (grammar 4.3), so an engine can
     * read the domain data without running `main`. */
    public fun values(
        checked: CheckedProgram,
        names: List<String>,
    ): Either<GuestError, List<GValue>> = either { Evaluator(checked, OutputSink()).callEach(names) }

    internal fun execute(
        checked: CheckedProgram,
        sink: OutputSink = OutputSink(),
    ): RunResult {
        val cursor = sink.mark()
        val outcome: Either<GuestError, GValue> = either { Evaluator(checked, sink).runMain() }
        return outcome.fold(
            { error -> RunResult(sink.since(cursor), error, null) },
            { value -> RunResult(sink.since(cursor), null, value) },
        )
    }
}

/** The environment evaluator. The lazy module extends it through the
 * [evaluateArguments], [exposeValue], [surfaceValue], and [delaysFor] hooks,
 * keeping core behavior identical across modes. */
internal open class Evaluator(
    private val checked: CheckedProgram,
    protected val sink: OutputSink,
) {
    internal class ClassShape(
        val name: String,
        val data: Boolean,
        val singleton: Boolean,
        val properties: List<Property>,
        val methods: Map<String, FunctionDecl>,
    )

    protected val globals: Env = Env.root()
    protected val classTable = linkedMapOf<String, ClassShape>()

    context(r: Raise<GuestError>)
    fun runMain(): GValue {
        installDeclarations(checked.syntax)
        val main = globals.lookup("main")?.value
        if (main !is GValue.VFunction) return GValue.VUnit
        return Primitives.invoke(main, emptyList(), NO_POSITION)
    }

    context(r: Raise<GuestError>)
    fun callEach(names: List<String>): List<GValue> {
        installDeclarations(checked.syntax)
        return names.map { name ->
            val function = globals.lookup(name)?.value as? GValue.VFunction ?: r.raise(GuestError.UnassignedRead(NO_POSITION))
            Primitives.invoke(function, emptyList(), NO_POSITION)
        }
    }

    context(r: Raise<GuestError>)
    private fun installDeclarations(program: Program) {
        for (declaration in checked.vocabulary + program.declarations) {
            when (declaration) {
                is SealedInterface -> {
                    classTable[declaration.name] = ClassShape(declaration.name, false, false, emptyList(), emptyMap())
                }

                is DataClass -> {
                    classTable[declaration.name] = ClassShape(declaration.name, true, false, declaration.properties, emptyMap())
                }

                is PlainClass -> {
                    classTable[declaration.name] =
                        ClassShape(declaration.name, false, false, declaration.properties, declaration.methods.associateBy { it.name })
                }

                is DataObject -> {
                    classTable[declaration.name] = ClassShape(declaration.name, true, true, emptyList(), emptyMap())
                    globals.define(declaration.name, GValue.VObject(declaration.name, structural = true, fields = mutableMapOf()))
                }

                else -> {}
            }
        }
        for (declaration in program.declarations) {
            if (declaration is FunctionDecl) globals.define(declaration.name, functionValue(declaration, globals))
        }
        for (declaration in program.declarations) {
            if (declaration is TopProperty) {
                globals.define(declaration.property.name, eval(declaration.initializer, globals))
            }
        }
    }

    /** Core parameters are strict; the lazy module delays what it may. */
    protected open fun delaysFor(parameters: List<Parameter>): List<Boolean> = parameters.map { false }

    protected open fun delaysForLambda(parameters: List<LambdaParameter>): List<Boolean> = parameters.map { false }

    context(r: Raise<GuestError>)
    protected fun functionValue(
        declaration: FunctionDecl,
        definingEnv: Env,
    ): GValue.VFunction =
        GValue.VFunction(declaration.name, declaration.parameters.size, delaysFor(declaration.parameters)) { arguments ->
            callFunction(declaration, arguments, null, definingEnv)
        }

    context(r: Raise<GuestError>)
    protected fun methodValue(
        receiver: GValue,
        declaration: FunctionDecl,
    ): GValue.VFunction =
        GValue.VFunction(declaration.name, declaration.parameters.size, delaysFor(declaration.parameters)) { arguments ->
            callFunction(declaration, arguments, receiver, globals)
        }

    context(r: Raise<GuestError>)
    private fun callFunction(
        declaration: FunctionDecl,
        arguments: List<GValue>,
        thisValue: GValue?,
        definingEnv: Env,
    ): GValue {
        val env = Env.child(definingEnv)
        if (thisValue != null) env.define("this", thisValue)
        for ((parameter, value) in declaration.parameters.zip(arguments)) env.define(parameter.name, value)
        val body = declaration.body
        if (body !is Block) {
            return try {
                eval(body, env)
            } catch (signal: ReturnSignal) {
                signal.value
            }
        }
        return runBody(body.statements, env)
    }

    context(r: Raise<GuestError>)
    protected fun runBody(
        statements: List<Statement>,
        env: Env,
    ): GValue =
        try {
            var result: GValue = GValue.VUnit
            for (statement in statements) result = execStatement(statement, env)
            result
        } catch (signal: ReturnSignal) {
            signal.value
        }

    context(r: Raise<GuestError>)
    protected fun lambdaValue(
        body: List<Statement>,
        parameters: List<LambdaParameter>,
        env: Env,
    ): GValue.VFunction {
        val names = parameters.map { it.name }
        return GValue.VFunction(null, names.size, delaysForLambda(parameters)) { arguments ->
            val frame = Env.child(env)
            for ((name, value) in names.zip(arguments)) frame.define(name, value)
            runBody(body, frame)
        }
    }

    /** Argument evaluation; the lazy module wraps delayed parameters. */
    context(r: Raise<GuestError>)
    protected open fun evaluateArguments(
        target: GValue?,
        expression: Call,
        env: Env,
    ): List<GValue> = expression.arguments.map { eval(it.value, env) }

    /** Name exposure; the lazy module forces transparent parameter thunks. */
    context(r: Raise<GuestError>)
    protected open fun exposeValue(
        value: GValue,
        at: Expression,
    ): GValue = value

    /** Mode-only surface values (`lazyEnd`); null means ordinary lookup. */
    protected open fun surfaceValue(name: String): GValue? = null

    // ---------- expressions ----------

    context(r: Raise<GuestError>)
    protected fun eval(
        expression: Expression,
        env: Env,
    ): GValue =
        when (expression) {
            is Literal -> {
                checkedLiteralValue(expression, checked.types[expression])
            }

            is Name -> {
                readName(expression.text, expression, env)
            }

            is StringTemplate -> {
                templateValue(expression, env)
            }

            is Binary -> {
                binaryValue(expression, env)
            }

            is Unary -> {
                Primitives.unary(expression.operator, eval(expression.operand, env), expression.span)
            }

            is Elvis -> {
                elvisValue(expression, env)
            }

            is Is -> {
                val subject = eval(expression.value, env)
                GValue.VBool(Primitives.isTypeValue(subject, expression.type) != expression.negated)
            }

            is If -> {
                ifValue(expression, env)
            }

            is When -> {
                whenValue(expression, env)
            }

            is Lambda -> {
                lambdaValue(expression.body.statements, expression.parameters, env)
            }

            is Call -> {
                callValue(expression, env)
            }

            is Member -> {
                memberValue(expression, env)
            }

            is Index -> {
                Primitives.readIndex(eval(expression.receiver, env), eval(expression.index, env), expression.span)
            }

            is Block -> {
                blockValue(expression, env)
            }

            is Return -> {
                throw ReturnSignal(expression.value?.let { eval(it, env) } ?: GValue.VUnit)
            }

            is This -> {
                readName("this", expression, env)
            }

            is CallableReference -> {
                readName(expression.name, expression, env)
            }
        }

    context(r: Raise<GuestError>)
    protected fun readName(
        name: String,
        at: Expression,
        env: Env,
    ): GValue {
        val cell = env.lookup(name) ?: globals.lookup(name)
        if (cell == null) {
            surfaceValue(name)?.let { return it }
            r.raise(GuestError.UnassignedRead(at.span))
        }
        val value = cell.value
        if (value is GValue.VUnassigned) r.raise(GuestError.UnassignedRead(at.span))
        return exposeValue(value, at)
    }

    context(r: Raise<GuestError>)
    private fun templateValue(
        template: StringTemplate,
        env: Env,
    ): GValue {
        val out = StringBuilder()
        for (fragment in template.fragments) {
            out.append(sicp.guest.renderPrinted(eval(fragment, env)) ?: r.raise(GuestError.UnassignedRead(template.span)))
        }
        return GValue.VString(out.toString())
    }

    context(r: Raise<GuestError>)
    private fun binaryValue(
        expression: Binary,
        env: Env,
    ): GValue {
        if (expression.operator == "&&") {
            val left = Primitives.truth(eval(expression.left, env), expression.left.span)
            if (!left) return GValue.VBool(false)
            return GValue.VBool(Primitives.truth(eval(expression.right, env), expression.right.span))
        }
        if (expression.operator == "||") {
            val left = Primitives.truth(eval(expression.left, env), expression.left.span)
            if (left) return GValue.VBool(true)
            return GValue.VBool(Primitives.truth(eval(expression.right, env), expression.right.span))
        }
        val left = eval(expression.left, env)
        val right = eval(expression.right, env)
        return Primitives.binary(expression.operator, left, right, expression.span)
    }

    context(r: Raise<GuestError>)
    private fun elvisValue(
        expression: Elvis,
        env: Env,
    ): GValue {
        val left = eval(expression.left, env)
        if (left is GValue.VNull) return eval(expression.right, env)
        return left
    }

    context(r: Raise<GuestError>)
    private fun ifValue(
        expression: If,
        env: Env,
    ): GValue {
        val condition = Primitives.truth(eval(expression.condition, env), expression.condition.span)
        if (condition) return eval(expression.yes, env)
        expression.no?.let { return eval(it, env) }
        return GValue.VUnit
    }

    context(r: Raise<GuestError>)
    private fun whenValue(
        expression: When,
        env: Env,
    ): GValue {
        val subject = expression.subject?.let { eval(it, env) }
        for (branch in expression.branches) {
            if (branchMatches(branch, subject, env)) return eval(branch.body, env)
        }
        expression.otherwise?.let { return eval(it, env) }
        return GValue.VUnit
    }

    context(r: Raise<GuestError>)
    private fun branchMatches(
        branch: WhenBranch,
        subject: GValue?,
        env: Env,
    ): Boolean {
        val typePattern = branch.typePattern
        if (typePattern != null) {
            if (subject == null) return false
            return Primitives.isTypeValue(subject, typePattern)
        }
        val pattern = branch.pattern ?: return false
        if (subject == null) return Primitives.truth(eval(pattern, env), pattern.span)
        return valueEquals(eval(pattern, env), subject)
    }

    context(r: Raise<GuestError>)
    protected fun blockValue(
        block: Block,
        env: Env,
    ): GValue {
        if (block in checked.lambdaCoercions) {
            return lambdaValue(block.statements, emptyList(), env)
        }
        val frame = Env.child(env)
        var result: GValue = GValue.VUnit
        for (statement in block.statements) result = execStatement(statement, frame)
        return result
    }

    // ---------- calls and members ----------

    context(r: Raise<GuestError>)
    private fun callValue(
        expression: Call,
        env: Env,
    ): GValue {
        val callee = expression.callee
        if (callee is Name) return namedCall(callee, expression, env)
        if (callee is Member) return memberCall(callee, expression, env)
        val fn = eval(callee, env)
        return Primitives.invoke(fn, evaluateArguments(fn, expression, env), expression.span)
    }

    context(r: Raise<GuestError>)
    protected open fun namedCall(
        callee: Name,
        expression: Call,
        env: Env,
    ): GValue {
        val bound = env.lookup(callee.text) ?: globals.lookup(callee.text)
        if (bound != null && bound.value !is GValue.VUnassigned) {
            val fn = exposeValue(bound.value, callee)
            return Primitives.invoke(fn, evaluateArguments(fn, expression, env), expression.span)
        }
        val shape = classTable[callee.text]
        if (shape != null) return construct(shape, expression, env)
        return Primitives.call(callee.text, evaluateArguments(null, expression, env), sink, expression.span)
    }

    context(r: Raise<GuestError>)
    private fun construct(
        shape: ClassShape,
        expression: Call,
        env: Env,
    ): GValue {
        if (shape.singleton) r.raise(GuestError.UnassignedRead(expression.span))
        val fields = linkedMapOf<String, GValue>()
        for ((property, argument) in shape.properties.zip(expression.arguments)) {
            fields[property.name] = eval(argument.value, env)
        }
        return GValue.VObject(shape.name, structural = shape.data, fields)
    }

    context(r: Raise<GuestError>)
    private fun memberCall(
        callee: Member,
        expression: Call,
        env: Env,
    ): GValue {
        val receiver = eval(callee.receiver, env)
        if (callee.safe && receiver is GValue.VNull) return GValue.VNull
        if (callee.name == "copy" && receiver is GValue.VObject) return copyObject(receiver, expression, env)
        val shape = (receiver as? GValue.VObject)?.let { classTable[it.className] }
        val method = shape?.methods?.get(callee.name)
        if (method != null) {
            val fn = methodValue(receiver, method)
            return fn.apply(r, evaluateArguments(fn, expression, env))
        }
        return Primitives.member(receiver, callee.name, evaluateArguments(null, expression, env), expression.span)
    }

    context(r: Raise<GuestError>)
    private fun copyObject(
        receiver: GValue.VObject,
        expression: Call,
        env: Env,
    ): GValue {
        val fields = LinkedHashMap(receiver.fields)
        for (argument in expression.arguments) {
            val name = argument.name ?: r.raise(GuestError.UnassignedRead(argument.span))
            fields[name] = eval(argument.value, env)
        }
        return GValue.VObject(receiver.className, structural = true, fields)
    }

    context(r: Raise<GuestError>)
    private fun memberValue(
        expression: Member,
        env: Env,
    ): GValue {
        val receiver = eval(expression.receiver, env)
        if (expression.safe && receiver is GValue.VNull) return GValue.VNull
        if (receiver is GValue.VObject) {
            receiver.fields[expression.name]?.let { return exposeValue(it, expression) }
            val method = classTable[receiver.className]?.methods?.get(expression.name)
            if (method != null) return methodValue(receiver, method)
        }
        return Primitives.property(receiver, expression.name, expression.span)
    }

    // ---------- statements ----------

    context(r: Raise<GuestError>)
    protected fun execStatement(
        statement: Statement,
        env: Env,
    ): GValue =
        when (statement) {
            is ExpressionStatement -> {
                eval(statement.expression, env)
            }

            is LocalProperty -> {
                env.define(statement.name, eval(statement.initializer, env))
                GValue.VUnit
            }

            is Destructure -> {
                destructureInto(statement, env)
                GValue.VUnit
            }

            is Assignment -> {
                assign(statement, env)
                GValue.VUnit
            }

            is While -> {
                runWhile(statement, env)
                GValue.VUnit
            }

            is For -> {
                runFor(statement, env)
                GValue.VUnit
            }

            is Return -> {
                throw ReturnSignal(statement.value?.let { eval(it, env) } ?: GValue.VUnit)
            }

            is Break -> {
                throw BreakSignal()
            }

            is Continue -> {
                throw ContinueSignal()
            }

            is FunctionDecl -> {
                env.define(statement.name, functionValue(statement, env))
                GValue.VUnit
            }
        }

    context(r: Raise<GuestError>)
    private fun destructureInto(
        statement: Destructure,
        env: Env,
    ) {
        val source = eval(statement.initializer, env)
        if (source is GValue.VPair) {
            bindDestructure(statement.names, listOf(source.first, source.second), env)
            return
        }
        val fields = (source as? GValue.VObject)?.fields ?: r.raise(GuestError.UnassignedRead(statement.span))
        bindDestructure(statement.names, fields.values.toList(), env)
    }

    private fun bindDestructure(
        names: List<String>,
        values: List<GValue>,
        env: Env,
    ) {
        for ((name, value) in names.zip(values)) env.define(name, value)
    }

    context(r: Raise<GuestError>)
    private fun assign(
        statement: Assignment,
        env: Env,
    ) {
        val target = statement.target
        if (target is Name) {
            val cell = env.lookup(target.text) ?: globals.lookup(target.text) ?: r.raise(GuestError.UnassignedRead(target.span))
            cell.value = eval(statement.value, env)
            return
        }
        if (target is Member) {
            val receiver = eval(target.receiver, env) as? GValue.VObject ?: r.raise(GuestError.UnassignedRead(target.span))
            receiver.fields[target.name] = eval(statement.value, env)
            return
        }
        if (target is Index) {
            Primitives.writeIndex(eval(target.receiver, env), eval(target.index, env), eval(statement.value, env), statement.span)
            return
        }
        r.raise(GuestError.UnassignedRead(statement.span))
    }

    context(r: Raise<GuestError>)
    private fun runWhile(
        statement: While,
        env: Env,
    ) {
        while (Primitives.truth(eval(statement.condition, env), statement.condition.span)) {
            try {
                val frame = Env.child(env)
                for (inner in statement.body.statements) execStatement(inner, frame)
            } catch (_: BreakSignal) {
                return
            } catch (_: ContinueSignal) {
                // re-test the condition
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun runFor(
        statement: For,
        env: Env,
    ) {
        val loopEnv = Env.child(env)
        for (item in iterableValues(statement, env)) {
            loopEnv.define(statement.name, item)
            try {
                val frame = Env.child(loopEnv)
                for (inner in statement.body.statements) execStatement(inner, frame)
            } catch (_: BreakSignal) {
                return
            } catch (_: ContinueSignal) {
                // next item
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun iterableValues(
        statement: For,
        env: Env,
    ): Iterable<GValue> {
        val endExpression = statement.end
        if (endExpression != null) {
            val start = eval(statement.iterable, env)
            val end = eval(endExpression, env)
            return Primitives.rangeValues(start, end, statement.span).asIterable()
        }
        val iterable = eval(statement.iterable, env)
        val list =
            when (iterable) {
                is GValue.VList -> iterable.items.toList()
                is GValue.VLazyList -> (sicp.guest.materialize(iterable) as GValue.VList).items.toList()
                else -> r.raise(GuestError.UnassignedRead(statement.span))
            }
        return list
    }
}
