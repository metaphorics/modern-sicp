(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 0.5: chain three fallible steps with [Result.bind], then map
    the error into a printable summary string.

    [age_summary "30"] is ["30 is a fine age"], [age_summary "abc"] is
    ["age check failed: not a number: abc"], and [age_summary "200"] is
    ["age check failed: 200 is out of range"]. *)

(** [parse_age s] is [Ok] the integer [s] denotes, or [Error "not a
    number: <s>"]. *)
val parse_age : string -> (int, string) result

(** [check_age n] is [Ok n] for [n] in [0] to [150], or [Error "<n> is out
    of range"]. *)
val check_age : int -> (int, string) result

(** [describe_age n] is [Ok "<n> is a fine age"]. *)
val describe_age : int -> (string, string) result

(** [age_summary s] runs the three steps with [Result.bind] and prefixes
    any error with ["age check failed: "]. *)
val age_summary : string -> string
