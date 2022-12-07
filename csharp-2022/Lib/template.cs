namespace AdventOfCode.csharp2022.Day00;

using ParsedData = List<string>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return -1;} }
    public override int testResultPartTwo { get{return -1;} }

    public override ParsedData Parse(string[] input)
        => input.ToList();

    public override int DoPartOne(ParsedData input)
        => -1;

    public override int DoPartTwo(ParsedData input)
        => -1;
}
