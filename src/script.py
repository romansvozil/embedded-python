from __future__ import annotations

from typing import TYPE_CHECKING
  
from asyncio import wait_for   
from asyncio import TimeoutError
from asyncio import create_task

if TYPE_CHECKING:
    import chrono_api.core as chrono_api


class ChronoScript: 
    async def run(self) -> None:

        entity = await chrono_api.get_character()
        print(entity.name)
        print(entity.level)

        try:
            await wait_for(chrono_api.walk(5), 2.)

        except TimeoutError:
            print("Walking was canceled!")

        async def wrap():
            await chrono_api.walk(5)

        create_task(wrap())
        await wrap()

SCRIPT = ChronoScript