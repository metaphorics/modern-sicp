(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.81 *)

type value =
  | Num of float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

val attach_tag : string -> value -> tagged
val type_tag : tagged -> string
val contents_of : tagged -> value
val put : string -> string list -> (value list -> value) -> unit
val get : string -> string list -> (value list -> value) option
val put_coercion : string -> string -> (value -> value) -> unit
val get_coercion : string -> string -> (value -> value) option

(** Raised by [apply_generic_loop] once its depth counter passes
    [max_depth], standing in for the stack overflow an unguarded
    infinite recursion would eventually hit. *)
exception Loop_detected of int

(** [apply_generic_loop depth op args] is 2.5.2's coercion
    [apply_generic], with an explicit depth counter in place of an
    unbounded call stack. *)
val apply_generic_loop : int -> string -> value list -> value

(** [apply_generic_fixed op args] is part (c)'s fix: coercion is never
    attempted when both arguments already share a type. *)
val apply_generic_fixed : string -> value list -> value

(** Installs Louis's two self-coercions
    ([real -> real] and [complex -> complex], both
    the identity) plus ["exp"] for ["real"; "real"]
    only, exactly as the exercise states. *)
val install_louis_setup : unit -> unit

(** [ex_2_81_a ()] calls [apply_generic_loop] with ["exp"] on two
    "complex" arguments -- an operation with no ["complex"; "complex"]
    entry, but a self-coercion Louis installed regardless. It returns
    the depth [Loop_detected] carries, proving the call recurses
    without making progress until the guard stops it. *)
val ex_2_81_a : unit -> int

(** [ex_2_81_c ()] is [true] when the same call through
    [apply_generic_fixed] raises immediately instead of looping. *)
val ex_2_81_c : unit -> bool
