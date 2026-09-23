(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Errors: the named definitions behind the listings of section 0.8. *)

(** [Empty_queue] is the exception a broken invariant raises: the book's
    programs reserve exceptions for conditions that should be impossible. *)
exception Empty_queue

(** [dequeue q] is the first element of [q] with the rest, and raises
    [Empty_queue] on an empty queue: the caller broke the invariant. *)
val dequeue : 'a list -> 'a * 'a list

(** [checked_sqrt n] is the square root of [n]; [assert] states the
    invariant that [n] is not negative, and a violation stops the program
    with [Assert_failure]. *)
val checked_sqrt : int -> float

(** The error of one fallible step: the input was not a number, or the
    number was odd. *)
type step_error =
  | Bad_number of string
  | Odd of int

(** [parse_int s] is [Ok] the integer [s] denotes, or [Error
      (Bad_number s)]. *)
val parse_int : string -> (int, step_error) result

(** [halve_even n] is [Ok (n / 2)] for even [n], and [Error (Odd n)]
    otherwise. *)
val halve_even : int -> (int, step_error) result

(** [label n] describes the halved value; the third fallible step. *)
val label : int -> (string, step_error) result

(** [message e] renders [e] as a printable summary line. *)
val message : step_error -> string

(** [half_report s] chains the three fallible steps with [Result.bind] and
    maps the error into its printable summary: [half_report "20"] is [Ok
    "10 is half of the input"], [half_report "7"] is [Error "7 is odd"],
    and [half_report "x"] is [Error "not a number: x"]. *)
val half_report : string -> (string, string) result
