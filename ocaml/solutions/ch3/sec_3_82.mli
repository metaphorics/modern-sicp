(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.82 *)

(** Exercise 3.82: Monte Carlo integration as a stream of estimates
    that sharpens as it is walked. The map class is [T]: like 3.81,
    the seeded stateless generator makes every estimate replayable. *)

(** Uniform points of the unit square, two seeded draws per trial. *)
val unit_pairs : (float * float) Sicp_ch3.Sec_3_5.Streams.stream

(** [estimate_integral predicate] is the stream of running estimates
    of the predicate's area over the unit square, one element per
    trial and no trial-count argument: walk it farther, estimate
    sharper. *)
val estimate_integral : (float * float -> bool) -> float Sicp_ch3.Sec_3_5.Streams.stream

(** The quarter-disk predicate whose area is pi/4, and four times the
    running area estimates: a stream of pi estimates. *)
val in_quarter_disk : float * float -> bool

val pi_estimates : float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_82 ()] is the pi estimate at 1000 trials, the estimate at
    10000 trials, and whether the latter sits within 0.1 of pi. *)
val ex_3_82 : unit -> float * float * bool
