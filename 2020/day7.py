#!/usr/bin/env python3
import re
fname="day7.input"

with open(fname) as f:
    data = {}
    for line in f:
        line = line.rstrip()
        # print(line)
        [bag, contents_string] = re.split(' contain ',line.rstrip())
        bag=bag.rstrip("s")
        contents={}
        if contents_string != "no other bags.":
            for content in re.split(", ", contents_string):
                content=content.rstrip("s.")
                [value, innerbag] = re.split(' ',content, 1)
                contents[innerbag]=int(value)
        data[bag]=contents
        # print(data)

def find_bag(bag_to_find):
    return bag_to_find in data

def match_rule(bag_to_find, in_bag):
    rule=data[in_bag]
    for innerbag, count in rule.items():
        if innerbag==bag_to_find or match_rule(bag_to_find, innerbag):
            return True
    return False


def find_bags_that_can_contain(bag_to_find):
    found = 0
    for bag, rule in data.items():
        if match_rule(bag_to_find, bag):
            found=found+1
    return found


def find_bags_inside(bag_to_find):
    found = 0
    rule=data[bag_to_find]
    for innerbag, count in rule.items():
        # print(innerbag, count)
        found = found + count
        found = found + (count * find_bags_inside(innerbag))
    return found


f = find_bags_that_can_contain("shiny gold bag")
print(f)

f = find_bags_inside("shiny gold bag")
print(f)
