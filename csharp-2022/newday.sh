#!/bin/bash

#TODO: This should all be in C#

set -eu

mkdir -p input

DAY=$1
EXDAY=$(printf "%02d" $1)
YEAR=2022

# curl -b session=$(cat ${HOME}/.aocrc) https://adventofcode.com/${YEAR}/day/${DAY}/input > input/day$EXDAY.txt


cat <<EOF > src/Day$EXDAY.cs
namespace AdventOfCode.csharp2022.Day$EXDAY;

using ParsedData = string[];

class Solution : Solver<ParsedData> {

    public override ParsedData Parse(string[] input)
        => input;

    public override int DoPartOne(ParsedData input)
        => -1;

    public override int DoPartTwo(ParsedData input)
        => -1;
}

EOF
