(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.82 *)

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

(** [coerce_all_to target args] tries to coerce every argument to
    [target]'s type, [None] if any argument has no path there. *)
val coerce_all_to : value -> value list -> value list option

(** [apply_generic_n op args] is [apply_generic] generalized to any
    number of arguments: try the direct lookup, then try coercing
    every argument to the type of each argument in turn, stopping at
    the first type that works for all of them. *)
val apply_generic_n : string -> value list -> value

(** [ex_2_82_a ()] installs ["add3"] for three ["real"]
    arguments plus a ["rational" -> "real"] coercion, then
    calls [apply_generic_n] on a mix of one rational and two
    reals: coercing every argument to the first argument's
    type finds the installed entry. *)
val ex_2_82_a : unit -> value

(** [ex_2_82_b ()] is the hint's counterexample: an operation installed
    only for [\["a"; "b"\]] with no [a -> b] or [b -> a] coercion in
    either direction, so trying to coerce every argument to one
    common type never finds it, even though the exact mixed-type
    entry exists. Returns [true] when [apply_generic_n] indeed fails
    to find it. *)
val ex_2_82_b : unit -> bool
