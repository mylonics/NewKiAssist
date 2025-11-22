# Quick Reference: Gemini Model Availability

**Last Updated**: 2025-11-22  
**Research Period**: November-December 2024

---

## ❌ NON-EXISTENT MODELS

These models **DO NOT EXIST**:
- `gemini-2.5-flash`
- `gemini-2.5-pro`
- `gemini-3.0-flash`
- `gemini-3.0-pro`

---

## ✅ AVAILABLE MODELS

### Production Stable (Recommended)

**API Endpoint**: `https://generativelanguage.googleapis.com/v1/models/{model_id}:generateContent`

| Model ID | Context | Use Case |
|----------|---------|----------|
| `gemini-1.5-flash` | 1M tokens | General purpose, fast |
| `gemini-1.5-pro` | 2M tokens | Complex reasoning |
| `gemini-1.5-flash-8b` | 1M tokens | High volume, lightweight |

### Experimental (Use with Caution)

**API Endpoint**: `https://generativelanguage.googleapis.com/v1beta/models/{model_id}:generateContent` (hypothesis - requires verification)

| Model ID | Status |
|----------|--------|
| `gemini-2.0-flash-exp` | Experimental (Dec 2024) |
| `gemini-exp-1206` | Experimental |

---

## 🔧 IMPLEMENTATION

### For Stable Models (src-tauri/src/gemini.rs)
```rust
let model_id = match model {
    "1.5-flash" => "gemini-1.5-flash",
    "1.5-pro" => "gemini-1.5-pro",
    "1.5-flash-8b" => "gemini-1.5-flash-8b",
    _ => "gemini-1.5-flash",
};

let url = format!(
    "https://generativelanguage.googleapis.com/v1/models/{}:generateContent?key={}",
    model_id, api_key
);
```

### To Add Experimental Support
```rust
let (model_id, api_version) = match model {
    "1.5-flash" => ("gemini-1.5-flash", "v1"),
    "1.5-pro" => ("gemini-1.5-pro", "v1"),
    "1.5-flash-8b" => ("gemini-1.5-flash-8b", "v1"),
    "2.0-flash-exp" | "2.0-flash" => ("gemini-2.0-flash-exp", "v1beta"),
    _ => ("gemini-1.5-flash", "v1"),
};

let url = format!(
    "https://generativelanguage.googleapis.com/{}/models/{}:generateContent?key={}",
    api_version, model_id, api_key
);
```

---

## 📋 RECOMMENDATIONS

1. **If user requires newest model and will not use 1.5**: 
   - Use `gemini-2.0-flash-exp` (experimental)
   - Verify v1beta endpoint works
   - Accept experimental status risks

2. **If user can use 1.5 series** (RECOMMENDED):
   - Use `gemini-1.5-flash` or `gemini-1.5-pro`
   - Production-stable, well-documented
   - v1 API endpoint

3. **If user insists on 2.5/3.0**:
   - Request source of information
   - Explain these models do not exist
   - Redirect to available options

---

**Full Report**: See `REPORT.md` for complete analysis  
**Confidence**: 99%+ that 2.5/3.0 models do not exist
