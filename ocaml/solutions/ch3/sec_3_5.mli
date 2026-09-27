(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.5 *)

(** Exercise 3.5: Monte Carlo integration, estimating pi by the area
    of a unit circle. Following @ref{3.1}'s note, the generator is a
    [Sicp_common.Random.t] passed explicitly, not hidden behind
    OCaml's own global [Random] state. *)

(** [random_in_range generator low high] is a value chosen uniformly
    from [[low, high)], drawn from [generator]. *)
val random_in_range : Sicp_common.Random.t -> float -> float -> float

(** [estimate_integral predicate x1 x2 y1 y2 trials generator] is the
    Monte Carlo estimate of the integral of [predicate] over the
    rectangle [[x1, x2] * [y1, y2]], from [trials] sample points drawn
    from [generator]. *)
val estimate_integral
  :  (float -> float -> bool)
  -> float
  -> float
  -> float
  -> float
  -> int
  -> Sicp_common.Random.t
  -> float

(** [ex_3_05 ()] is the estimate of pi the statement asks for, by
    measuring the area of a unit circle with a fixed seed. *)
val ex_3_05 : unit -> float
