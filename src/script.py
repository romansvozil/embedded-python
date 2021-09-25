from __future__ import annotations

from typing import TYPE_CHECKING
import threading

from asyncio import wait_for   
from asyncio import TimeoutError

if TYPE_CHECKING:
    import chrono_api


class ChronoScript:    
    def __init__(self) -> None:
        ...

    def handle_map_event(self, event: str) -> None:
        print(threading.get_ident())
        print(event)

    async def run(self) -> None:
        print(threading.get_ident())
        try:
            await wait_for(chrono_api.walk(5), 2.)
        except TimeoutError:
            print("Walking was canceled!")


SCRIPT = ChronoScript