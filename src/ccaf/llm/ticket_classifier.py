from ccaf.llm.llm_client import ClaudeClient


class TicketClassifier:
    def __init__(self, claude: ClaudeClient) -> None:
        self._claude = claude

    def classify(self, message: str) -> str:
        prompt = f"""
Classify this customer support ticket into one category:
- billing
- technical_support
- order_status
- refund
- other

Return only the category.

Ticket:
{message}
"""

        return self._claude.generate(prompt)
