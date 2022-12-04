using System;
using System.Reflection;
using System.Text;
using System.Net;
using CommandLine;
using AdventOfCode;

class Runner {

    public class CommandLineOptions
    {
        [Value(index: 0, Required = true, HelpText = "Day to solve for (0 for all)")]
        public int Day { get; set; }

        [Option('t', "test", Required = false, HelpText = "Use test input.")]
        public bool Test { get; set; }

		[Option('d', "download", Required = false, HelpText = "Download input.")]
        public bool Download { get; set; }
    }

    static void Main(string[] args)
    {
        Parser.Default.ParseArguments<CommandLineOptions>(args)
            .WithParsed( opts =>
            {
				if (opts.Download)
				{
                    var task = Download(opts.Day);
					task.Wait();
				}
                else if (opts.Day == 0)
                {
                    int day = 1;
                    int success;
                    do
                    {
                        System.Console.WriteLine("Day {0}:", day);
                        success = Solve(day++, opts.Test);
                    }
                    while (success == 0);
                }
				else
				{
					Solve(opts.Day, opts.Test);
				}
            });
    }

    static string InputFileName(int day, bool test)
        => string.Format("input/day{0:00}{1}.txt", day, test ? "_test" : "");

    private static int Solve(int day, bool test)
    {
        string[] input;
        try
        {
            input = System.IO.File.ReadAllLines(InputFileName(day, test));
        }
        catch (Exception)
        {
            System.Console.WriteLine("No input file {0}", InputFileName(day, test));
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

	private static async Task Download(int day)
	{
        // Get session id.
        string[] sessionid;
        string sessionFileName = string.Format("{0}/.aocrc", Environment.GetFolderPath(Environment.SpecialFolder.UserProfile));
        try
        {
            sessionid = System.IO.File.ReadAllLines(sessionFileName);
        }
        catch (Exception)
        {
            System.Console.WriteLine("No session id in {0}", sessionFileName);
            return;
        }

        // Download input file.
        string inputFileName = InputFileName(day, false);
        if (!File.Exists(inputFileName))
        {
            var clientHandler = new HttpClientHandler
            {
                AllowAutoRedirect = true,
                UseCookies = true,
                CookieContainer = new CookieContainer()
            };
            using (var httpClient = new HttpClient(clientHandler))
            {

                CookieContainer gaCookies = new CookieContainer();
                Uri target = new Uri(string.Format("https://adventofcode.com/2022/day/{0}/input",day));

                clientHandler.CookieContainer.Add(new Cookie("session", sessionid[0]) { Domain = target.Host });

                // get the file (cookies are sent automatically).
                var client = httpClient.GetAsync(target.AbsoluteUri);

                if (!client.Result.IsSuccessStatusCode)
                {
                    System.Console.WriteLine("Could not download {0}", target.AbsoluteUri);
                }
                else
                {
                    HttpContent content = client.Result.Content;
                    var contentStream = await content.ReadAsStreamAsync();
                    using (var fileStream = File.Create(inputFileName))
                    {
                        contentStream.CopyTo(fileStream);
                    }

                    System.IO.File.ReadAllLines(inputFileName).ToList()
                        .ForEach(i => Console.WriteLine(i.ToString()));
                    Console.WriteLine("Input saved to {0}", inputFileName);

                }
            }
        }

        // Create empty test input file.
        string testInputFile = InputFileName(day, true);
        if (!File.Exists(testInputFile))
        {
            File.Create(testInputFile).Dispose();
            Console.WriteLine("Created test input file {0}", testInputFile);
        }

        // Create source code using template.
        string sourceFile = string.Format("src/Day{0:00}.cs", day);
        string templateFile = "Lib/template.cs";
        if (!File.Exists(sourceFile))
        {
            File.Copy(templateFile, sourceFile);
            Console.WriteLine("Created source file {0}", sourceFile);
        }
	}

}
