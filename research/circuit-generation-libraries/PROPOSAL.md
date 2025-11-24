# PROPOSAL.md - Implementation Recommendations

## Recommendation Summary

**Primary Recommendation**: **circuit-synth**

**Secondary Recommendation**: **atopile** (for specific use cases)

---

## Justification for circuit-synth Selection

### 1. AI Integration Alignment

circuit-synth is the only library with explicit, built-in AI/LLM integration:

- Claude Code agents for circuit design assistance
- llm.txt file following llmstxt.org convention for AI-readable documentation
- Slash commands for common operations
- Domain-specific AI experts

This directly aligns with KiAssist's goal of AI-powered KiCAD design assistance.

### 2. Python Ecosystem Benefits

- Python is widely known among electronics engineers and hobbyists
- LLMs have extensive Python training data, improving generation accuracy
- Easy serialization to JSON/YAML for database storage
- Rich ecosystem of supporting libraries

### 3. KiCAD Integration Quality

- Native generation of .kicad_sch, .kicad_pcb, .kicad_pro files
- Bidirectional workflow (import existing KiCAD projects)
- Automatic reference synchronization
- Integration with kicad-cli for manufacturing outputs

### 4. Database Suitability

Circuit definitions as Python functions can be:
- Serialized to JSON/YAML for database storage
- Version controlled with git
- Indexed and searched programmatically
- Composed from smaller reusable components

### 5. Active Development

- Frequent releases (multiple per month)
- Responsive to issues
- Growing feature set

---

## Integration Approach for KiAssist

### Architecture Overview

```
┌─────────────────────────────────────────────┐
│                 KiAssist                     │
│  ┌─────────────────────────────────────────┐│
│  │          Frontend (Electron/Tauri)       ││
│  └─────────────────────────────────────────┘│
│                      │                       │
│  ┌─────────────────────────────────────────┐│
│  │           Backend (Python)               ││
│  │  ┌─────────────────────────────────────┐││
│  │  │        circuit-synth Library        │││
│  │  └─────────────────────────────────────┘││
│  └─────────────────────────────────────────┘│
│                      │                       │
│  ┌─────────────────────────────────────────┐│
│  │        Circuit Database (SQLite/JSON)    ││
│  └─────────────────────────────────────────┘│
└─────────────────────────────────────────────┘
                       │
                       ▼
              ┌─────────────────┐
              │     KiCAD       │
              │  (.kicad_* files)│
              └─────────────────┘
```

### Phase 1: Basic Integration (Weeks 1-2)

1. **Install circuit-synth as dependency**
   ```bash
   pip install circuit-synth
   ```

2. **Create Python wrapper module**
   - Encapsulate circuit-synth API
   - Add error handling and logging
   - Provide JSON serialization utilities

3. **Implement basic circuit generation**
   - Simple component placement
   - Net connections
   - KiCAD project export

### Phase 2: Database Integration (Weeks 3-4)

1. **Design circuit schema**
   ```python
   circuit_record = {
       "id": "uuid",
       "name": "Power Supply 3.3V",
       "description": "LDO-based power supply",
       "tags": ["power", "ldo", "3.3v"],
       "code": "def power_supply(...): ...",
       "metadata": {
           "created": "2025-11-24",
           "author": "user",
           "version": "1.0"
       }
   }
   ```

2. **Implement storage backend**
   - SQLite for local storage
   - JSON export/import
   - Search and filtering

3. **Create circuit library browser**
   - List available circuits
   - Preview circuit details
   - Import into new projects

### Phase 3: AI Integration (Weeks 5-6)

1. **Connect to AI service**
   - Google Generative AI (as per existing KiAssist research)
   - Claude API (leveraging circuit-synth's built-in support)

2. **Implement circuit generation prompts**
   - Use circuit-synth's llm.txt as system context
   - Create structured prompts for circuit requests
   - Validate AI-generated code before execution

3. **Add circuit modification support**
   - Natural language circuit editing
   - Component substitution
   - Parameter adjustment

### Phase 4: Advanced Features (Weeks 7-8)

1. **Import existing KiCAD projects**
   - Leverage circuit-synth's bidirectional sync
   - Extract circuits for database storage

2. **Circuit composition**
   - Combine multiple database circuits
   - Automatic net merging

3. **Manufacturing preparation**
   - BOM generation
   - Gerber export integration

---

## Fallback Strategy

If circuit-synth proves inadequate during implementation:

### Option A: Switch to atopile

**When to consider**:
- Package registry ecosystem becomes valuable
- Constraint-based component selection is needed
- VS Code extension provides better user experience

**Migration effort**: MEDIUM (similar concepts, different syntax)

### Option B: Multi-library Support

**When to consider**:
- Different libraries suit different use cases
- User preference varies

**Implementation**:
- Abstract circuit generation interface
- Implement adapters for each library

---

## Technical Requirements

### Dependencies

| Package | Purpose | Version |
|---------|---------|---------|
| circuit-synth | Core circuit generation | >=0.11.3 |
| kicad-sch-api | Schematic manipulation | >=0.5.5 |
| Python | Runtime | >=3.12 |
| KiCAD | Target CAD tool | >=8.0 |

### System Requirements

| Platform | Status | Notes |
|----------|--------|-------|
| Windows | Supported | Python required |
| macOS | Supported | Python required |
| Linux | Supported | Python required |

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Circuit generation time | <5 seconds | Automated testing |
| AI generation accuracy | >80% valid circuits | Manual review |
| Database query time | <100ms | Performance testing |
| User satisfaction | >4/5 stars | User surveys |

---

## Next Steps

1. [ ] Create proof-of-concept circuit-synth integration
2. [ ] Design circuit database schema
3. [ ] Implement AI generation pipeline
4. [ ] Build circuit library browser UI
5. [ ] Test with real-world circuit designs
6. [ ] Document API for KiAssist developers

---

## Appendix: Alternative Considered

### Why Not pcb (diodeinc)?

- Zener DSL has steeper learning curve
- Less AI integration documentation
- Windows support marked as "experimental"
- Focus on PCB layout rather than schematic

### Why atopile as Secondary?

- Mature project with proven track record
- Package registry demonstrates database patterns
- Strong IDE integration
- May be valuable for future feature development
