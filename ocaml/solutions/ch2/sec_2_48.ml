(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.48: directed line
   segments. *)

type vect =
  { x : float
  ; y : float
  }

type segment = vect * vect

let make_segment start_v end_v = start_v, end_v
let start_segment (start_v, _end_v) = start_v
let end_segment (_start_v, end_v) = end_v

(* A fixed segment from [1, 2] to [3, 4], for the selectors to be
   checked against. *)
let ex_2_48 () = make_segment { x = 1.0; y = 2.0 } { x = 3.0; y = 4.0 }
