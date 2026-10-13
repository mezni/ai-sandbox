import re


def prompt_rewriter(prompt: str, style: str = "concise") -> str:
    """Rewrites and optimizes a given text prompt for AI consumption."""
    if not isinstance(prompt, str):
        raise TypeError("Prompt must be a string.")

    cleaned = re.sub(r"\s+", " ", prompt).strip()
    if not cleaned:
        return ""

    if style == "concise":
        return (
            f"Optimize the following instruction for clarity and directness: {cleaned}"
        )
    elif style == "detailed":
        return f"Please provide a comprehensive, step-by-step response to the following prompt with detailed context: {cleaned}"
    elif style == "markdown":
        return f"### Task\n{cleaned}\n\n### Requirements\n- Be precise\n- Use markdown formatting"
    else:
        return cleaned
