(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Modules as values: the signatures, functors, and first-class packages
    behind the listings of section 0.9. *)

module type NUMBER = sig
  type t

  val zero : t
  val add : t -> t -> t
  val to_string : t -> string
end

module Int_number : NUMBER with type t = int = struct
  type t = int

  let zero = 0
  let add = ( + )
  let to_string = string_of_int
end

module Float_number : NUMBER with type t = float = struct
  type t = float

  let zero = 0.0
  let add = ( +. )
  let to_string = Printf.sprintf "%g"
end

module Summer (M : NUMBER) = struct
  let sum xs = List.fold_left M.add M.zero xs
end

module Int_summer = Summer (Int_number)
module Float_summer = Summer (Float_number)

type number_package = (module NUMBER)

let int_package : number_package = (module Int_number)
let float_package : number_package = (module Float_number)
let describe_zero (module M : NUMBER) = M.to_string M.zero
let sum_with (module M : NUMBER) xs = M.to_string (List.fold_left M.add M.zero xs)
