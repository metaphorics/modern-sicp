(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

let find_variable name frames =
  let rec in_frame displacement = function
    | [] -> None
    | n :: rest ->
      if n = name then Some displacement else in_frame (displacement + 1) rest
  in
  let rec go frame = function
    | [] -> None
    | names :: rest ->
      (match in_frame 0 names with
       | Some displacement -> Some (frame, displacement)
       | None -> go (frame + 1) rest)
  in
  go 0 frames
;;

let address_to_string = function
  | Some (frame, displacement) -> Printf.sprintf "(%d, %d)" frame displacement
  | None -> "not found"
;;

let book_environment = [ [ "y"; "z" ]; [ "a"; "b"; "c"; "d"; "e" ]; [ "x"; "y" ] ]

let ex_5_41 () =
  Ok
    (List.map
       (fun name -> name ^ ": " ^ address_to_string (find_variable name book_environment))
       [ "c"; "x"; "w" ])
;;
