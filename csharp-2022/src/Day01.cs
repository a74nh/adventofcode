namespace AdventOfCode.csharp2022.Day01;

using ParsedData = List<List<int>>;

class Solution : Solver<ParsedData, int>
{
    public override int testResultPartOne { get{return 24000;} }
    public override int testResultPartTwo { get{return 45000;} }

    //Couldnt find a way to LINQ this.
    public override ParsedData Parse(string[] input)
    {
        ParsedData elves = new ParsedData();
        elves.Add(new List<int>());

        foreach(string line in input)
        {
            try
            {
                elves.Last().Add(int.Parse(line));
            }
            catch (Exception)
            {
                elves.Add(new List<int>());
            }
        }
        return elves;
    }

    private IEnumerable<int> CalsPerElf(ParsedData input)
        => input.Select(list => list.Sum());

    public override int DoPartOne(ParsedData input)
        => CalsPerElf(input).Max();

    public override int DoPartTwo(ParsedData input)
        => CalsPerElf(input).OrderByDescending(i => i).Take(3).Sum();
}
