from __future__ import annotations

from typing import TYPE_CHECKING

from asyncio import wait_for   
from asyncio import TimeoutError

if TYPE_CHECKING:
    from chrono_api import ChronoApi


class ChronoScript:    
    def __init__(self, chrono_api: ChronoApi, _locals: dict):
        self.locals = _locals
        self.api = chrono_api

    def _init_locals(self):
        lx = globals()
        for k, v in self.locals.items():
            lx[k] = v

    async def run(self):
        self._init_locals()
         
        try:
            await wait_for(self.api.walk(5), 2.)
        except TimeoutError:
            print("Future was canceled!")


SCRIPT = ChronoScript