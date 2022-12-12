namespace AdventOfCode.csharp2022.Day12;

using System.Drawing;
using Grid = Dictionary<System.Drawing.Point, int>;
using ParsedData = Tuple<System.Drawing.Point, System.Drawing.Point, Dictionary<System.Drawing.Point, int>>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 31;} }
    public override int testResultPartTwo { get{return 29;} }

    public override ParsedData Parse(string[] input)
    {
        var start = new Point();
        var end = new Point();
        int y=-1;
        var grid = input.ToList().Select(line => { y++; int x=0; return line.ToDictionary(c => new Point(++x,y), c =>
        {
            switch(c)
            {
                case 'S':
                    start = new Point(x,y);
                    return -1;
                case 'E':
                    end = new Point(x,y);
                    return 25;
                default:
                    return c - 'a';
            }
        });}).SelectMany(dict => dict).ToDictionary(pair => pair.Key, pair => pair.Value);
        return new ParsedData(start, end, grid);
    }

    public int bfs(Point start, Point end, Grid grid)
    {
        var queue = new Queue<Point>();
        var visited = new Dictionary<Point, int>();
        queue.Enqueue(start);
        visited.Add(start, 1);
        Size[] dirs = {new Size(0, -1), new Size(-1, 0), new Size(1, 0), new Size(0, 1)};

        while(queue.Count!=0)
        {
            Point current = queue.Dequeue();
            foreach(Size dir in dirs)
            {
                try
                {
                    var testing = Point.Add(current, dir);
                    if (visited.ContainsKey(testing)) continue;
                    if (grid[testing]-1<=grid[current])
                    {
                        visited.Add(testing,visited[current]+1);
                        if(testing.Equals(end)) return visited[testing];
                        queue.Enqueue(testing);
                    }
                }
                catch (Exception) {}
            }
        }
        return int.MaxValue;
    }

    public override int DoPartOne(ParsedData input)
        => bfs(input.Item1, input.Item2, input.Item3)-1;

    public override int DoPartTwo(ParsedData input)
        => input.Item3.Where(p => p.Value==0).Select(p => bfs(p.Key, input.Item2, input.Item3)).Min()-1;
}
