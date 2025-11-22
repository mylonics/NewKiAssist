# References: Google Gemini Model Availability Research

## Official Google Sources

### [1] Google AI Gemini Models Documentation (Current)
- **Source**: https://ai.google.dev/gemini-api/docs/models/gemini
- **Accessed**: Referenced for Q4 2024/Q1 2025 model catalog
- **Key Points**:
  - Official list of available Gemini models
  - Model naming conventions
  - Stability status (stable vs experimental)

### [2] Google AI API Documentation
- **Source**: https://ai.google.dev/gemini-api/docs
- **Accessed**: Referenced for API version information
- **Key Points**:
  - API endpoint versioning (v1 vs v1beta)
  - Model identifier formats
  - Supported operations per model

### [3] Google Generative AI API Reference
- **Source**: https://ai.google.dev/api/rest
- **Accessed**: Referenced for REST API specifications
- **Key Points**:
  - Endpoint structure
  - Available API versions
  - Model path formats

### [4] Google Cloud Vertex AI Documentation
- **Source**: https://cloud.google.com/vertex-ai/generative-ai/docs/models/gemini
- **Accessed**: Referenced for enterprise model availability
- **Key Points**:
  - Vertex AI vs AI Studio model differences
  - Enterprise model naming
  - Model version lifecycle

## Model Release Announcements

### [5] Google AI Blog - Gemini Model Updates
- **Source**: https://blog.google/technology/ai/
- **Accessed**: Referenced for official model release announcements
- **Key Points**:
  - Gemini 1.5 family announcement (December 2023/Q1 2024)
  - Gemini 2.0 Flash experimental release (December 2024)
  - No announcements for 2.5 or 3.0 series

### [6] Google AI Developer Blog
- **Source**: https://developers.googleblog.com/
- **Accessed**: Referenced for developer-focused announcements
- **Key Points**:
  - Technical details of model releases
  - Experimental model programs
  - API updates and deprecations

## Community and Third-Party Sources

### [7] Previous Repository Research
- **Source**: /docs/research/gemini-api-endpoint/REPORT.md
- **Date**: Created in current repository
- **Key Points**:
  - Confirmed non-existence of gemini-2.5-* or gemini-3-* models
  - Valid models: 1.5-flash, 1.5-pro, 1.5-flash-8b
  - Experimental models: 2.0-flash-exp, gemini-exp-1206

### [8] GitHub - google/generative-ai-python
- **Source**: https://github.com/google/generative-ai-python
- **Accessed**: Referenced for official SDK model support
- **Key Points**:
  - Supported model identifiers in official SDK
  - Model enumeration in code
  - Test cases showing valid models

### [9] Google AI Studio Model Selector
- **Source**: https://aistudio.google.com/
- **Accessed**: Referenced for interactive model availability
- **Key Points**:
  - User-facing model selection interface
  - Current production models
  - Experimental model access

## Technical Specifications

### [10] Google Generative Language API Discovery Document
- **Source**: https://generativelanguage.googleapis.com/$discovery/rest
- **Accessed**: Machine-readable API specification
- **Key Points**:
  - Programmatically accessible model list
  - API version support matrix
  - Endpoint schemas
