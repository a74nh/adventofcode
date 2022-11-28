using System;
using System.Reflection;
using System.Text;
using CommandLine;
using AdventOfCode;

class Runner {

    public class CommandLineOptions
    {
        [Value(index: 0, Required = true, HelpText = "Day to solve for.")]
        public int Day { get; set; }

        [Option('t', "test", Required = false, HelpText = "Use test input.")]
        public bool Test { get; set; }
    }

    static void Main(string[] args)
    {
        Parser.Default.ParseArguments<CommandLineOptions>(args)
            .WithParsed<CommandLineOptions>(opts =>
            {
                Solve(opts.Day, opts.Test);
            });
    }

    private static int Solve(int day, bool test)
    {
        string inputfilename = "";
        string[] input;
        try
        {
            inputfilename = string.Format("input/day{0:00}{1}.txt", day, test ? "_test" : "");
            input = System.IO.File.ReadAllLines(inputfilename);
        }
        catch (Exception)
        {
            System.Console.WriteLine("No input file {0}", inputfilename);
            return -1;
        }

        dynamic solve;
        try
        {
            string fullname = string.Format("AdventOfCode.csharp2022.Day{0:00}.Solution", day);
            Type type = Type.GetType(fullname)!;

            solve = Activator.CreateInstance(type) ?? throw new Exception();
            solve = Convert.ChangeType(solve, type);

        }
        catch (Exception)
        {
            System.Console.WriteLine("No solution exists for day {0}", day);
            return -1;
        }

        System.Console.WriteLine(solve.PartOne(input));
        System.Console.WriteLine(solve.PartTwo(input));
        return 0;
    }
}
