(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.75 *)

type op =
  | Real_part
  | Imag_part
  | Magnitude
  | Angle

type t = op -> float

val apply_generic : op -> t -> float
val make_from_real_imag : float -> float -> t

(** [ex_2_75 r a] is [make_from_mag_ang] in message-passing style,
    analogous to [make_from_real_imag]. *)
val ex_2_75 : float -> float -> t
