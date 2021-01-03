#!/usr/bin/env python3
import re
fname="day12.input"

from dataclasses import dataclass, field
from typing import List

@dataclass
class Instr:
    opcode: str
    val: int

@dataclass
class Coordinate:
    north: int = 0
    east: int = 0
    def __str__(self):
        return f'n{self.north} e{self.east}'
    def go_north(self,p):
        self.north=self.north+p
    def go_east(self,p):
        self.east=self.east+p
    def manhatten(self):
        return abs(self.north)+abs(self.east)

@dataclass
class State:
    code: List[Instr] = field(default_factory=list)
    intstr_pos: int = 0
    ship_pos: Coordinate = Coordinate(0,0)

    #Ouch! Force ship_pos to reset, otherwise keeps previous instance's value
    def __post_init__(self):
        self.ship_pos = Coordinate(0,0)

    def __str__(self):
        try:
            c=self.code[self.intstr_pos]
        except:
            c="finished"
        return f'{self.intstr_pos}={c} ship[{self.ship_pos}]'

    def run_instruction(self):
        self.intstr_pos = self.intstr_pos + 1

    def finished_execution(self):
        return self.intstr_pos >= len(self.code)

    def run_until_complete(self):
        while not self.finished_execution():
            # print(self)
            self.run_instruction()
        return


@dataclass
class State_shiponly(State):
    degrees_pos: int = 90

    def run_instruction(self):
        i = self.code[self.intstr_pos]
        if i.opcode == "N":
            self.ship_pos.go_north(i.val)
        elif i.opcode == "S":
            self.ship_pos.go_north(-i.val)
        elif i.opcode == "E":
            self.ship_pos.go_east(i.val)
        elif i.opcode == "W":
            self.ship_pos.go_east(-i.val)
        elif i.opcode == "R":
            self.degrees_pos = self.degrees_pos+i.val
            self.degrees_pos = self.degrees_pos % 360
        elif i.opcode == "L":
            self.degrees_pos = self.degrees_pos-i.val
            self.degrees_pos = self.degrees_pos % 360
        elif i.opcode == "F":
            if self.degrees_pos == 0:
                self.ship_pos.go_north(i.val)
            elif self.degrees_pos == 90:
                self.ship_pos.go_east(i.val)
            elif self.degrees_pos == 180:
                self.ship_pos.go_north(-i.val)
            elif self.degrees_pos == 270:
                self.ship_pos.go_east(-i.val)
            else:
               print(self)
               raise ValueError 
        else:
            print(self)
            raise ValueError
        self.intstr_pos = self.intstr_pos + 1

    def __str__(self):
        prefix = super().__str__()
        return f'{prefix} d{self.degrees_pos}'


@dataclass
class State_wayfinder(State):
    waypoint_pos: Coordinate = Coordinate(1,10)

    def run_instruction(self):
        i = self.code[self.intstr_pos]
        if i.opcode == "N":
            self.waypoint_pos.go_north(i.val)
        elif i.opcode == "S":
            self.waypoint_pos.go_north(-i.val)
        elif i.opcode == "E":
            self.waypoint_pos.go_east(i.val)
        elif i.opcode == "W":
            self.waypoint_pos.go_east(-i.val)
        elif i.opcode == "R":
            if i.val == 90:
                old=self.waypoint_pos.east
                self.waypoint_pos.east = self.waypoint_pos.north
                self.waypoint_pos.north = -old
            elif i.val == 180:
                self.waypoint_pos.east = -self.waypoint_pos.east
                self.waypoint_pos.north = -self.waypoint_pos.north
            elif i.val == 270:
                old=self.waypoint_pos.east
                self.waypoint_pos.east = -self.waypoint_pos.north
                self.waypoint_pos.north = old
            else:
               print(self)
               raise ValueError 
        elif i.opcode == "L":
            if i.val == 90:
                old=self.waypoint_pos.east
                self.waypoint_pos.east = -self.waypoint_pos.north
                self.waypoint_pos.north = old
            elif i.val == 180:
                self.waypoint_pos.east = -self.waypoint_pos.east
                self.waypoint_pos.north = -self.waypoint_pos.north
            elif i.val == 270:
                old=self.waypoint_pos.east
                self.waypoint_pos.east = self.waypoint_pos.north
                self.waypoint_pos.north = -old
            else:
               print(self)
               raise ValueError
        elif i.opcode == "F":
            n_dist = i.val*self.waypoint_pos.north
            e_dist = i.val*self.waypoint_pos.east
            self.ship_pos.go_north(n_dist)
            self.ship_pos.go_east(e_dist)
        else:
            print(self)
            raise ValueError
        self.intstr_pos = self.intstr_pos + 1

    def __str__(self):
        prefix = super().__str__()
        return f'{prefix} waypoint[{self.waypoint_pos}]'


with open(fname) as f:
    data = []
    for line in f:
        line=line.rstrip()
        instr=line[0]
        val=line[1:]
        i = Instr(instr, int(val))
        data.append(i)
        # print(i.opcode)

# print(data)





s = State_shiponly(data)
s.run_until_complete()
# print(s)
print(s.ship_pos.manhatten())

s = State_wayfinder(data)
s.run_until_complete()
# print(s)
print(s.ship_pos.manhatten())

