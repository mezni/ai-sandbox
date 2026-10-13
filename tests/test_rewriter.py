import pytest

from my_ai_lib.rewriter import prompt_rewriter


def test_prompt_rewriter_concise():
    assert "Optimize" in prompt_rewriter("hello world", style="concise")


def test_prompt_rewriter_empty():
    assert prompt_rewriter("") == ""


def test_prompt_rewriter_invalid_type():
    with pytest.raises(TypeError):
        prompt_rewriter(123)  # type: ignore
