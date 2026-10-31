import os
from dotenv import load_dotenv

load_dotenv()


def get_api_key() -> str:
    api_key = os.getenv("ANTHROPIC_API_KEY")
    if not api_key:
        raise ValueError("ANTHROPIC_API_KEY is not configured")
    return api_key


def get_model() -> str:
    return os.getenv("ANTHROPIC_MODEL", "claude-haiku-4-5-20251001")
