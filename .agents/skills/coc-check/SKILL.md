---
name: coc-check
description: Inspect and verify Codernic Sovereign Package Manager dependencies and registry health.
---
<?xml version="1.0" encoding="UTF-8"?>
<codernic_skill id="coc-check" kind="workflow" version="1.0.0">
  <mental_model>
    Verify package integrity, dependency DAGs, and registry synchronization using COC.
    Maintains clean boundaries between upstream sources and managed package caches.
  </mental_model>

  <workflow>
    <step order="1" name="Package Integrity Validation">
      <action>Validate installed packages against cryptographic hashes using coc-pm CLI.</action>
      <command>coc-pm verify --json</command>
    </step>
    <step order="2" name="System Health Check">
      <action>Run system health check across local sovereign dependencies.</action>
      <command>coc-pm health --json</command>
    </step>
    <step order="3" name="Package Boundary Inspection">
      <action>Inspect package boundaries and ensure upstream source edits only.</action>
      <command>coc-pm check-boundaries --json</command>
    </step>
  </workflow>

  <invariants>
    <invariant id="INV-ZERO-MCP">Zero MCP calls; invoke coc-pm CLI with --json output only.</invariant>
    <invariant id="INV-UPSTREAM-BOUND">Never mutate .codernic/packages/ directly; modify upstream source repositories.</invariant>
  </invariants>

  <verification>
    <step name="verify">coc-pm verify --json</step>
    <step name="health">coc-pm health --json</step>
  </verification>
</codernic_skill>
