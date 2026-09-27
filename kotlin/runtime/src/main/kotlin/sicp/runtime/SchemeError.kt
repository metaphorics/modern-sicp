// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

/**
 * The edition's one typed error for the interpreted language: every `error`
 * in the book raises one of these variants through `Raise<SchemeError>`, and
 * every checked runtime operation fails with one. Domain errors of the host
 * programs (`IntervalError`, `AccountError`, `QueryError`) stay per-domain
 * sealed types in their own chapters.
 */
public sealed class SchemeError {
    /** A name lookup walked the whole environment chain without a hit. */
    public data class Unbound(
        val name: String,
    ) : SchemeError() {
        public override fun toString(): String = "unbound variable: $name"
    }

    /** The operator position of an application did not evaluate to a procedure. */
    public data class NotApplicable(
        val v: Value,
    ) : SchemeError() {
        public override fun toString(): String = "not a procedure: $v"
    }

    /** A procedure received the wrong number of arguments. */
    public data class WrongArity(
        val procedure: String,
        val expected: String,
        val got: Int,
    ) : SchemeError() {
        public override fun toString(): String = "$procedure: wrong number of arguments, expected $expected, got $got"
    }

    /** A value reached an operation that only accepts another shape. */
    public data class TypeMismatch(
        val detail: String,
    ) : SchemeError() {
        public override fun toString(): String = "type mismatch: $detail"
    }

    /** Division or modulo by zero. */
    public data object DivisionByZero : SchemeError() {
        public override fun toString(): String = "division by zero"
    }

    /** Checked `Long` arithmetic ran past the fixed width. */
    public data object Overflow : SchemeError() {
        public override fun toString(): String = "integer overflow"
    }

    /** A reader rejected malformed surface syntax. */
    public data class Parse(
        val detail: String,
    ) : SchemeError() {
        public override fun toString(): String = "parse error: $detail"
    }

    /** The `amb` engine has no alternative left: `try-again` past the last
     * choice. */
    public data object Backtrack : SchemeError() {
        public override fun toString(): String = "no more alternatives"
    }

    /** The book's `(error "message" irritant ...)` raised by user code. */
    public data class UserRaised(
        val message: String,
        val irritants: List<Value>,
    ) : SchemeError() {
        public override fun toString(): String =
            if (irritants.isEmpty()) {
                message
            } else {
                irritants.joinToString(separator = " ", prefix = "$message ")
            }
    }

    /** A message-passing object received a message it does not answer. */
    public data class UnknownMessage(
        val v: Value,
    ) : SchemeError() {
        public override fun toString(): String = "message not understood: $v"
    }

    /** `makeAccount` refused a withdrawal larger than the balance. */
    public data object InsufficientFunds : SchemeError() {
        public override fun toString(): String = "insufficient funds"
    }

    /** The chapter 4 dispatch met an expression shape it does not know. */
    public data class UnknownExpressionType(
        val v: Value,
    ) : SchemeError() {
        public override fun toString(): String = "unknown expression type: $v"
    }

    /** `apply` met a procedure shape it does not know. */
    public data class UnknownProcedureType(
        val v: Value,
    ) : SchemeError() {
        public override fun toString(): String = "unknown procedure type: $v"
    }

    /** A `restore` ran on an empty machine stack. */
    public data object StackUnderflow : SchemeError() {
        public override fun toString(): String = "stack underflow: restore on an empty stack"
    }

    /** A `goto`, `branch`, or label-valued assign named a label the
     * controller never defined. */
    public data class UnknownLabel(
        val name: String,
    ) : SchemeError() {
        public override fun toString(): String = "unknown label: $name"
    }

    /** Any other machine fault: an unknown operation name, a label used as
     * an operation operand, a register the machine was not built with. */
    public data class MachineFault(
        val detail: String,
    ) : SchemeError() {
        public override fun toString(): String = "machine fault: $detail"
    }
}
