#!/usr/bin/env python3
import re
fname="day14.input"

from dataclasses import dataclass, field
from typing import List
from typing import Dict
import copy

data=[0,1,5,10,3,12,19]

@dataclass
class State:
    starter_numbers: List[int]
    last_spoken: Dict[int, int] = field(default_factory = lambda: ({0: 0}))

    def play_game(self,max_turn):
        turn=1
        for spoken in data:
            self.thus_spoke_elf(spoken,turn)
            # print(turn,spoken)
            turn=turn+1

        while turn <= max_turn:
            spoken=self.what_to_say(spoken)
            self.thus_spoke_elf(spoken,turn)
            # print(turn,spoken)
            turn=turn+1

        return spoken

    def what_to_say(self, prev_spoken):
        prev_spoken_times=self.last_spoken[prev_spoken]
        # print(prev_spoken_times)
        try:
            speak = prev_spoken_times[-1] - prev_spoken_times[-2]
        except:
            speak = 0
        return speak

    def thus_spoke_elf(self, speak, turn):
        try:
            self.last_spoken[speak].append(turn)
        except:
            self.last_spoken[speak]=[turn]

s = State(data)
print(s.play_game(2020))

s = State(data)
print(s.play_game(30000000))
