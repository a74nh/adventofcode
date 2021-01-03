#!/usr/bin/env python3
import re
fname="day2.input"

with open(fname) as f:
    # w, h = [int(x) for x in next(f).split()] # read first line
    data = []
    for line in f: # read rest of lines
        s = re.split('[- +:\n]',line)
        d = { "min":int(s[0]), "max":int(s[1]), "letter":s[2], "password":s[4] }
        data.append(d)

# print(data)

def count_str(fullstring,match):
  x=0
  for c in fullstring:
    if match == c:
        x=x+1
  return x 

valid=[]
for d in data:
    count = count_str(d["password"], d["letter"])
    if count >= d["min"] and count <= d["max"]:
        valid.append(d["password"])

print(len(valid))

valid=[]
for d in data:
    # print(d["password"])
    # print(d["min"]+1)
    # print(d["max"]+1)
    try:
      m1 = d["password"][d["min"]-1] ==  d["letter"]
    except:
      m1 = False
    try:
      m2 = d["password"][d["max"]-1] ==  d["letter"]
    except:
      m2 = False
    if m1 ^ m2:
        valid.append(d["password"])
        # print("{}: {} [{} {} {}] [{} {} {}] {}".format(m1 ^ m2, d["letter"],
        # m1, d["min"], d["password"][d["min"]-1],
        # m2, d["max"], d["password"][d["max"]-1],
        # d["password"]))

print(len(valid))

