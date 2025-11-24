# Proposal: Correct Implementation for kicad-python

## Recommended Changes

### 1. Update Package Dependency
Add to `pyproject.toml` dependencies:
```toml
"kicad-python>=0.5.0"
```

**Rationale**: Version 0.5.0 includes Flatpak socket autodetection and latest KiCad 9.0.5 compatibility.

### 2. Fix Import Statements
Replace current imports in `kicad_ipc.py`:

**Current (lines 106-107):**
```python
from kicad import KiCad, KiCadConnectionConfig, DocumentType
```

**Correct:**
```python
from kipy import KiCad
from kipy.proto.common.types import base_types_pb2
```

**Rationale**: 
- Module name is `kipy`, not `kicad` [Ref 5]
- `KiCadConnectionConfig` does not exist [Ref 5, GAPS.md #1]
- `DocumentType` is in protobuf types module [Ref 5, ANALYSIS.md]

### 3. Update Connection Code
Replace lines 112-116:

**Current:**
```python
config = KiCadConnectionConfig(
    socket_path=socket_path,
    client_name="kiassist-probe"
)
kicad = KiCad(config)
```

**Correct:**
```python
kicad = KiCad(
    socket_path=socket_path,
    client_name="kiassist-probe"
)
```

**Rationale**: Constructor accepts parameters directly [Ref 5, ANALYSIS.md].

### 4. Update DocumentType Reference
Replace line 129:

**Current:**
```python
docs = kicad.get_open_documents(DocumentType.DOCTYPE_PCB)
```

**Correct:**
```python
docs = kicad.get_open_documents(base_types_pb2.DocumentType.DOCTYPE_PCB)
```

**Rationale**: DocumentType is enum from protobuf module [Ref 5, ANALYSIS.md].

### 5. Enhance Error Handling (Optional Improvement)
Consider catching specific exceptions:

```python
try:
    from kipy import KiCad
    from kipy.proto.common.types import base_types_pb2
    from kipy.errors import ApiError
except ImportError:
    print("Warning: kicad-python package not available...")
    return None

# Later in probe_kicad_instance:
try:
    kicad = KiCad(socket_path=socket_path, client_name="kiassist-probe")
    # ... rest of code
except ApiError as e:
    print(f"Warning: KiCad API error at {socket_path}: {e}")
    return None
except Exception as e:
    print(f"Warning: Could not probe KiCad instance at {socket_path}: {e}")
    return None
```

**Rationale**: More precise error handling, distinguishes API errors from other exceptions.

## Implementation Priority
1. ✅ **Critical**: Fix imports (items 2, 3, 4) - code will not run without these
2. ⚠️ **High**: Add dependency (item 1) - required for installation
3. 📝 **Optional**: Enhance error handling (item 5) - improves diagnostics

## Validation Steps
1. Install kicad-python: `pip install kicad-python>=0.5.0`
2. Verify imports: `python -c "from kipy import KiCad"`
3. Test socket discovery with KiCad running
4. Verify instance detection returns expected data structure
