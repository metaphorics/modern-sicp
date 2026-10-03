(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.38: open-coding calls of primitive procedures.

    The edition's compiler already open-codes its operators: [a + 1]
    compiles to two loads into [arg1] and [arg2] and one [apply-binary].
    A call of a prelude procedure such as [Array.get a i] still compiles
    to the general call: an argument list, a lookup of the procedure, the
    [primitive-exact?] test, and both branches.  This compiler open-codes
    the saturated calls of the prelude procedures in [open_coded]: the
    operands are spread into [arg1] and [arg2] and one machine operation
    of the procedure's name computes the result.  Like the book's
    version it treats those names as reserved; exercise 5.44 repairs
    that. *)

(** [open_coded] is each open-coded prelude name with its arity. *)
val open_coded : (string * int) list

(** [end_with_linkage linkage seq] is [seq] followed by the code of
    [linkage], preserving [continue] for a [Return]. *)
val end_with_linkage
  :  Sicp_ch5.Sec_5_5.linkage
  -> Sicp_ch5.Sec_5_5.seq
  -> Sicp_ch5.Sec_5_5.seq

(** [spread_arguments compile state operands operation] compiles
    [operands] into [arg1] and [arg2] in order, preserving [env] and the
    argument registers already filled around each later operand, then
    runs [operation]. *)
val spread_arguments
  :  (Sicp_ch5.Sec_5_5.state
      -> Sicp_common.Ast.expr
      -> string
      -> Sicp_ch5.Sec_5_5.linkage
      -> Sicp_ch5.Sec_5_5.seq)
  -> Sicp_ch5.Sec_5_5.state
  -> Sicp_common.Ast.expr list
  -> Sicp_ch5.Sec_5_5.seq
  -> Sicp_ch5.Sec_5_5.seq

(** [open_coded_call compile state name operands target linkage] is the
    open-coded call of the prelude procedure [name]. *)
val open_coded_call
  :  (Sicp_ch5.Sec_5_5.state
      -> Sicp_common.Ast.expr
      -> string
      -> Sicp_ch5.Sec_5_5.linkage
      -> Sicp_ch5.Sec_5_5.seq)
  -> Sicp_ch5.Sec_5_5.state
  -> string
  -> Sicp_common.Ast.expr list
  -> string
  -> Sicp_ch5.Sec_5_5.linkage
  -> Sicp_ch5.Sec_5_5.seq

(** [open_coded_name operator operands] is the open-coded name the call
    of [operator] on [operands] invokes, if any. *)
val open_coded_name : Sicp_common.Ast.expr -> Sicp_common.Ast.expr list -> string option

(** [compile_open_coding state e target linkage] is the compiler with
    open-coded prelude calls. *)
val compile_open_coding
  :  Sicp_ch5.Sec_5_5.state
  -> Sicp_common.Ast.expr
  -> string
  -> Sicp_ch5.Sec_5_5.linkage
  -> Sicp_ch5.Sec_5_5.seq

(** [operations] is one machine operation per open-coded name. *)
val operations : (string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op) list

(** [open_coded_count seq] is the number of open-coded operations
    [seq] performs. *)
val open_coded_count : Sicp_ch5.Sec_5_5.seq -> int

(** [ex_5_38 ()] compiles an array sum with and without open coding,
    reports sizes, open-coded counts, answers, and steps, and lists the
    open-coded statements of the summing procedure. *)
val ex_5_38 : unit -> (string list, Sicp_common.Eval_error.t) result
