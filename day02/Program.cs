
var input = File.ReadAllLines("input.txt"); var part = Environment.GetEnvironmentVariable("part"); Console.WriteLine(part != "part2" ? PartOne(input) : PartTwo(input));
const int LOSS = 0; const int DRAW = 3; const int WIN = 6; const int ROCK_PLAYED = 1; const int PAPER_PLAYED = 2; const int SCISSORS_PLAYED = 3;
int PartOne(IEnumerable<string> input) => input.Select(game => game switch
{
    "A X" => DRAW + ROCK_PLAYED,
    "A Y" => WIN + PAPER_PLAYED,
    "A Z" => LOSS + SCISSORS_PLAYED,
    "B X" => LOSS + ROCK_PLAYED,
    "B Y" => DRAW + PAPER_PLAYED,
    "B Z" => WIN + SCISSORS_PLAYED,
    "C X" => WIN + ROCK_PLAYED,
    "C Y" => LOSS + PAPER_PLAYED,
    "C Z" => DRAW + SCISSORS_PLAYED,

    _ => throw new Exception("Missing case")
}).Sum();
int PartTwo(IEnumerable<string> input) => input.Select(game => game switch
    {
        "A X" => LOSS + SCISSORS_PLAYED,
        "A Y" => DRAW + ROCK_PLAYED,
        "A Z" => WIN + PAPER_PLAYED,
        "B X" => LOSS + ROCK_PLAYED,
        "B Y" => DRAW + PAPER_PLAYED,
        "B Z" => WIN + SCISSORS_PLAYED,
        "C X" => LOSS + PAPER_PLAYED,
        "C Y" => DRAW + SCISSORS_PLAYED,
        "C Z" => WIN + ROCK_PLAYED,
        _ => throw new Exception("Missing case")
    }).Sum();