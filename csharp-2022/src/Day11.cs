namespace AdventOfCode.csharp2022.Day11;

using ParsedData = Tuple<List<Item>,List<Monkey>,uint>;

class Item {

    public Item(uint _worry, int _monkey)
    {
        worry = _worry;
        monkey = _monkey;
    }
    public uint worry;
    public int monkey;
}

class Monkey {
    public char operOp;
    public uint operVal;
    public uint test;
    public int trueMonkey;
    public int falseMonkey;
    public uint inspected;
}

class Solution : Solver<ParsedData, uint> {

    public override uint testResultPartOne { get{return 10605;} }
    public override uint testResultPartTwo { get{return 2713310158;} }

    public override ParsedData Parse(string[] input)
    {
        var monkies = new List<Monkey>();
        var items = new List<Item>();

        foreach(string line in input)
        {
            var words = line.Split(new char[] {' ',','}, StringSplitOptions.RemoveEmptyEntries);

            if (words.Length == 0) continue;
            if (String.Equals(words[0],"Monkey")) monkies.Add(new Monkey());
            else if (String.Equals(words[0],"Starting"))
            {
                for(int x=2;;x++) {
                    try { items.Add(new Item(uint.Parse(words[x]), monkies.Count-1)); }
                    catch (Exception) { break; }
                }
            }
            else if (String.Equals(words[0],"Operation:"))
            {
                monkies.Last().operOp = words[4][0];
                try { monkies.Last().operVal = uint.Parse(words[5]); }
                catch (Exception) { monkies.Last().operVal = 0; }
            }
            else if (String.Equals(words[0],"Test:")) monkies.Last().test = uint.Parse(words[3]);
            else if (String.Equals(words[1],"true:")) monkies.Last().trueMonkey = int.Parse(words[5]);
            else if (String.Equals(words[1],"false:")) monkies.Last().falseMonkey = int.Parse(words[5]);
        }
        return new ParsedData(items, monkies, monkies.Select(m => m.test).Aggregate((uint)1, (x,y) => x * y));
    }

    public void Round(ref List<Item> items, ref List<Monkey> monkies, uint modulo, bool relief)
    {
        for(int m=0; m < monkies.Count; m++)
        {
            for(int i=0; i < items.Count; i++)
            {
                if(items[i].monkey != m) continue;
                monkies[m].inspected++;
                // Inspect
                if (monkies[m].operVal==0) items[i].worry *= items[i].worry;
                else if (monkies[m].operOp=='*') items[i].worry *= monkies[m].operVal;
                else items[i].worry += monkies[m].operVal;
                // Mitigate worry
                if (relief) items[i].worry /= 3;
                items[i].worry %= modulo;
                //Test
                items[i].monkey = (items[i].worry % monkies[m].test == 0)
                    ? monkies[m].trueMonkey : monkies[m].falseMonkey;
            }
        }
    }

    public uint Run(List<Item> items, List<Monkey> monkies, uint modulo, int rounds, bool relief)
    {
        Enumerable.Range(0, rounds).ToList().ForEach(i => Round(ref items, ref monkies, modulo, relief));
        return monkies.Select(m => m.inspected).OrderByDescending(i => i).ToList().Take(2).Aggregate((uint)1, (x,y) => x * y);
    }

    public override uint DoPartOne(ParsedData input)
        => Run(input.Item1, input.Item2, input.Item3, 20, true);

    public override uint DoPartTwo(ParsedData input)
        => Run(input.Item1, input.Item2, input.Item3, 10000, false);
}
