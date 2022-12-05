using System.Text.RegularExpressions;

namespace AdventOfCode.csharp2022.Day05;

using Board = List<List<char>>;
using Move = Tuple<int, int, int>;
using Moves = List<Tuple<int, int, int>>;
using ParsedData = Tuple<List<List<char>>, List<Tuple<int, int, int>>>;

class Solution : Solver<ParsedData, string> {

    private void AddToCol(ref Board board, int col, char entry)
    {
        List<char> column;
        try
        {
            column = board[col];
        }
        catch (Exception)
        {
            board.Add(new List<char>());
            column = board[col];
        }
        if (entry == ' ')
        {
            return;
        }
        column.Add(entry);
    }

    public override ParsedData Parse(string[] input)
    {
        var board = new Board();
        var moves = new Moves();
        bool moveSection = false;
        Regex regex = new Regex(@"\D+");
        
        foreach(string line in input)
        {
            if (string.IsNullOrEmpty(line) || line[1]=='1')
            {
                moveSection = true;
            }
            else if (!moveSection)
            {
                // Parse board row.
                int col = 0;
                int pos = 1;
                while(true)
                {
                    try
                    {
                        AddToCol(ref board, col, line[pos]);
                        col++;
                        pos+=4;
                    }
                    catch (System.IndexOutOfRangeException)
                    {
                        break;
                    }
                }
            }
            else
            {
                List<int> split = regex.Split(line)
                                    .Where(i => !string.IsNullOrEmpty(i))
                                    .Select(i => int.Parse(i)).ToList();
                moves.Add(new Move(split[0], split[1], split[2]));
            }
        }

        board.ForEach(col => col.Reverse());
        return new ParsedData(board, moves);
    }

    public void DoMove9000(ref Board board, int num, int from, int to)
    {
        while (num > 0)
        {
            // Remove.
            int last = board[from-1].Count - 1;
            char piece = board[from-1][last];
            board[from-1].RemoveAt(last);
            // Add.
            board[to-1].Add(piece);
            num--;
        }
    }

    public void DoMove9001(ref Board board, int num, int from, int to)
    {
        while (num > 0)
        {
            // Remove.
            int last = board[from-1].Count - num;
            char piece = board[from-1][last];
            board[from-1].RemoveAt(last);
            // Add.
            board[to-1].Add(piece);
            num--;
        }
    }

    public override string DoPartOne(ParsedData input)
    {
        var board = input.Item1;
        var moves = input.Item2;
        moves.ForEach(move => DoMove9000(ref board, move.Item1, move.Item2, move.Item3));
        return new string(board.Select(col => col[col.Count-1]).ToArray());
    }

    public override string DoPartTwo(ParsedData input)
    {
        var board = input.Item1;
        var moves = input.Item2;
        moves.ForEach(move => DoMove9001(ref board, move.Item1, move.Item2, move.Item3));
        return new string(board.Select(col => col[col.Count-1]).ToArray());
    }
}
