(* SPDX-License-Identifier: GPL-3.0-only *)

(** The evaluator environment: a lexical chain of immutable bindings,
    newest first, the environment model of SICP section 4.1.3.  Closures
    capture it; mutation lives in references, never in the environment,
    so every closure over one reference observes the same cell (grammar
    section 5).

    A [let rec] group binds its names through fresh cells before its
    right-hand sides run, so a recursive closure captures the whole
    group exactly as grammar section 4 requires.  A cell read while the
    group's right-hand sides are still running answers the value the
    cell will hold: the statically constructive recursive values the
    pinned compiler admits (grammar section 4), such as a closure whose
    captured environment names the closure itself.

    [Env.t] and [Value.env] are the same type; the representation is
    owned by [Value] because closures capture it. *)

(** An environment. *)
type t = Value.env

(** A cell of a recursive binding group; [fill] completes it. *)
type cell = Value.t option ref

(** [empty] is the environment with no bindings. *)
val empty : t

(** [extend bindings env] is [env] with the newest frame of [bindings]
    in front; a name in [bindings] shadows any outer binding. *)
val extend : (string * Value.t) list -> t -> t

(** [bind name value env] is [extend [(name, value)] env]. *)
val bind : string -> Value.t -> t -> t

(** [extend_recursive names env] is the environment binding each name of
    [names] to a fresh empty cell in front of [env], paired with those
    cells in order. *)
val extend_recursive : string list -> t -> t * cell list

(** [fill cell value] completes the recursive binding [cell]. *)
val fill : cell -> Value.t -> unit

(** [find env name] is the value [name] is bound to in the nearest
    binding of [env], or [None] when no binding does. *)
val find : t -> string -> Value.t option

(** [get_exn env name] is the value [name] is bound to, or raises
    [Not_found] when no binding does. *)
val get_exn : t -> string -> Value.t

(** [find_at env n] is the binding at position [n] of [env], counting
    from 0 at the newest, as its name and value; [None] when [n] is
    negative or past the end, or when the binding is a recursive cell
    not yet filled. *)
val find_at : t -> int -> (string * Value.t) option
