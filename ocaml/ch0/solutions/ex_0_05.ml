(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

let parse_age s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("not a number: " ^ s)
;;

let check_age n =
  if n >= 0 && n <= 150 then Ok n else Error (string_of_int n ^ " is out of range")
;;

let describe_age n = Ok (string_of_int n ^ " is a fine age")

let age_summary s =
  match Result.bind (Result.bind (parse_age s) check_age) describe_age with
  | Ok msg -> msg
  | Error e -> "age check failed: " ^ e
;;
