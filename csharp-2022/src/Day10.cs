namespace AdventOfCode.csharp2022.Day10;

using ParsedData = List<int>;

abstract class Machine {

    public int Run(ParsedData input)
    {
        int register = 1;
        int cycle = 1;

        foreach(var i in input)
        {
            Do(cycle++, register);
            if (i!=0) Do(cycle++, register);
            register += i;
        }
        return Result();
    }

    public abstract void Do(int cycle, int register);
    public abstract int Result();
}

class Profiler : Machine {
    private int strength = 0;

    public override void Do(int cycle, int register)
    {
        strength += ((((cycle+20)%40) == 0 || cycle == 20) ? (cycle*register) : 0);
    }

    public override int Result() => strength;
}

class Screen : Machine {
    private bool[][] pixels = Enumerable.Range(0, 6).Select(i => new bool[40]).ToArray();

    public override void Do(int cycle, int spritex)
    {
        int x = (cycle-1)%40;
        if (x<=spritex+1 && x>=spritex-1) pixels[(cycle-1)/40][x] = true;
    }

    public override int Result()
    {
        foreach(var y in pixels)
        {
            foreach(var x in y) Console.Write(x?"#":" ");
            Console.WriteLine();
        }
        return -1;
    }
}

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 13140;} }
    public override int testResultPartTwo { get{return -1;} }

    public override ParsedData Parse(string[] input)
        => input.ToList().Select(x=>
                { try {return int.Parse(x.Split(" ")[1]); }
                    catch (Exception) { return 0; }
                }).ToList();

    public override int DoPartOne(ParsedData input)
         => (new Profiler()).Run(input);

    public override int DoPartTwo(ParsedData input)
         => (new Screen()).Run(input);
}
