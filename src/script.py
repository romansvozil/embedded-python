from __future__ import annotations

from asyncio import wait_for   
from asyncio import TimeoutError
from asyncio import create_task

import chrono_api.core as chrono_api
from chrono_api.utils import print_numbers

class ChronoScript: 
    async def run(self) -> None:
        print_numbers()

        entity = await chrono_api.get_character()
        print(entity.name)
        print(entity.level)

        try:
            await wait_for(chrono_api.walk(5), 2.)

        except TimeoutError:
            print("Walking was canceled!")

        async def wrap():
            # most of the "raw" calls should be wrapped in other async function, 
            # because the pyo3-asyncio currently runs the future right after 
            # creating the object, which doesn't really make sense
            await chrono_api.walk(5)

        create_task(wrap())
        await wrap()

SCRIPT = ChronoScript