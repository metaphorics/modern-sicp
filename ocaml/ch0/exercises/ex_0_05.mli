(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.5's statement is in the book, section 0.8; the four vals
    below are the chain it asks you to write. *)

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
