(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.49: the four
   primitive painters. *)

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

(** The outline of the designated frame. *)
val ex_2_49_outline : painter

(** The ``X'' connecting opposite corners. *)
val ex_2_49_x : painter

(** The diamond connecting the side midpoints. *)
val ex_2_49_diamond : painter

(** A crude line drawing of a wave. *)
val ex_2_49_wave : painter
