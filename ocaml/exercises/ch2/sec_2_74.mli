(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.74 *)

module type PERSONNEL_FILE = sig
  type file
  type record

  val get_record : file -> string -> record option
  val get_salary : record -> int
end

type division

val make_division : (module PERSONNEL_FILE with type file = 'f) -> 'f -> division

type any_record

val get_record : division -> string -> any_record option
val get_salary : any_record -> int
val find_employee_record : division list -> string -> any_record

(** [find_employee_record_opt divisions name] is
    [find_employee_record divisions name] restated so a missing
    employee is [None] instead of a raised exception -- Exercise
    2.74a, this edition's addition. *)
val find_employee_record_opt : division list -> string -> any_record option

val sample_divisions : division list
val ex_2_74 : unit -> int

(** [ex_2_74a ()] (this edition's addition, extending 2.74) looks up
    an employee no division has: [None], not a raised exception. *)
val ex_2_74a : unit -> any_record option
