from typing import Optional

from pydantic import BaseModel


class Character(BaseModel):
    name: str
    type: int
    level: int = 1
