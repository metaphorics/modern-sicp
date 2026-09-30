// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

import arrow.core.raise.Raise
import arrow.core.raise.either

/** Recursive-descent parser for grammar sections 2.2 and 2.3. It produces
 * the shared syntax tree only; admission classification of Section 6 forms
 * happens here so no evaluator ever sees unadmitted structure. */
public class Parser(
    private val tokens: List<Token>,
) {
    private var index: Int = 0

    private fun peek(): Token = tokens[index]

    private fun peekAt(k: Int): Token = tokens.getOrElse(index + k) { tokens[tokens.size - 1] }

    private fun next(): Token = tokens[index++]

    private fun here(): Place = peek().span.start

    context(r: Raise<AdmissionError>)
    private fun spanFrom(start: Place): Span = Span(start, peek().span.start)

    context(r: Raise<AdmissionError>)
    private fun fail(
        category: String,
        what: String,
    ): Nothing = r.raise(AdmissionError.HostInvalid(category, Span(here(), here()), what))

    context(r: Raise<AdmissionError>)
    private fun failUnsupported(
        category: String,
        what: String,
    ): Nothing = r.raise(AdmissionError.Unsupported(category, Span(here(), here()), what))

    private fun atSymbol(text: String): Boolean {
        val t = peek()
        return t is Token.Symbol && t.text == text
    }

    private fun atKeyword(text: String): Boolean {
        val t = peek()
        return t is Token.Keyword && t.text == text
    }

    private fun atIdent(text: String): Boolean {
        val t = peek()
        return t is Token.Ident && t.text == text
    }

    private fun eatSymbol(text: String): Boolean {
        if (!atSymbol(text)) return false
        index++
        return true
    }

    private fun eatKeyword(text: String): Boolean {
        if (!atKeyword(text)) return false
        index++
        return true
    }

    context(r: Raise<AdmissionError>)
    private fun expectSymbol(text: String) {
        if (!eatSymbol(text)) fail("Syntax", "expected `$text`")
    }

    context(r: Raise<AdmissionError>)
    private fun expectKeyword(text: String) {
        if (!eatKeyword(text)) fail("Syntax", "expected `$text`")
    }

    context(r: Raise<AdmissionError>)
    private fun expectIdent(): String {
        val t = peek()
        if (t !is Token.Ident) fail("Syntax", "expected an identifier")
        index++
        return t.text
    }

    context(r: Raise<AdmissionError>)
    private fun expectEof() {
        if (peek() !is Token.Eof) fail("Syntax", "unexpected trailing input")
    }

    private fun peekText(): String =
        when (val t = peek()) {
            is Token.Keyword -> t.text
            is Token.Ident -> t.text
            else -> ""
        }

    /** Words that are valid Kotlin but sit outside the admitted grammar;
     * naming them keeps the rejection class honest. */
    private val rejectionWords: Map<String, String> =
        mapOf(
            "try" to "Exceptions",
            "catch" to "Exceptions",
            "finally" to "Exceptions",
            "throw" to "Exceptions",
            "do" to "MiscKotlin",
            "enum" to "MiscKotlin",
            "open" to "MiscKotlin",
            "override" to "MiscKotlin",
            "lateinit" to "MiscKotlin",
            "suspend" to "MiscKotlin",
            "infix" to "ExtensionsOperators",
            "operator" to "ExtensionsOperators",
            "inline" to "MiscKotlin",
            "external" to "MiscKotlin",
            "abstract" to "MiscKotlin",
            "companion" to "MiscKotlin",
            "private" to "MiscKotlin",
            "internal" to "MiscKotlin",
            "protected" to "MiscKotlin",
            "vararg" to "NamedDefaultVarargs",
            "reified" to "UserGenerics",
            "annotation" to "MiscKotlin",
            "by" to "Delegation",
        )

    context(r: Raise<AdmissionError>)
    private fun rejectWord(word: String) {
        val category = rejectionWords[word] ?: return
        failUnsupported(category, "`$word` is outside the admitted grammar")
    }

    /** Parses a whole compilation unit. */
    context(r: Raise<AdmissionError>)
    public fun parseProgram(): Program {
        val start = here()
        val declarations = mutableListOf<Declaration>()
        while (peek() !is Token.Eof) declarations.add(parseDeclaration())
        return Program(declarations, spanFrom(start))
    }

    /** Parses one top-level expression, for single-expression probe programs. */
    context(r: Raise<AdmissionError>)
    public fun parseSingleExpression(): Expression {
        val expression = parseExpression()
        expectEof()
        return expression
    }

    context(r: Raise<AdmissionError>)
    private fun parseDeclaration(): Declaration {
        val start = here()
        var isTailrec = false
        var isData = false
        var isSealed = false
        while (true) {
            rejectWord(peekText())
            when {
                eatKeyword("public") -> Unit
                eatKeyword("tailrec") -> isTailrec = true
                eatKeyword("data") -> isData = true
                eatKeyword("sealed") -> isSealed = true
                else -> break
            }
        }
        return dispatchDeclaration(start, isTailrec, isData, isSealed)
    }

    context(r: Raise<AdmissionError>)
    private fun dispatchDeclaration(
        start: Place,
        tailrec: Boolean,
        isData: Boolean,
        isSealed: Boolean,
    ): Declaration =
        when {
            atKeyword("fun") -> {
                next()
                parseFunction(start, tailrec)
            }

            atKeyword("class") -> {
                next()
                if (isData) parseDataClass(start) else parsePlainClass(start)
            }

            atKeyword("interface") -> {
                next()
                if (!isSealed) failUnsupported("MiscKotlin", "plain interface")
                SealedInterface(expectIdent(), spanFrom(start))
            }

            atKeyword("object") -> {
                next()
                if (!isData) failUnsupported("MiscKotlin", "object declaration")
                DataObject(expectIdent(), parseOptionalParent(), spanFrom(start))
            }

            atKeyword("typealias") -> {
                next()
                parseTypeAlias(start)
            }

            atKeyword("val") || atKeyword("var") -> {
                parseTopProperty(start)
            }

            else -> {
                fail("Syntax", "expected a declaration")
            }
        }

    context(r: Raise<AdmissionError>)
    private fun parseOptionalParent(): String? {
        if (!eatSymbol(":")) return null
        return expectIdent()
    }

    context(r: Raise<AdmissionError>)
    private fun parseTypeAlias(start: Place): Declaration {
        val name = expectIdent()
        expectSymbol("=")
        return TypeAlias(name, parseType(), spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseTopProperty(start: Place): Declaration {
        val mutable = atKeyword("var")
        next()
        val name = expectIdent()
        expectSymbol(":")
        val type = parseType()
        expectSymbol("=")
        return TopProperty(Property(name, type, mutable, spanFrom(start)), parseExpression(), spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseDataClass(start: Place): Declaration {
        val name = expectIdent()
        val properties = parseDataProperties()
        return DataClass(name, properties, parseOptionalParent(), spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseDataProperties(): List<Property> =
        parenList {
            val start = here()
            expectKeyword("val")
            val name = expectIdent()
            expectSymbol(":")
            Property(name, parseType(), mutable = false, spanFrom(start))
        }

    context(r: Raise<AdmissionError>)
    private fun parsePlainClass(start: Place): Declaration {
        val name = expectIdent()
        val properties = parseConstructorProperties()
        val parent = parseOptionalParent()
        expectSymbol("{")
        val methods = mutableListOf<FunctionDecl>()
        while (!eatSymbol("}")) {
            rejectWord(peekText())
            expectKeyword("fun")
            methods.add(parseFunction(here(), tailrec = false))
        }
        return PlainClass(name, properties, parent, methods, spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseConstructorProperties(): List<Property> =
        parenList {
            val start = here()
            val mutable = atKeyword("var")
            next()
            val name = expectIdent()
            expectSymbol(":")
            Property(name, parseType(), mutable, spanFrom(start))
        }

    context(r: Raise<AdmissionError>)
    private fun parseFunction(
        start: Place,
        tailrec: Boolean,
    ): FunctionDecl {
        val name = expectIdent()
        val parameters = parseParameters()
        val result = if (eatSymbol(":")) parseType() else null
        val body = if (eatSymbol("=")) parseExpression() else parseBlock()
        return FunctionDecl(name, parameters, result, body, tailrec, spanFrom(start))
    }

    /** A parenthesized comma list with an optional trailing comma; the
     * closing paren is consumed exactly once, here. */
    context(r: Raise<AdmissionError>)
    private fun <T> parenList(item: () -> T): List<T> {
        expectSymbol("(")
        val out = mutableListOf<T>()
        while (true) {
            if (eatSymbol(")")) return out
            out.add(item())
            if (eatSymbol(",")) continue
            expectSymbol(")")
            return out
        }
    }

    context(r: Raise<AdmissionError>)
    private fun parseParameters(): List<Parameter> =
        parenList {
            val start = here()
            val annotation = parseParameterAnnotation()
            val name = expectIdent()
            expectSymbol(":")
            Parameter(name, parseType(), annotation, spanFrom(start))
        }

    context(r: Raise<AdmissionError>)
    private fun parseParameterAnnotation(): String? {
        if (!eatSymbol("@")) return null
        val word = expectIdent()
        if (word != "Strict" && word != "Delayed") {
            r.raise(AdmissionError.Unsupported("MiscKotlin", Span(here(), here()), "annotation @$word"))
        }
        return word
    }

    context(r: Raise<AdmissionError>)
    private fun parseType(): GuestType {
        val base = parseTypeBase()
        return if (eatSymbol("?")) GuestType.Nullable(base) else base
    }

    context(r: Raise<AdmissionError>)
    private fun parseTypeBase(): GuestType {
        val t = peek()
        if (t is Token.Ident) {
            index++
            return GuestType.Named(t.text, parseTypeArgumentList())
        }
        if (atSymbol("(")) {
            val parameters = parenList { parseType() }
            expectSymbol("->")
            return GuestType.Function(parameters, parseType())
        }
        return fail("Syntax", "expected a type")
    }

    context(r: Raise<AdmissionError>)
    private fun parseTypeArgumentList(): List<GuestType> {
        if (!eatSymbol("<")) return emptyList()
        val args = mutableListOf<GuestType>()
        args.add(parseTypeArgument())
        while (eatSymbol(",")) args.add(parseTypeArgument())
        expectSymbol(">")
        return args
    }

    context(r: Raise<AdmissionError>)
    private fun parseTypeArgument(): GuestType {
        if (atIdent("out") || atKeyword("in")) failUnsupported("UserGenerics", "type variance is outside the admitted grammar")
        return parseType()
    }

    context(r: Raise<AdmissionError>)
    private fun parseBlock(): Block {
        val start = here()
        expectSymbol("{")
        val statements = mutableListOf<Statement>()
        while (!eatSymbol("}")) statements.add(parseStatement())
        return Block(statements, spanFrom(start))
    }

    private enum class Mutability { VAL, VAR }

    context(r: Raise<AdmissionError>)
    private fun parseStatement(): Statement {
        val start = here()
        rejectWord(peekText())
        if (atKeyword("val") || atKeyword("var")) {
            val mutability = if (atKeyword("var")) Mutability.VAR else Mutability.VAL
            next()
            return parseLocalProperty(start, mutability)
        }
        if (eatKeyword("fun")) return parseFunction(start, tailrec = false)
        if (eatKeyword("while")) return parseWhile(start)
        if (eatKeyword("for")) return parseFor(start)
        if (eatKeyword("return")) return parseReturn(start)
        if (eatKeyword("break")) return Break(spanFrom(start))
        if (eatKeyword("continue")) return Continue(spanFrom(start))
        return parseExpressionStatement(start)
    }

    context(r: Raise<AdmissionError>)
    private fun parseLocalProperty(
        start: Place,
        mutability: Mutability,
    ): Statement {
        if (mutability == Mutability.VAR && atSymbol("(")) {
            failUnsupported("MiscKotlin", "destructuring declaration must use `val`")
        }
        if (atSymbol("(")) return parseDestructure(start)
        val name = expectIdent()
        val annotation = if (eatSymbol(":")) parseType() else null
        expectSymbol("=")
        return LocalProperty(name, annotation, parseExpression(), mutability == Mutability.VAR, spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseDestructure(start: Place): Statement {
        val names = parenList { expectIdent() }
        expectSymbol("=")
        return Destructure(names, parseExpression(), spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseWhile(start: Place): Statement {
        expectSymbol("(")
        val condition = parseExpression()
        expectSymbol(")")
        return While(condition, parseBlock(), spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseFor(start: Place): Statement {
        expectSymbol("(")
        val name = expectIdent()
        expectKeyword("in")
        val iterable = parseExpression()
        val end = if (eatSymbol("..")) parseExpression() else null
        expectSymbol(")")
        return For(name, iterable, end, parseBlock(), spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseReturn(start: Place): Return {
        if (bareReturnFollows()) return Return(null, spanFrom(start))
        return Return(parseExpression(), spanFrom(start))
    }

    private fun bareReturnFollows(): Boolean =
        when {
            atSymbol(")") || atSymbol("}") || atSymbol(",") || atSymbol("->") || atSymbol(";") -> true
            peek() is Token.Eof -> true
            atKeyword("else") -> true
            else -> false
        }

    context(r: Raise<AdmissionError>)
    private fun parseExpressionStatement(start: Place): Statement {
        if (eatKeyword("if")) return ExpressionStatement(parseIfStatement(start), spanFrom(start))
        val expression = parseExpression()
        val assignmentOps = setOf("=", "+=", "-=", "*=", "/=", "%=")
        val t = peek()
        if (t is Token.Symbol && t.text in assignmentOps) {
            index++
            return Assignment(expression, t.text, parseExpression(), spanFrom(start))
        }
        return ExpressionStatement(expression, spanFrom(start))
    }

    // Expression parsing: Kotlin precedence, loosest first.

    context(r: Raise<AdmissionError>)
    public fun parseExpression(): Expression = parseDisjunction()

    context(r: Raise<AdmissionError>)
    private fun parseDisjunction(): Expression {
        var left = parseConjunction()
        while (atSymbol("||")) {
            val start = left.span.start
            index++
            left = Binary(left, "||", parseConjunction(), spanFrom(start))
        }
        return left
    }

    context(r: Raise<AdmissionError>)
    private fun parseConjunction(): Expression {
        var left = parseEquality()
        while (atSymbol("&&")) {
            val start = left.span.start
            index++
            left = Binary(left, "&&", parseEquality(), spanFrom(start))
        }
        return left
    }

    context(r: Raise<AdmissionError>)
    private fun parseEquality(): Expression {
        var left = parseComparison()
        while (atSymbol("==") || atSymbol("!=") || atSymbol("===") || atSymbol("!==")) {
            val start = left.span.start
            val op = symbolText()
            index++
            left = Binary(left, op, parseComparison(), spanFrom(start))
        }
        return left
    }

    private fun symbolText(): String = (peek() as Token.Symbol).text

    context(r: Raise<AdmissionError>)
    private fun parseComparison(): Expression {
        var left = parseNamedCheck()
        while (atSymbol("<") || atSymbol(">") || atSymbol("<=") || atSymbol(">=")) {
            val start = left.span.start
            val op = symbolText()
            index++
            left = Binary(left, op, parseNamedCheck(), spanFrom(start))
        }
        return left
    }

    context(r: Raise<AdmissionError>)
    private fun parseNamedCheck(): Expression {
        var left = parseElvis()
        while (true) {
            // A new `when` branch may begin with `is T ->`. Without the
            // source line boundary, that `is` is mistaken for a postfix type
            // check on the previous branch's result expression.
            if (index > 0 && peek().span.start.line > tokens[index - 1].span.end.line) return left
            val start = left.span.start
            if (eatKeyword("is")) {
                left = Is(left, parseType(), negated = false, spanFrom(start))
                continue
            }
            if (atSymbol("!") && peekAt(1) is Token.Keyword && peekAt(1).text() == "is") {
                index += 2
                left = Is(left, parseType(), negated = true, spanFrom(start))
                continue
            }
            if (peekText() == "in" || (atSymbol("!") && peekAt(1).text() == "in")) {
                failUnsupported("MiscKotlin", "`in` outside `for`")
            }
            if (atIdent("as")) failUnsupported("UncheckedCast", "unchecked cast")
            return left
        }
    }

    private fun Token.text(): String =
        when (this) {
            is Token.Keyword -> text
            is Token.Ident -> text
            is Token.Symbol -> text
            else -> ""
        }

    context(r: Raise<AdmissionError>)
    private fun parseElvis(): Expression {
        val left = parseInfixTo()
        if (!atSymbol("?:")) return left
        val start = left.span.start
        index++
        return Elvis(left, parseElvis(), spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseInfixTo(): Expression {
        var left = parseAdditive()
        while (atIdent("to")) {
            val start = left.span.start
            index++
            left = Binary(left, "to", parseAdditive(), spanFrom(start))
        }
        return left
    }

    context(r: Raise<AdmissionError>)
    private fun parseAdditive(): Expression {
        var left = parseMultiplicative()
        while (atSymbol("+") || atSymbol("-")) {
            val start = left.span.start
            val op = symbolText()
            index++
            left = Binary(left, op, parseMultiplicative(), spanFrom(start))
        }
        return left
    }

    context(r: Raise<AdmissionError>)
    private fun parseMultiplicative(): Expression {
        var left = parseUnary()
        while (atSymbol("*") || atSymbol("/") || atSymbol("%")) {
            val start = left.span.start
            val op = symbolText()
            index++
            left = Binary(left, op, parseUnary(), spanFrom(start))
        }
        return left
    }

    context(r: Raise<AdmissionError>)
    private fun parseUnary(): Expression {
        val start = here()
        if (atSymbol("-") || atSymbol("!")) {
            val op = symbolText()
            index++
            val operand = parseUnary()
            if (op == "-" && operand is Literal &&
                operand.kind in setOf(LiteralKind.INT, LiteralKind.LONG, LiteralKind.DOUBLE) &&
                !operand.text.startsWith("-")
            ) {
                return operand.copy(text = "-${operand.text}", span = spanFrom(start))
            }
            return Unary(op, operand, spanFrom(start))
        }
        return parsePostfix()
    }

    context(r: Raise<AdmissionError>)
    private fun parsePostfix(): Expression {
        var expr = parsePrimary()
        while (true) {
            val start = expr.span.start
            if (atSymbol("!!")) failUnsupported("ForceUnwrap", "force unwrap")
            if (atSymbol("(")) {
                expr = finishCall(expr, emptyList(), start)
                continue
            }
            if (atSymbol("<") && typeArgumentsFollow()) {
                val args = parseTypeArguments()
                expr = finishCall(expr, args, start)
                continue
            }
            if (atSymbol("[")) {
                index++
                val indexExpr = parseExpression()
                expectSymbol("]")
                expr = Index(expr, indexExpr, spanFrom(start))
                continue
            }
            if (atSymbol(".") || atSymbol("?.")) {
                val safe = atSymbol("?.")
                index++
                expr = Member(expr, expectIdent(), safe, spanFrom(start))
                continue
            }
            if (atSymbol("{")) {
                val argument = trailingLambdaArgument()
                expr =
                    if (expr is Call) {
                        expr.copy(arguments = expr.arguments + argument, span = spanFrom(start))
                    } else {
                        Call(expr, emptyList(), listOf(argument), spanFrom(start))
                    }
                continue
            }
            return expr
        }
    }

    context(r: Raise<AdmissionError>)
    private fun trailingLambdaArgument(): Argument {
        val lambda = parseBraceLambda()
        return Argument(null, lambda, lambda.span)
    }

    /** A brace group in call position is always a lambda: with a parameter
     * head it is a [Lambda]; without one it is the zero-parameter form the
     * expected function type gives its parameters. */
    context(r: Raise<AdmissionError>)
    private fun parseBraceLambda(): Lambda {
        val start = here()
        if (lambdaHeadFollows()) return parseLambda()
        val block = parseBlock()
        return Lambda(emptyList(), block, spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun finishCall(
        callee: Expression,
        typeArguments: List<GuestType>,
        start: Place,
    ): Call {
        val arguments =
            parenList {
                val argStart = here()
                val name =
                    if (peek() is Token.Ident && peekAt(1).let { it is Token.Symbol && it.text == "=" }) {
                        val n = expectIdent()
                        expectSymbol("=")
                        n
                    } else {
                        null
                    }
                Argument(name, parseExpression(), spanFrom(argStart))
            }
        return Call(callee, typeArguments, arguments, spanFrom(start))
    }

    /** `<` starts type arguments only when a full argument list and a call
     * parenthesis follow; otherwise the token is comparison. */
    private fun typeArgumentsFollow(): Boolean {
        val probe = Parser(tokens.subList(index, tokens.size))
        val parsed = either { probe.parseTypeArguments() }
        return parsed.isRight() && probe.peek().let { it is Token.Symbol && it.text == "(" }
    }

    context(r: Raise<AdmissionError>)
    private fun parseTypeArguments(): List<GuestType> {
        expectSymbol("<")
        val args = mutableListOf<GuestType>()
        args.add(parseType())
        while (eatSymbol(",")) args.add(parseType())
        expectSymbol(">")
        return args
    }

    context(r: Raise<AdmissionError>)
    private fun parsePrimary(): Expression {
        val start = here()
        val t = next()
        return when (t) {
            is Token.Number -> {
                Literal(
                    if (t.suffix == "L") t.text.removeSuffix("L") else t.text,
                    if (t.floating) {
                        LiteralKind.DOUBLE
                    } else if (t.suffix == "L") {
                        LiteralKind.LONG
                    } else {
                        LiteralKind.INT
                    },
                    t.span,
                )
            }

            is Token.Bool -> {
                Literal(if (t.value) "true" else "false", LiteralKind.BOOLEAN, t.span)
            }

            is Token.Str -> {
                stringNode(t)
            }

            is Token.Ident -> {
                Name(t.text, t.span)
            }

            is Token.Keyword -> {
                keywordPrimary(t, start)
            }

            is Token.Symbol -> {
                symbolPrimary(t, start)
            }

            is Token.Eof -> {
                fail("Syntax", "unexpected end of input")
            }
        }
    }

    context(r: Raise<AdmissionError>)
    private fun stringNode(t: Token.Str): Expression {
        val texts = t.fragments.filterIsInstance<Token.Str.Fragment.Text>()
        if (texts.size == t.fragments.size) {
            return Literal(texts.joinToString("") { it.text }, LiteralKind.STRING, t.span)
        }
        val parts = mutableListOf<Expression>()
        for (fragment in t.fragments) {
            when (fragment) {
                is Token.Str.Fragment.Text -> parts.add(Literal(fragment.text, LiteralKind.STRING, t.span))
                is Token.Str.Fragment.Name -> parts.add(Name(fragment.name, t.span))
                is Token.Str.Fragment.Embedded -> parts.add(embeddedExpression(fragment, t))
            }
        }
        return StringTemplate(parts, t.span)
    }

    context(r: Raise<AdmissionError>)
    private fun embeddedExpression(
        fragment: Token.Str.Fragment.Embedded,
        host: Token.Str,
    ): Expression {
        val sub = Parser(fragment.tokens + Token.Eof(host.span))
        val expression = sub.parseExpression()
        sub.expectEof()
        return expression
    }

    context(r: Raise<AdmissionError>)
    private fun keywordPrimary(
        t: Token.Keyword,
        start: Place,
    ): Expression =
        when (t.text) {
            "null" -> {
                Literal("null", LiteralKind.NULL, t.span)
            }

            "true" -> {
                Literal("true", LiteralKind.BOOLEAN, t.span)
            }

            "false" -> {
                Literal("false", LiteralKind.BOOLEAN, t.span)
            }

            "this" -> {
                This(t.span)
            }

            "if" -> {
                parseIf(start)
            }

            "when" -> {
                parseWhen(start)
            }

            "return" -> {
                parseReturn(start)
            }

            else -> {
                rejectWord(t.text)
                fail("Syntax", "unexpected keyword `${t.text}`")
            }
        }

    context(r: Raise<AdmissionError>)
    private fun symbolPrimary(
        t: Token.Symbol,
        start: Place,
    ): Expression {
        if (t.text == "(") {
            val inner = parseExpression()
            expectSymbol(")")
            return inner
        }
        if (t.text == "{") {
            // parsePrimary consumed the brace; parseBraceExpression probes and
            // parses from the brace itself.
            index--
            return parseBraceExpression()
        }
        if (t.text == "::") return CallableReference(expectIdent(), spanFrom(start))
        return fail("Syntax", "unexpected symbol `${t.text}`")
    }

    context(r: Raise<AdmissionError>)
    private fun parseBraceExpression(): Expression {
        if (lambdaHeadFollows()) return parseLambda()
        return parseBlock()
    }

    /** True when the brace group carries a `->` parameter head, including
     * annotated parameters (`{ x: Int -> ... }`). */
    private fun lambdaHeadFollows(): Boolean {
        val probe = Parser(tokens.subList(index, tokens.size))
        return either { probe.parseLambda() }.isRight()
    }

    context(r: Raise<AdmissionError>)
    private fun parseLambda(): Lambda {
        val start = here()
        expectSymbol("{")
        if (eatSymbol("->")) return Lambda(emptyList(), parseLambdaBody(), spanFrom(start))
        val parameters = mutableListOf<LambdaParameter>()
        while (true) {
            val paramStart = here()
            val name = expectIdent()
            val annotation = if (eatSymbol(":")) parseType() else null
            parameters.add(LambdaParameter(name, annotation, spanFrom(paramStart)))
            if (eatSymbol(",")) {
                if (eatSymbol("->")) return Lambda(parameters, parseLambdaBody(), spanFrom(start))
                continue
            }
            expectSymbol("->")
            return Lambda(parameters, parseLambdaBody(), spanFrom(start))
        }
    }

    context(r: Raise<AdmissionError>)
    private fun parseLambdaBody(): Block {
        val start = here()
        val statements = mutableListOf<Statement>()
        while (!eatSymbol("}")) statements.add(parseStatement())
        return Block(statements, spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseIf(start: Place): Expression = parseIfWith(start) { parseBranchBody() }

    /** An `if` in statement position: a braceless branch is a statement, so
     * `if (c) x = 1` assigns. In expression position an assignment branch
     * is not an expression, so [parseIf] keeps expression branches. */
    context(r: Raise<AdmissionError>)
    private fun parseIfStatement(start: Place): Expression = parseIfWith(start) { parseStatementBranchBody() }

    context(r: Raise<AdmissionError>)
    private fun parseIfWith(
        start: Place,
        branchBody: () -> Expression,
    ): Expression {
        expectSymbol("(")
        val condition = parseExpression()
        expectSymbol(")")
        val yes = branchBody()
        val no = if (eatKeyword("else")) branchBody() else null
        return If(condition, yes, no, spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseBranchBody(): Expression {
        if (atSymbol("{")) return parseBraceExpression()
        return parseExpression()
    }

    /** A braceless statement branch: an expression stays itself; an
     * assignment becomes a one-statement block. */
    context(r: Raise<AdmissionError>)
    private fun parseStatementBranchBody(): Expression {
        if (atSymbol("{")) return parseBraceExpression()
        val start = here()
        return when (val statement = parseExpressionStatement(start)) {
            is ExpressionStatement -> statement.expression
            else -> Block(listOf(statement), spanFrom(start))
        }
    }

    context(r: Raise<AdmissionError>)
    private fun parseWhen(start: Place): Expression {
        val subject = parseWhenSubject()
        expectSymbol("{")
        val branches = mutableListOf<WhenBranch>()
        var otherwise: Expression? = null
        while (!eatSymbol("}")) {
            val branchStart = here()
            if (eatKeyword("else")) {
                expectSymbol("->")
                otherwise = parseBranchBody()
                continue
            }
            if (eatKeyword("is")) {
                val type = parseType()
                expectSymbol("->")
                branches.add(WhenBranch(null, type, parseBranchBody(), spanFrom(branchStart)))
                continue
            }
            val pattern = parseExpression()
            expectSymbol("->")
            branches.add(WhenBranch(pattern, null, parseBranchBody(), spanFrom(branchStart)))
        }
        return When(subject, branches, otherwise, spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun parseWhenSubject(): Expression? {
        if (!eatSymbol("(")) return null
        val subject = parseExpression()
        expectSymbol(")")
        return subject
    }
}
