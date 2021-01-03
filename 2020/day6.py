#!/usr/bin/env python3
import re
fname="day6.input"

with open(fname) as f:
    data = []
    group = []
    for line in f:
        line = line.rstrip()
        if line:
            group.append(line)
        else:
            data.append({"answers":group})
            group=[]
    if group:
        data.append({"answers":group})

tot_count=0
for group in data:
    all_answers="".join(group["answers"])
    unique_answers="".join(set(all_answers))
    group["unique"]=unique_answers
    tot_count=tot_count+len(unique_answers)

print(tot_count)

tot_counti=0
for group in data:
    a=set(group["answers"][0])
    for answer in group["answers"][1:]:
        a=a.intersection(set(answer))
    group["intersect"]=a
    tot_counti=tot_counti+len(a)

print(tot_counti)



