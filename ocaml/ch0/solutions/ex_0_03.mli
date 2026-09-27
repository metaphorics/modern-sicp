(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.3: define a [shape] variant with an [area] function; then
    add a constructor and let the compiler list every site that breaks.

    This module is the finished state: [Triangle of float * float] was
    added after the fact, and the compiler answered with one warning
    naming [area] as the match that no longer covers [Triangle]. *)

(** The final shape type: the scaffold's two constructors plus the added
    [Triangle], which carries a base and a height. *)
type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float

(** [area shape] covers every constructor: [area (Circle 2.0)] is
    [12.5663706143591725], [area (Rectangle (3.0, 4.0))] is [12.], and
    [area (Triangle (6.0, 4.0))] is [12.]. *)
val area : shape -> float
