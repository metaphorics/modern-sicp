(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.46:
   the vector abstraction *)

type vect =
  { x : float
  ; y : float
  }

val make_vect : float -> float -> vect
val xcor_vect : vect -> float
val ycor_vect : vect -> float
val add_vect : vect -> vect -> vect
val sub_vect : vect -> vect -> vect
val scale_vect : float -> vect -> vect

(** [add_vect (1, 2) (3, 4)], [sub_vect (3, 4) (1, 2)], and
    [scale_vect 2.0 (1, 2)], the statement's operations on one fixed
    set of inputs. *)
val ex_2_46 : unit -> vect * vect * vect
