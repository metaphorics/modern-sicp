// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.44: open coding must respect a rebound name.
// The warning the exercise asks for is the lexical analysis over the
// checked syntax: an open-codable name that an inner binding rebinds is
// reported, so the open-coding path declines it. The knob carries the
// open-coded set; the probe's rebound name must not change the answer.

package sicp.ch5.solutions

import sicp.ch5.Compiler
import sicp.ch5.CompilerOptions

/** The probe: an inner binding shadows a name the open-coded set would
 *  take. */
private val shadowedSource: String =
    """
    fun probe(): Long {
        val plus: Long = 1L
        return plus + 2L
    }

    fun main() {
        println(probe())
    }
    """.trimIndent()

/** The shadowing report: which names an inner binding rebinds beside
 *  the run's answer and the open-coded run's agreement. */
public fun openCodingShadowingReport(): List<String> {
    val checked = admitProgram(shadowedSource)
    val reboundNames = mutableSetOf<String>()
    val openCodedNames = setOf("plus")
    val pending = ArrayDeque<sicp.guest.Node>()

    fun recordBinding(name: String) {
        if (name in openCodedNames) reboundNames.add(name)
    }

    fun scheduleBranch(branch: sicp.guest.WhenBranch) {
        branch.pattern?.let(pending::addLast)
        pending.addLast(branch.body)
    }

    fun scheduleStatement(statement: sicp.guest.Statement) {
        when (statement) {
            is sicp.guest.LocalProperty -> {
                recordBinding(statement.name)
                pending.addLast(statement.initializer)
            }

            is sicp.guest.Destructure -> {
                statement.names.forEach(::recordBinding)
                pending.addLast(statement.initializer)
            }

            is sicp.guest.Assignment -> {
                pending.addLast(statement.target)
                pending.addLast(statement.value)
            }

            is sicp.guest.While -> {
                pending.addLast(statement.condition)
                pending.addLast(statement.body)
            }

            is sicp.guest.For -> {
                recordBinding(statement.name)
                pending.addLast(statement.iterable)
                statement.end?.let(pending::addLast)
                pending.addLast(statement.body)
            }

            is sicp.guest.Return -> {
                statement.value?.let(pending::addLast)
            }

            is sicp.guest.Break, is sicp.guest.Continue -> {}

            is sicp.guest.ExpressionStatement -> {
                pending.addLast(statement.expression)
            }

            is sicp.guest.FunctionDecl -> {
                statement.parameters.forEach { recordBinding(it.name) }
                pending.addLast(statement.body)
            }
        }
    }

    fun scheduleExpression(expression: sicp.guest.Expression) {
        when (expression) {
            is sicp.guest.Name, is sicp.guest.Literal, is sicp.guest.This, is sicp.guest.CallableReference -> {}

            is sicp.guest.Block -> {
                expression.statements.forEach(::scheduleStatement)
            }

            is sicp.guest.If -> {
                pending.addLast(expression.condition)
                pending.addLast(expression.yes)
                expression.no?.let(pending::addLast)
            }

            is sicp.guest.When -> {
                expression.subject?.let(pending::addLast)
                expression.branches.forEach(::scheduleBranch)
                expression.otherwise?.let(pending::addLast)
            }

            is sicp.guest.Lambda -> {
                expression.parameters.forEach { recordBinding(it.name) }
                pending.addLast(expression.body)
            }

            is sicp.guest.Binary -> {
                pending.addLast(expression.left)
                pending.addLast(expression.right)
            }

            is sicp.guest.Unary -> {
                pending.addLast(expression.operand)
            }

            is sicp.guest.Elvis -> {
                pending.addLast(expression.left)
                pending.addLast(expression.right)
            }

            is sicp.guest.Is -> {
                pending.addLast(expression.value)
            }

            is sicp.guest.Return -> {
                expression.value?.let(pending::addLast)
            }

            is sicp.guest.Call -> {
                pending.addLast(expression.callee)
                expression.arguments.forEach { pending.addLast(it.value) }
            }

            is sicp.guest.Member -> {
                pending.addLast(expression.receiver)
            }

            is sicp.guest.Index -> {
                pending.addLast(expression.receiver)
                pending.addLast(expression.index)
            }

            is sicp.guest.StringTemplate -> {
                expression.fragments.forEach(pending::addLast)
            }
        }
    }
    for (function in checked.syntax.declarations.filterIsInstance<sicp.guest.FunctionDecl>()) {
        function.parameters.forEach { recordBinding(it.name) }
        pending.addLast(function.body)
    }
    while (pending.isNotEmpty()) {
        when (val node = pending.removeLast()) {
            is sicp.guest.Statement -> scheduleStatement(node)
            is sicp.guest.Expression -> scheduleExpression(node)
            else -> Unit
        }
    }
    val openCoded =
        outputLines(
            Compiler.compileAndRun(checked, CompilerOptions(openCodedPrimitives = setOf("+"))),
        )
    val generic = outputLines(Compiler.compileAndRun(checked, CompilerOptions(openCodedPrimitives = emptySet())))
    check(openCoded == generic) { "the open-coded run disagrees with the generic run" }
    return listOf(
        "rebound names reported: ${reboundNames.joinToString(" ").ifEmpty { "none" }}",
        "the probe answers: ${openCoded.joinToString(" ")}",
    )
}
