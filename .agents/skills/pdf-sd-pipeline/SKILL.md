---
name: pdf-sd-pipeline
description: PDF document ingestion, OCR extraction, Stable Diffusion integration, and image pipeline workflows.
---
<?xml version="1.0" encoding="UTF-8"?>
<codernic_skill id="pdf-sd-pipeline" kind="workflow" version="1.0.0">
  <mental_model>
    PDF document ingestion, OCR extraction, Stable Diffusion integration, and image pipeline workflows.
    Coordinates document parsing, prompt conditioning, and asset generation.
  </mental_model>

  <workflow>
    <step order="1" name="PDF Ingestion and Parsing">
      <action>Extract text, layers, and document layout via native engine.</action>
    </step>
    <step order="2" name="Prompt Conditioning">
      <action>Perform generation prompt conditioning and latent space optimization.</action>
    </step>
    <step order="3" name="Output Evaluation and Indexing">
      <action>Evaluate generation quality and index generated assets.</action>
    </step>
  </workflow>

  <invariants>
    <invariant id="INV-ZERO-MCP">Zero MCP calls; invoke CLI with --json output only.</invariant>
    <invariant id="INV-PIRSIG-AST">Max 200 lines per file, max 40 lines per function.</invariant>
    <invariant id="INV-ZERO-STUB">Complete implementation without placeholder logic.</invariant>
  </invariants>

  <verification>
    <step name="skill-verify">coc-pm skill verify .agents/skills/pdf-sd-pipeline/SKILL.md --json</step>
  </verification>
</codernic_skill>
