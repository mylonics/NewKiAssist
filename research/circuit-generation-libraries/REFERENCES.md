# REFERENCES.md - Technical References

## Primary Sources

### [1] circuit-synth GitHub Repository
- **URL**: https://github.com/circuit-synth/circuit-synth
- **Description**: Python-based circuit design with KiCad integration and AI acceleration
- **Language**: Python
- **License**: MIT
- **Stars**: 46 (as of 2025-11-24)
- **Created**: 2025-07-25
- **Last Commit**: 2025-11-19

### [2] pcb (diodeinc) GitHub Repository
- **URL**: https://github.com/diodeinc/pcb
- **Description**: CLI for circuit boards using Zener language (Starlark-based DSL)
- **Language**: Rust
- **License**: MIT
- **Stars**: Not specified in API response
- **Created**: Recent (v0.2.28 as of 2025-11-21)
- **Last Commit**: 2025-11-21

### [3] atopile GitHub Repository
- **URL**: https://github.com/atopile/atopile
- **Description**: Design circuit boards with code - declarative .ato language
- **Language**: Python (with Zig performance components)
- **License**: MIT
- **Stars**: Not specified in API response
- **Created**: Mature project
- **Last Commit**: 2025-11-07

## Documentation Links

### circuit-synth Documentation
- [4] **ReadTheDocs**: https://docs.circuit-synth.com
- [5] **llm.txt**: https://github.com/circuit-synth/circuit-synth/blob/main/llm.txt
- [6] **kicad-sch-api**: https://github.com/circuit-synth/kicad-sch-api (companion library)
- [7] **PyPI Package**: https://pypi.org/project/circuit-synth/

### pcb (diodeinc) Documentation
- [8] **Zener Language Spec**: https://github.com/diodeinc/pcb/blob/main/docs/pages/spec.mdx
- [9] **Examples Directory**: https://github.com/diodeinc/pcb/tree/main/examples
- [10] **Installation Releases**: https://github.com/diodeinc/pcb/releases

### atopile Documentation
- [11] **Official Documentation**: https://docs.atopile.io/
- [12] **Package Registry**: https://packages.atopile.io/
- [13] **VS Code Extension**: https://marketplace.visualstudio.com/items?itemName=atopile.atopile
- [14] **Discord Community**: https://discord.gg/CRe5xaDBr3

## Example Projects

### circuit-synth Examples
- [15] **ESP32-C6 Development Board**: Included in cs-new-project template
- [16] **Circuit Patterns Library**: 7 pre-made patterns (buck/boost converters, battery chargers, sensors)

### pcb (diodeinc) Examples
- [17] **blinky.zen**: Simple LED circuit example
- [18] **Standard Library**: @stdlib:v0.2.23 with generic components

### atopile Examples
- [19] **NONOS Smart Speaker**: https://github.com/atopile/nonos
- [20] **AI-Pin (Humane Pin clone)**: https://github.com/atopile/ai-pin
- [21] **Hyperion Display**: https://github.com/atopile/hyperion

## Related Tools and Dependencies

### KiCAD Integration
- [22] **KiCAD Official**: https://kicad.org/
- [23] **KiCAD CLI (kicad-cli)**: Command-line interface for KiCAD operations
- [24] **KiCAD S-Expression Format**: Native file format for schematics and PCBs

### AI/LLM Integration
- [25] **Claude Code Integration** (circuit-synth): Built-in AI agents and skills
- [26] **llmstxt.org Convention**: Standard for AI-readable project documentation

## Academic and Industry References

### Code-Based Circuit Design Philosophy
- [27] **Hardware Description Languages**: Influence from Verilog/VHDL paradigm
- [28] **Infrastructure as Code**: Software engineering practices applied to hardware

## Component Search/Sourcing
- [29] **JLCPCB Component Search**: Integrated in circuit-synth
- [30] **DigiKey API**: Component availability and sourcing
- [31] **SnapEDA**: Component library source

## Related Projects (Acknowledgments in READMEs)
- [32] **starlark-rust**: https://github.com/facebookexperimental/starlark-rust (pcb foundation)
- [33] **tscircuit**: https://github.com/tscircuit/tscircuit (inspiration for pcb)
