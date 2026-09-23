(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.47: two frame
   constructors. *)

type vect =
  { x : float
  ; y : float
  }

(** The flat triple. *)
type frame1 = vect * vect * vect

(** The origin paired with a pair of edges. *)
type frame2 = vect * (vect * vect)

val make_frame1 : vect -> vect -> vect -> frame1
val origin_frame1 : frame1 -> vect
val edge1_frame1 : frame1 -> vect
val edge2_frame1 : frame1 -> vect
val make_frame2 : vect -> vect -> vect -> frame2
val origin_frame2 : frame2 -> vect
val edge1_frame2 : frame2 -> vect
val edge2_frame2 : frame2 -> vect

(** Both representations built from the same three corners of the unit
    square. *)
val ex_2_47 : unit -> frame1 * frame2
