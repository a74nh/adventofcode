#!/usr/bin/env python3
import re
fname="day14.input"

from dataclasses import dataclass, field
from typing import List
from typing import Dict
import copy

@dataclass
class Instr:
    def foo():
        pass

@dataclass
class Mem_Instr(Instr):
    loc: int
    val: int

@dataclass
class Mask_Instr(Instr):
    mask: str

@dataclass
class State:
    code: List[Instr] = field(default_factory=list)
    position: int = 0


@dataclass
class State:
    code: List[Instr] = field(default_factory=list)
    instr_pos: int = 0
    mem: Dict[int, int] = field(default_factory = lambda: ({0: 0}))
    mask: str = ""

    def __str__(self):
        try:
            c=self.code[self.instr_pos]
        except:
            c="finished"
        mem_entries=len(mem)
        return f'{self.instr_pos}={c} mems{mem_entries}'

    def run_instruction(self):
        self.instr_pos = self.instr_pos + 1

    def finished_execution(self):
        return self.instr_pos >= len(self.code)

    def run_until_complete(self):
        while not self.finished_execution():
            # print(self)
            self.run_instruction()
        return


class State_valmask(State):

    def run_instruction(self):
        i = self.code[self.instr_pos]
        # print(i)
        if type(i)==Mem_Instr:
            self.mem[i.loc]=self.apply_mask(i.val)
        elif type(i)==Mask_Instr:
            self.mask=i.mask
        else:
            print(self)
            raise ValueError
        self.instr_pos = self.instr_pos + 1

    def sum_mem(self):
        ret=0
        for loc,val in self.mem.items():
            ret = ret + val
        return ret

    def apply_mask(self,val):
        assert(val == mask_to_int(int_to_mask(val)))
        bin_val=int_to_mask(val)
        # print("pval",val)
        # print("orig",bin_val)
        # print("mask",self.mask)
        for index, v in enumerate(bin_val):
            # if type(self.mask[index]) == int:
            if self.mask[index] != 9:
                bin_val[index] = self.mask[index]
        # print("post",bin_val)
        # print("fval",mask_to_int(bin_val))
        # print()
        return mask_to_int(bin_val)



class State_memmask(State):

    def run_instruction(self):
        i = self.code[self.instr_pos]
        # print(i)
        if type(i)==Mem_Instr:
            mem_mask=self.apply_mask(i.loc)
            new_mems=self.mask_to_mem_locs(mem_mask)
            for m in new_mems:
                loc=mask_to_int(m)
                # print(loc)
                self.mem[loc]=i.val
        elif type(i)==Mask_Instr:
            self.mask=i.mask
        else:
            print(self)
            raise ValueError
        self.instr_pos = self.instr_pos + 1

    def sum_mem(self):
        ret=0
        for loc,val in self.mem.items():
            ret = ret + val
        return ret

    def apply_mask(self,val):
        bin_val=int_to_mask(val)
        for index, v in enumerate(bin_val):
            if self.mask[index] != 0:
                bin_val[index] = self.mask[index]
        return bin_val

    def mask_to_mem_locs(self,mem_mask):
        mem_masks=[mem_mask]
        pos=0
        mask_len=36
        while True:
            # print(mem_masks)
            while mem_masks[0][pos] != 9:
                pos=pos+1
                if pos>=mask_len:
                    return mem_masks
            new_masks=[]
            for m in mem_masks:
                m[pos]=1
                new_masks.append(copy.deepcopy(m))
                m[pos]=0
            mem_masks = mem_masks + new_masks

def int_to_mask(val):
    ret=[int(x) for x in list('{0:0b}'.format(val))]
    return [0]*(36-len(ret))+ret

def mask_to_int(val):
    return int(''.join(str(x) for x in val),2)

with open(fname) as f:
    data = []
    for line in f:
        [opcode, val] = re.split(' = ',line.rstrip())
        if opcode[0:3]=="mem":
            loc=re.split("[\[\]]",opcode)[1]
            data.append(Mem_Instr(int(loc),int(val)))
        elif opcode=="mask":
            def convert(x):
                if x=="X":
                    # return x
                    return 9
                return int(x)
            val=[convert(x) for x in list(val)]
            data.append(Mask_Instr(val))
# print(data)


s = State_valmask(data)
s.run_until_complete()
print(s.sum_mem())

s = State_memmask(data)
s.run_until_complete()
print(s.sum_mem())

