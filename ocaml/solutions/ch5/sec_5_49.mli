(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.49: a read-compile-execute-print loop.

    No evaluator is involved: each top-level item is compiled into a new
    labelled block, the compiled-code machine of [Sicp_ch5.Sec_5_5.load]
    is assembled with every block so far and the shared runtime, and the
    new block runs with the global environment the earlier blocks built.
    Keeping the earlier blocks keeps the entry points of the compiled
    procedures they defined. *)

(** [read_compile_execute_print ~emit items] runs [items] through the
    loop and answers one printed line per item. *)
val read_compile_execute_print
  :  emit:(string -> unit)
  -> Sicp_common.Ast.item list
  -> (string list, Sicp_common.Eval_error.t) result

(** [source] defines [fib] and [double] and binds [fib 12] and
    [double 441]. *)
val source : string

(** [ex_5_49 ()] runs [source] through the loop. *)
val ex_5_49 : unit -> (string list, Sicp_common.Eval_error.t) result
