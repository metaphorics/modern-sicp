(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.26: the monitored stack explores the evaluator's
    tail-recursive property with the iterative factorial of 1.2.1.
    This module also owns the measurement helpers the other measuring
    exercises (5.27 to 5.29) share. *)

(** One measured interaction: the argument [n] and the statistics of
    the call. *)
type point =
  { n : int
  ; stats : Sec_5_23.stats
  }

(** [measure ~controller source ns] runs [source n] on a fresh
    evaluator over [controller] for each [n] of [ns] and is the
    measurement of each program's last top-level evaluation. *)
val measure
  :  controller:Sec_5_23.controller
  -> (int -> string)
  -> int list
  -> (point list, Sicp_common.Eval_error.t) result

(** [fit_linear samples] is the [a], [b] of [y = an + b] through the
    first and last [(n, y)] sample, or [None] for fewer than two samples
    or a non-integral slope. *)
val fit_linear : (int * int) list -> (int * int) option

(** [holds (a, b) samples] is [true] when every sample satisfies
    [y = an + b]. *)
val holds : int * int -> (int * int) list -> bool

(** [render_linear (a, b)] is [an + b] in the table's notation, such as
    [26n + 25] or [23n - 10]. *)
val render_linear : int * int -> string

(** [formula_line quantity samples] is the fitted linear formula of
    [quantity] over [samples] and whether it holds on every sample. *)
val formula_line : string -> (int * int) list -> string

(** [render_point name p] is one table line for [p]. *)
val render_point : string -> point -> string

(** [pushes points] and [depths points] are the [(n, total pushes)] and
    [(n, maximum depth)] samples of [points]. *)
val pushes : point list -> (int * int) list

val depths : point list -> (int * int) list

(** [iterative_source n] is the 1.2.1 iterative factorial with its
    internal [iter], ending with the call at [n]. *)
val iterative_source : int -> string

(** [ex_5_26 ()] measures the iterative factorial for n = 1 to 6 and
    answers the table plus the two answers: the maximum depth is
    independent of n, and the pushes fit [an + b] with the fitted
    constants verified on every point. *)
val ex_5_26 : unit -> (string list, Sicp_common.Eval_error.t) result
