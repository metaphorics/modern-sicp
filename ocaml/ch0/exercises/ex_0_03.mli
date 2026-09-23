(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.3: define a [shape] variant with an [area] function; then
    add a constructor and let the compiler list every site that breaks.

    The scaffold is the starting point: two constructors, both [area]
    arms pending. The reference solution adds [Triangle]. The stub arms
    raise [Sicp_common.Pending.Pending_solution] until solved. *)

(** The starting shape type: circles carry a radius, rectangles a width
    and a height. *)
type shape =
  | Circle of float
  | Rectangle of float * float

(** [area shape] dispatches on the constructor. *)
val area : shape -> float
