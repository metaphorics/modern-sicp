(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Modules as values: the signatures, functors, and first-class packages
    behind the listings of section 0.9. *)

(** One arithmetic signature over an abstract [t]: the contract a numeric
    representation must satisfy. *)
module type NUMBER = sig
  type t

  val zero : t
  val add : t -> t -> t
  val to_string : t -> string
end

(** Exact integer arithmetic as a [NUMBER] whose [t] is known to be
    [int]: the summers below can then work on plain [int list]s. *)
module Int_number : NUMBER with type t = int

(** Inexact float arithmetic as a [NUMBER] whose [t] is known to be
    [float]. *)
module Float_number : NUMBER with type t = float

(** [Summer (M)] is the fold-based summer over any [NUMBER]: one piece of
    code, one implementation per representation. *)
module Summer (M : NUMBER) : sig
  val sum : M.t list -> M.t
end

(** [Int_summer] is [Summer (Int_number)]; [Int_summer.sum [1; 2; 3]] is
    [6]. *)
module Int_summer : sig
  val sum : int list -> int
end

(** [Float_summer] is [Summer (Float_number)]; [Float_summer.sum [1.5;
    2.5]] is [4.]. *)
module Float_summer : sig
  val sum : float list -> float
end

(** A first-class module packed as an ordinary value: [number_package]
    carries a whole [NUMBER] implementation, hiding which one behind the
    abstract [t] inside it. *)
type number_package = (module NUMBER)

(** [int_package] packages [Int_number] as a value. *)
val int_package : number_package

(** [float_package] packages [Float_number] as a value. *)
val float_package : number_package

(** [describe_zero package] unpacks [package] and renders its [zero]:
    the one operation a caller can perform on an opaque package without
    already knowing which representation it hides. *)
val describe_zero : number_package -> string

(** [sum_with (module M) xs] folds [xs] with [M] and renders the result;
    [M] must be a literal module expression at the call site; a package
    value's hidden [M.t] cannot otherwise be tied to the type of [xs]. *)
val sum_with : (module M : NUMBER) -> M.t list -> string
