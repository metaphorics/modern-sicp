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
import sicp.guest.Literal
import sicp.guest.LocalProperty
import sicp.guest.Member
import sicp.guest.Mode
import sicp.guest.NO_POSITION
import sicp.guest.Name
import sicp.guest.OutputSink
import sicp.guest.PlainClass
import sicp.guest.Primitive
import sicp.guest.Primitives
import sicp.guest.Program
import sicp.guest.Property
import sicp.guest.Return
import sicp.guest.RunResult
import sicp.guest.SealedInterface
import sicp.guest.Span
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

/** The analyzer of section 4.1.7: analysis turns syntax into execution
 * procedures once; execution then runs closures without redispatching on
 * syntax. Observables match the direct evaluator exactly. */
public object Analyzed {
    /** Admits, analyzes, and runs [source]; admission failures execute nothing. */
    public fun run(
        source: String,
        mode: Mode = Mode.CORE,
    ): Either<AdmissionError, RunResult> = either { execute(Admission.admitOrRaise(source, mode)) }

    /** Analyzes and runs an already-checked program. */
    public fun run(checked: CheckedProgram): RunResult = execute(checked)

    private fun execute(checked: CheckedProgram): RunResult {
        val sink = OutputSink()
        val outcome: Either<GuestError, GValue> = either { Analysis(checked, sink).runMain() }
        return outcome.fold(
            { error -> RunResult(sink.contents(), error, null) },
            { value -> RunResult(sink.contents(), null, value) },
        )
    }
}

/** One execution procedure: run against an environment, yield a value. */
private typealias Exec = (Env) -> GValue

