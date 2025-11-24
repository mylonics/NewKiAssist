# Knowledge Gaps

## 1. KiCadConnectionConfig Class
**Status**: Missing from kicad-python 0.5.0

**Evidence**: Filesystem analysis of installed package shows no `KiCadConnectionConfig` class in `kipy` module.

**Impact**: Current code at line 112-115 of `kicad_ipc.py` uses this non-existent class:
```python
config = KiCadConnectionConfig(
    socket_path=socket_path,
    client_name="kiassist-probe"
)
```

**Resolution**: Constructor parameters should be passed directly to `KiCad()`:
```python
kicad = KiCad(socket_path=socket_path, client_name="kiassist-probe")
```

## 2. Multiple Socket File Handling
**Status**: [HYPOTHESIS] Multiple KiCad instances create separate socket files

**Evidence**: Current code in `discover_socket_files()` searches for `api*.sock` pattern (lines 73-74), suggesting multiple sockets with names like `api1.sock`, `api2.sock`, etc.

**Gap**: No official documentation found confirming socket naming convention for multiple instances.

**[HYPOTHESIS]**: KiCad creates one socket per instance with sequential numbering or random identifiers. Default socket is `api.sock`.

**Validation needed**: Test with multiple KiCad instances to confirm actual socket naming pattern.

## 3. Connection Timeout Behavior
**Status**: [HYPOTHESIS] Connection attempt times out after 2000ms by default

**Evidence**: `KiCad.__init__` has `timeout_ms: int=2000` parameter (line 125 in kicad.py).

**Gap**: Documentation for behavior when KiCad is not running or socket is stale.

**[HYPOTHESIS]**: Failed connections raise an exception (likely `ApiError` from `kipy.client`). Code at line 145-147 catches generic `Exception`.

**Impact**: Current error handling may be too broad.

## 4. Project Name Extraction
**Status**: [HYPOTHESIS] Project name from document path may not always be available

**Evidence**: Current code (lines 128-135) wraps project name extraction in try/except, defaulting to "No Project Open".

**Gap**: Conditions under which `get_open_documents()` returns empty list or raises exception.

**[HYPOTHESIS]**: Returns empty list when no documents open, may raise exception if KiCad version incompatible or connection lost during call.

## 5. Version Compatibility Check
**Status**: Missing implementation

**Evidence**: `KiCad.check_version()` method exists (confirmed in dir() output) but not used in current code.

**Recommendation**: Should verify KiCad version >= 9.0.0 before attempting API calls.

**Impact**: May fail with cryptic errors on older KiCad versions.

## 6. Documentation Availability
**Status**: Official documentation site (https://docs.kicad.org/kicad-python-main/) exists but content not accessible via curl

**Gap**: Unable to verify complete API surface, method signatures, or usage examples beyond package inspection.

**Mitigation**: Package source code and docstrings provide sufficient information for basic usage.
