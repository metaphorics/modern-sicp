(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.27 *)

(** Reference solution of exercise 1.27. *)

val expmod : int -> int -> int -> int

(** [fools_fermat n] holds when [a ** n = a (mod n)] for every
    [1 <= a < n]. *)
val fools_fermat : int -> bool

(** The six Carmichael numbers below 100,000,000 that
    @ref{Footnote 47} lists. *)
val carmichael_numbers : int list

(** [ex_1_27 ()] pairs each of [carmichael_numbers] with
    [fools_fermat], every entry [true]. *)
val ex_1_27 : unit -> (int * bool) list
