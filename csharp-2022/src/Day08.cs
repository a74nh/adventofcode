namespace AdventOfCode.csharp2022.Day08;

using MoreLinq;
using ParsedData = List<List<int>>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 21;} }
    public override int testResultPartTwo { get{return 8;} }

    public override ParsedData Parse(string[] j)
        => j.Select(i => i.ToList().Select(i => i - '0').ToList()).ToList();

    public override int DoPartOne(ParsedData j)
        => Enumerable.Range(0, j.Count).Select(y =>
            Enumerable.Range(0, j[0].Count).Select(x =>
                Enumerable.Range(0, x).Select(i => j[y][i] < j[y][x]).All(i => i)
                || Enumerable.Range(x+1, j[0].Count-x-1).Select(i => j[y][i] < j[y][x]).All(i => i)
                || Enumerable.Range(0, y).Select(i => j[i][x] < j[y][x]).All(i => i)
                || Enumerable.Range(y+1, j.Count-y-1).Select(i => j[i][x] < j[y][x]).All(i => i))
            .ToList().Where(i => i).Count()).Sum();

    public override int DoPartTwo(ParsedData j)
        => Enumerable.Range(0, j.Count).Select(y =>
            Enumerable.Range(0, j[0].Count).Select(x =>
                Enumerable.Range(0, x).Reverse().TakeUntil(i => j[y][x] <= j[y][i]).Count()
                * Enumerable.Range(x+1, j[0].Count-x-1).TakeUntil(i => j[y][x] <= j[y][i]).Count()
                * Enumerable.Range(0, y).Reverse().TakeUntil(i => j[y][x] <= j[i][x]).Count()
                * Enumerable.Range(y+1, j.Count-y-1).TakeUntil(i => j[y][x] <= j[i][x]).Count())
            .ToList().Max()).Max();
}
