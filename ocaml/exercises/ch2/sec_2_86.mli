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

module Make_complex (S : Scalar) : sig
  type t = S.t * S.t

  val make_from_real_imag : S.t -> S.t -> t
  val make_from_mag_ang : S.t -> S.t -> t
  val real_part : t -> S.t
  val imag_part : t -> S.t
  val magnitude : t -> S.t
  val angle : t -> S.t
end

module Float_scalar : Scalar with type t = float

module Complex_float : sig
  type t = float * float

  val make_from_real_imag : float -> float -> t
  val make_from_mag_ang : float -> float -> t
  val real_part : t -> float
  val imag_part : t -> float
  val magnitude : t -> float
  val angle : t -> float
end

val ex_2_86 : unit -> float * float * float * float
