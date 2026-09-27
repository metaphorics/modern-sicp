(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.79 *)

(** Exercise 3.79: [solve-2nd] generalized to y'' = f(y', y). The map
    class is [T]. *)

(** [solve_2nd_general f dt y0 dy0] is the stream of y values of
    y'' = f(y', y) from initial values y_0 and dy_0; [f] receives the
    current derivative stream and position stream, delayed as far as
    the integrators require. *)
val solve_2nd_general
  :  (float -> float -> float)
  -> float
  -> float
  -> float
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_79 ()] holds when the general solver with f(dy, y) = -y
    agrees with exercise 3.78's dedicated harmonic oscillator at the
    checked indices. *)
val ex_3_79 : unit -> bool
