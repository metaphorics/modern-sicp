(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.3 *)

(** Exercise 2.3: two rectangle representations sharing one
    [perimeter]/[area] pair. *)

type point = float * float

module type Rectangle = sig
  type t

  val width : t -> float
  val height : t -> float
end

module Perimeter_area (R : Rectangle) : sig
  val perimeter : R.t -> float
  val area : R.t -> float
end

module Two_corners : sig
  type t

  val make : point -> point -> t
  val width : t -> float
  val height : t -> float
end

module Corner_and_dims : sig
  type t

  val make : point -> float -> float -> t
  val width : t -> float
  val height : t -> float
end

val ex_2_03 : unit -> (float * float) * (float * float)
