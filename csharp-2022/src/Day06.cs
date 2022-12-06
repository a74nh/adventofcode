namespace AdventOfCode.csharp2022.Day06;

using ParsedData = List<char>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 7;} }
    public override int testResultPartTwo { get{return 19;} }

    public override ParsedData Parse(string[] input)
        => input[0].ToList();

    public bool IsMarker(ParsedData input, int start, int length)
        => input.Take(start+length).Skip(start).GroupBy(x => x).Count() == length;

    public int FindMarker(ParsedData input, int length)
    {
        for (int i=0; i<input.Count-length; i++)
        {
            if (IsMarker(input, i, length)) return i+length;
        }
        return -1;
    }

    public override int DoPartOne(ParsedData input)
        => FindMarker(input, 4);

    public override int DoPartTwo(ParsedData input)
        => FindMarker(input, 14);
}
