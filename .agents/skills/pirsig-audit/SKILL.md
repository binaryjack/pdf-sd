---
name: pirsig-audit
description: Audit source files against Pirsig AST invariants (max 200 lines per file, max 40 lines per function, zero any types, zero placeholders).
---
<?xml version="1.0" encoding="UTF-8"?>
<codernic_skill id="pirsig-audit" kind="workflow" version="1.0.0">
  <mental_model>
    Audit source files against Codernic Pirsig sovereign AST quality invariants.
    Enforces structural bounds, complete typing, anti-vaporware, and zero-warning compilation.
  </mental_model>

  <workflow>
    <step order="1" name="Structural Bounds Audit">
      <action>Verify line count limits: max 200 lines per file and max 40 lines per function.</action>
    </step>
    <step order="2" name="Anti-Vaporware and De-stubbing Scan">
      <action>Verify zero todo!(), unimplemented!(), any, or mock responses.</action>
    </step>
    <step order="3" name="Zero-Warning Compilation Gate">
      <action>Execute clippy pass with zero warning tolerance under -D warnings.</action>
    </step>
  </workflow>

  <invariants>
    <invariant id="INV-PIRSIG-BOUNDS">Max 200 lines per file, max 40 lines per function.</invariant>
    <invariant id="INV-ZERO-STUB">Absolute ban on todo!(), unimplemented!(), any, or mock responses.</invariant>
    <invariant id="INV-ZERO-WARNINGS">Zero compiler and clippy warnings (-D warnings).</invariant>
  </invariants>

  <verification>
    <step name="clippy">cargo clippy -- -D warnings</step>
    <step name="test">cargo test</step>
  </verification>
</codernic_skill>
