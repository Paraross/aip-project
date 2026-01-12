from pydantic import BaseModel, Field

class Text(BaseModel):
    text: str = Field(min_length=1)
