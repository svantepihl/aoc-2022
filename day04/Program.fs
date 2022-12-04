open System.IO
let input = File.ReadLines "input.txt"

let IsPartOne = not (System.Environment.GetEnvironmentVariable("part") = "part2")

let isContained (pair: array<int> * array<int>) =
    match (pair) with
    | (a, b) when a.[0] <= b.[0] && a.[1] >= b.[1] -> 1
    | (a, b) when a.[0] >= b.[0] && a.[1] <= b.[1] -> 1
    | _ -> 0

let isOverlapping (pair: array<int> * array<int>) =
    match (pair) with
    | (a, b) when a.[0] <= b.[0] && a.[1] >= b.[0] -> 1
    | (a, b) when a.[0] >= b.[0] && a.[1] <= b.[1] -> 1
    | (a, b) when a.[0] <= b.[1] && a.[1] >= b.[1] -> 1
    | _ -> 0

let result =
    input
    |> Seq.map (fun line -> line.Split(","))
    |> Seq.map (fun line -> (line.[0].Split("-") |> Array.map int), line.[1].Split("-") |> Array.map int)
    |> Seq.map (fun pair -> if IsPartOne then isContained pair else isOverlapping pair)
    |> Seq.sum

printfn "%d" result
