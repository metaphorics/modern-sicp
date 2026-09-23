(* SPDX-License-Identifier: GPL-3.0-only *)

(** The evaluator environment: a chain of mutable frames, newest first, as
    in the plan's environment model. [Env.t] and [Value.env] are the same
    type; the representation is owned by [Value] because compound
    procedures capture it. *)

(** An environment. The newest frame is searched first; [define] extends it
    and [set] mutates the nearest enclosing binding. *)
type t = Value.env

(** [empty ()] is the environment with one fresh global frame. *)
val empty : unit -> t

(** [extend names values outer] is a new environment whose fresh frame
    binds each name of [names] to the value at the same position of
    [values], in front of [outer]. Returns [Error (Arity_mismatch ...)]
    when the two lists differ in length. *)
val extend : string list -> Value.t list -> t -> (t, Eval_error.t) result

(** [find_binding env name] is the value [name] is bound to in the nearest
    frame of [env] that binds it, or [None] when no frame does. *)
val find_binding : t -> string -> Value.t option

(** [define env name value] binds [name] to [value] in the newest frame of
    [env], shadowing any outer binding. *)
val define : t -> string -> Value.t -> unit

(** [set env name value] rebinds [name] to [value] in the nearest
    enclosing frame that binds it, or returns [Error (Unbound_variable
    name)] when no frame does. *)
val set : t -> string -> Value.t -> (unit, Eval_error.t) result
