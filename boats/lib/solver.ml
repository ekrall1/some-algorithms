(** solve the boats capacity problem 
    input files are structured in 2 rows
    row 1 is the max capacity of each boat (total weight)
    row 2 is a list of the weights of each person
 **)

open Base

type input = Input of int * int list
type result = ResultInt of int | Unknown

let extract_int = function
  | "" -> None
  | t -> Some (Int.of_string (Stdlib.String.trim t))

let parse_lines (lines : string list) =
  match lines with
  | [ line1; line2; _ ] ->
      let capacity = Int.of_string line1 in
      let lst =
        line2
        |> Stdlib.String.split_on_char ' '
        |> List.filter_map ~f:(fun x -> extract_int x)
      in
      (capacity, lst)
  | _ -> failwith "failed"

(* parse text into required input type *)
let input_text_parser (t : string) : input =
  t |> Stdlib.String.split_on_char '\n' |> parse_lines |> fun (capacity, lst) ->
  Input (capacity, lst)

(* output to text *)
let result_text = function
  | Unknown -> "Solver not implemented"
  | ResultInt x -> Int.to_string x

(* the algorithm *)
let solve_fn (capacity : int) (lst : int list) : int =
  let _ = Stdlib.List.sort compare lst in
  let boats = ref 0 in
  let smallest = ref 0 in
  let biggest = ref (Stdlib.List.length lst - 1) in
  while !biggest > !smallest do
    if Stdlib.List.nth lst !biggest + Stdlib.List.nth lst !smallest <= capacity
    then (
      biggest := !biggest - 1;
      smallest := !smallest + 1)
    else biggest := !biggest - 1;
    boats := !boats + 1
  done;
  !boats

(* solve *)
let solver (i : input) : result =
  match i with
  | Input (capacity, lst) -> solve_fn capacity lst |> fun x -> ResultInt x

let solution (problem_input : string) : string =
  problem_input |> input_text_parser |> solver |> result_text
