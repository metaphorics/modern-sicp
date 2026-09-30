(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.18 *)

module Ast = Sicp_common.Ast
module S = Sicp_ch4.Sec_4_1

let binding name rhs = { Ast.name = Some name; rhs }

(* The temporaries' names hold a space, so no body can see them. *)
let temporary index = "scanned value " ^ string_of_int index

let scan_out_alternative e =
  match Ast.view e with
  | Ast.Let (true, bindings, body) ->
    let at = Ast.at e in
    let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
    let values =
      List.mapi
        (fun i (b : Ast.binding) ->
           binding (temporary i) (Sec_4_16.deref_names names b.rhs))
        bindings
    in
    let assignments =
      List.mapi
        (fun i (b : Ast.binding) ->
           match b.name with
           | Some name ->
             Some (Ast.assign ~at (Ast.var ~at name) (Ast.var ~at (temporary i)))
           | None -> None)
        bindings
      |> List.filter_map Fun.id
    in
    let body =
      List.fold_right
        (fun a rest -> Ast.sequence ~at a rest)
        assignments
        (Sec_4_16.deref_names names body)
    in
    Ast.let_
      ~at
      false
      (List.map (fun n -> binding n (Ast.make_ref Sec_4_16.unassigned)) names)
      (Ast.let_ ~at false values body)
  | _ -> e
;;

let eval_alternative =
  let rec self e env = Sec_4_16.scanning scan_out_alternative ~self e env in
  self
;;

(* The shape of the book's [solve]: [y] uses [dy] only inside a
   delayed procedure, while [dy] uses [y]'s value as it is built. *)
let solve =
  "(fun u -> let rec y = [ (fun v -> match dy with (n, _) -> n) ] and dy = (2, y) in \
   match dy with (n, g :: _) -> n + g () | (n, []) -> n) ()"
;;

let ex_4_18 () =
  [ Sec_4_1.run_source Sec_4_16.eval solve
  ; Sec_4_1.run_source eval_alternative solve
  ; Sec_4_1.run_source S.eval_expr solve
  ; Sec_4_1.run_source
      eval_alternative
      "(fun u -> let rec y n = if n = 0 then 1 else 2 * dy (n - 1) and dy n = y n in y \
       3) ()"
  ]
;;
