(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.85 *)

(** A three-level tower -- integer, real, complex -- narrow enough to
    keep [drop] focused on the book's own three worked examples,
    which never need the rational level. *)
type value =
  | Int of int
  | Real of float
  | Cpx of float * float
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
val tower_level : string -> int

(** Installs ["raise"] for "integer" and "real", and ["project"] for
    "real" (round to the nearest integer) and "complex" (drop the
    imaginary part) -- the two projections the exercise's own footnote
    and text ask for. *)
val install_raise_and_project : unit -> unit

val install_homogeneous_add : unit -> unit
val raise_one_level : value -> value

(** [apply_generic op args] is 2.84's successive-raising dispatch,
    restated for this section's three-level tower. *)
val apply_generic : string -> value list -> value

(** [drop v] projects [v] one level down, raises the result back, and
    keeps going while raising back reproduces [v] exactly -- the
    project-then-raise equality test the exercise specifies. A value
    with no ["project"] entry (the bottom, "integer") returns
    unchanged. *)
val drop : value -> value

(** [apply_generic_drop op args] is [apply_generic]'s result, dropped:
    the "simplifies its answers" rewrite the exercise's last sentence
    asks for. *)
val apply_generic_drop : string -> value list -> value

(** [ex_2_85 ()] drops 1.5+0i, 1+0i, and 2+3i to the type tag each one
    settles at, then adds (2,3) to (-2,-3) through
    [apply_generic_drop] and reports the sum's settled tag. *)
val ex_2_85 : unit -> string * string * string * string

(** Exercise 2.85a is added by this edition and extends exercise
    2.85; SICP numbers stop at 2.85. The idiom note for 2.5 picks this
    addition: coercion can loop (2.81's Louis Reasoner bug is exactly
    a self-loop in this graph), so before installing a coercion table
    this edition can ask whether it describes a directed acyclic
    graph. *)

(** A coercion edge [(from_type, to_type)], one entry per installed
    [put_coercion]. *)
type edge = string * string

(** [has_cycle edges] runs a three-color depth-first search (white:
    unseen, gray: on the current path, black: finished) over the
    directed graph [edges] describes, following the same "detect a
    back edge to a gray node" test any graph-cycle detector uses.
    A coercion from a type to itself is the smallest possible cycle:
    one gray node with an edge back to itself. *)
val has_cycle : edge list -> bool

(** [ex_2_85a ()] is [(acyclic_tower, self_coercion_cycle,
    three_step_cycle)]: the tower's chain of coercions has no cycle;
    Exercise 2.81's Louis Reasoner self-coercion
    [("scheme-number", "scheme-number")] is a one-edge cycle, the
    exact graph shape behind that exercise's infinite recursion; and
    an [a -> b -> c -> a] chain is a three-edge cycle. *)
val ex_2_85a : unit -> bool * bool * bool
