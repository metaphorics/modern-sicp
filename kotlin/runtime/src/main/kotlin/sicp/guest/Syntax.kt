// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

/** Source coordinates are one-based; offsets delimit the original source. */
public data class Place(
    val offset: Int,
    val line: Int,
    val column: Int,
)

public data class Span(
    val start: Place,
    val end: Place,
)

/** The position of engine-generated events that have no source location. */
public val NO_POSITION: Span = Span(Place(0, 1, 1), Place(0, 1, 1))

public sealed interface Node {
    public val span: Span
}

public data class Program(
    val declarations: List<Declaration>,
    override val span: Span,
) : Node

public sealed interface Declaration : Node

public data class Parameter(
    val name: String,
    val type: GuestType,
    val annotation: String?,
    override val span: Span,
) : Node

public data class Property(
    val name: String,
    val type: GuestType,
    val mutable: Boolean,
    override val span: Span,
) : Node

public data class FunctionDecl(
    val name: String,
    val parameters: List<Parameter>,
    val result: GuestType?,
    val body: Expression,
    val tailrec: Boolean,
    override val span: Span,
) : Declaration,
    Statement

public data class TopProperty(
    val property: Property,
    val initializer: Expression,
    override val span: Span,
) : Declaration

public data class TypeAlias(
    val name: String,
    val target: GuestType,
    override val span: Span,
) : Declaration

public data class DataClass(
    val name: String,
    val properties: List<Property>,
    val parent: String?,
    override val span: Span,
) : Declaration

public data class PlainClass(
    val name: String,
    val properties: List<Property>,
    val parent: String?,
    val methods: List<FunctionDecl>,
    override val span: Span,
) : Declaration

public data class SealedInterface(
    val name: String,
    override val span: Span,
) : Declaration

public data class DataObject(
    val name: String,
    val parent: String?,
    override val span: Span,
) : Declaration

public sealed interface GuestType {
    public data class Named(
        val name: String,
        val arguments: List<GuestType> = emptyList(),
    ) : GuestType

    public data class Nullable(
        val base: GuestType,
    ) : GuestType

    public data class Function(
        val parameters: List<GuestType>,
        val result: GuestType,
    ) : GuestType

    public data object Nothing : GuestType

    public data object Null : GuestType
}

public sealed interface Statement : Node

public data class LocalProperty(
    val name: String,
    val annotation: GuestType?,
    val initializer: Expression,
    val mutable: Boolean,
    override val span: Span,
) : Statement

public data class Destructure(
    val names: List<String>,
    val initializer: Expression,
    override val span: Span,
) : Statement

public data class Assignment(
    val target: Expression,
    val operator: String,
    val value: Expression,
    override val span: Span,
) : Statement

public data class While(
    val condition: Expression,
    val body: Block,
    override val span: Span,
) : Statement

public data class For(
    val name: String,
    val iterable: Expression,
    val end: Expression?,
    val body: Block,
    override val span: Span,
) : Statement

public data class Return(
    val value: Expression?,
    override val span: Span,
) : Statement,
    Expression

public data class Break(
    override val span: Span,
) : Statement

public data class Continue(
    override val span: Span,
) : Statement

public data class ExpressionStatement(
    val expression: Expression,
    override val span: Span,
) : Statement

public sealed interface Expression : Node

public data class Block(
    val statements: List<Statement>,
    override val span: Span,
) : Expression

public data class Literal(
    val text: String,
    val kind: LiteralKind,
    override val span: Span,
) : Expression

public enum class LiteralKind { INT, LONG, DOUBLE, BOOLEAN, STRING, NULL }

public data class Name(
    val text: String,
    override val span: Span,
) : Expression

public data class This(
    override val span: Span,
) : Expression

public data class CallableReference(
    val name: String,
    override val span: Span,
) : Expression

public data class If(
    val condition: Expression,
    val yes: Expression,
    val no: Expression?,
    override val span: Span,
) : Expression

public data class WhenBranch(
    val pattern: Expression?,
    val typePattern: GuestType?,
    val body: Expression,
    override val span: Span,
) : Node

public data class When(
    val subject: Expression?,
    val branches: List<WhenBranch>,
    val otherwise: Expression?,
    override val span: Span,
) : Expression

public data class LambdaParameter(
    val name: String,
    val annotation: GuestType?,
    override val span: Span,
) : Node

public data class Lambda(
    val parameters: List<LambdaParameter>,
    val body: Block,
    override val span: Span,
) : Expression

public data class Binary(
    val left: Expression,
    val operator: String,
    val right: Expression,
    override val span: Span,
) : Expression

public data class Unary(
    val operator: String,
    val operand: Expression,
    override val span: Span,
) : Expression

public data class Elvis(
    val left: Expression,
    val right: Expression,
    override val span: Span,
) : Expression

public data class Is(
    val value: Expression,
    val type: GuestType,
    val negated: Boolean,
    override val span: Span,
) : Expression

public data class Argument(
    val name: String?,
    val value: Expression,
    override val span: Span,
) : Node

public data class Call(
    val callee: Expression,
    val typeArguments: List<GuestType>,
    val arguments: List<Argument>,
    override val span: Span,
) : Expression

public data class Member(
    val receiver: Expression,
    val name: String,
    val safe: Boolean,
    override val span: Span,
) : Expression

public data class Index(
    val receiver: Expression,
    val index: Expression,
    override val span: Span,
) : Expression

public data class StringTemplate(
    val fragments: List<Expression>,
    override val span: Span,
) : Expression

public enum class Mode { CORE, LAZY, SEARCH, QUERY, QUERY_LAZY, QUERY_SEARCH }

/** Admission is a value, so no evaluator can mistake a rejected program for a checked one. */
public class CheckedProgram internal constructor(
    public val syntax: Program,
    public val types: Map<Expression, GuestType>,
    public val mode: Mode,
    /** Block nodes the checker read as zero-parameter lambdas; identity-keyed
     * so structurally equal blocks stay distinct. */
    public val lambdaCoercions: Set<Expression>,
) {
    /** The host-supplied declarations the mode admits ahead of the program's
     * own: the query vocabulary in the query modes, none otherwise. */
    public val vocabulary: List<Declaration>
        get() = if (mode.admitsQuery) QUERY_VOCABULARY else emptyList()
}

public sealed class AdmissionError(
    public val category: String,
    public val position: Span,
    public val message: String,
) {
    public class HostInvalid(
        category: String,
        position: Span,
        message: String,
    ) : AdmissionError(category, position, message)

    public class Unsupported(
        category: String,
        position: Span,
        message: String,
    ) : AdmissionError(category, position, message)
}

public sealed class GuestError(
    public val category: String,
    public val position: Span,
) {
    public class DivisionByZero(
        position: Span,
    ) : GuestError("DivisionByZero", position)

    public class Overflow(
        position: Span,
    ) : GuestError("Overflow", position)

    public class IndexOutOfBounds(
        position: Span,
    ) : GuestError("IndexOutOfBounds", position)

    public class UnassignedRead(
        position: Span,
    ) : GuestError("UnassignedRead", position)

    public class UnassignedRegister(
        position: Span,
    ) : GuestError("UnassignedRegister", position)

    /** Engine category: a machine operation received a value of the wrong
     * shape for the instruction that consumed it. */
    public class ShapeFault(
        position: Span,
    ) : GuestError("ShapeFault", position)
}
