(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Eval_error = Sicp_common.Eval_error

type discipline =
  | Untagged
  | Tagged
  | Per_register

let bad detail = Error (Eval_error.Bad_instruction detail)
let empty r = bad ("restore " ^ r ^ " from an empty stack")

let arity expected args =
  Error (Eval_error.Arity_mismatch { expected; given = List.length args })
;;

(* A disciplined machine replaces each [Save r] with the action
   [save-r] of [r]'s name and contents and each [Restore r] with an
   assignment to [r] from [restore-r] of [r]'s name. The two operations
   own the discipline's stack, so the simulator runs unchanged and each
   save or restore is still one instruction. *)
let rewrite controller =
  List.map
    (function
      | M.Save r -> M.Perform ("save-r", [ M.Const (M.Str r); M.Reg r ])
      | M.Restore r -> M.Assign_op (r, "restore-r", [ M.Const (M.Str r) ])
      | i -> i)
    controller
;;

let tagged_operations () =
  let stack = ref [] in
  [ ( "save-r"
    , M.Action_op
        (function
          | [ M.Str r; v ] ->
            stack := (r, v) :: !stack;
            Ok ()
          | args -> arity 2 args) )
  ; ( "restore-r"
    , M.Value_op
        (function
          | [ M.Str r ] ->
            (match !stack with
             | [] -> empty r
             | (saved, v) :: rest when String.equal saved r ->
               stack := rest;
               Ok v
             | (saved, _) :: _ -> bad ("restore " ^ r ^ " but the stack holds " ^ saved))
          | args -> arity 1 args) )
  ]
;;

let per_register_operations () =
  let stacks = Hashtbl.create 8 in
  let held r = Option.value (Hashtbl.find_opt stacks r) ~default:[] in
  [ ( "save-r"
    , M.Action_op
        (function
          | [ M.Str r; v ] ->
            Hashtbl.replace stacks r (v :: held r);
            Ok ()
          | args -> arity 2 args) )
  ; ( "restore-r"
    , M.Value_op
        (function
          | [ M.Str r ] ->
            (match held r with
             | [] -> empty r
             | v :: rest ->
               Hashtbl.replace stacks r rest;
               Ok v)
          | args -> arity 1 args) )
  ]
;;

let make ~discipline ~registers ~operations ~controller =
  match discipline with
  | Untagged -> M.make_machine ~registers ~operations ~controller
  | Tagged ->
    M.make_machine
      ~registers
      ~operations:(tagged_operations () @ operations)
      ~controller:(rewrite controller)
  | Per_register ->
    M.make_machine
      ~registers
      ~operations:(per_register_operations () @ operations)
      ~controller:(rewrite controller)
;;

(* The Figure 5.12 machine with one instruction eliminated: the
   afterfib-n-2 exchange -- [n] from [val], [val] from the stack --
   becomes the single untagged [Restore "n"], which takes the saved
   Fibonacci value from wherever it was filed. *)
let fib_one_fewer_controller =
  let rec replace matched = function
    | [] -> if matched then Ok [] else Error ()
    | M.Label "afterfib-n-2" :: M.Assign ("n", M.Reg "val") :: M.Restore "val" :: rest ->
      let* rest = replace true rest in
      Ok (M.Label "afterfib-n-2" :: M.Restore "n" :: rest)
    | i :: rest ->
      let* rest = replace matched rest in
      Ok (i :: rest)
  in
  match replace false Sec_5_5.fib_controller with
  | Ok controller -> controller
  | Error () -> failwith "fib_controller no longer contains the afterfib-n-2 exchange"
;;

let run_fib discipline controller n =
  let* m =
    make
      ~discipline
      ~registers:[ "n"; "val"; "continue" ]
      ~operations:M.arith_operations
      ~controller
  in
  let* () = M.set_register m "n" (M.Int n) in
  let* () = M.start m in
  M.get_register m "val"
;;

let save_save_restore = M.[ Save "y"; Save "x"; Restore "y" ]

let run_sequence discipline =
  let* m =
    make ~discipline ~registers:[ "x"; "y" ] ~operations:[] ~controller:save_save_restore
  in
  let* () = M.set_register m "y" (M.Int 7) in
  let* () = M.set_register m "x" (M.Int 8) in
  let* () = M.start m in
  M.get_register m "y"
;;

let reported = function
  | Ok v -> "leaves y = " ^ M.value_to_string v
  | Error e -> "reports -- Error: " ^ Eval_error.to_string e
;;

let fib_pairs_match () =
  let rec over = function
    | [] -> Ok true
    | n :: ns ->
      let* v = run_fib Untagged Sec_5_5.fib_controller n in
      let* w = run_fib Untagged fib_one_fewer_controller n in
      let* rest = over ns in
      Ok (v = w && rest)
  in
  over (List.init 10 Fun.id)
;;

let ex_5_11 () =
  let sequence =
    String.concat
      "; "
      (List.map (M.instruction_to_string M.value_to_string) save_save_restore)
  in
  let* all_match = fib_pairs_match () in
  let* tagged_fib6 = run_fib Tagged Sec_5_5.fib_controller 6 in
  let underflow =
    let* m =
      make
        ~discipline:Per_register
        ~registers:[ "x" ]
        ~operations:[]
        ~controller:M.[ Restore "x" ]
    in
    M.start m
  in
  Ok
    [ "(a) untagged: " ^ sequence ^ " " ^ reported (run_sequence Untagged)
    ; ("(a) fib with afterfib-n-2's exchange replaced by one Restore \"n\": answers \
        n=0..9 "
       ^ if all_match then "match the original" else "DIVERGE")
    ; "(b) tagged: " ^ sequence ^ " " ^ reported (run_sequence Tagged)
    ; "(b) tagged fib 6 = " ^ M.value_to_string tagged_fib6
    ; "(c) per-register: " ^ sequence ^ " " ^ reported (run_sequence Per_register)
    ; ("(c) restore from an empty per-register stack "
       ^
       match underflow with
       | Ok () -> "ran on an empty stack"
       | Error e -> "reports -- Error: " ^ Eval_error.to_string e)
    ]
;;
