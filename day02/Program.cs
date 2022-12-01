Console.WriteLine((Environment.GetEnvironmentVariable("part") == "part1")
? PartOne(File.ReadAllText("input.txt"))
: PartTwo(File.ReadAllText("input.txt")));
int PartOne(string input) => 0; 
int PartTwo(string input) => 0;