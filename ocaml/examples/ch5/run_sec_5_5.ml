(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the 5.5 compiler translates factorial to typed
   instruction sequences whose preserved registers show the
   [preserving] lesson, and the compiled code runs to the answer the
   evaluators give. *)

module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module Check = Sicp_common.Check
module Replay = Sicp_ch1.Replay

let program source =
  match Check.check ~filename:"replay.ml" source with
  | Ok p -> p
  | Error d -> failwith (Check.diagnostic_to_string d)
;;

let factorial =
  "let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n\n\
   let () = print_int (factorial 5)"
;;

let () =
  let out = Buffer.create 16 in
  (match C.run ~emit:(Buffer.add_string out) (program factorial) with
   | Ok _ -> ()
   | Error e -> Buffer.add_string out ("error: " ^ Sicp_common.Eval_error.to_string e));
  Replay.expect (Buffer.contents out) "120";
  (* A constant with a [Next] linkage needs nothing and modifies only
     its target; a variable needs [env]. *)
  let state = C.new_state () in
  let constant =
    C.compile state (Sicp_common.Ast.scalar (Sicp_common.Ast.Int 3)) "val" C.Next
  in
  let variable = C.compile state (Sicp_common.Ast.var "x") "val" C.Next in
  Replay.expect
    (String.concat "," constant.needs ^ "/" ^ String.concat "," constant.modifies)
    "/val";
  Replay.expect (String.concat "," variable.needs) "env";
  (* In [(factorial (n - 1)) * n] the recursive call clobbers [env] and
     [continue], which the multiplication and the return still need, so
     the compiled body saves and restores exactly those around it. *)
  let body =
    match
      Check.items
        (program "let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n")
    with
    | [ Sicp_common.Ast.Value_item (_, [ binding ]) ] ->
      C.compile (C.new_state ()) binding.rhs "val" C.Next
    | _ -> failwith "one recursive binding"
  in
  let saves =
    List.filter_map
      (function
        | M.Save r -> Some r
        | _ -> None)
      body.statements
    |> List.sort_uniq String.compare
  in
  Replay.expect (String.concat "," saves) "continue,env"
;;
