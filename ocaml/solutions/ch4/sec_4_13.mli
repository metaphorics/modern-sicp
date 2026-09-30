(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.13 *)

(** Exercise 4.13: removing a binding.

    [make_unbound] removes a binding from the first frame of the
    environment only.  A frame further out is shared: every procedure
    created in it, and every call frame extending it, sees the same
    bindings, so removing a name there would change the meaning of code
    that never asked for the removal.  Removing the first frame's
    binding can only uncover an outer binding of the same name. *)

(** An environment of association-list frames. *)
type env

(** [empty] is the environment with no frames. *)
val empty : env

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

(** [ex_4_13 ()] builds a global frame over [x] and [y] and two call
    frames extending it, one shadowing [x] and defining [w].  It removes
    the shadowing [x] and shows the global [x] reappear, refuses to
    remove the global [y] through the call frame and shows the other
    call still sees it, and removes the defined [w]. *)
val ex_4_13 : unit -> (string list, Sicp_common.Eval_error.t) result
