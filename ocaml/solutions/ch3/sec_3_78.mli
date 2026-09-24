(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.78 *)

(** Exercise 3.78: [solve-2nd], the feedback network for
    y'' - a y' - b y = 0. The map class is [T]: both integrators take
    delayed inputs, exercise 3.77's shape. *)

(** [solve_2nd a b dt y0 dy0] is the stream of y values of the
    homogeneous second-order equation, from initial values y_0 and
    dy_0; the two integrators close their loops through delayed
    arguments. *)
val solve_2nd
  :  float
  -> float
  -> float
  -> float
  -> float
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_78 ()] is the computed y at t = 1.57 of the harmonic
    oscillator (a = 0, b = -1, y_0 = 0, dy_0 = 1): one Euler step shy
    of sin(pi/2) = 1, about 0.99999968. *)
val ex_3_78 : unit -> float
