# Changelog

## 2025-11-24 - Initial Research

### Created
- `SCOPE.md`: Defined research objectives and constraints
- `REFERENCES.md`: Documented 6 primary sources (PyPI, GitLab, package inspection)
- `ANALYSIS.md`: Comparative analysis of package metadata, import patterns, dependencies
- `GAPS.md`: Identified 6 knowledge gaps with hypotheses
- `PROPOSAL.md`: Concrete implementation recommendations with code examples
- `REPORT.md`: Comprehensive findings summary

### Key Findings
1. Package name: `kicad-python` (PyPI) installs as `kipy` module
2. Current stable version: 0.5.0 (released 2025-10-13)
3. Current code has 3 critical import errors:
   - Wrong module name (`kicad` → `kipy`)
   - Non-existent class (`KiCadConnectionConfig`)
   - Wrong DocumentType import path
4. Requires KiCad 9.0+ with API server enabled
5. Python 3.9+ required (incompatible with Python 3.8)

### Research Methodology
1. PyPI API analysis for package metadata
2. Local package installation and filesystem inspection
3. Python introspection of module structure
4. Documentation URL validation
5. Dependency tree analysis

### Files Analyzed
- `/home/runner/work/NewKiAssist/NewKiAssist/python-lib/kiassist_utils/kicad_ipc.py` (lines 106-145)
- `/home/runner/work/NewKiAssist/NewKiAssist/python-lib/pyproject.toml`
- Installed package at `/home/runner/.local/lib/python3.12/site-packages/kipy/`

### Next Steps
1. Implement import corrections from PROPOSAL.md
2. Add kicad-python to dependencies
3. Test with running KiCad instance to validate hypotheses in GAPS.md
4. Consider adding version compatibility check
