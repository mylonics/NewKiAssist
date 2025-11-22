# Analysis: Gemini API Endpoint Options

## API Version Comparison

| Aspect | v1 | v1beta |
|--------|-----|--------|
| **Stability** | Stable, production-ready [1,5] | Beta, subject to changes [5] |
| **Model Support** | Supports latest Gemini models [2] | Legacy support, being phased out [5] |
| **Deprecation Status** | Active, recommended [5] | Deprecated for new projects [5] |
| **Breaking Changes** | Minimal, follows semver [1] | Possible without notice [5] |
| **Documentation Coverage** | Primary focus [1,2] | Maintenance mode [5] |
| **Community Adoption** | Increasing (2024+) [6,7] | Legacy codebases [7] |

**Recommendation**: Use `v1` for production applications.

---

## Model ID Format Options

### Option A: Model Name Only
**Format**: `gemini-1.5-flash`  
**Endpoint**: `https://generativelanguage.googleapis.com/v1/models/gemini-1.5-flash:generateContent`

| Pros | Cons |
|------|------|
| Simpler format | Error reported in issue |
| Matches marketing names | May require "models/" prefix with v1 |
| Easier to read | Unclear if valid for v1 |

### Option B: Full Model Resource Name
**Format**: `models/gemini-1.5-flash`  
**Endpoint**: `https://generativelanguage.googleapis.com/v1/models/models/gemini-1.5-flash:generateContent`

| Pros | Cons |
|------|------|
| Matches resource naming pattern [2] | Creates double "models/" in URL |
| Used in some API responses [8] | Redundant with endpoint path |

### Option C: Model Name in Path (No Prefix)
**Format**: URL path only  
**Endpoint**: `https://generativelanguage.googleapis.com/v1/models/gemini-1.5-flash:generateContent`

| Pros | Cons |
|------|------|
| Clean URL structure [1,2,4] | Requires verifying exact model names |
| Matches official examples [4] | Must match Google's model catalog |
| Most common pattern [6] | |

**Recommendation**: Option C - model name directly in path without prefix.

---

## Current Available Models (Late 2024/Early 2025)

Based on official documentation [3]:

| Model Identifier | Status | Use Case | Context Window |
|-----------------|--------|----------|----------------|
| `gemini-1.5-flash` | ✅ Active | Fast responses, cost-effective | 1M tokens |
| `gemini-1.5-flash-8b` | ✅ Active | High volume, low latency | 1M tokens |
| `gemini-1.5-pro` | ✅ Active | Complex reasoning tasks | 2M tokens |
| `gemini-2.0-flash-exp` | ⚠️ Experimental | Preview of v2.0 | 1M tokens |
| `gemini-exp-1206` | ⚠️ Experimental | Experimental features | Variable |

**Notes**:
- "gemini-1.5-flash" and "gemini-1.5-pro" are stable model identifiers [3]
- Experimental models have "-exp" suffix [3]
- No "gemini-2.5-*" or "gemini-3-*" models exist as of early 2025 [3]
- Model names are lowercase with hyphens [2,3]

---

## Endpoint Structure Analysis

### Current Implementation (Incorrect)
```
https://generativelanguage.googleapis.com/v1beta/models/{model_id}:generateContent?key={api_key}
```
**Issues**:
- Uses deprecated v1beta [5]
- Model ID format may need verification

### Recommended Implementation
```
https://generativelanguage.googleapis.com/v1/models/{model_id}:generateContent?key={api_key}
```
**Changes**:
1. Replace `v1beta` → `v1`
2. Ensure model_id has no "models/" prefix
3. Model ID examples: `gemini-1.5-flash`, `gemini-1.5-pro`

**Example URLs**:
```
https://generativelanguage.googleapis.com/v1/models/gemini-1.5-flash:generateContent?key=YOUR_API_KEY
https://generativelanguage.googleapis.com/v1/models/gemini-1.5-pro:generateContent?key=YOUR_API_KEY
```

---

## Code Impact Analysis

### Changes Required in `src-tauri/src/gemini.rs`

| Line | Current | Required Change | Impact |
|------|---------|----------------|--------|
| 48 | `v1beta` | `v1` | Minimal - string replacement |
| 40-44 | Model mappings | Update to valid names | Remove non-existent models |

**Risk Level**: Low - single-line change for API version, model name cleanup.

---

## Summary Table

| Question | Answer | Confidence | Reference |
|----------|--------|------------|-----------|
| Correct API version? | `v1` | High | [1,2,5] |
| Model ID format? | No "models/" prefix in variable | High | [2,4,6] |
| Endpoint structure? | `v1/models/{model}:generateContent` | High | [1,2,4] |
| Valid models? | `gemini-1.5-flash`, `gemini-1.5-pro` | High | [3] |
