(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** Exercise 5.21: the two [count-leaves] machines over the
    list-structure memory, checked against the host oracle over the
    same cells. *)

let ( >>= ) = Result.bind

module Memory = Sicp_ch5.Sec_5_3

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

(** [count tree] is the host oracle: the Scheme definition, read
    directly. *)
let rec count = function
  | Leaf _ -> 1
  | Node [] -> 0
  | Node (t :: rest) -> count t + count (Node rest)
;;

(** The machine of Exercise 5.21a: pure recursion, the two answers
    combined by [+], the car answer parked in [temp] while the cdr is
    counted. *)
let recursive_controller =
  {|(controller
   (assign continue (label count-done))
 count-loop
   (test (op null?) (reg tree))
   (branch (label null-answer))
   (test (op pair?) (reg tree))
   (branch (label tree-case))
   (assign val (const 1))
   (goto (reg continue))
 null-answer
   (assign val (const 0))
   (goto (reg continue))
 tree-case
   (save continue)
   (save tree)
   (assign continue (label after-car))
   (assign tree (op car) (reg tree))
   (goto (label count-loop))
 after-car
   (restore tree)
   (assign tree (op cdr) (reg tree))
   (save val)
   (assign continue (label after-cdr))
   (goto (label count-loop))
 after-cdr
   (restore temp)
   (assign val (op +) (reg val) (reg temp))
   (restore continue)
   (goto (reg continue))
 count-done)|}
;;

(** The machine of Exercise 5.21b: the explicit counter, [n]
    accumulating through both subcalls so only [continue] and [tree]
    ever take stack room. *)
let iterative_controller =
  {|(controller
   (assign n (const 0))
   (assign continue (label count-done))
 count-loop
   (test (op null?) (reg tree))
   (branch (label null-case))
   (test (op pair?) (reg tree))
   (branch (label tree-case))
   (assign n (op +) (reg n) (const 1))
   (goto (reg continue))
 null-case
   (goto (reg continue))
 tree-case
   (save continue)
   (save tree)
   (assign continue (label after-car))
   (assign tree (op car) (reg tree))
   (goto (label count-loop))
 after-car
   (restore tree)
   (assign tree (op cdr) (reg tree))
   (assign continue (label after-cdr))
   (goto (label count-loop))
 after-cdr
   (restore continue)
   (goto (reg continue))
 count-done)|}
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

(** [ex_5_21 ()] runs both machines over three planted trees and prints
    each answer beside the oracle's, with the machines' stack use. *)
let ex_5_21 () =
  let trees =
    [ "(1 2 (3 (4 5)))", Node [ Leaf 1; Leaf 2; Node [ Leaf 3; Node [ Leaf 4; Leaf 5 ] ] ]
    ; "((7))", Node [ Node [ Leaf 7 ] ]
    ; "()", Node []
    ]
  in
  let rec go = function
    | [] -> Ok []
    | (name, tree) :: rest ->
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
