# Followup — arch-agent

## Step 1: Define the product
### input/output
- User: I want a chatbot for my bank
- System: Here is a AI solution poposal ...

### Architecture
User -> Prompt -> LLM -> Response

## Problems
- Prompt is hardcoded
- Prompt changes require code changes
- No prompt versionning
- No testing
- No domain model
- No evaluation