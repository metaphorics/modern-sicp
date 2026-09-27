(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.75 *)

(** Exercise 3.75: Louis's smoothed zero-crossing detector, its bug,
    and the fix that compares each average with the previous average.
    The map class is [T]. *)

val sign : float -> int
val sign_change_detector : float -> float -> int

(** Louis's construction: it smooths, then compares the smoothed point
    with the raw previous value. *)
val make_zero_crossings_louis
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> float
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** The repair: the detector compares each smoothed point with the
    previous smoothed point. *)
val make_zero_crossings_smoothed
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> float
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** A noisy signal: one true negative crossing, plus spurious dips. *)
val noisy_signal : float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_75 ()] is Louis's first fourteen crossings, the fixed
    detector's fourteen, and how many nonzero crossings each produces.
    Louis's version stirs the averages with raw values and reports
    extra flips at the noise; the fixed one reports the signal's single
    crossing. *)
val ex_3_75 : unit -> int list * int list * int * int
