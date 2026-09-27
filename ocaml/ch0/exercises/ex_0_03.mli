(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.3's statement is in the book, section 0.4; the shape type
    and [area] below are the starting point it names. *)

(** The starting shape type: circles carry a radius, rectangles a width
    and a height. *)
type shape =
  | Circle of float
  | Rectangle of float * float

(** [area shape] dispatches on the constructor. *)
val area : shape -> float
