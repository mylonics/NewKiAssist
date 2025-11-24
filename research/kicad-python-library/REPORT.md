# Research Report: kicad-python Library

## Objective
Identify correct package information and usage for the kicad-python library to enable IPC-based detection of running KiCad instances.

## Summary
**Key Technical Findings:**
1. PyPI package name: `kicad-python` version 0.5.0 (released 2025-10-13)
2. Import module name: `kipy` (not `kicad`)
3. No `KiCadConnectionConfig` class exists; parameters passed directly to `KiCad()` constructor
4. Requires Python >=3.9, KiCad >=9.0 with API server enabled
5. Dependencies: protobuf <6,>=5.29, pynng <0.9.0,>=0.8.0

**Current Code Issues:**
- Incorrect import: `from kicad import ...` should be `from kipy import ...`
- Non-existent class: `KiCadConnectionConfig` does not exist in package
- Incorrect DocumentType import path

## Justification
The kicad-python package is the official Python API for KiCad IPC communication [Ref 1, 2, 6]. Version 0.5.0 was selected as current stable release with:
- KiCad 9.0.4-9.0.5 compatibility
- Flatpak socket path autodetection (critical for Linux users)
- 130 KB package size with minimal dependencies
- MIT license
- Active maintenance (5 releases in 2025)

Alternative approaches (SWIG-based bindings) are obsolete and do not support IPC [Ref 6: "requires communication with a running instance of KiCad"].

## Implementation Details

### 1. Correct PyPI Package Name
**Answer:** `kicad-python`

**Installation:**
```bash
pip install kicad-python==0.5.0
```

### 2. Current Stable Version
**Answer:** 0.5.0 (released October 13, 2025)

**Version History:**
- 0.5.0 (2025-10-13): Latest, adds Flatpak autodetect
- 0.4.0 (2025-07-08): Footprint operations, net creation
- 0.3.0 (2025-03-29): Padstack APIs, board origin
- 0.2.0 (2025-02-19): KiCad 9.0.0 release
- 0.1.0 (2024-12-21): First IPC-API release

### 3. Correct Import Statement
**Answer:** The current import is **incorrect**

**Current (wrong):**
```python
from kicad import KiCad, KiCadConnectionConfig, DocumentType
```

**Correct:**
```python
from kipy import KiCad
from kipy.proto.common.types import base_types_pb2
# Then use: base_types_pb2.DocumentType.DOCTYPE_PCB
```

**Explanation:**
- Package installs as module `kipy`, not `kicad`
- `KiCadConnectionConfig` class does not exist
- `DocumentType` is protobuf enum in `kipy.proto.common.types.base_types_pb2`

### 4. Key Classes/Functions for IPC Detection

**KiCad Class** (main entry point):
```python
KiCad(
    socket_path: Optional[str] = None,      # Defaults to platform-specific path
    client_name: Optional[str] = None,      # Defaults to random "anonymous-XXXXXXXX"
    kicad_token: Optional[str] = None,      # For authentication
    timeout_ms: int = 2000                  # Connection timeout
)
```

**Key Methods:**
| Method | Return Type | Purpose |
|--------|-------------|---------|
| `ping()` | bool | Test connection to KiCad |
| `get_version()` | KiCadVersion | Get KiCad version (major, minor, patch, full_version) |
| `get_open_documents(doc_type)` | Sequence[DocumentSpecifier] | List open documents of specified type |
| `check_version()` | void | Verify API version compatibility |

**DocumentType Enum Values:**
- `DOCTYPE_UNKNOWN = 0`
- `DOCTYPE_SCHEMATIC = 1`
- `DOCTYPE_SYMBOL = 2`
- `DOCTYPE_PCB = 3` ← Primary use case
- `DOCTYPE_FOOTPRINT = 4`
- `DOCTYPE_DRAWING_SHEET = 5`
- `DOCTYPE_PROJECT = 6`

**Default Socket Paths:**
- Windows: `%TEMP%\kicad\api.sock`
- Linux: `/tmp/kicad/api.sock`
- Flatpak: `~/.var/app/org.kicad.KiCad/cache/tmp/kicad/api.sock` (auto-detected in 0.5.0)
- Override: Set `KICAD_API_SOCKET` environment variable

### 5. Documentation and Examples

**Official Documentation:**
- URL: https://docs.kicad.org/kicad-python-main/
- Status: Referenced in package metadata but content not accessible via automated tools

**Repository:**
- GitLab: https://gitlab.com/kicad/code/kicad-python
- Issues: https://gitlab.com/kicad/code/kicad-python/-/issues

**Example Usage (Instance Detection):**
```python
from kipy import KiCad
from kipy.proto.common.types import base_types_pb2

def detect_kicad_instance(socket_path: str):
    try:
        # Connect to KiCad instance
        kicad = KiCad(
            socket_path=socket_path,
            client_name="instance-detector"
        )
        
        # Get version
        version = kicad.get_version()
        
        # Get open PCB documents
        docs = kicad.get_open_documents(
            base_types_pb2.DocumentType.DOCTYPE_PCB
        )
        
        # Extract project info
        if docs:
            doc = docs[0]
            project_path = doc.project.path
            project_name = Path(project_path).stem
        else:
            project_name = "No Project Open"
            
        return {
            "socket": socket_path,
            "version": f"{version.major}.{version.minor}.{version.patch}",
            "project": project_name
        }
    except Exception as e:
        print(f"Connection failed: {e}")
        return None
```

**Requirements:**
- Python >=3.9
- KiCad >=9.0 with API server enabled in Preferences → Plugins
- Network access to socket path

**Dependencies (auto-installed):**
```
protobuf>=5.29,<6
pynng>=0.8.0,<0.9.0
typing_extensions>=4.13.2  (Python <3.13 only)
```

## Gaps
See `GAPS.md` for detailed analysis. Critical gaps:
1. Multiple socket naming convention unverified ([HYPOTHESIS]: sequential numbering)
2. Documentation site content not accessible programmatically
3. Error behavior for stale sockets not documented
4. Project extraction may fail if no documents open

All gaps are non-blocking for basic implementation. Hypotheses marked in GAPS.md require empirical testing with running KiCad instances.

## Security Summary
No security vulnerabilities identified. Package:
- Uses MIT license (permissive)
- Maintained by official KiCad organization
- Dependencies are well-established (protobuf by Google, pynng for NNG protocol)
- No network exposure (local IPC only)
- Authentication via optional token parameter
