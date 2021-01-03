#!/usr/bin/env python3
import re
fname="day10.input"

from dataclasses import dataclass, field
from typing import List

@dataclass
class Adapter:
    val: int
    seqs: int = 0
    # seq: List[int] = field(default_factory=list)

data = [0]
with open(fname) as f:
    for line in f:
        data.append(int(line.rstrip()))
data.sort()
adata=[]
for d in data:
    adata.append(Adapter(d))
data=adata
# print(data)

def find_nearest_next_adapter(index):
    adapter=data[index]
    index=index+1
    if index<len(data):
        return index
    return False

index=0
diff_counts=[0,0,0,0]
while True:
    next_adapter_index=find_nearest_next_adapter(index)
    if not next_adapter_index:
        diff_counts[3] = diff_counts[3] + 1
        break
    adapter=data[index]
    next_adapter=data[next_adapter_index]
    diff = next_adapter.val - adapter.val
    diff_counts[diff] = diff_counts[diff] + 1
    index = next_adapter_index

print(diff_counts[1]*diff_counts[3])

def find_next_adapter_with_diff(index,diff):
    adapter=data[index]
    try:
        while True:
            index=index+1
            next_adapter=data[index]
            if adapter.val + diff == next_adapter.val:
                return index
    except:
        return False

def generate_jolt_seq(index):
    # print(index)
    adapter=data[index]
    for diff in range(3,0,-1):
        next_adapter_index=find_next_adapter_with_diff(index,diff)
        if not next_adapter_index:
            continue
        next_adapter=data[next_adapter_index]
        adapter.seqs = adapter.seqs + next_adapter.seqs

data[len(data)-1].seqs=1
for index in range(len(data)-2,-1,-1):
    generate_jolt_seq(index)

print(data[0].seqs)

