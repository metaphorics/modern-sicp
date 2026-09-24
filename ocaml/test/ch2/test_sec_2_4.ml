(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 2.4. [sicp_ch2_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module S73 = Sicp_ch2_solutions.Sec_2_73
module S74 = Sicp_ch2_solutions.Sec_2_74
module S75 = Sicp_ch2_solutions.Sec_2_75
module S76 = Sicp_ch2_solutions.Sec_2_76

let eq_bool name a b = Alcotest.check Alcotest.bool name true (a = b)
let feq = Alcotest.float 1e-9

let ex_2_73_data_directed_deriv () =
  let sum, prod, expt = S73.ex_2_73 () in
  eq_bool "2.73: d/dx (x + 3) = 1" (sum = S73.Const 1) true;
  eq_bool "2.73: d/dx (x * y) = y" (prod = S73.Var "y") true;
  eq_bool
    "2.73: d/dx (x**3) = 3 * x**2"
    (expt
     = S73.Compound
         ("*", [ S73.Const 3; S73.Compound ("**", [ S73.Var "x"; S73.Const 2 ]) ]))
    true
;;

let ex_2_73_expt_edge_and_error () =
  let table = S73.make_table () in
  S73.install_sum_rule table;
  S73.install_product_rule table;
  S73.install_expt_rule table;
  eq_bool
    "2.73: d/dx (x**1) = 1"
    (S73.deriv table (S73.Compound ("**", [ S73.Var "x"; S73.Const 1 ])) "x" = S73.Const 1)
    true;
  Alcotest.check_raises
    "2.73: an operator with no installed rule raises"
    (Invalid_argument "deriv: unknown expression type: /")
    (fun () ->
       ignore (S73.deriv table (S73.Compound ("/", [ S73.Var "x"; S73.Const 2 ])) "x"))
;;

(* Two divisions, deliberately built from unrelated structures: an
   association list of [Hashtbl]s keyed by string fields, and a
   [Hashtbl] of a plain OCaml record. *)
module Division_a = struct
  type file = (string * (string, string) Hashtbl.t) list
  type record = (string, string) Hashtbl.t

  let get_record file name = List.assoc_opt name file

  let get_salary record =
    match Hashtbl.find_opt record "salary" with
    | Some s -> int_of_string s
    | None -> invalid_arg "Division_a.get_salary: no salary field"
  ;;
end

module Division_b = struct
  type file = (string, int * string) Hashtbl.t
  type record = int * string

  let get_record file name = Hashtbl.find_opt file name
  let get_salary (salary, _address) = salary
end

let divisions =
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
  [ S74.make_division (module Division_a) division_a_file
  ; S74.make_division (module Division_b) division_b_file
  ]
;;

let ex_2_74_find_employee_record () =
  eq_bool
    "2.74: Ben's salary, found in division A"
    (S74.get_salary (S74.find_employee_record divisions "Ben Bitdiddle") = 60000)
    true;
  eq_bool
    "2.74: Alyssa's salary, found in division B"
    (S74.get_salary (S74.find_employee_record divisions "Alyssa Hacker") = 80000)
    true;
  Alcotest.check_raises
    "2.74: an employee in no division raises"
    (Invalid_argument "find_employee_record: no record for Nobody")
    (fun () -> ignore (S74.find_employee_record divisions "Nobody"));
  eq_bool "2.74: ex_2_74 () is Ben's salary" (S74.ex_2_74 () = 60000) true
;;

let ex_2_74a_option_instead_of_exception () =
  eq_bool
    "2.74a: a found employee is Some"
    (Option.is_some (S74.find_employee_record_opt divisions "Ben Bitdiddle"))
    true;
  eq_bool
    "2.74a: a missing employee is None"
    (S74.find_employee_record_opt divisions "Nobody" = None)
    true;
  eq_bool "2.74a: ex_2_74a () is None" (S74.ex_2_74a () = None) true
;;

let ex_2_75_message_passing () =
  let z = S75.ex_2_75 5.0 0.0 in
  Alcotest.check feq "2.75: real_part of (5, 0)" 5.0 (S75.apply_generic S75.Real_part z);
  Alcotest.check feq "2.75: imag_part of (5, 0)" 0.0 (S75.apply_generic S75.Imag_part z);
  Alcotest.check feq "2.75: magnitude of (5, 0)" 5.0 (S75.apply_generic S75.Magnitude z);
  Alcotest.check feq "2.75: angle of (5, 0)" 0.0 (S75.apply_generic S75.Angle z);
  let z2 = S75.ex_2_75 2.0 (Float.pi /. 2.0) in
  Alcotest.check
    feq
    "2.75: real_part of (2, pi/2)"
    0.0
    (S75.apply_generic S75.Real_part z2);
  Alcotest.check
    feq
    "2.75: imag_part of (2, pi/2)"
    2.0
    (S75.apply_generic S75.Imag_part z2)
;;

let ex_2_76_three_organizations_agree () =
  let explicit_results, dd_results, mp_results = S76.ex_2_76 () in
  eq_bool
    "2.76: explicit dispatch and data-directed agree"
    (explicit_results = dd_results)
    true;
  eq_bool "2.76: data-directed and message-passing agree" (dd_results = mp_results) true;
  let (rect_real, rect_mag), (polar_real, polar_mag) = explicit_results in
  Alcotest.check feq "2.76: rectangular (3, 4) real_part" 3.0 rect_real;
  Alcotest.check feq "2.76: rectangular (3, 4) magnitude" 5.0 rect_mag;
  Alcotest.check feq "2.76: polar (5, 0) real_part" 5.0 polar_real;
  Alcotest.check feq "2.76: polar (5, 0) magnitude" 5.0 polar_mag
;;

let () =
  Alcotest.run
    "sec_2_4"
    [ ( "exercises"
      , Alcotest.
          [ test_case "2.73 data-directed deriv" `Quick ex_2_73_data_directed_deriv
          ; test_case "2.73 expt edge and error" `Quick ex_2_73_expt_edge_and_error
          ; test_case "2.74 find_employee_record" `Quick ex_2_74_find_employee_record
          ; test_case
              "2.74a option instead of exception"
              `Quick
              ex_2_74a_option_instead_of_exception
          ; test_case "2.75 message passing" `Quick ex_2_75_message_passing
          ; test_case
              "2.76 three organizations agree"
              `Quick
              ex_2_76_three_organizations_agree
          ] )
    ]
;;
