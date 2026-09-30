(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1

let type_names = [| "assign"; "test"; "branch"; "goto"; "save"; "restore"; "perform" |]

(* Assembly removes labels, so every instruction here has a rank. *)
let type_rank = function
  | M.Label _ | M.Assign _ | M.Assign_op _ -> 0
  | M.Test _ -> 1
  | M.Branch _ -> 2
  | M.Goto _ | M.Goto_reg _ -> 3
  | M.Save _ -> 4
  | M.Restore _ -> 5
  | M.Perform _ -> 6
;;

let show_instruction = M.instruction_to_string M.value_to_string

(* Sources are rendered by the engine's own instruction renderer, so
   the source notation cannot drift from the trace notation: an
   instruction whose target is empty renders as [Assign ("", src)], and
   the source is everything after its first ", " without the closing
   parenthesis. *)
let source_to_string src =
  let rendered = show_instruction (M.Assign ("", src)) in
  let start = String.index rendered ',' + 2 in
  String.sub rendered start (String.length rendered - start - 1)
;;

(* Keeps the first occurrence of each element, in order. *)
let dedup xs =
  List.rev
    (List.fold_left (fun seen x -> if List.mem x seen then seen else x :: seen) [] xs)
;;

let names_where f insts = List.sort_uniq String.compare (List.concat_map f insts)

let sources_of insts r =
  List.concat_map
    (function
      | M.Assign (target, src) when String.equal target r -> [ src ]
      | M.Assign_op (target, _, inputs) when String.equal target r -> inputs
      | _ -> [])
    insts
  |> List.map source_to_string
  |> dedup
;;

let analysis controller =
  let* program = M.assemble controller in
  let insts = Array.to_list program.code in
  let sorted =
    List.map (fun i -> type_rank i, show_instruction i, i) insts
    |> List.sort_uniq compare
    |> List.map (fun (_, _, i) -> i)
  in
  let type_counts =
    List.init (Array.length type_names) (fun rank ->
      let count = List.length (List.filter (fun i -> type_rank i = rank) sorted) in
      type_names.(rank) ^ ": " ^ string_of_int count)
  in
  let entry_points =
    names_where
      (function
        | M.Goto_reg r -> [ r ]
        | _ -> [])
      insts
  in
  let stacked =
    names_where
      (function
        | M.Save r | M.Restore r -> [ r ]
        | _ -> [])
      insts
  in
  let assigned =
    names_where
      (function
        | M.Assign (r, _) | M.Assign_op (r, _, _) -> [ r ]
        | _ -> [])
      insts
  in
  Ok
    ((("unique instructions, sorted by type: " ^ string_of_int (List.length sorted))
      :: type_counts)
     @ [ "entry-point registers: " ^ String.concat ", " entry_points
       ; "saved/restored registers: " ^ String.concat ", " stacked
       ]
     @ List.map
         (fun r -> "sources of " ^ r ^ ": " ^ String.concat ", " (sources_of insts r))
         assigned)
;;

let ex_5_12 () =
  let* fib_lines = analysis Sec_5_5.fib_controller in
  let* fact_lines = analysis Sec_5_5.factorial_recursive_controller in
  Ok (("fib:" :: fib_lines) @ ("factorial:" :: fact_lines))
;;
