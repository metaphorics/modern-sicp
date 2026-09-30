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
  | Unknown_operation of string
  | Unknown_label of string
  | Bad_instruction of string
  | Branch_without_test
  | Unknown_register of string
  | Invalid_form of string
  | Bounds_error of string
  | User_error of string

let pp ppf = function
  | Unbound_variable name -> Format.fprintf ppf "unbound variable %s" name
  | Arity_mismatch { expected; given } ->
    Format.fprintf ppf "arity mismatch: expected %d, given %d" expected given
  | Type_error detail -> Format.fprintf ppf "type error: %s" detail
  | Not_applicable printed ->
    Format.fprintf ppf "not applicable: %s is not a procedure" printed
  | Unknown_operation name -> Format.fprintf ppf "unknown operation %s" name
  | Unknown_label name -> Format.fprintf ppf "unknown label %s" name
  | Bad_instruction detail -> Format.fprintf ppf "bad instruction: %s" detail
  | Branch_without_test -> Format.fprintf ppf "branch without test"
  | Unknown_register name -> Format.fprintf ppf "unknown register %s" name
  | Division_by_zero -> Format.fprintf ppf "division by zero"
  | Invalid_form detail -> Format.fprintf ppf "invalid form: %s" detail
  | Bounds_error detail -> Format.fprintf ppf "bounds error: %s" detail
  | User_error message -> Format.fprintf ppf "%s" message
;;

let to_string e = Format.asprintf "%a" pp e
