# SCOPE.md - Research Scope Definition

## Objective

Identify the optimal circuit generation library for integration with KiAssist, a cross-platform desktop application for AI-powered KiCAD design assistance. The selected library must support:

1. Creating a database of circuits that can be queried to create new projects
2. AI-assisted creation of new circuits to add to the database
3. Native KiCAD compatibility

## Libraries Under Evaluation

| Library | Repository | Primary Language |
|---------|------------|------------------|
| circuit-synth | https://github.com/circuit-synth/circuit-synth | Python |
| pcb (diodeinc) | https://github.com/diodeinc/pcb | Rust (Zener/Starlark DSL) |
| atopile | https://github.com/atopile/atopile | Python (.ato DSL) |

## Constraints

1. **KiCAD Compatibility**: Library must export to KiCAD native formats (.kicad_sch, .kicad_pcb, .kicad_pro)
2. **Programmatic Generation**: Must support scriptable circuit creation for AI integration
3. **Database Storage Potential**: Circuit definitions must be storable and retrievable
4. **Cross-Platform**: Must work on Windows, macOS, and Linux
5. **Active Maintenance**: Library should have recent commits and active development
6. **Documentation Quality**: Sufficient documentation for integration development
7. **Learning Curve**: Reasonable complexity for end-user adoption

## Success Criteria

| Criterion | Weight | Description |
|-----------|--------|-------------|
| KiCAD Export Quality | HIGH | Direct generation of .kicad_* files without manual conversion |
| AI/LLM Friendliness | HIGH | Syntax and structure suitable for AI generation |
| Database Storage | HIGH | Text-based, serializable circuit definitions |
| Component Library Support | MEDIUM | Access to standard component libraries |
| Documentation | MEDIUM | Comprehensive API documentation and examples |
| Community Activity | MEDIUM | GitHub stars, commits, issue resolution |
| Installation Complexity | LOW | Package manager availability, dependency count |

## Out of Scope

- SPICE simulation integration (optional feature only)
- PCB autorouting capabilities (KiCAD handles this)
- BOM generation (secondary concern)
- Manufacturing file export (Gerbers handled by KiCAD)

## Research Methodology

1. Analyze GitHub repositories for each library
2. Review documentation and code examples
3. Compare syntax and API design
4. Evaluate KiCAD integration mechanisms
5. Assess AI generation suitability
6. Document findings in structured reports

## Timeline

Research completed: 2025-11-24
