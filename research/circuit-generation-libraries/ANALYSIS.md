# ANALYSIS.md - Comparative Analysis

## Executive Comparison Table

| Criterion | circuit-synth | pcb (diodeinc) | atopile |
|-----------|--------------|----------------|---------|
| **Language** | Python | Zener (Starlark DSL) | .ato (custom DSL) |
| **KiCAD Version** | KiCAD 8.0+ | KiCAD 9.x | KiCAD (version flexible) |
| **Export Formats** | .kicad_sch, .kicad_pcb, .kicad_pro | .kicad_pcb | .kicad_pcb |
| **Installation** | pip/uv install | Shell script/binary | pip/uv + VS Code extension |
| **AI Integration** | Built-in Claude Code agents | None documented | None documented |
| **Database Suitability** | HIGH (Python dicts) | MEDIUM (text files) | HIGH (text files) |
| **Learning Curve** | Low (Python) | Medium (new DSL) | Medium (new DSL) |
| **Maturity** | 4 months | 4 months | 1+ years |
| **Documentation** | Extensive | Good | Good |

---

## Detailed Analysis by Library

### 1. circuit-synth [Ref 1]

#### Syntax Example
```python
from circuit_synth import *

@circuit(name="Power_Supply")
def power_supply(vbus_in, vcc_3v3_out, gnd):
    regulator = Component(
        symbol="Regulator_Linear:AMS1117-3.3",
        ref="U",
        footprint="Package_TO_SOT_SMD:SOT-223-3_TabPin2"
    )
    cap_in = Component(symbol="Device:C", ref="C", value="10uF")
    
    regulator["VI"] += vbus_in
    regulator["VO"] += vcc_3v3_out
    cap_in[1] += vbus_in
    cap_in[2] += gnd
```

#### KiCAD Integration
- **Mechanism**: Direct generation of .kicad_sch, .kicad_pcb, .kicad_pro files
- **Companion Library**: kicad-sch-api (v0.5.5+) for schematic manipulation
- **Bidirectional Sync**: Can import existing KiCAD projects and modify them
- **Round-trip Support**: Automatic source reference rewriting for ref numbering

#### AI Integration Features
- Claude Code agents for circuit design assistance
- Slash commands: `/find-symbol`, `/find-parts`, `/generate-validated-circuit`
- llm.txt file following llmstxt.org convention
- Domain-specific AI experts: circuit-architect, simulation-expert, component-search

#### Manufacturing Support
- BOM export (CSV format)
- PDF schematic generation
- Gerber file export via kicad-cli
- JLCPCB component search integration

#### Pros
1. Native Python - no new language to learn
2. Extensive AI/LLM integration built-in
3. Bi-directional KiCAD workflow
4. Component availability search (JLCPCB, DigiKey)
5. Reusable circuit patterns library (7 pre-built patterns)
6. Active development with frequent releases

#### Cons
1. Relatively new project (created July 2025)
2. Requires KiCAD 8.0+ specifically
3. 191 open issues as of analysis date

---

### 2. pcb by diodeinc [Ref 2]

#### Syntax Example (Zener Language)
```python
load("@stdlib:v0.2.23/properties.zen", "Layout")
load("@stdlib:v0.2.23/board_config.zen", "Board", "BASE_4L")

Resistor = Module("@stdlib:v0.2.23/generics/Resistor.zen")
Led = Module("@stdlib:v0.2.23/generics/Led.zen")

vcc = Net("VCC")
gnd = Net("GND")

Resistor(
    name = "R1",
    value = "1kohm",
    package = "0402",
    P1 = vcc,
    P2 = led_anode
)

Board(
    name = "blinky",
    config = BASE_4L,
    layout_path = "layout/blinky"
)
```

#### KiCAD Integration
- **Mechanism**: Generates KiCAD PCB layout files via `pcb layout` command
- **Target**: KiCAD 9.x specifically
- **Workflow**: Compile → Layout → Open in KiCAD

#### CLI Commands
- `pcb build` - Validate and compile design
- `pcb layout` - Generate PCB layout
- `pcb open` - Open in KiCAD
- `pcb fmt` - Format .zen files
- `pcb lsp` - Language server for editor integration

#### Module System
- Hierarchical design with `Module()` for reusability
- Configuration via `config()` function
- IO interfaces via `io()` function
- Standard library with generic components

#### Pros
1. Rust implementation (performance, reliability)
2. Strong type safety in language design
3. LSP support for editor integration
4. WebAssembly support for browser-based execution
5. Cross-platform installers (macOS, Linux, Windows)
6. Version-locked standard library

