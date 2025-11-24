# kicad-python Library Research

**Research Date:** 2025-11-24  
**Status:** ✅ Complete  
**Researcher:** Scribe Agent

## Quick Reference

### Answer Summary
1. **PyPI Package Name:** `kicad-python`
2. **Current Stable Version:** 0.5.0 (released 2025-10-13)
3. **Import Statement:** ❌ Current code is INCORRECT
   - Should be: `from kipy import KiCad`
   - NOT: `from kicad import KiCad, KiCadConnectionConfig, DocumentType`
4. **Key Classes for IPC:**
   - `KiCad` (main connection class)
   - `base_types_pb2.DocumentType` (enum for document types)
   - Methods: `ping()`, `get_version()`, `get_open_documents()`
5. **Documentation:** https://docs.kicad.org/kicad-python-main/

### Critical Finding
⚠️ **The current code at `python-lib/kiassist_utils/kicad_ipc.py` lines 106-145 will NOT work** due to:
- Wrong module name (`kicad` should be `kipy`)
- Non-existent `KiCadConnectionConfig` class
- Incorrect `DocumentType` import path

## Document Structure

```
research/kicad-python-library/
├── README.md           ← You are here
├── REPORT.md          ← 📊 START HERE: Complete findings
├── PROPOSAL.md        ← 🔧 Implementation guide with code fixes
├── ANALYSIS.md        ← 📈 Comparison tables and technical details
├── GAPS.md            ← ⚠️ Unknowns and hypotheses
├── REFERENCES.md      ← 📚 Source citations
├── SCOPE.md           ← 🎯 Research objectives
└── CHANGELOG.md       ← 📝 Research history
```

## Reading Order

1. **For Quick Answers:** Read `REPORT.md` sections 1-5
2. **For Implementation:** Read `PROPOSAL.md` for exact code changes
3. **For Deep Dive:** Read `ANALYSIS.md` for comparative tables
4. **For Caveats:** Read `GAPS.md` for unverified assumptions

## Key Insights

### Correct Usage Pattern
```python
# ✅ CORRECT
from kipy import KiCad
from kipy.proto.common.types import base_types_pb2

kicad = KiCad(socket_path="ipc:///tmp/kicad/api.sock", client_name="myapp")
version = kicad.get_version()
docs = kicad.get_open_documents(base_types_pb2.DocumentType.DOCTYPE_PCB)
```

### Requirements
- Python ≥3.9
- KiCad ≥9.0 with API server enabled
- Dependencies: protobuf, pynng

### Installation
```bash
pip install kicad-python==0.5.0
```

## Research Provenance
- **Sources:** 6 references (PyPI API, package inspection, official docs)
- **Methodology:** Package installation + filesystem analysis + API introspection
- **Validation:** Direct import testing, dependency verification
- **Confidence:** High (verified through actual package installation)
