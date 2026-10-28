---
name: pdf-sd
description: Sovereign PDF Document Ingestion, OCR Layout Extraction, and Generative Pipeline Engine.
---
<?xml version="1.0" encoding="UTF-8"?>
<codernic_skill id="pdf-sd" kind="application" package="@app/pdf-sd" version="0.1.0">
  <mental_model>
    @app/pdf-sd is a sovereign document processing and generative media engine.
    Integrates PDF document parsing, native Rust core-engine extraction, and pipeline workflows.
    Enforces complete implementations, Pirsig AST bounds, and zero-stub reliability.
  </mental_model>

  <sdk_surface>
    <crate name="core-engine" role="Native cdylib for PDF parsing, text extraction, and lopdf processing" />
    <pipeline name="pdf-sd-pipeline" role="Document layout parsing, OCR extraction, and asset generation" />
  </sdk_surface>

  <invariants>
    <invariant id="INV-001">Strict wire envelope: all CLI --json commands output SdkEnvelope&lt;T&gt; with namespace '@app/pdf-sd'.</invariant>
    <invariant id="INV-002">Pirsig complexity: max 200 lines per file, max 40 lines per function, zero warnings under cargo clippy -D warnings.</invariant>
    <invariant id="INV-003">Zero vaporware: absolute production readiness and strictly verified functionality in production surfaces.</invariant>
  </invariants>

  <verification>
    <step name="skill-verify">coc-pm skill verify skill/SKILL.md --json</step>
  </verification>
</codernic_skill>
