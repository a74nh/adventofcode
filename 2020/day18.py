#!/usr/bin/env python3
import re
import copy
fname="day18.input"

from dataclasses import dataclass, field
from typing import List

data = []
with open(fname) as f:
    for line in f:
    	line=line.rstrip().replace("(","( ").replace(")"," )")
    	data.append(line.split(" "))
# print(data)


def evaluate(expression):
	ret = _evaluate(expression,0)
	return ret[0]

def _evaluate(expression,pos):
	acc=0
	operator=None
	while True:

		#Extract
		try:
			current=expression[pos]
		except IndexError:
			return [acc,pos]
		pos=pos+1

		#Decode
		if current == "(":
			[current,pos]=_evaluate(expression,pos)
		elif current == ")":
			return [acc,pos]
		elif current.isdigit():
			current=int(current)
		else:
			operator=current
			continue

		#Evaluate
		if not operator:
			# print(pos-1,current,acc,operator,expression)
			acc = current
			pass
		elif operator == "+":
			# print(acc,operator,current,acc+current)
			acc=acc+current
		elif operator == "*":
			# print(acc,operator,current,acc*current)
			acc=acc*current
		else:
			print(pos-1,current,acc,operator,expression)
			raise ValueError
		operator=None


def evaluate2(expression):
	ret = _evaluate2(expression,0)
	return ret[0]

def _evaluate2(expression,pos=0):
	ret = __evaluate2(expression,pos,"+")
	ret = __evaluate2(expression,pos,"*")
	return ret

def __evaluate2(expression,pos,eval_op):
	acc=0
	operator=None
	while True:

		#Extract
		try:
			current=expression[pos]
		except IndexError:
			return [acc,pos]
		pos=pos+1

		#Decode
		if current == "(":
			[current,pos]=_evaluate2(expression,pos)
		elif current == ")":
			return [acc,pos]
		elif current.isdigit():
			current=int(current)
		elif current==eval_op:
			operator=current
			continue
		else:
			operator=None
			continue


		#Evaluate
		print(pos-1,current,acc,operator,expression)
		if not operator:
			# print(pos-1,current,acc,operator,expression)
			acc = current
			pass
		elif operator == "+":
			print(acc,operator,current,acc+current)
			acc=acc+current
		elif operator == "*":
			print(acc,operator,current,acc*current)
			acc=acc*current
		else:
			print(pos-1,current,acc,operator,expression)
			raise ValueError
		operator=None



tot=0
for d in data:
	v=evaluate(d)
	# v=evaluate2(d)
	tot=tot+v
	# print(v,"\n")
print(tot)
