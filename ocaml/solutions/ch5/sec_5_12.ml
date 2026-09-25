(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.12: the assembler's analysis of a controller -- the
    instruction inventory and the register scan-out the data-path
    design needs. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2

(** The type of an instruction, in the order the analysis sorts by:
    assigns, then tests, branches, gotos, saves, restores, performs. *)
let type_rank = function
  | Machine.Assign _ | Machine.Assign_op _ -> 0
  | Machine.Test _ -> 1
  | Machine.Branch _ -> 2
  | Machine.Goto_label _ | Machine.Goto_reg _ -> 3
  | Machine.Save _ -> 4
  | Machine.Restore _ -> 5
  | Machine.Perform _ -> 6
;;

let type_names = [| "assign"; "test"; "branch"; "goto"; "save"; "restore"; "perform" |]

let rec uniq_sorted = function
  | [] -> []
  | x :: xs ->
    let rest = uniq_sorted (List.filter (fun y -> y <> x) xs) in
    x :: rest
;;

(** [analysis controller] computes the four lists the exercise asks
    for: the deduplicated instructions sorted by type, the entry-point
    registers, the saved and restored registers, and each register's
    sources. *)
let analysis controller =
  Machine.parse_program controller
  >>= fun (program : Machine.program) ->
  let insts = Array.to_list program.code in
  let by_type =
    List.sort
      (fun a b ->
         let c = compare (type_rank a) (type_rank b) in
         if c = 0
         then compare (Machine.instruction_to_string a) (Machine.instruction_to_string b)
         else c)
      insts
  in
  let sorted = uniq_sorted by_type in
  let entry_points =
    insts
    |> List.concat_map (function
      | Machine.Goto_reg r -> [ r ]
      | _ -> [])
    |> List.sort_uniq String.compare
  in
  let stacked =
    insts
    |> List.concat_map (function
      | Machine.Save r | Machine.Restore r -> [ r ]
      | _ -> [])
    |> List.sort_uniq String.compare
  in
  let assigned =
    insts
    |> List.concat_map (function
      | Machine.Assign (r, _) | Machine.Assign_op (r, _, _) -> [ r ]
      | _ -> [])
    |> List.sort_uniq String.compare
  in
  let source_lines =
    List.map
      (fun r ->
         let srcs =
           insts
           |> List.concat_map (function
             | Machine.Assign (target, src) when String.equal target r -> [ src ]
             | Machine.Assign_op (target, _, inputs) when String.equal target r -> inputs
             | _ -> [])
           |> List.map Machine.source_to_string
           |> uniq_sorted
         in
         "sources of " ^ r ^ ": " ^ String.concat ", " srcs)
      assigned
  in
  let type_counts =
    List.map
      (fun rank ->
         let count = List.length (List.filter (fun i -> type_rank i = rank) sorted) in
         type_names.(rank) ^ ": " ^ string_of_int count)
      [ 0; 1; 2; 3; 4; 5; 6 ]
  in
  Ok
    ([ "unique instructions, sorted by type: " ^ string_of_int (List.length sorted) ]
     @ type_counts
     @ [ "entry-point registers: " ^ String.concat ", " entry_points
       ; "saved/restored registers: " ^ String.concat ", " stacked
       ]
     @ source_lines)
;;

(** [ex_5_12 ()] analyzes the Fibonacci machine of Figure 5.12 and the
    factorial machine of Figure 5.11, whose [val] sources the book
    quotes. *)
let ex_5_12 () =
  let fib_controller =
    {|(controller
   (assign continue (label fib-done))
 fib-loop
   (test (op <) (reg n) (const 2))
   (branch (label immediate-answer))
   (save continue)
   (assign continue (label afterfib-n-1))
   (save n)
   (assign n (op -) (reg n) (const 1))
   (goto (label fib-loop))
 afterfib-n-1
   (restore n)
   (restore continue)
   (assign n (op -) (reg n) (const 2))
   (save continue)
   (assign continue (label afterfib-n-2))
   (save val)
   (goto (label fib-loop))
 afterfib-n-2
   (assign n (reg val))
   (restore val)
   (restore continue)
   (assign val (op +) (reg val) (reg n))
   (goto (reg continue))
 immediate-answer
   (assign val (reg n))
   (goto (reg continue))
 fib-done)|}
  in
  let factorial_controller =
    {|(controller
   (assign continue (label fact-done))
 fact-loop
   (test (op =) (reg n) (const 1))
   (branch (label base-case))
   (save continue)
   (save n)
   (assign n (op -) (reg n) (const 1))
   (assign continue (label after-fact))
   (goto (label fact-loop))
 after-fact
   (restore n)
   (restore continue)
   (assign val (op *) (reg n) (reg val))
   (goto (reg continue))
 base-case
   (assign val (const 1))
   (goto (reg continue))
 fact-done)|}
  in
  analysis fib_controller
  >>= fun fib_lines ->
  analysis factorial_controller
  >>= fun fact_lines -> Ok ([ "fib:" ] @ fib_lines @ [ "factorial:" ] @ fact_lines)
;;