internal class Analysis(
    private val checked: CheckedProgram,
    private val sink: OutputSink,
) {
    private val globals = Env.root()
    private val classTable = linkedMapOf<String, ClassShape>()

    private class ClassShape(
        val name: String,
        val data: Boolean,
        val singleton: Boolean,
        val properties: List<Property>,
        val methods: Map<String, FunctionDecl>,
        val analyzedMethods: Map<String, Exec>,
    )

    context(r: Raise<GuestError>)
    fun runMain(): GValue {
        installDeclarations(checked.syntax)
        val main = globals.lookup("main")?.value
        if (main !is GValue.VFunction) return GValue.VUnit
        return Primitives.invoke(main, emptyList(), NO_POSITION)
    }

    context(r: Raise<GuestError>)
    private fun installDeclarations(program: Program) {
        for (declaration in checked.vocabulary + program.declarations) {
            when (declaration) {
                is SealedInterface -> {
                    classTable[declaration.name] = ClassShape(declaration.name, false, false, emptyList(), emptyMap(), emptyMap())
                }

                is DataClass -> {
                    classTable[declaration.name] = ClassShape(declaration.name, true, false, declaration.properties, emptyMap(), emptyMap())
                }

                is PlainClass -> {
                    classTable[declaration.name] =
                        ClassShape(
                            declaration.name,
                            false,
                            false,
                            declaration.properties,
                            declaration.methods.associateBy { it.name },
                            declaration.methods.associate { it.name to analyzeBody(it) },
                        )
                }

                is DataObject -> {
                    classTable[declaration.name] = ClassShape(declaration.name, true, true, emptyList(), emptyMap(), emptyMap())
                    globals.define(declaration.name, GValue.VObject(declaration.name, structural = true, fields = mutableMapOf()))
                }

                else -> {}
            }
        }
        val analyzedFunctions = mutableMapOf<String, Exec>()
        for (declaration in program.declarations) {
            if (declaration is FunctionDecl) {
                val bodyExec = analyzeBody(declaration)
                analyzedFunctions[declaration.name] = bodyExec
                globals.define(declaration.name, functionValue(declaration, bodyExec, globals))
            }
        }
        for (declaration in program.declarations) {
            if (declaration is TopProperty) {
                globals.define(declaration.property.name, analyze(declaration.initializer)(globals))
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun analyzeBody(declaration: FunctionDecl): Exec {
        val body = declaration.body
        if (body !is Block) {
            val bodyExec = analyze(body)
            return { env ->
                try {
                    bodyExec(env)
                } catch (signal: ReturnSignal) {
                    signal.value
                }
            }
        }
        val statements = body.statements.map { analyzeStatement(it) }
        return { env -> runStatements(statements, env) }
    }

    context(r: Raise<GuestError>)
    private fun functionValue(
        declaration: FunctionDecl,
        bodyExec: Exec,
        definingEnv: Env,
    ): GValue.VFunction =
        GValue.VFunction(declaration.name, declaration.parameters.size) { arguments ->
            val env = Env.child(definingEnv)
            for ((parameter, value) in declaration.parameters.zip(arguments)) env.define(parameter.name, value)
            bodyExec(env)
        }

    context(r: Raise<GuestError>)
    private fun methodValue(
        receiver: GValue,
        declaration: FunctionDecl,
        bodyExec: Exec,
    ): GValue.VFunction {
        return GValue.VFunction(declaration.name, declaration.parameters.size) { arguments ->
            val env = Env.child(globals)
            env.define("this", receiver)
            for ((parameter, value) in declaration.parameters.zip(arguments)) env.define(parameter.name, value)
            bodyExec(env)
        }
    }

    context(r: Raise<GuestError>)
    private fun runStatements(
        statements: List<Exec>,
        env: Env,
    ): GValue =
        try {
            var result: GValue = GValue.VUnit
            for (statement in statements) result = statement(env)
            result
        } catch (signal: ReturnSignal) {
            signal.value
        }

    // ---------- analysis ----------

    context(r: Raise<GuestError>)
    private fun analyze(expression: Expression): Exec =
        when (expression) {
            is Literal -> {
                literalExec(expression)
            }

            is Name -> {
                nameExec(expression)
            }

            is StringTemplate -> {
                templateExec(expression)
            }

            is Binary -> {
                binaryExec(expression)
            }

            is Unary -> {
                val operand = analyze(expression.operand)
                val exec: Exec = { env -> Primitives.unary(expression.operator, operand(env), expression.span) }
                exec
            }

            is Elvis -> {
                elvisExec(expression)
            }

            is Is -> {
                val subject = analyze(expression.value)
                val exec: Exec = { env -> GValue.VBool(Primitives.isTypeValue(subject(env), expression.type) != expression.negated) }
                exec
            }

            is If -> {
                ifExec(expression)
            }

            is When -> {
                whenExec(expression)
            }

            is Lambda -> {
                lambdaExec(expression)
            }

            is Call -> {
                callExec(expression)
            }

            is Member -> {
                memberExec(expression)
            }

            is Index -> {
                val receiver = analyze(expression.receiver)
                val index = analyze(expression.index)
                val exec: Exec = { env -> Primitives.readIndex(receiver(env), index(env), expression.span) }
                exec
            }

            is Block -> {
                blockExec(expression)
            }

            is Return -> {
                val value = expression.value?.let { analyze(it) }
                val exec: Exec = { env -> throw ReturnSignal(value?.invoke(env) ?: GValue.VUnit) }
                exec
            }

            is This -> {
                nameExec(Name("this", expression.span))
            }

            is CallableReference -> {
                nameExec(Name(expression.name, expression.span))
            }
        }

    private fun literalExec(literal: Literal): Exec {
        val value: GValue = checkedLiteralValue(literal, checked.types[literal])
        return { _ -> value }
    }

    context(r: Raise<GuestError>)
    private fun nameExec(name: Name): Exec =
        { env ->
            val cell = env.lookup(name.text) ?: globals.lookup(name.text) ?: r.raise(GuestError.UnassignedRead(name.span))
            val value = cell.value
            if (value is GValue.VUnassigned) r.raise(GuestError.UnassignedRead(name.span))
            value
        }

    context(r: Raise<GuestError>)
    private fun templateExec(template: StringTemplate): Exec {
        val fragments = template.fragments.map { analyze(it) }
        return { env ->
            val out = StringBuilder()
            for (fragment in fragments) {
                out.append(sicp.guest.renderPrinted(fragment(env)) ?: r.raise(GuestError.UnassignedRead(template.span)))
            }
            GValue.VString(out.toString())
        }
    }

    context(r: Raise<GuestError>)
    private fun binaryExec(expression: Binary): Exec {
        val left = analyze(expression.left)
        val right = analyze(expression.right)
        return { env ->
            when (expression.operator) {
                "&&" -> {
                    if (!Primitives.truth(left(env), expression.left.span)) {
                        GValue.VBool(false)
                    } else {
                        GValue.VBool(Primitives.truth(right(env), expression.right.span))
                    }
                }

                "||" -> {
                    if (Primitives.truth(left(env), expression.left.span)) {
                        GValue.VBool(true)
                    } else {
                        GValue.VBool(Primitives.truth(right(env), expression.right.span))
                    }
                }

                else -> {
                    Primitives.binary(expression.operator, left(env), right(env), expression.span)
                }
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun elvisExec(expression: Elvis): Exec {
        val left = analyze(expression.left)
        val right = analyze(expression.right)
        return { env ->
            val value = left(env)
            if (value is GValue.VNull) right(env) else value
        }
    }

    context(r: Raise<GuestError>)
    private fun ifExec(expression: If): Exec {
        val condition = analyze(expression.condition)
        val yes = analyze(expression.yes)
        val no = expression.no?.let { analyze(it) }
        return { env ->
            if (Primitives.truth(condition(env), expression.condition.span)) yes(env) else no?.invoke(env) ?: GValue.VUnit
        }
    }

    context(r: Raise<GuestError>)
    private fun whenExec(expression: When): Exec {
        val subject = expression.subject?.let { analyze(it) }
        val branches =
            expression.branches.map { branch ->
                Triple(branch, branch.pattern?.let { analyze(it) }, analyze(branch.body))
            }
        val otherwise = expression.otherwise?.let { analyze(it) }
        return { env ->
            val subjectValue = subject?.invoke(env)
            var answer: GValue? = null
            for ((branch, patternExec, body) in branches) {
                if (branchMatches(branch, patternExec, subjectValue, env)) {
                    answer = body(env)
                    break
                }
            }
            answer ?: otherwise?.invoke(env) ?: GValue.VUnit
        }
    }

    context(r: Raise<GuestError>)
    private fun branchMatches(
        branch: WhenBranch,
        patternExec: Exec?,
        subject: GValue?,
        env: Env,
    ): Boolean {
        val typePattern = branch.typePattern
        if (typePattern != null) {
            if (subject == null) return false
            return Primitives.isTypeValue(subject, typePattern)
        }
        if (patternExec == null) return false
        if (subject == null) return Primitives.truth(patternExec(env), branch.pattern?.span ?: branch.span)
        return valueEquals(patternExec(env), subject)
    }

    context(r: Raise<GuestError>)
    private fun lambdaExec(expression: Lambda): Exec {
        val parameters = expression.parameters.map { it.name }
        val body = expression.body.statements.map { analyzeStatement(it) }
        return { env ->
            GValue.VFunction(null, parameters.size) { arguments ->
                val frame = Env.child(env)
                for ((name, value) in parameters.zip(arguments)) frame.define(name, value)
                runStatements(body, frame)
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun blockExec(block: Block): Exec {
        if (block in checked.lambdaCoercions) {
            val body = block.statements.map { analyzeStatement(it) }
            return { env ->
                GValue.VFunction(null, 0) { runStatements(body, Env.child(env)) }
            }
        }
        val statements = block.statements.map { analyzeStatement(it) }
        return { env ->
            val frame = Env.child(env)
            var result: GValue = GValue.VUnit
            for (statement in statements) result = statement(frame)
            result
        }
    }

    context(r: Raise<GuestError>)
    private fun callExec(expression: Call): Exec {
        val callee = expression.callee
        if (callee is Name) return namedCallExec(callee, expression)
        if (callee is Member) return memberCallExec(callee, expression)
        val fn = analyze(callee)
        val arguments = analyzeArguments(expression)
        return { env -> Primitives.invoke(fn(env), arguments.map { it(env) }, expression.span) }
    }

    context(r: Raise<GuestError>)
    private fun namedCallExec(
        callee: Name,
        expression: Call,
    ): Exec {
        val arguments = analyzeArguments(expression)
        val ctor = classTable[callee.text]
        return { env ->
            val values = arguments.map { it(env) }
            val bound = env.lookup(callee.text) ?: globals.lookup(callee.text)
            when {
                bound != null && bound.value !is GValue.VUnassigned -> {
                    Primitives.invoke(bound.value, values, expression.span)
                }

                ctor != null -> {
                    val fields = linkedMapOf<String, GValue>()
                    for ((property, argument) in ctor.properties.zip(values)) fields[property.name] = argument
                    GValue.VObject(ctor.name, structural = ctor.data, fields)
                }

                else -> {
                    Primitives.call(callee.text, values, sink, expression.span)
                }
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun memberCallExec(
        callee: Member,
        expression: Call,
    ): Exec {
        val receiver = analyze(callee.receiver)
        val arguments = analyzeArguments(expression)
        return { env ->
            val target = receiver(env)
            if (callee.safe && target is GValue.VNull) {
                GValue.VNull
            } else {
                val values = arguments.map { it(env) }
                when {
                    callee.name == "copy" && target is GValue.VObject -> {
                        val fields = LinkedHashMap(target.fields)
                        for (argument in expression.arguments) {
                            val name = argument.name ?: r.raise(GuestError.UnassignedRead(argument.span))
                            fields[name] = values[expression.arguments.indexOfFirst { it.name == name }]
                        }
                        GValue.VObject(target.className, structural = true, fields)
                    }

                    else -> {
                        val shape = (target as? GValue.VObject)?.let { classTable[it.className] }
                        val method = shape?.methods?.get(callee.name)
                        val bodyExec = shape?.analyzedMethods?.get(callee.name)
                        if (method != null && bodyExec != null) {
                            methodValue(target, method, bodyExec).apply(r, values)
                        } else {
                            Primitives.member(target, callee.name, values, expression.span)
                        }
                    }
                }
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun analyzeArguments(expression: Call): List<Exec> = expression.arguments.map { analyze(it.value) }

    context(r: Raise<GuestError>)
    private fun memberExec(expression: Member): Exec {
        val receiver = analyze(expression.receiver)
        return { env ->
            val target = receiver(env)
            if (expression.safe && target is GValue.VNull) {
                GValue.VNull
            } else if (target is GValue.VObject) {
                val field = target.fields[expression.name]
                val shape = classTable[target.className]
                val method = shape?.methods?.get(expression.name)
                val bodyExec = shape?.analyzedMethods?.get(expression.name)
                when {
                    field != null -> field
                    method != null && bodyExec != null -> methodValue(target, method, bodyExec)
                    else -> Primitives.property(target, expression.name, expression.span)
                }
            } else {
                Primitives.property(target, expression.name, expression.span)
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun analyzeStatement(statement: Statement): Exec =
        when (statement) {
            is ExpressionStatement -> {
                analyze(statement.expression)
            }

            is LocalProperty -> {
                val initializer = analyze(statement.initializer)
                val exec: Exec = { env ->
                    env.define(statement.name, initializer(env))
                    GValue.VUnit
                }
                exec
            }

            is Destructure -> {
                destructureExec(statement)
            }

            is Assignment -> {
                assignmentExec(statement)
            }

            is While -> {
                whileExec(statement)
            }

            is For -> {
                forExec(statement)
            }

            is Return -> {
                val value = statement.value?.let { analyze(it) }
                val exec: Exec = { env -> throw ReturnSignal(value?.invoke(env) ?: GValue.VUnit) }
                exec
            }

            is Break -> {
                { _ -> throw BreakSignal() }
            }

            is Continue -> {
                { _ -> throw ContinueSignal() }
            }

            is FunctionDecl -> {
                val bodyExec = analyzeBody(statement)
                val exec: Exec = { env ->
                    env.define(statement.name, functionValue(statement, bodyExec, env))
                    GValue.VUnit
                }
                exec
            }
        }

    context(r: Raise<GuestError>)
    private fun destructureExec(statement: Destructure): Exec {
        val initializer = analyze(statement.initializer)
        val names = statement.names
        return { env ->
            val source = initializer(env)
            val values =
                when (source) {
                    is GValue.VPair -> listOf(source.first, source.second)
                    is GValue.VObject -> source.fields.values.toList()
                    else -> r.raise(GuestError.UnassignedRead(statement.span))
                }
            for ((name, value) in names.zip(values)) env.define(name, value)
            GValue.VUnit
        }
    }

    context(r: Raise<GuestError>)
    private fun assignmentExec(statement: Assignment): Exec {
        val target = statement.target
        val operator = statement.operator
        val value = analyze(statement.value)
        if (target is Name) {
            return { env ->
                val cell = env.lookup(target.text) ?: globals.lookup(target.text) ?: r.raise(GuestError.UnassignedRead(target.span))
                cell.value = Primitives.assigned(operator, cell.value, value(env), statement.span)
                GValue.VUnit
            }
        }
        if (target is Member) {
            val receiver = analyze(target.receiver)
            return { env ->
                val objectValue = receiver(env) as? GValue.VObject ?: r.raise(GuestError.UnassignedRead(target.span))
                val current = objectValue.fields[target.name]
                if (current == null && operator != "=") r.raise(GuestError.UnassignedRead(target.span))
                objectValue.fields[target.name] = Primitives.assigned(operator, current ?: GValue.VUnit, value(env), statement.span)
                GValue.VUnit
            }
        }
        if (target is Index) {
            val receiver = analyze(target.receiver)
            val index = analyze(target.index)
            return { env ->
                val receiverValue = receiver(env)
                val indexValue = index(env)
                val current = if (operator == "=") GValue.VUnit else Primitives.readIndex(receiverValue, indexValue, statement.span)
                Primitives.writeIndex(receiverValue, indexValue, Primitives.assigned(operator, current, value(env), statement.span), statement.span)
                GValue.VUnit
            }
        }
        return { _ -> r.raise(GuestError.UnassignedRead(statement.span)) }
    }

    context(r: Raise<GuestError>)
    private fun whileExec(statement: While): Exec {
        val condition = analyze(statement.condition)
        val body = statement.body.statements.map { analyzeStatement(it) }
        return { env ->
            var running = true
            while (running && Primitives.truth(condition(env), statement.condition.span)) {
                try {
                    val frame = Env.child(env)
                    for (inner in body) inner(frame)
                } catch (_: BreakSignal) {
                    running = false
                } catch (_: ContinueSignal) {
                    // re-test the condition
                }
            }
            GValue.VUnit
        }
    }

    context(r: Raise<GuestError>)
    private fun forExec(statement: For): Exec {
        val iterable = analyze(statement.iterable)
        val end = statement.end?.let { analyze(it) }
        val body = statement.body.statements.map { analyzeStatement(it) }
        val name = statement.name
        return { env ->
            val items: Iterable<GValue> =
                if (end != null) {
                    Primitives.rangeValues(iterable(env), end(env), statement.span).asIterable()
                } else {
                    when (val source = iterable(env)) {
                        is GValue.VList -> source.items.toList()
                        is GValue.VLazyList -> (sicp.guest.materialize(source) as GValue.VList).items.toList()
                        else -> r.raise(GuestError.UnassignedRead(statement.span))
                    }
                }
            val loopEnv = Env.child(env)
            var running = true
            for (item in items) {
                if (!running) break
                loopEnv.define(name, item)
                try {
                    val frame = Env.child(loopEnv)
                    for (inner in body) inner(frame)
                } catch (_: BreakSignal) {
                    running = false
                } catch (_: ContinueSignal) {
                    // next item
                }
            }
            GValue.VUnit
        }
    }
}
