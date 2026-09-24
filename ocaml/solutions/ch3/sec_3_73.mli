(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.73 *)

(** Exercise 3.73: the RC circuit as a signal processor: the voltage
    is R times the current plus the integral of current over C. The
    map class is [T]. *)

(** [rc r c dt current v0] is the stream of capacitor voltages of the
    series RC circuit: R i + (1/C integral of i) with initial
    capacitor voltage [v0]. *)
val rc
  :  float
  -> float
  -> float
  -> float Sicp_ch3.Sec_3_5.Streams.stream
  -> float
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_73 ()] is the first five voltages of the R = 5 ohm,
    C = 1 farad, dt = 0.5 s circuit under a constant 1 ampere current
    from v_0 = 0: [5; 5.5; 6; 6.5; 7]. *)
val ex_3_73 : unit -> float list
