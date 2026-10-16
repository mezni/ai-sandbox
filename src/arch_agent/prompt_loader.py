from pathlib import Path


class PromptLoader:
    def __init__(self, prompt_dir: str = "prompts") -> None:
        self.prompt_dir = Path(prompt_dir)

    def load(self, prompt_name: str) -> str:
        path = self.prompt_dir / f"{prompt_name}.txt"
        return path.read_text(encoding="utf-8")
