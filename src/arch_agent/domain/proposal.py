from dataclasses import dataclass


@dataclass
class ArchitectureProposal:
    problem_statement: str
    requirements: list[str]
    ai_pattern: str
    architecture: str
    risks: list[str]
    roadmap: str
