(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.80 *)

(** Exercise 3.80: the series RLC circuit as a pair of coupled signal
    streams. The map class is [T]. *)

(** [rlc r l c dt vc0 il0] is the pair of state streams (v_C, i_L) of
    the series RLC circuit from initial values v_C0 and i_L0: each
    integrator's input is the other's output, delayed. *)
val rlc
  :  float
  -> float
  -> float
  -> float
  -> float
  -> float
  -> float Sicp_ch3.Sec_3_5.Streams.stream * float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_80 ()] is the first four capacitor voltages and inductor
    currents of the R = 1 ohm, L = 1 henry, C = 0.2 farad circuit at
    dt = 0.1 s from v_C0 = 10 volts, i_L0 = 0 amps: the voltage holds
    10 then gives way as the current builds, 10, 10, 9.5, 8.55 against
    0, 1, 1.9, 2.71. *)
val ex_3_80 : unit -> float list * float list
