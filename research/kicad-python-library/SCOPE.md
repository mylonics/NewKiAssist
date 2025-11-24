# Research Scope: kicad-python Library

## Objective
Identify correct package information and usage patterns for the kicad-python library to enable IPC-based detection of running KiCad instances.

## Context
Current code imports from `kicad` package with:
```python
from kicad import KiCad, KiCadConnectionConfig, DocumentType
```

Issue requirement: "kicad-python library should be used to make the IPC calls to detect running instances."

## Research Questions
1. What is the correct PyPI package name?
2. What is the current stable version?
3. What is the correct import statement?
4. What classes/functions are available for IPC calls?
5. What documentation/examples exist?

## Constraints
- Must support KiCad 9.0 or higher
- Must enable IPC communication with running KiCad instances
- Must be compatible with Python 3.8+
- Code located at: `/home/runner/work/NewKiAssist/NewKiAssist/python-lib/kiassist_utils/kicad_ipc.py`
