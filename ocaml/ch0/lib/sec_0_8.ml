(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Errors: the named definitions behind the listings of section 0.8. *)

exception Empty_queue

let dequeue = function
  | [] -> raise Empty_queue
  | head :: tail -> head, tail
;;

let checked_sqrt n =
  assert (n >= 0);
  sqrt (float_of_int n)
;;

type step_error =
  | Bad_number of string
  | Odd of int

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Bad_number s)
;;

let halve_even n = if n mod 2 = 0 then Ok (n / 2) else Error (Odd n)
let label n = Ok (string_of_int n ^ " is half of the input")

let message = function
  | Bad_number s -> "not a number: " ^ s
  | Odd n -> string_of_int n ^ " is odd"
;;

let half_report s =
  Result.bind (Result.bind (parse_int s) halve_even) label |> Result.map_error message
;;
