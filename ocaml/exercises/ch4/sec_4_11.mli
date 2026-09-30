(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.11 *)

(** Exercise 4.11: frames as association lists. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** An environment of association-list frames. *)
type env

(** [extend_environment names values env] is [env] with a new first
    frame binding each name to the value in the same position, or an
    [Arity_mismatch] error when the lists differ in length. *)
val extend_environment
  :  string list
  -> Sicp_common.Value.t list
  -> env
  -> (env, Sicp_common.Eval_error.t) result

(** [lookup_variable_value name env] is the value of the nearest binding
    of [name], or an [Unbound_variable] error. *)
val lookup_variable_value
  :  string
  -> env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [set_variable_value name value env] changes the nearest binding of
    [name] to [value], or answers an [Unbound_variable] error. *)
val set_variable_value
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [define_variable name value env] binds [name] to [value] in the
    first frame of [env], replacing a binding already there.  The empty
    environment has no frame to define in. *)
val define_variable
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [frames env] is the bindings of each frame of [env], innermost frame
    first and newest binding first. *)
val frames : env -> (string * Sicp_common.Value.t) list list

(** [show_frames env] is [frames env] printed as one line. *)
val show_frames : env -> string

(** [ex_4_11 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_11 : unit -> (string list, Sicp_common.Eval_error.t) result
