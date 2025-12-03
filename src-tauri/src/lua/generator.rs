// Lua generator - generates WezTerm Lua config files using Tera templates

use std::path::Path;
use tera::{Context, Tera};

/// Error type for Lua generation
#[derive(Debug)]
pub struct LuaGenError {
    pub message: String,
}

impl std::fmt::Display for LuaGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lua generation error: {}", self.message)
    }
}

impl std::error::Error for LuaGenError {}

impl From<tera::Error> for LuaGenError {
    fn from(err: tera::Error) -> Self {
        LuaGenError {
            message: err.to_string(),
        }
    }
}

/// Lua generator that uses Tera templates
pub struct LuaGenerator {
    tera: Tera,
}

impl LuaGenerator {
    /// Create a new LuaGenerator with templates from the given directory
    pub fn new(templates_dir: &Path) -> Result<Self, LuaGenError> {
        let pattern = templates_dir.join("**/*.lua").to_string_lossy().to_string();
        let tera = Tera::new(&pattern).map_err(|e| LuaGenError {
            message: format!("Failed to load templates: {}", e),
        })?;
        
        Ok(Self { tera })
    }

    /// Create a new LuaGenerator with templates embedded at compile time
    /// Uses include_str! for bundled templates
    pub fn new_embedded() -> Result<Self, LuaGenError> {
        let tera = Tera::default();
        
        // Templates will be added here as they are created
        // Example:
        // tera.add_raw_template("colors_custom.lua", include_str!("../../templates/colors_custom.lua"))?;
        
        Ok(Self { tera })
    }

    /// Render a template with the given context
    pub fn render(&self, template_name: &str, context: &Context) -> Result<String, LuaGenError> {
        self.tera
            .render(template_name, context)
            .map_err(|e| e.into())
    }

    /// Add a template at runtime (useful for testing)
    pub fn add_template(&mut self, name: &str, content: &str) -> Result<(), LuaGenError> {
        self.tera
            .add_raw_template(name, content)
            .map_err(|e| e.into())
    }

    /// Get list of available templates
    pub fn get_template_names(&self) -> Vec<&str> {
        self.tera.get_template_names().collect()
    }
}

/// Create a Tera context from a serializable value
pub fn create_context<T: serde::Serialize>(value: &T) -> Result<Context, LuaGenError> {
    Context::from_serialize(value).map_err(|e| LuaGenError {
        message: format!("Failed to create context: {}", e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_generator_embedded() {
        let generator = LuaGenerator::new_embedded();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_add_and_render_template() {
        let mut generator = LuaGenerator::new_embedded().unwrap();
        
        let template = r#"-- Test config
local M = {}
M.value = "{{ value }}"
return M
"#;
        
        generator.add_template("test.lua", template).unwrap();
        
        let mut context = Context::new();
        context.insert("value", "hello");
        
        let result = generator.render("test.lua", &context).unwrap();
        assert!(result.contains("M.value = \"hello\""));
    }

    #[test]
    fn test_create_context() {
        #[derive(serde::Serialize)]
        struct TestData {
            name: String,
            count: i32,
        }

        let data = TestData {
            name: "test".to_string(),
            count: 42,
        };

        let context = create_context(&data);
        assert!(context.is_ok());
    }
}
