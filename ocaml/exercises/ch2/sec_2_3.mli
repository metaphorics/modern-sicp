(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.3 *)

(** Exercise 2.3: two representations for a rectangle, and one
    [perimeter]/[area] pair that works against either through a shared
    [Rectangle] signature. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

type point = float * float

module type Rectangle = sig
  type t

  val width : t -> float
  val height : t -> float
end

(** [perimeter] and [area], stated once against [Rectangle] and reused
    unchanged against both representations below. *)
module Perimeter_area (R : Rectangle) : sig
  val perimeter : R.t -> float
  val area : R.t -> float
end

(** First representation: two opposite, axis-aligned corners. *)
module Two_corners : sig
  type t

  val make : point -> point -> t
  val width : t -> float
  val height : t -> float
end

(** Second representation: one corner plus a width and a height. *)
module Corner_and_dims : sig
  type t

  val make : point -> float -> float -> t
  val width : t -> float
  val height : t -> float
end

(** [ex_2_03 ()] is [(perimeter, area)] for the same 3-by-4 rectangle
    built both ways, proving [Perimeter_area] needed no change:
    [((p1, a1), (p2, a2))] where [p1 = p2] and [a1 = a2]. *)
val ex_2_03 : unit -> (float * float) * (float * float)
