(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.75 *)

(** The operation a message-passing complex number understands. *)
type op =
  | Real_part
  | Imag_part
  | Magnitude
  | Angle

(** A message-passing complex number: a closure that answers a
    dispatched [op] with a [float]. *)
type t = op -> float

(** [apply_generic op z] sends [op] to [z]. *)
val apply_generic : op -> t -> float

(** [make_from_real_imag x y] is the book's own message-passing
    complex number, built from real and imaginary parts -- the model
    this exercise asks to follow. *)
val make_from_real_imag : float -> float -> t

(** [ex_2_75 r a] is [make_from_mag_ang] in message-passing style,
    analogous to [make_from_real_imag]. *)
val ex_2_75 : float -> float -> t
