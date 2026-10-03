(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** Exercise 5.21: the two [count_leaves] machines over the
    list-structure memory, checked against the host oracle over the
    same cells. *)

let ( >>= ) = Result.bind

module Memory = Sicp_ch5.Sec_5_3
module M = Sicp_ch5.Sec_5_1

(** The object-language trees the machines count, plantable in the word
    vector: a leaf is a number, a node a list of subtrees. *)
type tree =
  | Leaf of int
  | Node of tree list

(** [plant mem tree] builds the tree's cells through the same
    allocation path the machine's [cons] uses. *)
let rec plant mem = function
  | Leaf n -> Ok (Memory.Num n)
  | Node subtrees ->
    let rec spine = function
      | [] -> Ok Memory.Empty
      | t :: rest -> plant mem t >>= fun w -> spine rest >>= fun d -> Memory.cons mem w d
    in
    spine subtrees
;;

(** [count tree] is the host oracle: the book's [count-leaves] as an
    OCaml function over the same trees. *)
let rec count = function
  | Leaf _ -> 1
  | Node [] -> 0
  | Node (t :: rest) -> count t + count (Node rest)
;;

(** The machine of Exercise 5.21a: pure recursion, the two answers
    combined by [+], the car answer parked in [temp] while the cdr is
    counted. *)
let recursive_controller =
  [ M.Assign ("continue", M.Label_ref "count-done")
  ; M.Label "count-loop"
  ; M.Test ("null?", [ M.Reg "tree" ])
  ; M.Branch "null-answer"
  ; M.Test ("pair?", [ M.Reg "tree" ])
  ; M.Branch "tree-case"
  ; M.Assign ("val", M.Const (Memory.Num 1))
  ; M.Goto_reg "continue"
  ; M.Label "null-answer"
  ; M.Assign ("val", M.Const (Memory.Num 0))
  ; M.Goto_reg "continue"
  ; M.Label "tree-case"
  ; M.Save "continue"
  ; M.Save "tree"
  ; M.Assign ("continue", M.Label_ref "after-car")
  ; M.Assign_op ("tree", "car", [ M.Reg "tree" ])
  ; M.Goto "count-loop"
  ; M.Label "after-car"
  ; M.Restore "tree"
  ; M.Assign_op ("tree", "cdr", [ M.Reg "tree" ])
  ; M.Save "val"
  ; M.Assign ("continue", M.Label_ref "after-cdr")
  ; M.Goto "count-loop"
  ; M.Label "after-cdr"
  ; M.Restore "temp"
  ; M.Assign_op ("val", "+", [ M.Reg "val"; M.Reg "temp" ])
  ; M.Restore "continue"
  ; M.Goto_reg "continue"
  ; M.Label "count-done"
  ]
;;

(** The machine of Exercise 5.21b: the explicit counter, [n]
    accumulating through both subcalls so only [continue] and [tree]
    ever take stack room. *)
let iterative_controller =
  [ M.Assign ("n", M.Const (Memory.Num 0))
  ; M.Assign ("continue", M.Label_ref "count-done")
  ; M.Label "count-loop"
  ; M.Test ("null?", [ M.Reg "tree" ])
  ; M.Branch "null-case"
  ; M.Test ("pair?", [ M.Reg "tree" ])
  ; M.Branch "tree-case"
  ; M.Assign_op ("n", "+", [ M.Reg "n"; M.Const (Memory.Num 1) ])
  ; M.Goto_reg "continue"
  ; M.Label "null-case"
  ; M.Goto_reg "continue"
  ; M.Label "tree-case"
  ; M.Save "continue"
  ; M.Save "tree"
  ; M.Assign ("continue", M.Label_ref "after-car")
  ; M.Assign_op ("tree", "car", [ M.Reg "tree" ])
  ; M.Goto "count-loop"
  ; M.Label "after-car"
  ; M.Restore "tree"
  ; M.Assign_op ("tree", "cdr", [ M.Reg "tree" ])
  ; M.Assign ("continue", M.Label_ref "after-cdr")
  ; M.Goto "count-loop"
  ; M.Label "after-cdr"
  ; M.Restore "continue"
  ; M.Goto_reg "continue"
  ; M.Label "count-done"
  ]
;;

(** [run controller answer tree] plants the tree and runs one machine;
    the answer is [val] for the recursive machine, [n] for the
    iterative one. *)
let run controller answer tree =
  let mem = Memory.make_memory ~size:64 ~root_capacity:6 ~free:0 in
  plant mem tree
  >>= fun root ->
  Memory.make_machine
    ~registers:[ "tree"; "val"; "n"; "continue"; "temp" ]
    ~operations:[]
    ~controller
    ~memory:mem
  >>= fun m ->
  Memory.set_register m "tree" root
  >>= fun () ->
  Memory.start m
  >>= fun () ->
  Memory.get_register m answer
  >>= fun v -> Ok (Memory.word_to_string v, Memory.print_stack_statistics m)
;;

(** [notation tree] is the tree in OCaml list notation, read back from
    its planted cells. *)
let notation tree =
  let mem = Memory.make_memory ~size:32 ~root_capacity:0 ~free:0 in
  plant mem tree >>= fun w -> Ok (Memory.write mem w)
;;

(** [ex_5_21 ()] runs both machines over three planted trees and prints
    each answer beside the oracle's, with the machines' stack use. *)
let ex_5_21 () =
  let trees =
    [ Node [ Leaf 1; Leaf 2; Node [ Leaf 3; Node [ Leaf 4; Leaf 5 ] ] ]
    ; Node [ Node [ Leaf 7 ] ]
    ; Node []
    ]
  in
  let rec go = function
    | [] -> Ok []
    | tree :: rest ->
      notation tree
      >>= fun name ->
      run recursive_controller "val" tree
      >>= fun (rec_answer, rec_stats) ->
      run iterative_controller "n" tree
      >>= fun (it_answer, it_stats) ->
      go rest
      >>= fun rest_lines ->
      Ok
        ((name
          ^ ": recursive "
          ^ rec_answer
          ^ ", iterative "
          ^ it_answer
          ^ ", oracle "
          ^ string_of_int (count tree)
          ^ ";"
          ^ " recursive stack "
          ^ rec_stats
          ^ ", iterative stack "
          ^ it_stats)
         :: rest_lines)
  in
  go trees >>= fun lines -> Ok lines
;;
