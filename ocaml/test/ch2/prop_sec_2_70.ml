(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Property sweep for Exercise 2.70a (this edition's addition, extending
   2.70): decoding an encoded message always gives back the message,
   for any Huffman tree the algorithm can build and any message drawn
   from its alphabet. A handful of hand-picked examples cannot show
   this; QCheck can, by generating many random alphabets and
   messages. *)

module S70 = Sicp_ch2_solutions.Sec_2_70
open QCheck2

(* An alphabet of 2 to 8 symbols named [s0] through [s(k-1)], each with
   a positive relative frequency. A single-symbol alphabet needs zero
   bits per occurrence, so [decode] cannot recover how many times the
   lone symbol repeated from the bit stream alone; the algorithm's
   domain is alphabets of two symbols or more. *)
let alphabet_gen =
  Gen.bind (Gen.int_range 2 8) (fun k ->
    Gen.bind
      (Gen.list_size (Gen.return k) (Gen.int_range 1 20))
      (fun weights ->
         Gen.return (List.mapi (fun i w -> Printf.sprintf "s%d" i, w) weights)))
;;

(* An alphabet paired with a message of 0 to 30 symbols drawn from it. *)
let alphabet_and_message_gen =
  Gen.bind alphabet_gen (fun pairs ->
    let symbols = List.map fst pairs in
    Gen.bind
      (Gen.list_size (Gen.int_range 0 30) (Gen.oneof_list symbols))
      (fun message -> Gen.return (pairs, message)))
;;

let decode_after_encode_is_identity =
  Test.make
    ~name:"2.70a: decode (encode message tree) tree = message"
    ~count:300
    alphabet_and_message_gen
    (fun (pairs, message) ->
       let tree = S70.generate_huffman_tree pairs in
       S70.roundtrip tree message)
;;

let () = QCheck_base_runner.run_tests_main [ decode_after_encode_is_identity ]
