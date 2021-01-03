#!/usr/bin/env python3
import re
import yaml
from dataclasses import dataclass, field
from typing import List
from typing import Dict
import copy

fname="day17.input"

@dataclass
class Layer:
    dimension: int = 0
    points: Dict[int, int] = field(default_factory = lambda: ({}))
    # min_points: []
    # max_points: []

    def __post_init__(self):
        assert self.dimension>0

    def len(self):
        return len(self.points)

    def get_pos(self,coords):
        coord=coords[-self.dimension-1]
        try:
            nextlayer=self.points[coord]
        except KeyError:
            #Next dimension has no entries yet. Therefore everything must be unset.
            return False
        return nextlayer.get_pos(coords)

    def set_pos(self,coords):
        coord=coords[-self.dimension-1]
        try:
            nextlayer=self.points[coord]
        except KeyError:
            #Next dimension has no entries yet. Create a new one
            if self.dimension==1:
                self.points[coord]=FinalLayer()
            else:
                self.points[coord]=Layer(self.dimension-1)
            nextlayer=self.points[coord]
        nextlayer.set_pos(coords)


    def clear_pos(self,coords):
        coord=coords[-self.dimension-1]
        try:
            nextlayer=self.points[coord]
        except KeyError:
            #Next dimension has no entries yet. Therefore is already unset.
            return
        nextlayer.clear_pos(coords)
        if nextlayer.len() == 0:
            #The whole layer is now empty. Delete it
            self.points.pop(coord)

    def count_active(self):
        r=0
        for k,v in self.points.items():
            r=r+v.count_active()
        return r

    def __repr__(self):
        r=""
        # if self.dimension==1:
        for k in sorted(self.points):
            v=self.points[k]
            r=r+f"\n{k}:{v}"
        return r
        # for 
        # r=f'dim{self.dimension}:{self.points}\n'
        # return r

    def iter_coords(self,prefix=[]):
        for k,v in self.points.items():
            for x in v.iter_coords(prefix+[k]):
                yield x

    def get_min_max_coords(self,min_coords,max_coords):
        min_coords[-self.dimension-1] = min(min(self.points),min_coords[-self.dimension-1])
        max_coords[-self.dimension-1] = max(max(self.points),max_coords[-self.dimension-1])
        for k,v in self.points.items():
            v.get_min_max_coords(min_coords,max_coords)


@dataclass
class FinalLayer(Layer):

    def __post_init__(self):
        assert self.dimension==0

    def get_pos(self,coords):
        coord=coords[-1]
        try:
            val=self.points[coord]
        except KeyError:
            #must be unset.
            return False
        assert type(val)==bool
        return val

    def set_pos(self,coords):
        coord=coords[-1]
        self.points[coord]=True

    def clear_pos(self,coords):
        coord=coords[-1]
        #Remove the point completely
        self.points.pop(coord,None)

    def count_active(self):
        return len(self.points)

    def __repr__(self):
        # r=""
        # for i in range(min(self.points),max(self.points)+1):
        #     if i in self.points:
        #         r=r+"#"
        #     else:
        #         r=r+"."
        # return r
        return f'{sorted(self.points)}'


    def iter_coords(self,prefix=[]):
        for x in self.points:
                yield prefix+[x]

    def get_min_max_coords(self,min_coords,max_coords):
        min_coords[-1] = min(min(self.points),min_coords[-1])
        max_coords[-1] = max(max(self.points),max_coords[-1])


@dataclass
class Grid:
    dimensions: int = 3
    layers: Layer = field(default_factory = lambda: Layer(1))

    def __post_init__(self):
        assert self.dimensions>0
        if self.dimensions==1:
            self.layers=FinalLayer()
        else:
            self.layers.dimension = self.dimensions-1

    def get_pos(self,points):
        assert len(points) == self.dimensions
        return self.layers.get_pos(points)

    def set_pos(self,points,newval):
        if newval:
            return self.layers.set_pos(points)
        return self.layers.clear_pos(points)

    def count_active(self):
        return self.layers.count_active()

    def __repr__(self):
        return f'GRID({self.dimensions}){self.layers}\n'

    def check_adjacent_occupied(self,points):
        assert len(points) == self.dimensions
        return self._check_adjacent_occupied(points,[0]*self.dimensions,0)

    def _check_adjacent_occupied(self,points,offsets,pos):
        ret=0
        if pos+1==self.dimensions:
            #Final layer
            for o in range(-1,2):
                offsets[pos] = o
                if all(x==0 for x in offsets):
                    continue
                p=[x + y for x, y in zip(points, offsets)]
                val=self.get_pos(p)
                if val:
                    ret=ret+1
                # print("checking:",p,val)

            offsets[pos]=0
        else:
            for o in range(-1,2):
                offsets[pos] = o
                ret=ret+self._check_adjacent_occupied(points,offsets,pos+1)
            offsets[pos]=0
        return ret

    def get_min_max_coords(self):
        min_coords=[float('inf')]*self.dimensions
        max_coords=[float('-inf')]*self.dimensions
        self.layers.get_min_max_coords(min_coords,max_coords)
        return (min_coords,max_coords)

    def iterate_over_min_max_coords_with_border(self,border,func):
        (min_coords,max_coords) = self.get_min_max_coords()
        min_coords = [x-border for x in min_coords]
        max_coords = [x+border for x in max_coords]
        coords=copy.deepcopy(min_coords)
        self._iterate_over_min_max_coords(min_coords,max_coords,coords,0,func)

    def _iterate_over_min_max_coords(self,min_coords,max_coords,coord,pos,func):
        if pos==self.dimensions:
            func(coord)
            return
        for x in range(min_coords[pos],max_coords[pos]+1):
            coord[pos]=x
            self._iterate_over_min_max_coords(min_coords,max_coords,coord,pos+1,func)
            
@dataclass
class Life:
    grid : Grid
    next_grid : Grid = field(default_factory = lambda: None)

    def do_cube(self,coord):
        # print(coord)
        adjancent=self.grid.check_adjacent_occupied(coord)
        active=self.grid.get_pos(coord)
        if active:
            if adjancent==2 or adjancent==3:
                self.next_grid.set_pos(coord,True)
                # print("...setting")
        else:
            if adjancent==3:
                self.next_grid.set_pos(coord,True)
                # print("....setting")

    def do_turn(self):
        self.next_grid = Grid(self.grid.dimensions)
        (min_coords,max_coords) = self.grid.get_min_max_coords()
        # print("min_coords",min_coords)
        # print("max_coords",max_coords)
        self.grid.iterate_over_min_max_coords_with_border(1,self.do_cube)
        self.grid=self.next_grid

    def do_turns(self,turns):
        # print(l)
        for t in range(turns):
            # print("turn",t+1)
            self.do_turn()
            # print(l)
            # print(self.grid.count_active())

    def __repr__(self):
        return f'{self.grid}'


def read_input(g,dimensions):
    with open(fname) as f:
        data = []
        d=[0]*(dimensions-2)
        y=0
        for line in f:
            x=0
            for char in line.rstrip():
                if char=="#":
                    # print([z,y,x])
                    g.set_pos(d+[y,x],True)
                x=x+1
            y=y+1


for d in [3,4]:
    g = Grid(d)
    read_input(g,d)
    # print(g)
    l = Life(g)
    l.do_turns(6)
    print(l.grid.count_active())

