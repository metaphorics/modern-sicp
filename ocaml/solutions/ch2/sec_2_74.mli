(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.74 *)

(** The operations headquarters needs from one division's personnel
    file: retrieve an employee's record by name, and read a salary
    out of a record. [file] and [record] are each division's own
    structures, unknown to headquarters and to every other division. *)
module type PERSONNEL_FILE = sig
  type file
  type record

  val get_record : file -> string -> record option
  val get_salary : record -> int
end

(** One division: a first-class [PERSONNEL_FILE] module paired with
    its own file value. This is the "type information" part (a)
    asks about: a division supplies data plus the module that
    knows how to read it. *)

type division

(** [make_division (module D) file] packages [file] with the module
    that knows how to read it. *)
val make_division : (module PERSONNEL_FILE with type file = 'f) -> 'f -> division

(** An employee record retrieved from some division, together with
    the module that can still make sense of it -- so [get_salary]
    works without headquarters ever learning which division answered. *)
type any_record

(** [get_record division name] is [name]'s record in [division], if
    it has one. *)
val get_record : division -> string -> any_record option

(** [get_salary record] is [record]'s salary, read by whichever
    division's module produced it -- part (b). *)
val get_salary : any_record -> int

(** [find_employee_record divisions name] is [name]'s record, found
    by trying [divisions] in order -- part (c). [Invalid_argument]
    when no division has it, the book's [error] call. *)
val find_employee_record : division list -> string -> any_record

(** [find_employee_record_opt divisions name] is
    [find_employee_record divisions name] restated so a missing
    employee is data, [None], rather than a raised exception --
    Exercise 2.74a, this edition's addition. *)
val find_employee_record_opt : division list -> string -> any_record option

(** Two divisions built from unrelated structures -- an association
    list of [Hashtbl]s and a [Hashtbl] of a plain OCaml record -- to
    ground the exercise in a concrete, testable example. *)
val sample_divisions : division list

(** [ex_2_74 ()] is Ben Bitdiddle's salary, found by searching
    [sample_divisions] across two divisions whose files share no
    structure. *)
val ex_2_74 : unit -> int

(** [ex_2_74a ()] (this edition's addition, extending 2.74) looks up
    an employee no division has: [None], not a raised exception. *)
val ex_2_74a : unit -> any_record option
