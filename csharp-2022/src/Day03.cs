namespace AdventOfCode.csharp2022.Day03;

using RuckSack = Tuple<List<int>,List<int>>;
using ParsedData = List<List<int>>;

class Solution : Solver<ParsedData> {

    private int ItemToPriority(char item)
        => item - ((item >= 'a') ? 'a' : 'A' - 26) + 1;

    public override ParsedData Parse(string[] input)
        => input.ToList<string>().Select(
                line => line.ToList<char>().Select(i => ItemToPriority(i)).ToList()).ToList();

    public RuckSack Compartmentalise(List<int> sack)
        => new RuckSack(sack.Take(sack.Count / 2).ToList(),sack.Skip(sack.Count / 2).ToList());

    public static List<List<T>> GroupIntoThrees<T>(IList<T> source)
        => source.Select((x, i) => new { Index = i, Value = x })
                    .GroupBy(x => x.Index / 3)
                    .Select(x => x.Select(v => v.Value).ToList())
                    .ToList();

    public static List<int> IntersectThree(List<int> a, List<int> b, List<int> c)
        => a.Intersect(b.Intersect(c)).ToList();

    public override int DoPartOne(ParsedData input)
        => input.Select(sack => Compartmentalise(sack)).Select(c => c.Item1.Intersect(c.Item2).Sum()).Sum();

    public override int DoPartTwo(ParsedData input)
        => GroupIntoThrees(input).Select(group => IntersectThree(group[0], group[1], group[2]).Sum()).Sum();
}

