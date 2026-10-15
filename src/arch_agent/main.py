def build_prompt(idea: str) -> str:
    return f"""
You are a senior AI Solutions Architect.
Analyze the following business idea:
{idea}

Provide:
1. Problem Statement
2. Functional Requirements
3. AI Opportunities
4. Recommended Solution
5. High Level Architecture
6. Risks
7. Next Steps
"""


def fake_llm(prompt: str) -> str:
    """Placeholder until we connect a real model."""
    return f"""
=== GENERATED SOLUTION ===
Prompt received:
{prompt}
"""


def main():
    idea = input("Describe your business idea:\n")
    prompt = build_prompt(idea)
    response = fake_llm(prompt)
    print(response)


if __name__ == "__main__":
    main()
