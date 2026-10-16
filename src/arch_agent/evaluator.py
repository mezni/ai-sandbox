from arch_agent.domain.evaluation import EvaluationReport
from arch_agent.domain.proposal import ArchitectureProposal


class ProposalEvaluator:
    def evaluate(
        self,
        proposal: ArchitectureProposal,
    ) -> EvaluationReport:
        checks = {
            "problem_statement": bool(proposal.problem_statement.strip()),
            "requirements": len(proposal.requirements) > 0,
            "ai_pattern": bool(proposal.ai_pattern.strip()),
            "architecture": bool(proposal.architecture.strip()),
            "risks": len(proposal.risks) > 0,
            "roadmap": bool(proposal.roadmap.strip()),
        }
        findings = [f"missing: {name}" for name, ok in checks.items() if not ok]
        return EvaluationReport(
            score=sum(checks.values()) / len(checks),
            passed=not findings,
            findings=findings,
        )
