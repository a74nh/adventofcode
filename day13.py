#!/usr/bin/env python3
import re
fname="day13.input"

from dataclasses import dataclass, field
from typing import List


with open(fname) as f:
    data = []
    lines=f.readlines()
    start_time=int(lines[0])
    def get(x):
        if x=="x":
            return x
        else:
            return int(x)
    busses=[get(i) for i in lines[1].split(",")]

# print(start_time)
# print(busses)

def first_bus_time_post(bus,start_time):
    if bus=="x":
        return 0
    return start_time-(start_time%bus)+bus

first_t=0
first_bus=0
for bus in busses:
    t=first_bus_time_post(bus,start_time)
    # print(bus,start_time,t)
    if t>0 and (first_t ==0 or t<first_t):
        first_t=t
        first_bus=bus
print((first_t-start_time)*first_bus)

