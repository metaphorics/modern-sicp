(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.51:
   below, two ways *)

type vect =
  { x : float
  ; y : float
  }

type frame =
  { origin : vect
  ; edge1 : vect
  ; edge2 : vect
  }

type segment = vect * vect
type painter = frame -> segment list

(** The direct construction, analogous to [beside]. *)
val ex_2_51_below : painter -> painter -> painter

(** The construction through [beside] and rotations. *)
val ex_2_51_below_rotate : painter -> painter -> painter
