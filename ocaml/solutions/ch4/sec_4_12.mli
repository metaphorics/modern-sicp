(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.12 *)

(** Exercise 4.12: the environment operations over shared traversals.

    Frames keep the book's representation, a list of names beside a
    list of value cells.  The three operations share two abstractions:
    [find_in_frame] scans one frame and [find_binding] scans the frames
    from the innermost outward.  Lookup and assignment are the whole
    traversal followed by a read or a write; definition is the
    one-frame scan followed by a write or an insertion. *)

(** An environment of two-list frames. *)
type env

(** [empty] is the environment with no frames. *)
val empty : env

(** [extend_environment names values env] is [env] with a new first
    frame, or an [Arity_mismatch] error when the lists differ in
    length. *)
val extend_environment
  :  string list
  -> Sicp_common.Value.t list
  -> env
  -> (env, Sicp_common.Eval_error.t) result

(** [find_binding name env] is the cell of the nearest binding of [name]
    in [env], or [None] if no frame binds it. *)
val find_binding : string -> env -> Sicp_common.Value.t ref option

(** [lookup_variable_value name env] reads the cell [find_binding]
    finds, or answers an [Unbound_variable] error. *)
val lookup_variable_value
  :  string
  -> env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [set_variable_value name value env] writes the cell [find_binding]
    finds, or answers an [Unbound_variable] error. *)
val set_variable_value
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [define_variable name value env] writes [name]'s cell in the first
    frame of [env], adding one when the frame has none.  The empty
    environment has no frame to define in. *)
val define_variable
  :  string
  -> Sicp_common.Value.t
  -> env
  -> (unit, Sicp_common.Eval_error.t) result

(** [ex_4_12 ()] builds an outer frame over [a] and [b] and an inner one
    shadowing [a], defines [c] and redefines [a] in the inner frame, and
    sets [b] through it.  It answers the inner and outer [a], the outer
    [b], [c] seen from each frame, and an assignment to an unbound
    name. *)
val ex_4_12 : unit -> (string list, Sicp_common.Eval_error.t) result
