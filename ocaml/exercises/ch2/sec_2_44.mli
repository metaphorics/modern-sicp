(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.44:
   up-split *)

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

(** [ex_2_44_up_split painter n] branches [painter] upwards [n]
    times. *)
val ex_2_44_up_split : painter -> int -> painter
