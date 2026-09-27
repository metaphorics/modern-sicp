(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.86 *)

module type Scalar = sig
  type t

  val add : t -> t -> t
  val mul : t -> t -> t
  val sine : t -> t
  val cosine : t -> t
  val sqrt : t -> t
  val atan2 : t -> t -> t
end

module Make_complex (S : Scalar) = struct
  type t = S.t * S.t

  let make_from_real_imag x y : t = x, y
  let real_part ((x, _) : t) = x
  let imag_part ((_, y) : t) = y
  let magnitude ((x, y) : t) = S.sqrt (S.add (S.mul x x) (S.mul y y))
  let angle ((x, y) : t) = S.atan2 y x
  let make_from_mag_ang r a : t = S.mul r (S.cosine a), S.mul r (S.sine a)
end

module Float_scalar = struct
  type t = float

  let add = ( +. )
  let mul = ( *. )
  let sine = sin
  let cosine = cos
  let sqrt = Stdlib.sqrt
  let atan2 = Stdlib.atan2
end

module Complex_float = Make_complex (Float_scalar)

let ex_2_86 () =
  let z = Complex_float.make_from_real_imag 3.0 4.0 in
  ( Complex_float.real_part z
  , Complex_float.imag_part z
  , Complex_float.magnitude z
  , Complex_float.angle z )
;;
