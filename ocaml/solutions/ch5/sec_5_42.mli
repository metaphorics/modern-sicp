(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.42: lexical addressing in the code generator.

    A variable whose name [find_variable] locates in its compile-time
    environment compiles to [lexical-address-lookup]; every other
    variable, a top-level prelude name, still compiles to
    [lookup-variable-value]. *)

(** [compile_lexical environments state e target linkage] is the
    compiler with lexical addressing over the compile-time environments
    [environments] of the unit [e] belongs to. *)
val compile_lexical
  :  Sec_5_40.t
  -> Sicp_ch5.Sec_5_5.state
  -> Sicp_common.Ast.expr
  -> string
  -> Sicp_ch5.Sec_5_5.linkage
  -> Sicp_ch5.Sec_5_5.seq

(** [compile_program_lexical items] is the unit [items] compiled with
    lexical addressing. *)
val compile_program_lexical : Sicp_common.Ast.item list -> Sicp_ch5.Sec_5_5.seq

(** [ex_5_42 ()] lists the lexical lookups of the nested example of
    exercise 5.40 and runs it, answering [180]. *)
val ex_5_42 : unit -> (string list, Sicp_common.Eval_error.t) result
