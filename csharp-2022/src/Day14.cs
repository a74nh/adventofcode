namespace AdventOfCode.csharp2022.Day14;

using System.Drawing;
using ParsedData = List<List<bool>>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 24;} }
    public override int testResultPartTwo { get{return 93;} }

    private int rowSize = 1000;

    private List<bool> NewRow()
        => new bool[rowSize].ToList();

    public override ParsedData Parse(string[] input)
    {
        var grid = Enumerable.Range(0, rowSize).Select(n => NewRow()).ToList();

        foreach(string line in input)
        {
            var structure = line.Split(" -> ").Select(p => p.Split(",").Select(c => int.Parse(c)).ToList()).ToList();
            var first = structure[0];
            structure.Skip(1).ToList().ForEach(second => {
                if (first[0]>second[0])
                {
                    Enumerable.Range(second[0], first[0]-second[0]+1).ToList().ForEach(x => grid[first[1]][x]=true);
                }
                else if (second[0]>first[0])
                {
                    Enumerable.Range(first[0], second[0]-first[0]+1).ToList().ForEach(x => grid[first[1]][x]=true);
                }
                if (first[1]>second[1])
                {
                    Enumerable.Range(second[1], first[1]-second[1]+1).ToList().ForEach(y => grid[y][first[0]]=true);
                }
                else if (second[1]>first[1])
                {
                    Enumerable.Range(first[1], second[1]-first[1]+1).ToList().ForEach(y => grid[y][first[0]]=true);
                }
                first = second;
            });
        }

        while (true)
        {
            if (grid[grid.Count - 1].Where(x => x).Count() != 0) break;
            grid.RemoveAt(grid.Count - 1);
        }
        return grid;
    }

    private void ShowGrid(ParsedData grid)
    {
        for (int y=0; y<grid.Count; y++)
        {
            for (int x=480; x<510; x++)
                Console.Write("{0}",grid[y][x]?"#":"." );
            Console.WriteLine();
        }
        Console.WriteLine();
    }

    private bool Fall(ref ParsedData grid, bool floor)
    {
        var sand = new Point(500,0);
        if (grid[sand.Y][sand.X]) return false;
        while (sand.Y < grid.Count-1)
        {
            if (!grid[sand.Y+1][sand.X])
            {
                sand.Y++; 
            }
            else if (!grid[sand.Y+1][sand.X-1])
            {
                sand.Y++; 
                sand.X--; 
            }
            else if (!grid[sand.Y+1][sand.X+1])
            {
                sand.Y++; 
                sand.X++; 
            }
            else
            {
                grid[sand.Y][sand.X]=true;
                return true;
            }
        }
        grid[sand.Y][sand.X]=true;
        return floor;
    }

    public override int DoPartOne(ParsedData input)
    {
        int ret=0;
        while(Fall(ref input, false)) ret++;
        return ret;
    }

    public override int DoPartTwo(ParsedData input)
    {
        input.Add(NewRow());
        int ret=0;
        while(Fall(ref input, true)) ret++;
        return ret;
    }
}
