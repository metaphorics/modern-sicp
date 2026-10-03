(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.36: the compiler's operand order, and a compiler that
    evaluates operands in the other order.

    The edition's compiler evaluates operands left to right: its
    argument-list constructor compiles the operands in list order and
    appends each value with [adjoin-arg].  The variant here compiles the
    operands last to first and puts each value in front of the list with
    [prepend-arg]; the argument list it builds is the same.  The
    instruction count does not change, but each [adjoin-arg] copies the
    list it extends while each [prepend-arg] allocates one cell, so the
    right-to-left order builds an argument list of n operands in linear
    rather than quadratic time. *)

(** [prepend_arg] is the [prepend-arg] machine operation. *)
val prepend_arg : string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op

(** [compile_right_to_left state e target linkage] is the compiler with
    operands evaluated last to first. *)
val compile_right_to_left
  :  Sicp_ch5.Sec_5_5.state
  -> Sicp_common.Ast.expr
  -> string
  -> Sicp_ch5.Sec_5_5.linkage
  -> Sicp_ch5.Sec_5_5.seq

(** [source] is a unit whose call prints each operand as it is
    evaluated. *)
val source : string

(** [ex_5_36 ()] runs [source] under both compilers and reports the
    printed operand orders, the statement and step counts, and the two
    answers. *)
val ex_5_36 : unit -> (string list, Sicp_common.Eval_error.t) result
