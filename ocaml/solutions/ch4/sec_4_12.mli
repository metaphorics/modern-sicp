(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.12: abstract environment traversals. The statement
    lives in the section; this signature is the pending exercise's
    public contract. *)

(** An environment whose frames are association lists of name-value
      pairs, newest frame first. *)
type env

(** [traverse env name inspect_frame] walks the frames newest first,
      applying [inspect_frame] to each frame's association list, and
      answers the first inspection that binds the name, or [None]
      when no frame does. The three environment operations below are
      expressed in terms of this one traversal. *)
val traverse
  :  env
  -> string
  -> ((string * Sicp_common.Value.t) list -> Sicp_common.Value.t option)
  -> Sicp_common.Value.t option

val lookup_variable_value
  :  string
  -> env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

val set_variable_value_
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [define_variable_ name value env] binds [name] in the newest
      frame, through the same frame abstraction. *)
val define_variable_
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [ex_4_12 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_12 : unit -> string list
