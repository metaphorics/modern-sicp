(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cons in SICP section 2.1
   exercise 2.5 *)

(** Exercise 2.5: pairs of non-negative integers represented as the
    single integer [2^a * 3^b]. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved.

    This edition keeps [cons]'s result in OCaml's native (63-bit)
    [int] rather than reaching for an arbitrary-precision library: the
    idiom plan allows this ([Zarith stays optional]), so [cons] is
    valid only while [a] and [b] are small enough that [2^a * 3^b]
    fits, documented on [cons] below rather than enforced by the type. *)

(** [cons a b] is [2^a * 3^b], valid for non-negative [a] and [b]
    whose product [2^a * 3^b] is representable as a native [int]
    (comfortably true for [a, b] under 20). *)
val cons : int -> int -> int

val car : int -> int
val cdr : int -> int
val ex_2_05 : int -> int -> bool
