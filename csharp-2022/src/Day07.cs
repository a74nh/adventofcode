using System.Diagnostics;

namespace AdventOfCode.csharp2022.Day07;


class ParsedFile
{
    public ParsedFile(string _name, int _size)
    {
        name = _name;
        size = _size;
    }
    
    public readonly string name;
    public int size;

    public virtual int CalcDirSizes() => size;
    public virtual int SumDirs(int max) => 0;
}

class ParsedDir : ParsedFile
{
    public ParsedDir() : base("/", 0) // Root Dir
    {
        parent = this;
        root = this;
    }

    private ParsedDir(string _name, ParsedDir _parent) : base(_name, 0)
    {
        parent = _parent;
        root = _parent.root;
    }

    public readonly ParsedDir parent;
    public readonly ParsedDir root;
    public List<ParsedFile> subfiles = new List<ParsedFile>();

    public ParsedFile addFile(string newName, int newSize)
    {
        var match = subfiles.Find(f => String.Equals(f.name, newName));
        if (match != null) return match;

        ParsedFile newFile = new ParsedFile(newName, newSize);
        subfiles.Add(newFile);
        return newFile;
    }

    public ParsedDir addDir(string newName)
    {
        if (String.Equals("/", newName)) return root;
        if (String.Equals("..", newName)) return parent;
        var match = subfiles.Find(f => String.Equals(f.name, newName));
        if (match != null) return (ParsedDir)match;

        ParsedDir newDir = new ParsedDir(newName, this);
        subfiles.Add(newDir);
        return newDir;
    }

    public override int CalcDirSizes()
        => size = subfiles.Select(x => x.CalcDirSizes()).Sum();

    public override int SumDirs(int max)
        => ((size<=max) ? size : 0) + subfiles.Select(x => x.SumDirs(max)).Sum();

    public List<int> FindSizes(int max)
    {
        var ret = subfiles.Where(x => x is ParsedDir).Select(x => ((ParsedDir)x).FindSizes(max)).ToList().SelectMany(x =>x).ToList();
        if (size>=max) ret.Add(size);
        return ret;
    }
}

class Solution : Solver<ParsedDir, int> {

    public override int testResultPartOne { get{return 95437;} }
    public override int testResultPartTwo { get{return 24933642;} }

    public override ParsedDir Parse(string[] input)
    {
        ParsedDir root = new ParsedDir();
        ParsedDir currentDir = root;

        input.Select(x => x.Split(' ')).ToList().ForEach(line =>
        {
            if (String.Equals(line[0], "$") && String.Equals(line[1], "cd"))
            {
                currentDir = currentDir.addDir(line[2]);
            }
            else if (String.Equals(line[0], "dir"))
            {
                currentDir.addDir(line[1]);
            }
            else
            {
                try { currentDir.addFile(line[1], Convert.ToInt32(line[0])); }
                catch (Exception) {}
            }
        });
        root.CalcDirSizes();
        return root;
    }

    public override int DoPartOne(ParsedDir input)
        => input.SumDirs(100000);

    public override int DoPartTwo(ParsedDir input)
        => input.FindSizes(input.size - 40000000).Min();
}
