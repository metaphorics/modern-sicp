(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.26: the monitored stack explores the evaluator's
    tail-recursive property with the iterative factorial of 1.2.1.
    This module also owns the monitored controller and the measurement
    helpers the other measuring exercises (5.27 to 5.29) share. *)

(** The monitored driver fragment: stats printed before the value. *)
val monitored_driver : string

(** One interaction's measured counters: the total number of pushes and
    the maximum stack depth of one interaction. *)
type stats

(** [pushes_of s] and [depth_of s] are the measured counters. *)
val pushes_of : stats -> int

val depth_of : stats -> int

(** The monitored controller: the base controller with the 5.4.4
    driver variant whose [print-result] prints the stack statistics
    before the value. *)
val controller : string

(** [run source] evaluates [source] on the monitored evaluator and
    answers the transcript, statistics lines included; the queue-dry
    end of the driver loop is the normal end. *)
val run : string -> (string list, Sicp_ch5.Sec_5_4.error) result

(** [stats_of transcript] is the measured counters of every interaction
    that printed one, in order. *)
val stats_of : string list -> stats list

(** [parse_stats line] reads one [(total-pushes = P maximum-depth = D)]
    line. *)
val parse_stats : string -> stats option

(** [measure source ns] runs [(factorial n)] for each [n] on a fresh
    machine and answers the counters of each call. *)
val measure : string -> int list -> (stats list, Sicp_ch5.Sec_5_4.error) result

(** [iterative_source] is the 1.2.1 iterative factorial the exercise
    defines. *)
val iterative_source : string

(** [fit_linear ns ps] is the [a], [b] of [p(n) = an + b] through the
    first and last point, or [None] for a single point. *)
val fit_linear : int list -> int list -> (int * int) option

(** [render_stats name n s] is one table line for the counters [s] of
    the call at [n]. *)
val render_stats : string -> int -> stats -> string

(** [ex_5_26 ()] measures the iterative factorial for n = 1 to 6 and
    answers the table plus the two answers: the maximum depth is
    independent of n, and the pushes fit [an + b] with the fitted
    constants verified on every point. *)
val ex_5_26 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
