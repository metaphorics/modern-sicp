(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Expressions, values, and types: the named definitions behind the
    listings of section 0.2. *)

(** [average_of_two_ints a b] adds [a] and [b] as [int], then crosses to
    [float] in one explicit [Float.of_int] step before dividing. OCaml
    never converts [int] to [float] on its own, so [a + b] stays [int]
    arithmetic and [Float.of_int (a + b)] is the whole expression's
    single conversion. *)
val average_of_two_ints : int -> int -> float

(** [warm_enough celsius] is the [bool] answer to whether [celsius] is at
    least [18.0], the section's example of a comparison operator that
    produces [bool] rather than [int]. *)
val warm_enough : float -> bool

(** [greet name] concatenates three [string] values with [^], the
    section's [string] operator. *)
val greet : string -> string
