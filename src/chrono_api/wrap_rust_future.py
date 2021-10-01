from typing import Callable

def wrap_rust_future(func: Callable):
    async def wrapped(*args, **kwargs):
        return await func(*args, **kwargs)
    return wrapped