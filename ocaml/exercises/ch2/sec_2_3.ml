(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.3 *)

type point = float * float

module type Rectangle = sig
  type t

  val width : t -> float
  val height : t -> float
end

module Perimeter_area (R : Rectangle) = struct
  let perimeter r = 2.0 *. (R.width r +. R.height r)
  let area r = R.width r *. R.height r
end

module Two_corners : sig
  type t

  val make : point -> point -> t
  val width : t -> float
  val height : t -> float
end = struct
  type t = point * point

  let make _a _b = raise Sicp_common.Pending.Pending_solution
  let width _r = raise Sicp_common.Pending.Pending_solution
  let height _r = raise Sicp_common.Pending.Pending_solution
end

module Corner_and_dims : sig
  type t

  val make : point -> float -> float -> t
  val width : t -> float
  val height : t -> float
end = struct
  type t = point * float * float

  let make _corner _w _h = raise Sicp_common.Pending.Pending_solution
  let width _r = raise Sicp_common.Pending.Pending_solution
  let height _r = raise Sicp_common.Pending.Pending_solution
end

let ex_2_03 () = raise Sicp_common.Pending.Pending_solution
