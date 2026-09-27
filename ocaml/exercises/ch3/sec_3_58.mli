(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.58 *)

(** Exercise 3.58: [expand] produces the successive digits of the
    fraction [num / den] written in the given radix. The map class is
    [T]: [quotient] and [remainder] become [/] and [mod] on integers. *)

(** [expand num den radix] is the infinite digit stream of
    [num / den] in [radix]: each element is the quotient of the scaled
    numerator, and the recursion continues on the remainder. *)
val expand : int -> int -> int -> int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_58 ()] is the first eight digits of 1/7 in base 10
    ([1; 4; 2; 8; 5; 7; 1; 4], the repeating 142857...) and the first
    five of 3/8 ([3; 7; 5; 0; 0], that is, 0.37500...). *)
val ex_3_58 : unit -> int list * int list
