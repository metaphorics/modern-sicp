(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.86 *)

(** The operations a complex number's real part, imaginary part,
    magnitude, and angle need from whatever numeric type backs them
    -- ordinary numbers, rationals, or "other numbers we might wish
    to add to the system," in the exercise's own words. A functor
    over this signature builds one [real_part]/[magnitude]/etc. that
    works for any scalar satisfying it, instead of a dispatch table
    entry per scalar type. *)
module type Scalar = sig
  type t

  val add : t -> t -> t
  val mul : t -> t -> t
  val sine : t -> t
  val cosine : t -> t
  val sqrt : t -> t
  val atan2 : t -> t -> t
end

(** [Make_complex(S)] is a complex-number package generic over [S.t]:
    real and imaginary parts, magnitude via [S.sqrt] of the sum of
    squares, angle via [S.atan2], and the two constructors, all built
    from [S]'s six operations and nothing else. *)
module Make_complex (S : Scalar) : sig
  type t = S.t * S.t

  val make_from_real_imag : S.t -> S.t -> t
  val make_from_mag_ang : S.t -> S.t -> t
  val real_part : t -> S.t
  val imag_part : t -> S.t
  val magnitude : t -> S.t
  val angle : t -> S.t
end

(** The ordinary-number instance: OCaml's own [float] already
    satisfies [Scalar]. *)
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

(** [ex_2_86 ()] builds the rectangular (3, 4) through
    [Complex_float.make_from_real_imag] and returns its
    [(real_part, imag_part, magnitude, angle)]. *)
val ex_2_86 : unit -> float * float * float * float
