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

## Step 2: Prompt separation
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

## Step 3: Prompt versionning
### Problem
- How de we compare v1 and v2?
- How do we rollback?
- Which version is active?

### Solution 
- Treat prompts like code versionning (ex: api/v1 )

## Architecture
Application -> Prompt Repository -> Registry -> Versioned Prompt

### Benefits
- Controlled deployment
- Rollback capability
- Future prompt evaluations
- Production-friendly prompt management