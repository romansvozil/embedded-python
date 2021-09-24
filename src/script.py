from __future__ import annotations

from typing import TYPE_CHECKING

from asyncio import wait_for   
from asyncio import TimeoutError

if TYPE_CHECKING:
    import chrono_api


class ChronoScript:    
    def __init__(self, _locals: dict):
        self.locals = _locals

    def _init_locals(self):
        """ :puke: """
        lx = globals()
        for k, v in self.locals.items():
            lx[k] = v

    async def run(self):
        self._init_locals() # has to be called at start of the run function
         
        try:
            await wait_for(chrono_api.walk(5), 2.)
        except TimeoutError:
            print("Walking was canceled!")


SCRIPT = ChronoScript