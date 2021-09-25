from __future__ import annotations

class Entity:
    x: int
    y: int
    name: str

class LivingEntity(Entity):
    hp: int
    max_hp: int
    mp: int
    max_map: int
    level: int

class Player(LivingEntity): ... 

class Monster(LivingEntity): ... 

class Item(Entity): ... 

class Portal: 
    x: int
    y: int

class Client:
    @property
    async def character(self) -> Player: ...
    
    @property
    async def monsters(self) -> list[Monster]: ...
    
    @property
    async def players(self) -> list[Player]: ...

    @property
    async def items(self) -> list[Item]: ...

    @property
    async def items(self) -> list[Portal]: ...

    async def walk(self, position: tuple[int, int]): ...

    async def attack(self, target: int): ...


async def get_clients() -> list[Client]: ...

async def walk(duration: int) -> None: ...