#### Cons
1. Requires learning Zener DSL (Starlark-based)
2. Windows support marked as "experimental"
3. Newer project with evolving API
4. Less AI/LLM integration documentation
5. Schematic generation not as prominent (focus on PCB)

---

### 3. atopile [Ref 3]

#### Syntax Example (.ato Language)
```python
# Declarative circuit definition
module PowerSupply:
    signal vcc ~ power
    signal gnd ~ power

    component regulator ~ some_package
    regulator.vin ~ vcc
    regulator.gnd ~ gnd

    assert regulator.output_voltage within 3.3V +/- 5%
```

#### KiCAD Integration
- **Mechanism**: Compiler generates/updates .kicad_pcb files
- **Workflow**: Write .ato → `ato build` → Open layout in KiCAD
- **Sync**: Bidirectional with layout preservation

#### Key Features
- **Constraint Solver**: Automatic parametric component selection
- **Package Registry**: https://packages.atopile.io/
- **VS Code Extension**: Full IDE integration with language services
- **Design Checks**: Built-in validation and assertions

#### Module Reusability
- Package system for sharing circuit modules
- Import/export functionality
- Version management for dependencies

#### Pros
1. Most mature project (1+ years development)
2. Package registry ecosystem
3. Constraint-based design (assertions on parameters)
4. Active community (Discord)
5. Strong IDE support (VS Code extension)
6. Automatic component selection

#### Cons
1. Custom DSL requires learning curve
2. Less explicit AI integration
3. Zig components may complicate installation
4. Less focus on schematic generation

---

## Comparative Analysis by Use Case

### Use Case 1: Database of Circuits

| Library | Suitability | Rationale |
|---------|-------------|-----------|
| circuit-synth | ⭐⭐⭐⭐⭐ | Python dicts serializable to JSON/YAML; functions as database entries |
| pcb | ⭐⭐⭐⭐ | .zen files are text-based, versionable |
| atopile | ⭐⭐⭐⭐⭐ | .ato files text-based; package registry demonstrates storage model |

### Use Case 2: AI-Assisted Circuit Creation

| Library | Suitability | Rationale |
|---------|-------------|-----------|
| circuit-synth | ⭐⭐⭐⭐⭐ | Built-in Claude agents; llm.txt; Python is LLM-friendly |
| pcb | ⭐⭐⭐ | Starlark is readable but less common in training data |
| atopile | ⭐⭐⭐ | Custom DSL may require fine-tuning for AI generation |

### Use Case 3: KiCAD Compatibility

| Library | Suitability | Rationale |
|---------|-------------|-----------|
| circuit-synth | ⭐⭐⭐⭐⭐ | Full .kicad_* generation; bidirectional sync |
| pcb | ⭐⭐⭐⭐ | KiCAD 9.x PCB generation; focused on layout |
| atopile | ⭐⭐⭐⭐ | KiCAD PCB output; well-tested integration |

---

## Code Complexity Comparison

### Circuit Definition Length (Simple LED Circuit)

| Library | Lines of Code | Verbosity |
|---------|---------------|-----------|
| circuit-synth | ~15 | Medium |
| pcb (Zener) | ~20 | Medium |
| atopile | ~10 | Low |

### Hierarchical Design Support

| Library | Mechanism | Complexity |
|---------|-----------|------------|
| circuit-synth | Python functions/decorators | Low |
| pcb | Module() with config()/io() | Medium |
| atopile | module/component keywords | Medium |

---

## Community and Maintenance Metrics

| Metric | circuit-synth | pcb | atopile |
|--------|--------------|-----|---------|
| GitHub Stars | 46 | Data not collected | Data not collected |
| Open Issues | 191 | Data not collected | Data not collected |
| Last Release | v0.11.3 (2025-10-31) | v0.2.28 (2025-11-21) | v0.12.4 (2025-09-08) |
| Commit Frequency | High (daily) | High (daily) | Moderate (weekly) |
| Contributors | Active | Active | Active |

---

## Risk Assessment

| Risk | circuit-synth | pcb | atopile |
|------|--------------|-----|---------|
| Project Abandonment | LOW | LOW | LOW |
| Breaking API Changes | MEDIUM | MEDIUM | LOW |
| KiCAD Version Lock | MEDIUM (8.0+) | MEDIUM (9.x) | LOW |
| Documentation Gaps | LOW | MEDIUM | LOW |
