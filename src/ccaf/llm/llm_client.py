from ccaf.llm.config import get_api_key, get_model
import anthropic


class ClaudeClient:
    def __init__(self) -> None:
        self._client = anthropic.Anthropic(api_key=get_api_key())
        self._model = get_model()

    def generate(self, prompt) -> str:
        response = self._client.messages.create(
            model=self._model,
            max_tokens=1000,
            messages=[{"role": "user", "content": prompt}],
        )
        for block in response.content:
            if block.content_type == "text":
                return block.text
        raise RuntimeError("Claude returned no text response")
