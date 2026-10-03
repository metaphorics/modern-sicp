// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.23: derived expressions via transformer
// operations. The exercise's suggestion -- the syntax transformers
// available as machine operations -- is realized as pure passes over the
// checked syntax: the guard form of `when` becomes the book's chain of
// `if`s, and `?:` becomes an explicit null test. The engine's
// `ev-structural` evaluates the derived forms directly, so the exercise's
// observable is the agreement of transform-then-run with the direct run
// of the same session.

package sicp.ch5.solutions

import sicp.ch4.Direct
import sicp.ch5.ExplicitControl
import sicp.guest.Admission
import sicp.guest.Assignment
import sicp.guest.Binary
import sicp.guest.Block
import sicp.guest.Break
import sicp.guest.Call
import sicp.guest.CallableReference
import sicp.guest.CheckedProgram
import sicp.guest.Continue
import sicp.guest.Destructure
import sicp.guest.Elvis
import sicp.guest.Expression
import sicp.guest.ExpressionStatement
import sicp.guest.For
import sicp.guest.FunctionDecl
import sicp.guest.If
import sicp.guest.Index
import sicp.guest.Is
import sicp.guest.Lambda
import sicp.guest.Literal
import sicp.guest.LiteralKind
import sicp.guest.LocalProperty
import sicp.guest.Member
import sicp.guest.Mode
import sicp.guest.Name
import sicp.guest.Return
import sicp.guest.StringTemplate
import sicp.guest.This
import sicp.guest.Unary
import sicp.guest.When
import sicp.guest.While

/** `?:` becomes an explicit null test: `left ?: right` is `if (left ==
 *  null) right else left`. The session's left operands are pure, so this
 *  simple form is observationally the block binding the general form
 *  uses. */
public fun elvisToTest(elvis: Elvis): Expression =
    If(
        Binary(elvis.left, "==", Literal("null", LiteralKind.NULL, elvis.span), elvis.span),
        elvis.right,
        elvis.left,
        elvis.span,
    )

/** The guard form of `when` becomes the book's chain of `if`s: each
 *  Boolean branch's body sits in the consequent, the next branch in the
 *  alternative, and a missing `else` ends the chain in `false`. */
public fun whenToIf(expression: When): Expression {
    if (expression.subject != null) {
        error("this exercise's transformer covers the guard form of when")
    }
    return guardChain(expression, expression.branches.map { it.pattern to it.body }, expression.otherwise)
}

private fun guardChain(
    at: When,
    branches: List<Pair<Expression?, Expression>>,
    otherwise: Expression?,
): Expression {
    val head = branches.firstOrNull() ?: return otherwise ?: Literal("false", LiteralKind.BOOLEAN, at.span)
    val (pattern, body) = head
    val test = pattern ?: error("a guard branch needs its Boolean test")
    return If(
        derivedToCore(test),
        derivedToCore(body),
        guardChain(at, branches.drop(1), otherwise),
        at.span,
    )
}

/** The derived expressions rewritten to core forms throughout
 *  [expression]: the two transformers applied bottom-up through the
 *  expression forms the section's programs use. */
public fun derivedToCore(expression: Expression): Expression =
    when (expression) {
        is When -> {
            whenToIf(expression)
        }

        is Elvis -> {
            derivedToCore(elvisToTest(expression))
        }

        is If -> {
            If(
                derivedToCore(expression.condition),
                derivedToCore(expression.yes),
                expression.no?.let { derivedToCore(it) },
                expression.span,
            )
        }

        is Binary -> {
            Binary(derivedToCore(expression.left), expression.operator, derivedToCore(expression.right), expression.span)
        }

        is Unary -> {
            Unary(expression.operator, derivedToCore(expression.operand), expression.span)
        }

        is Block -> {
            Block(expression.statements.map { statementToCore(it) }, expression.span)
        }

        is Return -> {
            Return(expression.value?.let { derivedToCore(it) }, expression.span)
        }

        else -> {
            expression
        }
    }

private fun statementToCore(statement: sicp.guest.Statement): sicp.guest.Statement =
    when (statement) {
        is Return -> {
            statement.copy(value = statement.value?.let { derivedToCore(it) })
        }

        is ExpressionStatement -> {
            statement.copy(expression = derivedToCore(statement.expression))
        }

        is LocalProperty -> {
            statement.copy(initializer = derivedToCore(statement.initializer))
        }

        is Destructure -> {
            statement.copy(initializer = derivedToCore(statement.initializer))
        }

        is Assignment -> {
            statement.copy(target = derivedToCore(statement.target), value = derivedToCore(statement.value))
        }

        is While -> {
            statement.copy(condition = derivedToCore(statement.condition), body = derivedToCore(statement.body) as Block)
        }

        is For -> {
            statement.copy(
                iterable = derivedToCore(statement.iterable),
                end = statement.end?.let { derivedToCore(it) },
                body = derivedToCore(statement.body) as Block,
            )
        }

        is FunctionDecl -> {
            statement.copy(body = derivedToCore(statement.body))
        }

        is Break, is Continue -> {
            statement
        }
    }

