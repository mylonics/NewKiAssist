# Analysis: kicad-python Library Options

## Package Identity

| Aspect | Finding | Source |
|--------|---------|--------|
| PyPI Package Name | `kicad-python` | [Ref 2] |
| Import Module Name | `kipy` | [Ref 5] - filesystem analysis |
| Latest Stable Version | 0.5.0 (released 2025-10-13) | [Ref 2] |
| Python Version Required | >=3.9 | [Ref 2] |
| License | MIT | [Ref 6] |

## Import Statement Comparison

| Current Code | Actual kicad-python 0.5.0 | Status |
|--------------|---------------------------|--------|
| `from kicad import KiCad` | `from kipy import KiCad` | ❌ Incorrect module name |
| `from kicad import KiCadConnectionConfig` | No such class exists | ❌ Class does not exist |
| `from kicad import DocumentType` | `from kipy.proto.common.types import base_types_pb2`<br/>then `base_types_pb2.DocumentType` | ❌ Incorrect import path |

## Correct API Usage

| Function | Correct Pattern | Complexity |
|----------|----------------|------------|
| Create connection | `KiCad(socket_path=str, client_name=str, kicad_token=str, timeout_ms=int)` | Simple - constructor accepts parameters directly |
| Get version | `kicad.get_version()` returns `KiCadVersion` object | Simple |
| Get open documents | `kicad.get_open_documents(base_types_pb2.DocumentType.DOCTYPE_PCB)` | Simple |
| Check connectivity | `kicad.ping()` | Simple |

## Dependencies

| Package | Version Constraint | Purpose |
|---------|-------------------|---------|
| protobuf | <6,>=5.29 | Protocol buffer serialization |
| pynng | <0.9.0,>=0.8.0 | NNG (nanomsg-next-gen) IPC transport |
| typing_extensions | >=4.13.2 (Python <3.13) | Type hints backport |

## Socket Path Discovery

| Platform | Default Path | Environment Override |
|----------|--------------|---------------------|
| Windows | `%TEMP%\kicad\api.sock` | `KICAD_API_SOCKET` |
| Linux/Unix | `/tmp/kicad/api.sock` | `KICAD_API_SOCKET` |
| Linux (Flatpak) | `~/.var/app/org.kicad.KiCad/cache/tmp/kicad/api.sock` | Auto-detected |

## DocumentType Enum Values

| Value | Integer | Usage |
|-------|---------|-------|
| DOCTYPE_UNKNOWN | 0 | Unspecified document |
| DOCTYPE_SCHEMATIC | 1 | Schematic editor |
| DOCTYPE_SYMBOL | 2 | Symbol editor |
| DOCTYPE_PCB | 3 | PCB editor (primary for this use case) |
| DOCTYPE_FOOTPRINT | 4 | Footprint editor |
| DOCTYPE_DRAWING_SHEET | 5 | Drawing sheet editor |
| DOCTYPE_PROJECT | 6 | Project file |

## Version History (Recent Releases)

| Version | Release Date | Key Features | KiCad Compatibility |
|---------|-------------|--------------|---------------------|
| 0.5.0 | 2025-10-13 | Pad.pad_to_die_length, Board.get_enabled_layers, Flatpak socket autodetect | KiCad 9.0.4-9.0.5 |
| 0.4.0 | 2025-07-08 | Footprint movement/rotation fixes, sheet_path property, Net.name setter | KiCad 9.0 |
| 0.3.0 | 2025-03-29 | Mounting style, padstack APIs, board origin, arc methods | KiCad 9.0 |
| 0.2.0 | 2025-02-19 | KiCad 9.0.0 release updates, MIT relicense | KiCad 9.0.0 |
| 0.1.0 | 2024-12-21 | First IPC-API release | KiCad 9.0.0-rc1 |

Note: Versions 0.0.1-0.0.2 are obsolete and unrelated to current codebase [Ref 6].
