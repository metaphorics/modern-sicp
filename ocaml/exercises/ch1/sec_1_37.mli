(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cont-frac in SICP section 1.3
   exercise 1.37 *)

(** Exercise 1.37: [cont_frac], a [k]-term finite continued fraction.
    The stubs raise [Sicp_common.Pending.Pending_solution] until they
    are solved. *)

(** [cont_frac n d k] is the [k]-term finite continued fraction
    [n 1 / (d 1 + n 2 / (d 2 + ... + n k / d k))]. *)
val cont_frac : (int -> float) -> (int -> float) -> int -> float

(** [smallest_k_for_4_decimal_places ()] is the smallest [k] at and
    beyond which [cont_frac (fun _ -> 1.) (fun _ -> 1.) k] stays
    accurate to 4 decimal places of [1. /. phi]. *)
val smallest_k_for_4_decimal_places : unit -> int

(** [ex_1_37 ()] is [(cont_frac (fun _ -> 1.) (fun _ -> 1.) k,
    smallest_k_for_4_decimal_places ())], for that [k]. *)
val ex_1_37 : unit -> float * int
