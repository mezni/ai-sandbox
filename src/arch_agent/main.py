from .prompt_renderer import PromptRenderer
from .prompt_repository import PromptRepository


def fake_llm(prompt: str) -> str:
    return "=== GENERATED SOLUTION ===\n\n" + prompt


def main() -> None:
    idea = input("Describe your idea:\n")
    repo = PromptRepository()
    template = repo.load("solution_designer")
    prompt = PromptRenderer.render(template, {"idea": idea})
    response = fake_llm(prompt)
    print(response)


if __name__ == "__main__":
    main()
