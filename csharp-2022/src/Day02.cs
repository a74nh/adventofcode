namespace AdventOfCode.csharp2022.Day02;

class Solution : Solver<List<Tuple<int,int>>> {

    public override List<Tuple<int,int>> Parse(string[] input)
        => input.Select(line => new Tuple<int, int>(line[0]-'A', line[2]-'X')).ToList();

    private int Win(int opponent) => (opponent+1)%3;
    private int Loose(int opponent) => (opponent==0) ? 2 : opponent-1;

    public override int DoPartOne(List<Tuple<int,int>> input)
        => input.Select(go => ((Win(go.Item1)==go.Item2) ? 6
                                : (go.Item2 == go.Item1) ? 3
                                : 0)
                                + go.Item2 + 1).Sum();

    public override int DoPartTwo(List<Tuple<int,int>> input)
        => input.Select(go => ((go.Item2==2) ? 6 + Win(go.Item1)
                                : (go.Item2 == 1) ? 3 + go.Item1
                                : Loose(go.Item1))
                                + 1).Sum();
}

