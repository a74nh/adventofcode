#!/usr/bin/env python3
import re
fname="day9.input"

with open(fname) as f:
    data = []
    for line in f:
        data.append(int(line.rstrip()))

# print(data)


def is_valid(position):
    # print(position)
    val=data[position]
    for x in range(0,position):
        for y in range(0,x):
            if data[x] + data[y] == val:
                # print (x,y,data[x],data[y],position,val)
                return True
    return False

def find_contiguous(value):
    for x in range(0,len(data)):
        tot=0
        minv=data[x]
        maxv=data[x]
        for y in range(x,len(data)):
            tot = tot + data[y]
            minv=min(data[y],minv)
            maxv=max(data[y],maxv)
            if tot == value:
                # print(tot,value,x,y,data[x],data[y])
                return minv + maxv
            if tot>value:
                break
    return False

for x in range(25,len(data)):
    if not is_valid(x):
        key=data[x]
        break
print(key)

ret=find_contiguous(key)
print(ret)
