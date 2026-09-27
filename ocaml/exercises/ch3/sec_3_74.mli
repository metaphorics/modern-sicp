(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.74 *)

(** Exercise 3.74: Alyssa's zero crossings, completed with the
    generalized stream-map of exercise 3.50 mapping the detector over
    the signal and the signal shifted by one. The map class is [T]. *)

(** The sign of a value, with 0 counted positive. *)
val sign : float -> int

(** [sign_change_detector value last_value] is +1 when the signal
    changes from negative to nonnegative, -1 from nonnegative to
    negative, and 0 otherwise. *)
val sign_change_detector : float -> float -> int

(** [zero_crossings sense_data] is the crossing signal of the input,
    built with the n-ary stream-map: the detector mapped over the
    signal and the signal shifted right by one zero. *)
val zero_crossings
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** The statement's sample signal. *)
val sense_data : float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_74 ()] is the first twelve crossings of the sample:
    [0; 0; 0; 0; 0; -1; 0; 0; 0; 0; 1; 0], the statement's own
    answer line. *)
val ex_3_74 : unit -> int list
