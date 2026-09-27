(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.4: the recursive and the iterative exponentiation
    machines. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_1

let rec all f = function
  | [] -> Ok []
  | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
;;

(** The recursive controller: each level saves [continue] -- the one
    register the subproblem clobbers -- sets [n] to [n - 1], and
    returns to [after-expt] to multiply [b] into the answer. [b] needs
    no save: the subproblem never changes it. *)
let expt_recursive_controller =
  {|(controller
   (assign continue (label expt-done))
 expt-loop
   (test (op =) (reg n) (const 0))
   (branch (label base-case))
   (save continue)
   (assign n (op -) (reg n) (const 1))
   (assign continue (label after-expt))
   (goto (label expt-loop))
 after-expt
   (restore continue)
   (assign val (op *) (reg b) (reg val))
   (goto (reg continue))
 base-case
   (assign val (const 1))
   (goto (reg continue))
 expt-done)|}
;;

(** The iterative controller: three registers, no stack, no [continue]. *)
let expt_iterative_controller =
  {|(controller
 expt-iter
   (test (op =) (reg counter) (const 0))
   (branch (label expt-done))
   (assign counter (op -) (reg counter) (const 1))
   (assign product (op *) (reg b) (reg product))
   (goto (label expt-iter))
 expt-done)|}
;;

(** [ex_5_04 ()] runs the recursive machine on (2, 10) and (3, 5), then
    the iterative machine on the same inputs. *)
let ex_5_04 () =
  let run controller ~registers ~inputs ~result =
    Machine.make_machine ~registers ~operations:Machine.arith_operations ~controller
    >>= fun m ->
    all (fun (name, v) -> Machine.set_register m name v) inputs
    >>= fun _ -> Machine.start m >>= fun () -> Machine.get_register m result
  in
  run
    expt_recursive_controller
    ~registers:[ "b"; "n"; "val"; "continue" ]
    ~inputs:[ "b", Machine.Int 2; "n", Machine.Int 10 ]
    ~result:"val"
  >>= fun r1 ->
  run
    expt_recursive_controller
    ~registers:[ "b"; "n"; "val"; "continue" ]
    ~inputs:[ "b", Machine.Int 3; "n", Machine.Int 5 ]
    ~result:"val"
  >>= fun r2 ->
  run
    expt_iterative_controller
    ~registers:[ "b"; "counter"; "product" ]
    ~inputs:[ "b", Machine.Int 2; "product", Machine.Int 1; "counter", Machine.Int 10 ]
    ~result:"product"
  >>= fun i1 ->
  run
    expt_iterative_controller
    ~registers:[ "b"; "counter"; "product" ]
    ~inputs:[ "b", Machine.Int 3; "product", Machine.Int 1; "counter", Machine.Int 5 ]
    ~result:"product"
  >>= fun i2 -> Ok (List.map Machine.value_to_string [ r1; r2; i1; i2 ])
;;
