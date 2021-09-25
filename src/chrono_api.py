from __future__ import annotations

from typing import Protocol


# class ChronoApi(Protocol):

class Client:
    @property
    async def character(self): ...
    
    @property
    async def monsters(self): ...
    
    @property
    async def players(self): ...

    async def walk(self, position: tuple[int, int]): ...

    async def attack(self, target: int): ...


async def get_clients() -> list[Client]: ...

async def walk(duration: int) -> None: ...