using System.Drawing;
var map = File.ReadAllLines("input.txt").Select(line => line.ToCharArray()).ToArray();

Console.WriteLine(Environment.GetEnvironmentVariable("part") != "part2" ? PartOne(map) : PartTwo(map));
static int PartOne(char[][] map)
{
    Point start = map.Select((line, y) => (line, y))
    .SelectMany(x => x.line
        .Select((c, x) => (c, x))
        .Where(c => c.c == 'S')
        .Select(c => new Point(c.x, x.y))).First();

    return Dijkstra(map, start, 'E', (map, current, neighbor) => getElevation(map, neighbor) - getElevation(map, current) <= 1);
}
static int PartTwo(char[][] map)
{
    Point start = map.Select((line, y) => (line, y))
    .SelectMany(x => x.line
        .Select((c, x) => (c, x))
        .Where(c => c.c == 'E')
        .Select(c => new Point(c.x, x.y))).First();

    return Dijkstra(map, start, 'a', (map, current, neighbor) => getElevation(map, neighbor) - getElevation(map, current) >= -1);
}

static int Dijkstra(char[][] map, Point start, char target, Func<char[][], Point, Point, bool> isReachable)
{
    var distances = new Dictionary<Point, int> { [start] = 0 };
    var queue = new PriorityQueue<Point, int>();
    queue.Enqueue(start, 0);

    while (queue.Count > 0)
    {
        var current = queue.Dequeue();
        if (map[current.Y][current.X] == target) return distances[current];
        GetNeighbors(map, current, isReachable)
            .Where(neighbor => !distances.ContainsKey(neighbor) || distances[neighbor] > distances[current] + 1)
            .ToList()
            .ForEach(neighbor =>
            {
                distances[neighbor] = distances[current] + 1;
                queue.Enqueue(neighbor, distances[current] + 1);
            });
    }
    throw new Exception("No path found");
}

static IEnumerable<Point> GetNeighbors(char[][] map, Point current, Func<char[][], Point, Point, bool> isReachable)
{
    var potentialNeighbors = new List<Point>
    {
        new Point(current.X - 1, current.Y),
        new Point(current.X + 1, current.Y),
        new Point(current.X, current.Y - 1),
        new Point(current.X, current.Y + 1)
    };
    return potentialNeighbors.Where(neighbor => isWihtinBounds(map, neighbor) && isReachable(map, current, neighbor));
}

static bool isWihtinBounds(char[][] map, Point point) => point.X >= 0 && point.X < map[0].Length && point.Y >= 0 && point.Y < map.Length;

static char getElevation(char[][] map, Point current)
{
    return map[current.Y][current.X] == 'S' ? 'a' : map[current.Y][current.X] == 'E' ? 'z' : map[current.Y][current.X];
}