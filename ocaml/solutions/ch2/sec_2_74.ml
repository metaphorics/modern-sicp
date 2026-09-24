(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.74 *)

(** Exercise 2.74: Insatiable's divisions each keep personnel records
    in their own structure. A [PERSONNEL_FILE] module is the "type
    information" each division supplies (part a); [division] packs
    one such module together with the division's own file value, so a
    list of divisions can hold genuinely different file and record
    types side by side. [find_employee_record] tries each division in
    turn (part c); taking over a new company (part d) is
    [make_division (module New) file :: divisions], with no change to
    any of the code below. *)

module type PERSONNEL_FILE = sig
  type file
  type record

  val get_record : file -> string -> record option
  val get_salary : record -> int
end

type division =
  | Division :
      (module PERSONNEL_FILE with type file = 'f and type record = 'r) * 'f
      -> division

let make_division (type f) (module D : PERSONNEL_FILE with type file = f) (file : f)
  : division
  =
  Division ((module D), file)
;;

type any_record =
  | Record : (module PERSONNEL_FILE with type record = 'r) * 'r -> any_record

(** [get_record division name] is [name]'s record in [division],
    wrapped with the module that can still read it -- part (a). *)
let get_record (Division ((module D), file)) name =
  Option.map (fun record -> Record ((module D), record)) (D.get_record file name)
;;

(** [get_salary record] applies whichever division's [get_salary]
    produced [record] -- part (b), generic across every division. *)
let get_salary (Record ((module D), record)) = D.get_salary record

let rec find_employee_record divisions name =
  match divisions with
  | [] -> invalid_arg (Printf.sprintf "find_employee_record: no record for %s" name)
  | division :: rest ->
    (match get_record division name with
     | Some record -> record
     | None -> find_employee_record rest name)
;;

(** Exercise 2.74a (this edition's addition, extending 2.74):
    [find_employee_record] restated so a missing employee is [None]
    instead of a raised exception. *)
let find_employee_record_opt divisions name =
  try Some (find_employee_record divisions name) with
  | Invalid_argument _ -> None
;;

module Sample_division_a = struct
  type file = (string * (string, string) Hashtbl.t) list
  type record = (string, string) Hashtbl.t

  let get_record file name = List.assoc_opt name file

  let get_salary record =
    match Hashtbl.find_opt record "salary" with
    | Some s -> int_of_string s
    | None -> invalid_arg "Sample_division_a.get_salary: no salary field"
  ;;
end

module Sample_division_b = struct
  type file = (string, int * string) Hashtbl.t
  type record = int * string

  let get_record file name = Hashtbl.find_opt file name
  let get_salary (salary, _address) = salary
end

let sample_divisions =
  let division_a_file =
    let ben = Hashtbl.create 4 in
    Hashtbl.replace ben "salary" "60000";
    Hashtbl.replace ben "address" "Slumerville";
    [ "Ben Bitdiddle", ben ]
  in
  let division_b_file =
    let file = Hashtbl.create 4 in
    Hashtbl.replace file "Alyssa Hacker" (80000, "Cambridge");
    file
  in
  [ make_division (module Sample_division_a) division_a_file
  ; make_division (module Sample_division_b) division_b_file
  ]
;;

let ex_2_74 () = get_salary (find_employee_record sample_divisions "Ben Bitdiddle")
let ex_2_74a () = find_employee_record_opt sample_divisions "Nobody"
