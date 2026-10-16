from arch_agent.domain.proposal import ArchitectureProposal


class ProposalParser:
    def parse(
        self,
        response: str,
    ) -> ArchitectureProposal:
        sections = {}
        current = None
        for line in response.splitlines():
            line = line.strip()
            if line.endswith(":"):
                current = line[:-1]
                sections[current] = []
                continue
            if current:
                sections[current].append(line)
        return ArchitectureProposal(
            problem_statement="\n".join(sections.get("PROBLEM", [])),
            requirements=[
                x.replace("-", "").strip()
                for x in sections.get("REQUIREMENTS", [])
                if x
            ],
            ai_pattern="\n".join(
                sections.get(
                    "AI_PATTERN",
                    [],
                )
            ),
            architecture="\n".join(
                sections.get(
                    "ARCHITECTURE",
                    [],
                )
            ),
            risks=[
                x.replace("-", "").strip()
                for x in sections.get(
                    "RISKS",
                    [],
                )
                if x
            ],
            roadmap="\n".join(
                sections.get(
                    "ROADMAP",
                    [],
                )
            ),
        )
