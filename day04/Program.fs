let input = System.IO.File.ReadLines "input.txt"
let IsPartOne = not (System.Environment.GetEnvironmentVariable("part") = "part2")

let isContained (pair: int * int * int * int) =
    match (pair) with
    | (startA, endA, startB, endB) when endA >= startB && startA <= endB -> 1
    | _ -> 0

let isOverlapping (pair: int * int * int * int) =
    match (pair) with
    | (startA, endA, startB, endB) when startA >= startB && endA <= endB -> 1
    | (startA, endA, startB, endB) when startB >= startA && endB <= endA -> 1
    | _ -> 0

printfn
    "%d"
    (input
     |> Seq.map (fun line ->
         line.Split [| '-'; ',' |]
         |> (fun line -> (int line.[0], int line.[1], int line.[2], int line.[3])))
     |> Seq.map (fun pair -> if IsPartOne then isContained pair else isOverlapping pair)
     |> Seq.sum)
