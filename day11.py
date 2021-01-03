#!/usr/bin/env python3
import re
import copy
fname="day11.input"

from dataclasses import dataclass, field
from typing import List

data = []
with open(fname) as f:
    for line in f:
        data.append(['.']+[char for char in line.rstrip()]+['.'])

#Pad for fun and profit
width=len(data[0])
empty=['.']*width
data = [empty] + data + [empty]
# print(data)
length=len(data)

def show(d):
	for row in d:
		print("".join(row))
	print()

def check_adjacent_occupied(x,y,data_curr):
	adjacent = 0
	for y_check in range(-1,2):
		for x_check in range(-1,2):
			if x_check == y_check == 0:
				continue
			# print(x_check,y_check)
			if data_curr[y+y_check][x+x_check] == "#":
				adjacent = adjacent + 1
	return adjacent

def check_viewing_occupied(x,y,data_curr):
	adjacent = 0
	for y_check in range(-1,2):
		for x_check in range(-1,2):
			if x_check == y_check == 0:
				continue
			# print(x_check,y_check)
			x_to_check=x
			y_to_check=y
			try:
				while True:
					x_to_check=x_to_check+x_check
					y_to_check=y_to_check+y_check
					# print(x,y,x_check,y_check,x_to_check,y_to_check,data_curr[y_to_check][x_to_check])
					if x_to_check < 0 or y_to_check < 0:
						break
					if data_curr[y_to_check][x_to_check] == "#":
						adjacent = adjacent + 1
						break
					if data_curr[y_to_check][x_to_check] == "L":
						break
			except:
				pass
	return adjacent


def generate(x,y,data_curr,data_next,check_func,first_vacate_value):
	if data_curr[y][x] == "L":
		# print(x,y,check_func(x,y,data_curr))
		if check_func(x,y,data_curr) == 0:
			data_next[y][x] = "#"
			return True
		else:
			data_next[y][x] = "L"
			return False
	elif data_curr[y][x] == "#":
		# print(x,y,check_func(x,y,data_curr))
		if check_func(x,y,data_curr) >= first_vacate_value:
			data_next[y][x] = "L"
			return True
		else:
			data_next[y][x] = "#"
			return False		
	return False

def generate_all(data_curr,data_next,check_func,first_vacate_value):
	changed = False
	for y in range(1,length-1):
		for x in range(1,width-1):
			changed = changed | generate(x,y,data_curr,data_next,check_func,first_vacate_value)
			# print(changed,y,x,data_curr[y][x], data_next[y][x])
	return changed

def count_occupied(data_curr):
	count = 0
	for y in range(1,length-1):
		for x in range(1,width-1):
			if data_curr[y][x] == "#":
				count = count + 1
	return count


def do_run(data,check_func,first_vacate_value):
	data_curr = copy.deepcopy(data)
	data_next = copy.deepcopy(data)

	iterations=0
	# show(data_curr)
	while(generate_all(data_curr,data_next,check_func,first_vacate_value)):
		iterations=iterations+1
		backup = data_curr 
		data_curr = data_next
		data_next = backup
		# show(data_curr)

	# print(iterations)
	print(count_occupied(data_curr))


do_run(data,check_adjacent_occupied,4)
do_run(data,check_viewing_occupied,5)
