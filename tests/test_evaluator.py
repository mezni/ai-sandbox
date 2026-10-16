from arch_agent.domain.proposal import (
    ArchitectureProposal,
)
from arch_agent.evaluator import (
    ProposalEvaluator,
)


def test_complete_proposal_passes():
    proposal = ArchitectureProposal(
        problem_statement="Problem",
        requirements=["req"],
        ai_pattern="RAG",
        architecture="Architecture",
        risks=["risk"],
        roadmap="Roadmap",
    )
    result = ProposalEvaluator().evaluate(proposal)
    assert result.passed


def test_missing_roadmap_fails():
    proposal = ArchitectureProposal(
        problem_statement="Problem",
        requirements=["req"],
        ai_pattern="RAG",
        architecture="Architecture",
        risks=[],
        roadmap="",
    )
    result = ProposalEvaluator().evaluate(proposal)
    assert not result.passed
