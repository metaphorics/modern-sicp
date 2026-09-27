(* SPDX-License-Identifier: GPL-3.0-only *)

type t = Value.env

let empty = Value.env_empty
let extend = Value.env_extend
let find_binding = Value.env_find_binding
let define = Value.env_define
let set = Value.env_set
