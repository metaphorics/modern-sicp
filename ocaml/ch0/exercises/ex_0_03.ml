(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

type shape =
  | Circle of float
  | Rectangle of float * float

let area = function
  | Circle _ -> raise Sicp_common.Pending.Pending_solution
  | Rectangle (_, _) -> raise Sicp_common.Pending.Pending_solution
;;
