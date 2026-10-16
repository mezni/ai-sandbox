from arch_agent.parser import ProposalParser


def fake_llm(prompt: str) -> str:
    return """
PROBLEM:
Lawyers spend excessive time
reviewing contracts manually.
REQUIREMENTS:
- upload contracts
- summarize contracts
- detect risks
AI_PATTERN:
RAG
ARCHITECTURE:
Web UI + Retrieval + LLM
RISKS:
- hallucinations
- confidential data exposure
ROADMAP:
Phase 1 MVP
"""


def test_parse():
    parser = ProposalParser()
    response = """
PROBLEM:
Problem
REQUIREMENTS:
- one
- two
AI_PATTERN:
RAG
ARCHITECTURE:
Architecture
RISKS:
- risk1
ROADMAP:
Phase 1
"""
    proposal = parser.parse(response)
    assert len(proposal.requirements) == 2
    assert proposal.ai_pattern == "RAG"
