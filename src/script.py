from __future__ import annotations

from asyncio import wait_for   
from asyncio import TimeoutError
from asyncio import create_task

import chrono_api.core as chrono_api
from chrono_api.script import ScriptBase

def cool(): 
    print("Called bro")

class Script(ScriptBase): 
    async def run(self) -> None:

        entity = await chrono_api.get_character()
        print(entity.name)
        print(entity.level)

        chrono_api.walk(10)
        chrono_api.walk(10)
        chrono_api.walk(10)

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
