// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55

package sicp.ch2.exercises

/** A host syntax tree for the quotation lesson, not a parsed source language. */
public sealed interface QuotationForm {
    public data class Name(
        public val value: String,
    ) : QuotationForm

    public data class Quoted(
        public val operand: QuotationForm,
    ) : QuotationForm
}

/**
 * Exercise 2.55: model a symbol name wrapped in two explicit quotation
 * nodes. Return the outer node's operand and explain why it is a quotation
 * node rather than the name directly.
 */
public fun ex_2_55(): QuotationForm = throw PendingExercise()
