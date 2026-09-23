(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.48: directed line
   segments. *)

type vect =
  { x : float
  ; y : float
  }

(** The pair of the start-point and end-point vectors. *)
type segment = vect * vect

val make_segment : vect -> vect -> segment
val start_segment : segment -> vect
val end_segment : segment -> vect

(** The segment from [1, 2] to [3, 4]. *)
val ex_2_48 : unit -> segment
