(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.56 *)

(** Exercise 3.56: Hamming's numbers -- the positive integers whose
    only prime factors are 2, 3, and 5 -- enumerated in order without
    repetitions by merging the three scaled copies of the stream
    itself. The map class is [T]. *)

(** [merge s1 s2] combines two ordered streams into one ordered stream
    of their elements, dropping repetitions. *)
val merge
  :  int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** [hamming] is 1 followed by the merge of twice, three times, and
    five times itself; it answers the four facts of the statement by
    construction. *)
val hamming : int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_56 ()] is the first twelve elements of [hamming]:
    [1; 2; 3; 4; 5; 6; 8; 9; 10; 12; 15; 16]. *)
val ex_3_56 : unit -> int list
