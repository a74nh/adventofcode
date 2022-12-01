namespace AdventOfCode.csharp2022.Day01;

class Solution : Solver<List<List<int>>>
{
    //Couldnt find a way to LINQ this.
    public override List<List<int>> Parse(string[] input)
    {
        List<List<int>> elves = new List<List<int>>();
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

    private IEnumerable<int> CalsPerElf(List<List<int>> input)
        => input.Select(list => list.Sum());

    public override int DoPartOne(List<List<int>> input)
        => CalsPerElf(input).Max();

    public override int DoPartTwo(List<List<int>> input)
        => CalsPerElf(input).OrderByDescending(i => i).Take(3).Sum();
}
