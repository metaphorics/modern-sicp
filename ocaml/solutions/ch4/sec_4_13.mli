(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.13: make-unbound! removes a binding. The statement
    lives in the section; this signature is the pending exercise's
    public contract. *)

(** An environment whose frames are association lists of name-value
      pairs, newest frame first. *)
type env

(** [make_unbound_ name env] removes the binding of [name] from the
      newest frame only and answers whether it removed one. The
      specification decision: only the newest frame, so an outer
      binding becomes visible again and no enclosing binding is
      ever destroyed. *)
val make_unbound_ : string -> env -> (bool, Sicp_common.Eval_error.t) result

val lookup_variable_value
  :  string
  -> env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

val define_variable_
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [ex_4_13 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_13 : unit -> string list
