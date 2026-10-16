from dataclasses import dataclass


@dataclass
class EvaluationReport:
    score: float
    passed: bool
    findings: list[str]
