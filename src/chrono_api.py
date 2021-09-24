from typing import Protocol


class ChronoApi(Protocol):

    @staticmethod
    async def walk(duration: int) -> None: ...

    @staticmethod
    async def get_character() -> None: ...

