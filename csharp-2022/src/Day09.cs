namespace AdventOfCode.csharp2022.Day09;

using System.Drawing;
using ParsedData = List<Tuple<System.Drawing.Point,int>>;

class Solution : Solver<ParsedData, int> {

    public override int testResultPartOne { get{return 13;} }
    public override int testResultPartTwo { get{return 1;} }

    private Point CharToDelta(char c)
    {
        if (c == 'U') return new Point(0,-1);
        if (c == 'D') return new Point(0,1);
        if (c == 'L') return new Point(-1,0);
        return new Point(1,0);
    }

    public override ParsedData Parse(string[] input)
        => input.ToList().Select(x=>
                {
                    var c = x.Split(" ");
                    return new Tuple<Point, int>(CharToDelta(c[0][0]), int.Parse(c[1]));
                }).ToList();


    private Point AddPoints(Point a, Point b)
        => new Point(a.X+b.X, a.Y+b.Y);

    private bool IsNextTo(Point first, Point second)
        => (first.X >= second.X-1 && first.X <= second.X+1
            && first.Y >= second.Y-1 && first.Y <= second.Y+1);

    private Point SyncTail(Point tail, Point head, Point dir)
    {
        if (dir.Y != 0) tail.X = head.X;
        if (dir.X != 0) tail.Y = head.Y;
        return tail;
    }

    private Point GetKnotDir(Point knot, Point prevKnot, Point prevKnotDir)
    {
        Point newDir = new Point(prevKnotDir.X,prevKnotDir.Y);
        if (prevKnotDir.X != 0)
        {
            newDir.Y=prevKnot.Y-knot.Y;
        }
        if (prevKnotDir.Y != 0)
        {
            newDir.X=prevKnot.X-knot.X;
        }
        return newDir;
    }

    // public override int DoPartOne(ParsedData input)
    // {
    //     Point head = new Point(0,0);
    //     Point tail = new Point(0,0);
    //     HashSet<Point> tailPositions = new HashSet<Point>();
    //     tailPositions.Add(tail);

    //     foreach(var command in input)
    //     {
    //         Point dir = command.Item1;
    //         int dist = command.Item2;

    //         for (; dist>0; dist--)
    //         {
    //             head = AddPoints(head, dir);
    //             if(IsNextTo(head, tail)) continue;
    //             tail = AddPoints(tail, dir);
    //             tail = SyncTail(tail, head, dir);
    //             tailPositions.Add(tail);
    //         }
    //     }
    //     return tailPositions.Count();
    // }

    public int DoKnots(ParsedData input, int totKnots)
    {
        var knots = new List<Point>();
        for(int x=0; x<totKnots; x++) knots.Add(new Point(0,0));

        HashSet<Point> tailPositions = new HashSet<Point>();
        tailPositions.Add(knots[totKnots-1]);

        foreach(var command in input)
        {
            int dist = command.Item2;

            for (; dist>0; dist--)
            {
                Point dir = command.Item1;
                knots[0] = AddPoints(knots[0], dir);
                for(int x=1; x<totKnots; x++)
                {
                    if(IsNextTo(knots[x], knots[x-1])) break;
                    dir = GetKnotDir(knots[x], knots[x-1], dir);
                    knots[x] = AddPoints(knots[x], dir);
                }
                tailPositions.Add(knots[totKnots-1]);
            }
        }
        return tailPositions.Count();
    }

    public override int DoPartOne(ParsedData input)
        => DoKnots(input, 2);

    public override int DoPartTwo(ParsedData input)
        => DoKnots(input, 10);

}
