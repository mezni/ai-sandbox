from arch_agent.prompt_renderer import PromptRenderer
from arch_agent.prompt_repository import PromptRepository


def test_active_version_loaded() -> None:
    repo = PromptRepository()
    template = repo.load("solution_designer")
    assert len(template) > 0


def test_render() -> None:
    template = "Hello {{name}}"
    result = PromptRenderer.render(template, {"name": "Mohamed"})
    assert result == "Hello Mohamed"
