(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.75 *)

(** Exercise 2.75: [make_from_mag_ang] in message-passing style,
    analogous to the book's own [make_from_real_imag]. *)

type op =
  | Real_part
  | Imag_part
  | Magnitude
  | Angle

type t = op -> float

let apply_generic op (z : t) = z op

let make_from_real_imag x y : t = function
  | Real_part -> x
  | Imag_part -> y
  | Magnitude -> sqrt ((x *. x) +. (y *. y))
  | Angle -> atan2 y x
;;

let ex_2_75 r a : t = function
  | Real_part -> r *. cos a
  | Imag_part -> r *. sin a
  | Magnitude -> r
  | Angle -> a
;;
