// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55

package sicp.ch2.exercises

/** A host syntax tree for nested quotation, kept distinct from executable source. */
public sealed interface QuotationForm {
    public data class Name(
        public val value: String,
    ) : QuotationForm

    public data class Quoted(
        public val operand: QuotationForm,
    ) : QuotationForm
}

/** The host tree makes both quotation layers explicit. */
public val doubledQuoteAbracadabra: QuotationForm =
    QuotationForm.Quoted(QuotationForm.Quoted(QuotationForm.Name("abracadabra")))

/** Inspect one layer: the result is another quotation node. */
public fun ex_2_55(): QuotationForm = (doubledQuoteAbracadabra as QuotationForm.Quoted).operand
