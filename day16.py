#!/usr/bin/env python3
import re
import yaml

fname="day16.input"

from dataclasses import dataclass, field
from typing import List

@dataclass
class Range:
    valmin: int
    valmax: int

    def valid_val(self,val):
    	return val>=self.valmin and val<=self.valmax

@dataclass
class RangeList:
    ranges: List[Range] = field(default_factory=list)

    def valid_val(self,val):
    	for r in self.ranges:
    		if r.valid_val(val):
    			return True
    	return False


data={}
inner=False
with open(fname) as f:
    lines=f.readlines()
    for line in lines:
    	line = line.rstrip()
    	if inner:
    		if line=="":
    			inner=False
    		else:
    			data[key].append([int(x) for x in line.split(",")])
    		continue
    	x=re.split(': *| or ',line)		
    	key=x[0]
    	if x[0] == "":
    		continue
    	elif x[1] == "":
    		inner=True
    		data[key]=[]
    	else:
    		[a1,a2]=x[1].split("-")
    		[b1,b2]=x[2].split("-")
    		data[key]=RangeList([Range(int(a1),int(a2)),Range(int(b1),int(b2))])

nearby_tickets=data["nearby tickets"]
del data["nearby tickets"]
my_ticket=data["your ticket"][0]
del data["your ticket"]
# print(data)


def is_value_valid(val,data):
	for field, ranges in data.items():
		# print(val,ranges)
		if ranges.valid_val(val):
			return True
	return False

def ticket_error_val(ticket,data):
	error=0
	for ticketval in ticket:
		if not is_value_valid(ticketval,data):
			# print(ticketval)
			error=error+ticketval
	return error


error_rate=0
valid_nearby_tickets=[]
for nticket in nearby_tickets:
	valid=ticket_error_val(nticket,data)
	if valid!=0:
		error_rate=error_rate+valid
	else:
		valid_nearby_tickets.append(nticket)
print(error_rate)

# print(len(nearby_tickets))
# print(len(valid_nearby_tickets))

def field_is_valid_for_index(index,field):
	for nticket in valid_nearby_tickets:
		val=nticket[index]
		if not field.valid_val(val):
			return False
	myval=my_ticket[index]
	if not field.valid_val(myval):
		return False
	return True

#Find all fields valid fro each index
found_fields=[]
for index in range(0,len(my_ticket)):
	found=False
	for fieldname,field in data.items():
		# if fieldname in found_fields:
		# 	continue
		if field_is_valid_for_index(index,field):
			try:
				found_fields[index].append(fieldname)
			except:
				found_fields.append([fieldname])
			# print(found_fields)
			found=True
			# break
	if not found:
		print(index)
		raise ValueError
# print(found_fields)

#Assuming there is always one unique match, reduce and simmer until only one for each
work_to_do=True
while work_to_do:
	for index in range(0,len(found_fields)):
		ff = found_fields[index]
		if len(ff)==1:
			for inner in found_fields:
				if inner != ff:
					try:
						inner.remove(ff[0])
					except:
						pass
	work_to_do=False
	for inner in found_fields:
		if len(inner)>1:
			work_to_do=True
# print(found_fields)

tot=1
for index in range(0,len(my_ticket)):
	if found_fields[index][0][:10] == "departure ":
		tot=tot*my_ticket[index]
print(tot)
