#!/usr/bin/env python3

fname="day1.input"

with open(fname) as f:
    # w, h = [int(x) for x in next(f).split()] # read first line
    nums = []
    for line in f: # read rest of lines
        nums.append(int(line))

# print(nums)


for x in nums:
    for y in nums:
        if x+y == 2020:
            print("{} {} {}".format(x,y,x*y))

for x in nums:
    for y in nums:
        for z in nums:
            if x+y+z == 2020:
                print("{} {} {} {}".format(x,y,z,x*y*z))