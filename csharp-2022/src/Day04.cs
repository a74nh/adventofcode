using System.Text.RegularExpressions;

namespace AdventOfCode.csharp2022.Day04;

using ParsedData = List<List<int>>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 2;} }
    public override int testResultPartTwo { get{return 4;} }

    public override ParsedData Parse(string[] input)
    {
        Regex regex = new Regex(@"\D+");
        return input.Select(line => regex.Split(line).Select(i => int.Parse(i)).ToList()).ToList();
    }

    private bool FullyContained(List<int> ranges)
        => (ranges[2] >= ranges[0] && ranges[3] <= ranges[1])
        || (ranges[0] >= ranges[2] && ranges[1] <= ranges[3]);

    private bool Overlaps(List<int> ranges)
        => (ranges[2] >= ranges[0] && ranges[2] <= ranges[1])
        || (ranges[3] >= ranges[0] && ranges[3] <= ranges[1])
        || (ranges[0] >= ranges[2] && ranges[0] <= ranges[3])
        || (ranges[1] >= ranges[2] && ranges[0] <= ranges[3]);

    public override int DoPartOne(ParsedData input)
        => input.Select(r => FullyContained(r)).Where(c => c).Count();

    public override int DoPartTwo(ParsedData input)
        => input.Select(r => Overlaps(r)).Where(c => c).Count();
}

