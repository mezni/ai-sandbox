import os
from types import SimpleNamespace

import pytest

from ccaf.llm import llm_client as llm_client_module
from ccaf.llm.llm_client import ClaudeClient
from ccaf.llm.ticket_classifier import TicketClassifier


class FakeClaudeClient:
    def generate(self, prompt: str) -> str:
        assert "technical_support" in prompt
        return "technical_support"


def test_ticket_classifier_uses_claude_client():
    classifier = TicketClassifier(FakeClaudeClient())

    result = classifier.classify("My internet connection has stopped working.")

    assert result == "technical_support"
