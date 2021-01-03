#!/usr/bin/env python3
import re
fname="day3.input"

with open(fname) as f:
    data = []
    for line in f:
        data.append(line.rstrip())

def get_location(data, x, y):
    if y >= len(data):
        raise ValueError
    line=data[y]
    x = x % len(line)
    # print("{} {} {}".format(x,y,line))
    return line[x]

def reached_botton(data, x, y):
    if y >= len(data):
        return True
    return False

def is_tree_at_position(data, x, y):
    # print("{} {}".format(x,y))
    c = get_location(data, x, y)
    if c == "#":
        return True
    elif c == ".":
        return False
    else:
        print(c)
        raise ValueError

def do_run(data,move_x,move_y):
    x=0
    y=0
    num_trees=0
    while True:
        x=x+move_x
        y=y+move_y
        if reached_botton(data,x,y):
            break
        if is_tree_at_position(data,x,y):
            num_trees=num_trees+1
    return num_trees

t=do_run(data,3,1)
print(t)

run_patterns=[[1,1],[3,1],[5,1],[7,1],[1,2]]

t=1
for p in run_patterns:
  t=t*do_run(data,p[0],p[1])
print(t)




