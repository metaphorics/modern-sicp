(* SPDX-License-Identifier: GPL-3.0-only *)

type t = Value.env
type cell = Value.t option ref

let empty : t = Value.env_empty ()
let extend bindings env = Value.env_extend bindings env
let bind name value env = Value.env_extend [ name, value ] env
let extend_recursive names env = Value.env_extend_recursive names env
let fill cell value = Value.env_fill cell value
let find env name = Value.env_find env name

let get_exn env name =
  match Value.env_find env name with
  | Some value -> value
  | None -> raise Not_found
;;

let find_at env n = Value.env_find_at env n
