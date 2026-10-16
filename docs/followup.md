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

## Step 1: Define the product
### Problem
if we change 
- You are an AI Architect
to
- You are a Principal AI Architect
and a week later quality drops
- Questions: which prompt chaned? when? why?, who changed it? which version was better?
### Solution
- Refactor: move prompts into dedicated file

## Architecture
User -> Prompt Loader -> Prompt Template -> Prompt Renderer -> LLM -> Response
