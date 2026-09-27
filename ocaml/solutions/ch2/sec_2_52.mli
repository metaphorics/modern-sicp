(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.52: square limit
   variations, and the added 2.52a. *)

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

(** The section's wave painter, the base of the variations. *)
val wave : painter

(** The wave painter of (a), with added segments forming a smile. *)
val ex_2_52_wave : painter

(** The corner split of (b), one copy of each split image per
    corner. *)
val ex_2_52_corner_split : painter -> int -> painter

(** The square limit of (c), assembling the corners in the changed
    pattern. *)
val ex_2_52_square_limit : painter -> int -> painter

(** Exercise 2.52a: the SVG text of the square limit of [wave] at
    level 4, rendered through the view box ["-0.1 -0.1 1.2 1.2"] by
    [render_view_box]. The golden file test/ch2/golden_ex_2_52a.svg is
    checked against this output byte for byte. *)
val ex_2_52a : unit -> string
