from __future__ import annotations

from typing import TYPE_CHECKING


if TYPE_CHECKING:
    from chrono_api import ChronoApi


class ChronoScript:    
    def __init__(self, chrono_api: ChronoApi):
        self.api = chrono_api

    async def run(self):
        from asyncio import wait_for   
        from asyncio import TimeoutError
        """ For some reason the imports get discarded, 
        that's why we have to import them right into the function """

        try:
            await wait_for(self.api.walk(5), 2.)
        except TimeoutError:
            print("Future was canceled!")


SCRIPT = ChronoScript