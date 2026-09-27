(* SPDX-License-Identifier: GPL-3.0-only *)

(** The S-expression reader for the shared Scheme subset: parentheses, the
    ['...] quote sugar, integers, floats, [#t]/[#f], strings with escapes
    for the double quote and the backslash, symbols, dotted pairs, and [;]
    line comments. Shared by the chapter 4 evaluator driver and the
    chapter 5 loaders. *)

(** A place in the input text: 1-based line, 0-based column. *)
type position =
  { line : int
  ; column : int
  }

type t =
  | Unexpected_eof of position
  (** [Unexpected_eof p] is input that ends while a form is still
          open. *)
  | Unexpected_char of char * position
  (** [Unexpected_char (c, p)] is [c] where no form may start, such as
          a stray close parenthesis or a character no symbol may contain. *)
  | Bad_number of string * position
  (** [Bad_number (token, p)] is a numeric-looking token that neither
          the integer nor the float grammar accepts. *)
  | Bad_literal of string * position
  (** [Bad_literal (token, p)] is a [#]-token other than [#t] and
          [#f]. *)
  | Bad_string of string * position
  (** [Bad_string (detail, p)] is an unterminated string or a string
          with an escape other than a double quote or a backslash. *)
  | Bad_form of string * position
  (** [Bad_form (detail, p)] is a well-parenthesized form the subset
          does not allow: an empty application, a stray dot, a special
          form with the wrong arity, or a smart-constructor rejection. *)

(** [pp ppf e] prints [e] as one line with its position. *)
val pp : Format.formatter -> t -> unit

(** [to_string e] is [pp] rendered to a string. *)
val to_string : t -> string

(** [read text] is the first form of [text] as an expression, skipping
    leading whitespace and comments and ignoring everything after that
    form, as Scheme's [read] does. [Error] when [text] holds no form or a
    malformed one. *)
val read : string -> (Ast.expr, t) result

(** [read_program text] is every top-level form of [text] as an expression
    list, definitions included, in order. An input of only whitespace and
    comments reads as [[]]. *)
val read_program : string -> (Ast.expr list, t) result
