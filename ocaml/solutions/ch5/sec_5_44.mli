(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.44: open coding that respects shadowing.

    The operators of the subset are syntax and cannot be rebound, but
    the prelude procedures that exercise 5.38 open-codes are ordinary
    names: a parameter, a [let], or a top-level binding may shadow
    [string_of_int].  The 5.38 compiler open-codes such a call anyway and
    computes the wrong answer.  This compiler open-codes a call only when
    the operator's name is bound in no frame of its compile-time
    environment (exercise 5.40), so shadowed names compile as ordinary
    calls. *)

(** [compile_scoped environments state e target linkage] is the 5.38
    compiler restricted to operators the compile-time environments
    [environments] leave unbound. *)
val compile_scoped
  :  Sec_5_40.t
  -> Sicp_ch5.Sec_5_5.state
  -> Sicp_common.Ast.expr
  -> string
  -> Sicp_ch5.Sec_5_5.linkage
  -> Sicp_ch5.Sec_5_5.seq

(** [shadowing] passes a procedure in a parameter named
    [string_of_int]. *)
val shadowing : string

(** [free] calls the prelude [string_of_int]. *)
val free : string

(** [ex_5_44 ()] runs [shadowing] and [free] under both compilers and
    reports the open-coded counts and answers. *)
val ex_5_44 : unit -> (string list, Sicp_common.Eval_error.t) result
