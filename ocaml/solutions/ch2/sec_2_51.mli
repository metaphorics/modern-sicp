(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.51: below, two
   ways. *)

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

(** [ex_2_51_below bottom top] paints [bottom] in the lower half of
    the frame and [top] in the upper half, built directly like
    [beside]. *)
val ex_2_51_below : painter -> painter -> painter

(** The same operation built from [beside] and rotations; for any
    frame it draws exactly the segments of [ex_2_51_below]. *)
val ex_2_51_below_rotate : painter -> painter -> painter
