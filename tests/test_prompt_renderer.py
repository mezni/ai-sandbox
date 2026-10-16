from arch_agent.prompt_renderer import PromptRenderer


def test_prompt_render() -> None:
    template = "Hello {{name}}"
    rendered = PromptRenderer.render(template, {"name": "Mohamed"})
    assert rendered == "Hello Mohamed"
