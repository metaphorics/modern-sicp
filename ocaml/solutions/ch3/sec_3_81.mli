(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.81 *)

(** Exercise 3.81: the random-number generator as a stream process over
    requests, with no assignment. The map class is [T]; the tailored
    note in the map column asks for replayable tests, which the fixed
    seed delivers. *)

(** A request asks the process to [Generate] the next number or to
    [Reset] the sequence to a given value. *)
type request =
  | Generate
  | Reset of Int64.t

(** [rand_stream requests seed0] is the stream of answers: each
    generate draws the next seeded value from the current one, each
    reset installs its value. A pure process over its inputs, so the
    same requests from the same seed answer the same stream. *)
val rand_stream
  :  request Sicp_ch3.Sec_3_5.Streams.stream
  -> Int64.t
  -> Int64.t Sicp_ch3.Sec_3_5.Streams.stream

(** The sample request script: generate, generate, reset to 42,
    generate, generate, reset to 7, generate. *)
val sample_requests : request Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_81 ()] is the seven answers from seed 42 and whether a
    replay of the same script answers them again identically. *)
val ex_3_81 : unit -> Int64.t list * bool
