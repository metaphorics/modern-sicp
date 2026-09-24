(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.11: frames as association lists. The statement lives
    in the section; this signature is the pending exercise's public
    contract. *)

(** An environment whose frames are association lists of name-value
    pairs, newest frame first: the alternative representation the
    statement asks for, owned by this module. *)
type env

(** [extend_environment names values base_env] is a fresh frame
      binding each name to the value at the same position, in front of
      [base_env]; it answers an arity error when the lists differ. *)
val extend_environment
  :  string list
  -> Sicp_common.Value.t list
  -> env
  -> (env, Sicp_common.Eval_error.t) result

(** [lookup_variable_value name env] is the value of the nearest
      binding of [name], or [Error (Unbound_variable name)]. *)
val lookup_variable_value
  :  string
  -> env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [set_variable_value_ name value env] rebinds the nearest binding
      of [name]. *)
val set_variable_value_
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [define_variable_ name value env] binds [name] in the newest
      frame. *)
val define_variable_
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [ex_4_11 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_11 : unit -> string list
