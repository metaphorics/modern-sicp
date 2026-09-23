(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.5: chain three fallible steps with [Result.bind], then map
    the error into a printable summary string.

    The three steps: parse a year of age from a string, check that it
    lies in [0] to [150], and describe it. The stubs raise
    [Sicp_common.Pending.Pending_solution] until solved. *)

(** [parse_age s] is [Ok] the integer [s] denotes, or [Error] a message. *)
val parse_age : string -> (int, string) result

(** [check_age n] is [Ok n] when [n] lies in [0] to [150], or [Error] a
    message. *)
val check_age : int -> (int, string) result

(** [describe_age n] is [Ok] a sentence about [n]. *)
val describe_age : int -> (string, string) result

(** [age_summary s] chains [parse_age], [check_age], and [describe_age]
    with [Result.bind] and renders any error as a printable summary. *)
val age_summary : string -> string
