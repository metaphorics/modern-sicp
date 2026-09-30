(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.13 *)

(** Exercise 4.13: removing a binding. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** An environment of association-list frames. *)
type env

(** [extend bindings env] is [env] with a new first frame holding
    [bindings]. *)
val extend : (string * Sicp_common.Value.t) list -> env -> env

(** [lookup_variable_value name env] is the value of the nearest binding
    of [name], or an [Unbound_variable] error. *)
val lookup_variable_value
  :  string
  -> env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [define_variable name value env] binds [name] in the first frame of
    [env], replacing a binding already there. *)
val define_variable
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [make_unbound name env] removes the binding of [name] from the first
    frame of [env], or answers an [Unbound_variable] error when that
    frame does not bind [name], even if an outer frame does. *)
val make_unbound : string -> env -> (unit, Sicp_common.Eval_error.t) result

(** [ex_4_13 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_13 : unit -> (string list, Sicp_common.Eval_error.t) result
