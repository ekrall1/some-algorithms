(** given a per-boat capacity and a list of people's weights,
    find the minimum number of boats to carry the people **)

let usage =
  {|boats [-input <arr_input_file>]

example: boats -file ./input/arr1

Command line utility to solve the boats for people problem.
|}

let input_file = ref ""
let anon_fun filename = input_file := filename
let specs = [ ("-file", Arg.Set_string input_file, "Problem input.") ]

(* exception Invalid_input of string *)

let read_input_file filename =
  let ch = open_in filename in
  let input_str = really_input_string ch (in_channel_length ch) in
  close_in ch;
  input_str

let () =
  let () = Arg.parse specs anon_fun usage in
  let solver = Boats.Solver.(solution) in
  let open Stdio in
  let input_text =
    if String.equal !input_file "" then In_channel.input_all Stdio.stdin
    else read_input_file !input_file
  in
  let answer = input_text |> solver in
  printf "\nSolution is: %s boats.\n" answer