/** The shape of an expression, spans ignored: the transform's structural
 *  contract reads this way. */
public fun expressionShape(expression: Expression): String =
    when (expression) {
        is If -> "(if ${expressionShape(
            expression.condition,
        )} ${expressionShape(expression.yes)} ${expression.no?.let { expressionShape(it) } ?: "-"})"

        is Binary -> "(${expression.operator} ${expressionShape(expression.left)} ${expressionShape(expression.right)})"

        is Unary -> "(${expression.operator} ${expressionShape(expression.operand)})"

        is Literal -> expression.text

        is Name -> expression.text

        is Block -> expression.statements.joinToString(" ", prefix = "(block ", postfix = ")") { statementShape(it) }

        is Return -> "(return ${expression.value?.let { expressionShape(it) } ?: "-"})"

        is When -> "(when ${expression.branches.joinToString(
            " ",
        ) { branch ->
            "${branch.pattern?.let {
                expressionShape(
                    it,
                )
            } ?: "*"} ${expressionShape(branch.body)}"
        }} ${expression.otherwise?.let { expressionShape(it) } ?: "-"})"

        is Elvis -> "(?: ${expressionShape(expression.left)} ${expressionShape(expression.right)})"

        is Is -> "(is ${expressionShape(expression.value)})"

        is Call -> "(call)"

        is Lambda -> "(lambda)"

        is Member -> "(member ${expressionShape(expression.receiver)})"

        is Index -> "(index ${expressionShape(expression.receiver)})"

        is StringTemplate -> "(template)"

        is CallableReference -> "(&${expression.name})"

        is This -> "this"
    }

private fun statementShape(statement: sicp.guest.Statement): String =
    when (statement) {
        is Return -> expressionShape(statement)
        is ExpressionStatement -> expressionShape(statement.expression)
        is LocalProperty -> "(local ${statement.name} ${expressionShape(statement.initializer)})"
        is Destructure -> "(destructure ${statement.names.joinToString(" ")} ${expressionShape(statement.initializer)})"
        is Assignment -> "(assign ${expressionShape(statement.target)} ${expressionShape(statement.value)})"
        is While -> "(while ${expressionShape(statement.condition)} ${expressionShape(statement.body)})"
        is For -> "(for ${statement.name} ${expressionShape(statement.iterable)} ${expressionShape(statement.body)})"
        is FunctionDecl -> "(fun ${statement.name} ${expressionShape(statement.body)})"
        is Break -> "break"
        is Continue -> "continue"
    }

/** The session in derived forms: a guard `when` and a `?:`. */
private val derivedSessionSource: String =
    """
    fun classify(n: Long): String {
        return when {
            n == 0L -> "zero"
            n == 1L -> "one"
            else -> "many"
        }
    }

    fun label(value: String?): String {
        return value ?: "missing"
    }

    fun main() {
        println(classify(0L))
        println(classify(1L))
        println(classify(7L))
        println(label(null))
        println(label("six"))
    }
    """.trimIndent()

/** The same session in core forms only: the transform's expected output,
 *  written out. */
private val coreSessionSource: String =
    """
    fun classify(n: Long): String {
        return if (n == 0L) "zero" else if (n == 1L) "one" else "many"
    }

    fun label(value: String?): String {
        return if (value == null) "missing" else value
    }

    fun main() {
        println(classify(0L))
        println(classify(1L))
        println(classify(7L))
        println(label(null))
        println(label("six"))
    }
    """.trimIndent()

private fun admit(source: String): CheckedProgram =
    Admission.admit(source, Mode.CORE).fold({ error -> error("the session did not admit: $error") }, { it })

private fun bodyOf(
    checked: CheckedProgram,
    name: String,
): Expression =
    checked.syntax.declarations
        .filterIsInstance<FunctionDecl>()
        .firstOrNull { it.name == name }
        ?.body
        ?: error("the session has no $name")

private fun outputOf(source: String): List<String> =
    Direct
        .run(source, Mode.CORE)
        .fold({ error -> error("the session failed to run: $error") }, { it })
        .output
        .split("\n")
        .filter { it.isNotEmpty() }

/** The session through both engines beside the transformed core form:
 *  the printed answers of the derived session, the structural verdict of
 *  the transform against the written-out core program, and the
 *  agreement verdict of the two engines on the same checked source. */
public fun derivedExpressionRuns(): List<String> {
    val derived = admit(derivedSessionSource)
    val core = admit(coreSessionSource)
    val structural = expressionShape(derivedToCore(bodyOf(derived, "classify"))) == expressionShape(bodyOf(core, "classify"))
    val direct = outputOf(derivedSessionSource)
    val machine =
        ExplicitControl
            .run(derived)
            .output
            .split("\n")
            .filter { it.isNotEmpty() }
    val coreOut = outputOf(coreSessionSource)
    return direct +
        listOf(
            "transformed syntax matches the core program: $structural",
            "direct and explicit-control runs agree: ${direct == machine && direct == coreOut}",
        )
}
