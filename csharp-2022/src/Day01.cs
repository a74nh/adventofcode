namespace AdventOfCode.csharp2022.Day01;

class Solution : Solver<int[]> {

    public int Count(int[] input)
        => input.Zip(input.Skip(1), (x, y) => y > x).Where(c => c).Count();

    public override int[] Parse(string[] input) => Array.ConvertAll(input, int.Parse);

    public override int DoPartOne(int[] input) => Count(input);

    public override int DoPartTwo(int[] input)
        => Count(input.Zip(input.Skip(1), input.Skip(2))
                        .Select(i => i.First+i.Second+i.Third)
                        .ToArray());
}
