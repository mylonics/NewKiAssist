# KiAssist Research Documentation

This directory contains structured research reports for technical decisions in the KiAssist project.

---

## 1. Python Backend Migration Research

**Date**: 2025-11-23  
**Status**: Complete  
**Recommendation**: pywebview

### Quick Summary

Analyzed 4 Python frameworks for migrating Tauri (Rust + Vue.js) desktop app to Python-only backend:

| Option | Binary Size | RAM Usage | Verdict |
|--------|-------------|-----------|---------|
| **pywebview** ✅ | 15-40 MB | 50-120 MB | **RECOMMENDED** |
| Eel | 80-150 MB | 150-300 MB | Rejected (Chrome dependency) |
| Flask/FastAPI + pywebview | 25-55 MB | 60-140 MB | Rejected (unnecessary complexity) |
| Electron + Python | 150-250 MB | 200-400 MB | Rejected (contradicts migration goal) |

### Read More

- **[REPORT.md](python-backend-migration/REPORT.md)**: Complete findings and recommendation
- **[ANALYSIS.md](python-backend-migration/ANALYSIS.md)**: Detailed comparison table with pros/cons
- **[PROPOSAL.md](python-backend-migration/PROPOSAL.md)**: Implementation roadmap and risk mitigation
- **[GAPS.md](python-backend-migration/GAPS.md)**: Unconfirmed assumptions requiring validation
- **[SCOPE.md](python-backend-migration/SCOPE.md)**: Research objectives and constraints
- **[REFERENCES.md](python-backend-migration/REFERENCES.md)**: 15 technical references
- **[CHANGELOG.md](python-backend-migration/CHANGELOG.md)**: Research activity log

---

## 2. Installer Packaging Research

**Date**: 2025-11-24  
**Status**: Complete  
**Recommendation**: One-folder PyInstaller + platform-specific installers

### Quick Summary

Researched installer solutions for PyInstaller-based cross-platform distribution:

| Platform | Installer Tool | Format | Installation Path |
|----------|---------------|--------|-------------------|
| **Windows** | Inno Setup 6.3+ | `.exe` setup | `%LOCALAPPDATA%\Programs\KiAssist\` |
| **macOS** | create-dmg | `.dmg` disk image | `/Applications/KiAssist.app` |
| **Linux** | AppImage (primary) | `.AppImage` | `~/.local/bin/` |
| **Linux** | DEB (secondary) | `.deb` package | `/opt/kiassist/` |

### Key Findings

1. **Performance**: One-folder mode eliminates 2-5 second decompression overhead
2. **Data Persistence**: API keys stored via OS keyring persist automatically across updates
3. **CI/CD Integration**: All tools available on GitHub Actions runners
4. **Code Signing**: Required for macOS, recommended for Windows, optional for Linux

### Read More

- **[REPORT.md](installer-packaging/REPORT.md)**: Complete findings and recommendations
- **[ANALYSIS.md](installer-packaging/ANALYSIS.md)**: Detailed comparison of installer tools and configurations
- **[PROPOSAL.md](installer-packaging/PROPOSAL.md)**: Implementation roadmap with code examples
- **[GAPS.md](installer-packaging/GAPS.md)**: Knowledge gaps and validation strategies
- **[SCOPE.md](installer-packaging/SCOPE.md)**: Research objectives and constraints
- **[REFERENCES.md](installer-packaging/REFERENCES.md)**: 23 technical references
- **[CHANGELOG.md](installer-packaging/CHANGELOG.md)**: Research activity log

---

## 3. Circuit Generation Libraries Research

**Date**: 2025-11-24  
**Status**: Complete  
**Recommendation**: circuit-synth

### Quick Summary

Compared 3 circuit generation libraries for KiAssist's circuit database and AI-assisted design features:

| Library | Score | Recommendation |
|---------|-------|----------------|
| **circuit-synth** ✅ | 4.5/5 | **PRIMARY** - Native AI integration, Python-based, full KiCAD support |
| **atopile** | 4.0/5 | Secondary - Hardware-as-code, good for modular designs |
| **pcb (diodeinc)** | 3.0/5 | Not Recommended - Custom DSL, experimental Windows support |

### Key Findings

1. **AI Integration**: circuit-synth has native Claude Code agents and llm.txt documentation
2. **KiCAD Support**: circuit-synth generates all KiCAD file types (.kicad_sch, .kicad_pcb, .kicad_pro)
3. **Database Compatibility**: Python-based circuits serialize easily to JSON/YAML
4. **Learning Curve**: Python syntax is familiar to LLMs and developers

### Read More

- **[REPORT.md](circuit-generation-libraries/REPORT.md)**: Complete findings and recommendations
- **[ANALYSIS.md](circuit-generation-libraries/ANALYSIS.md)**: Detailed comparison of all three libraries
- **[PROPOSAL.md](circuit-generation-libraries/PROPOSAL.md)**: Implementation roadmap for KiAssist integration
- **[GAPS.md](circuit-generation-libraries/GAPS.md)**: Knowledge gaps and validation strategies
- **[SCOPE.md](circuit-generation-libraries/SCOPE.md)**: Research objectives and constraints
- **[REFERENCES.md](circuit-generation-libraries/REFERENCES.md)**: 33 technical references
- **[CHANGELOG.md](circuit-generation-libraries/CHANGELOG.md)**: Research activity log

---

## Research Template

Each research topic follows the same structure:

1. **SCOPE.md** - Objective, constraints, success criteria
2. **REFERENCES.md** - Numbered technical sources
3. **ANALYSIS.md** - Comparative analysis with tables
4. **GAPS.md** - Knowledge gaps tagged with `[HYPOTHESIS]`
5. **PROPOSAL.md** - Implementation roadmap
6. **REPORT.md** - Executive summary with recommendations
7. **CHANGELOG.md** - Research activity log
