//! Providers module.
//!
//! Contains implementations of LLM providers and related utilities.

pub mod llm_provider;
pub mod gemini_adapter;

/// Model name for Gemini 3.0 Flash.
pub const GEMINI_FLASH_MODEL: &str = "gemini-3-flash-preview";
/// Model name for Gemini 3.1 Pro.
pub const GEMINI_PRO_MODEL: &str = "gemini-3.1-pro-preview";
