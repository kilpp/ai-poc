from typing import List, Optional

from pydantic import BaseModel, Field, validator


class SentimentRequest(BaseModel):
    text: str = Field(..., description="Text to analyze")
    language: Optional[str] = Field(
        default=None, description="Optional language hint (ISO 639-1)"
    )

    @validator("text")
    def text_must_not_be_empty(cls, value: str) -> str:  # noqa: D417
        if not value or not value.strip():
            raise ValueError("text must not be empty")
        return value


class BatchSentimentRequest(BaseModel):
    texts: List[str] = Field(..., description="List of texts to analyze")
    language: Optional[str] = Field(
        default=None, description="Optional language hint applied to all"
    )

    @validator("texts")
    def texts_must_not_be_empty(cls, value: List[str]) -> List[str]:  # noqa: D417
        if not value:
            raise ValueError("texts must not be empty")
        if any(not t.strip() for t in value):
            raise ValueError("texts must not contain empty items")
        return value


class SentimentResult(BaseModel):
    label: str
    score: float
    model: str
    language: Optional[str] = None


class HealthResponse(BaseModel):
    status: str
    model: str
