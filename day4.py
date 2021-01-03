#!/usr/bin/env python3
import re
import yaml

fname="day4.input"
fname_rules="day4.yaml"

with open(fname) as f:
    data = []
    entry={}
    for line in f:
        try:
            for field in re.split(' ',line.rstrip()):
                key,value=re.split(':',field)
                entry[key]=value
        except:
            if entry:
                data.append(entry)
                entry={}


with open(fname_rules, 'r') as stream:
    try:
        rules = yaml.safe_load(stream)
    except yaml.YAMLError as exc:
        print(exc)


def all_entries_present(rules,entry):
    for v in rules:
        if v not in entry:
            return False
    return True

def all_entries_valid(rules,entry):
    for rule, criteria in rules.items():
        field=entry[rule]
        # print(rule,criteria,field)
        if not globals()["validate_"+criteria["type"]](field,criteria["range"]):
            # print(criteria["type"],field,criteria["range"])
            return False
    return True

def valid_number(value,min,max):
    return int(value) >= int(min) and int(value) <= int(max)

def validate_year(value,valid_range):
    return valid_number(value,valid_range["min"],valid_range["max"] )

def validate_length(value,valid_range):
    suffix=value[-2:]
    prefix=value[0:-2]
    try:
        rule = valid_range[suffix]
    except:
        return False
    return valid_number(prefix,rule["min"],rule["max"] )

def validate_hex(value,valid_range):
    prefix=value[0:1]
    try:
      int_val = int(value[1:], 16)
    except:
        return False
    return prefix == "#" and len(value) == valid_range["size"]+1

def validate_name_color(value,valid_range):
    return value in valid_range

def validate_dec(value,valid_range):
    try:
        int_val = int(value, 10)
    except:
        return False
    return len(value) == valid_range["size"]

all_present=0
all_valid=0
for entry in data:
    if all_entries_present(rules,entry):
        all_present=all_present+1
        if all_entries_valid(rules, entry):
            all_valid=all_valid+1


print(all_present)
print(all_valid)
