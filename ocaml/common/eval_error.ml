(* SPDX-License-Identifier: GPL-3.0-only *)

type t =
  | Unbound_variable of string
  | Arity_mismatch of
      { expected : int
      ; given : int
      }
  | Type_error of string
  | Not_applicable of string
  | Division_by_zero
  | Invalid_form of string
  | User_error of string

let pp ppf = function
  | Unbound_variable name -> Format.fprintf ppf "unbound variable %s" name
  | Arity_mismatch { expected; given } ->
    Format.fprintf ppf "arity mismatch: expected %d, given %d" expected given
  | Type_error detail -> Format.fprintf ppf "type error: %s" detail
  | Not_applicable printed ->
    Format.fprintf ppf "not applicable: %s is not a procedure" printed
  | Division_by_zero -> Format.fprintf ppf "division by zero"
  | Invalid_form detail -> Format.fprintf ppf "invalid form: %s" detail
  | User_error message -> Format.fprintf ppf "%s" message
;;

let to_string e = Format.asprintf "%a" pp e
