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

(** The repair: average each raw value with the previous raw value
    first, then extract crossings from the averages alone. *)
val make_zero_crossings_smoothed
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** A noisy version of the 3.74 sample: the signal dips below zero
    once and comes back up, with sensor noise around both swings. *)
val noisy_signal : float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_75 ()] is Louis's first fourteen crossings, the repaired
    detector's fourteen, and how many nonzero crossings each produces.
    Louis's averages stir the raw value into the smoothed one, so his
    crossings lag the signal's swings; the repaired detector reports
    the signal's two crossings where the raw ones are. *)
val ex_3_75 : unit -> int list * int list * int * int
