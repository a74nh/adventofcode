using System;
using System.Reflection;
using System.Text;
using System.Net;
using CommandLine;
using AdventOfCode;

class Runner {

    public class CommandLineOptions
    {
        [Value(index: 0, Required = true, HelpText = "Day to solve for.")]
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
		var clientHandler = new HttpClientHandler
		{
			AllowAutoRedirect = true,
			UseCookies = true,
			CookieContainer = new CookieContainer()
		};

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

        string inputFileName = InputFileName(day, false);

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
                return;
            }

            HttpContent content = client.Result.Content;
            var contentStream = await content.ReadAsStreamAsync();
            using (var fileStream = File.Create(inputFileName))
            {
                contentStream.CopyTo(fileStream);
            }
		}

        System.IO.File.ReadAllLines(inputFileName).ToList().ForEach(i => Console.WriteLine(i.ToString()));
        Console.WriteLine("Input saved to {0}", inputFileName);
	}

}
