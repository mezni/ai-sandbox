import json
from pathlib import Path


class PromptRepository:
    def __init__(self, prompt_root: str = "prompts") -> None:
        self.prompt_root = Path(prompt_root)

    def get_active_version(self, prompt_name: str) -> str:
        registry_path = self.prompt_root / "registry.json"
        registry = json.loads(registry_path.read_text(encoding="utf-8"))
        return registry[prompt_name]["active_version"]

    def load(self, prompt_name: str) -> str:
        version = self.get_active_version(prompt_name)
        path = self.prompt_root / version / f"{prompt_name}.txt"
        return path.read_text(encoding="utf-8")
