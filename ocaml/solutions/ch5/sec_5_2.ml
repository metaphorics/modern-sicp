(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1

let ex_5_02 () =
  let controller = Sec_5_1.factorial_iterative_controller in
  let* program = M.assemble controller in
  let label_table =
    String.concat " " (List.map (fun (l, i) -> l ^ "=" ^ string_of_int i) program.labels)
  in
  let run n =
    M.run
      ~registers:[ "n"; "product"; "counter" ]
      ~operations:M.arith_operations
      ~inputs:[ "n", M.Int n; "product", M.Int 1; "counter", M.Int 1 ]
      ~controller
      "product"
  in
  let* five = run 5 in
  let* six = run 6 in
  Ok
    [ Printf.sprintf
        "%d instructions, %d labels"
        (Array.length program.code)
        (List.length program.labels)
    ; label_table
    ; M.value_to_string five
    ; M.value_to_string six
    ]
;;
