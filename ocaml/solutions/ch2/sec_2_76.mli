(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.76 *)

(** Explicit dispatch: a closed variant recognized by pattern
    matching, this edition's analogue of 2.4.2's [rectangular?]/
    [polar?] cond. *)
type explicit_z =
  | Rectangular of float * float
  | Polar of float * float

val real_part_explicit : explicit_z -> float
val magnitude_explicit : explicit_z -> float

(** Data-directed: an operation-and-type table, this edition's
    analogue of 2.4.3, scaled down to two operations. *)
type dd_value =
  | Num of float
  | Pair of float * float

type dd_tagged =
  { tag : string
  ; contents : dd_value
  }

type dd_table

val dd_make_table : unit -> dd_table
val dd_install_rectangular : dd_table -> unit
val dd_install_polar : dd_table -> unit

(** [dd_apply_generic table op tagged] looks [op] up under
    [tagged]'s tag and applies it to the untagged contents.
    [invalid_arg] when no package installed [op] for that tag. *)
val dd_apply_generic : dd_table -> string -> dd_tagged -> float

(** Message passing: each value is a closure that answers its own
    operation, this edition's analogue of Exercise 2.75. *)
type mp_op =
  | Real_part
  | Magnitude

type mp_z = mp_op -> float

val mp_rectangular : float -> float -> mp_z
val mp_polar : float -> float -> mp_z

(** A complex number's [real_part] paired with its [magnitude]. *)
type sample = float * float

(** [ex_2_76 ()] builds a rectangular complex number (3, 4) and a
    polar one (5, 0) three ways -- explicit dispatch, data-directed,
    and message-passing -- and returns each style's
    [(rectangular sample, polar sample)], to check the three
    organizations agree on what a complex number's [real_part] and
    [magnitude] are. *)
val ex_2_76 : unit -> (sample * sample) * (sample * sample) * (sample * sample)
