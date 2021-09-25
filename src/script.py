from __future__ import annotations

from typing import TYPE_CHECKING
  
from asyncio import wait_for   
from asyncio import TimeoutError

if TYPE_CHECKING:
    import chrono_api


class ChronoScript: 
    async def run(self) -> None:

        try:
            await wait_for(chrono_api.walk(5), 2.)

        except TimeoutError:
            print("Walking was canceled!")


SCRIPT = ChronoScript