#!/usr/bin/env python3
import re
fname="day8.input"

from dataclasses import dataclass, field
from typing import List


@dataclass
class Instr:
    opcode: str
    val: int

@dataclass
class State:
    code: Instr
    position: int = 0
    accumulator: int = 0
    visited: List[int] = field(default_factory=list)



with open(fname) as f:
    data = []
    for line in f:
        [instr, val] = re.split(' ',line.rstrip())
        i = Instr(instr, int(val))
        data.append(i)
        # print(i.opcode)


def run_instruction(state):
    i = state.code[state.position]
    state.visited[state.position]=True
    if i.opcode == "acc":
        state.accumulator = state.accumulator+i.val
        state.position = state.position + 1
    elif i.opcode == "jmp":
        state.position = state.position + i.val
    elif i.opcode == "nop":
        state.position = state.position + 1
    else:
        print(state.position, state.accumulator, i)
        raise ValueError

def check_already_here(state):
    try:
        return s.visited[state.position]
    except:
        s.visited = [False] * len(state.code)
        return False


def finished_execution(state):
    return state.position >= len(state.code)

def run_until_loop(s):
    while not finished_execution(s):
        if check_already_here(s):
            return
        run_instruction(s)
    return


s = State(data)
run_until_loop(s)
print(s.accumulator)

for index, instr in enumerate(data):
    # print(index, instr.opcode)
    if instr.opcode == "jmp":
        data[index].opcode="nop"
        s = State(data)
        run_until_loop(s)
        if finished_execution(s):
            print(s.accumulator)
            break
        data[index].opcode="jmp"
    elif instr.opcode == "nop":
        data[index].opcode="jmp"
        s = State(data)
        run_until_loop(s)
        if finished_execution(s):
            print(s.accumulator)
            break
        data[index].opcode="nop"


