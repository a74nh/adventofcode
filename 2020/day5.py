#!/usr/bin/env python3
import re
fname="day5.input"

with open(fname) as f:
    data = []
    for line in f:
        line = line.rstrip()
        data.append({"row":line[0:7], "seat":line[7:]})

def decode_data(row_data, min, max):
    # print(row_data,min,max)
    try:
        pivot=int((max-min)/2)+min
        if row_data[0] == "F" or row_data[0] == "L":
            return decode_data(row_data[1:],min,pivot)
        if row_data[0] == "B" or row_data[0] == "R":
            return decode_data(row_data[1:],pivot+1,max)
        raise ValueError
    except IndexError:
        return min


def get_row_number(row):
    return decode_data(row,0,127)

def get_column_number(seat):
    return decode_data(seat,0,7)


def get_seat_id(row_data):
    ret = (get_row_number(r["row"])*8) + get_column_number(r["seat"])
    # print(row_data,ret)
    return ret

def find_missing_seat_id(seat_ids):
    for num, name in enumerate(seat_ids, start=seat_ids[0]):
        if num != name:
            # print(num,name)
            return num

seat_ids=[]
max_id=0
for r in data:
    sid=get_seat_id(r)
    max_id=max(max_id,sid)
    seat_ids.append(sid)
seat_ids.sort()

print(max_id)
print(find_missing_seat_id(seat_ids))

