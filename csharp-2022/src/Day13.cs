namespace AdventOfCode.csharp2022.Day13;

using System.Text.Json;
using MoreLinq;
using ParsedData = List<(System.Text.Json.JsonElement Left, System.Text.Json.JsonElement Right)>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 13;} }
    public override int testResultPartTwo { get{return 140;} }

    public override ParsedData Parse(string[] input)
        => input.Batch(3)
            .Select(pair =>(Left: JsonDocument.Parse(pair.ElementAt(0)).RootElement,
                            Right: JsonDocument.Parse(pair.ElementAt(1)).RootElement)).ToList();

    private JsonElement ToArray(int x)
        => JsonDocument.Parse($"[{x}]").RootElement;

    // If l==r return 0, if l<r return 1, if l>r return -1
    private int Compare(JsonElement left, JsonElement right)
    {
        if (left.ValueKind == JsonValueKind.Array && right.ValueKind == JsonValueKind.Array)
        {
            foreach (var (eleml, elemr) in Enumerable.Zip(left.EnumerateArray(), right.EnumerateArray()))
            {
                int r = Compare(eleml, elemr);
                if (r!=0) return r;
            }
            return left.GetArrayLength() == right.GetArrayLength() ? 0
                : left.GetArrayLength() < right.GetArrayLength() ? 1
                : -1;
        }
        else if (left.ValueKind == JsonValueKind.Array)
        {
            return Compare(left, ToArray(right.GetInt32()));
        }
        else if (right.ValueKind == JsonValueKind.Array)
        {
            return Compare(ToArray(left.GetInt32()), right);
        }
        else
        {
            return left.GetInt32() == right.GetInt32() ? 0
                : left.GetInt32() < right.GetInt32() ? 1
                : -1;
        }
    }

    public override int DoPartOne(ParsedData input)
    {
        var matches = input.Select(p => Compare(p.Left, p.Right)).ToList();
        int ret=0;
        for(int x=0; x<matches.Count; x++)
        {
            if (matches[x]>=0) ret+=x+1;
        }
        return ret;
    }

    public override int DoPartTwo(ParsedData input)
    {
        int twopos=1;
        int sixpos=2;
        JsonElement twojson = ToArray(2);
        JsonElement sixjson = ToArray(6);
        foreach(var p in input)
        {
            twopos += (Compare(p.Left, twojson)==1) ? 1 : 0;
            twopos += (Compare(p.Right, twojson)==1) ? 1 : 0;
            sixpos += (Compare(p.Left, sixjson)==1) ? 1 : 0;
            sixpos += (Compare(p.Right, sixjson)==1) ? 1 : 0;
        }
        return twopos*sixpos;
    }
}
