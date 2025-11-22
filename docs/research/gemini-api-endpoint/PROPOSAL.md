# Proposal: Gemini API Endpoint Correction

## Recommended Solution

### Primary Change: Update API Version
**Action**: Replace `v1beta` with `v1` in endpoint URL  
**Location**: `src-tauri/src/gemini.rs`, line 48  
**Justification**: 
- v1 is stable and production-recommended [REFERENCES.md #1, #5]
- v1beta is deprecated [ANALYSIS.md - API Version Comparison]
- Error message indicates model not supported in v1beta [SCOPE.md]

### Secondary Change: Update Model Mappings
**Action**: Remove or update invalid model mappings  
**Location**: `src-tauri/src/gemini.rs`, lines 40-44  
**Justification**:
- Models "gemini-2.5-*" and "gemini-3-*" do not exist [ANALYSIS.md - Current Available Models]
- Mappings suggest UI may use incorrect identifiers [GAPS.md #5]
- Should use actual model names or remove fictional mappings

---

## Implementation Plan

### Step 1: Modify Endpoint URL (CRITICAL)
```rust
// Current (line 48)
"https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}"

// Proposed
"https://generativelanguage.googleapis.com/v1/models/{}:generateContent?key={}"
```

**Change**: `v1beta` → `v1`  
**Impact**: Resolves primary error  
**Risk**: Low - v1 is stable API

### Step 2: Update Model Mapping Logic (RECOMMENDED)
```rust
// Current (lines 39-45)
let model_id = match model {
    "2.5-flash" => "gemini-1.5-flash",
    "2.5-pro" => "gemini-1.5-pro",
    "3-flash" => "gemini-1.5-flash",
    "3-pro" => "gemini-1.5-pro",
    _ => "gemini-1.5-flash",
};

// Proposed Option A: Use actual model names
let model_id = match model {
    "1.5-flash" | "flash" => "gemini-1.5-flash",
    "1.5-pro" | "pro" => "gemini-1.5-pro",
    "1.5-flash-8b" | "flash-8b" => "gemini-1.5-flash-8b",
    _ => "gemini-1.5-flash",  // Safe default
};

// Proposed Option B: Pass through model names directly
let model_id = if model.starts_with("gemini-") {
    model  // Already in correct format
} else {
    format!("gemini-{}", model)  // Prefix if needed
};
```

**Rationale**: 
- Option A: Provides user-friendly aliases [ANALYSIS.md]
- Option B: Simplifies code, assumes correct input
- **Recommendation**: Option A for backward compatibility

### Step 3: Verification (POST-IMPLEMENTATION)
1. Test with valid API key
2. Confirm successful response from v1 endpoint
3. Validate all model variants work correctly

---

## Expected Outcomes

### Success Criteria
✅ API calls to Gemini succeed without "not found" error  
✅ Response format matches current parsing logic  
✅ Model selection functions as expected  

### Risk Mitigation
- **Risk**: Request/response format differs in v1  
  **Mitigation**: Hypothesis states formats are identical [GAPS.md #7, #8]; minimal change reduces risk
  
- **Risk**: Model names invalid in v1  
  **Mitigation**: Using documented model names from official source [ANALYSIS.md - Current Available Models]

---

## Alternative Approaches Considered

### Alternative 1: Stay with v1beta
**Rejected**: API version is deprecated, error indicates model not supported

### Alternative 2: Use Official SDK
**Rejected**: No official Rust SDK; REST API is appropriate for Tauri/Rust architecture

### Alternative 3: Change Model Names Only
**Rejected**: Error explicitly mentions "v1beta" API version issue; version must change

---

## Decision Matrix

| Approach | Stability | Effort | Maintenance | Score |
|----------|-----------|--------|-------------|-------|
| **v1 + clean mappings** | High | Low | Low | ⭐⭐⭐⭐⭐ |
| v1beta + workarounds | Low | Medium | High | ⭐⭐ |
| Complete refactor | High | High | Medium | ⭐⭐⭐ |

**Selected**: v1 + clean mappings (Option A from Step 2)

---

## Implementation Estimate
- **Effort**: 15-30 minutes
- **Lines Changed**: 2-10 lines
- **Testing Time**: 5-10 minutes with valid API key
- **Risk Level**: Low
