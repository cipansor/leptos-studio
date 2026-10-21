//! JsonSchemaGenerator: json_schema output.

use crate::config::GeneratorConfig;
use crate::domain::{AppError, AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;

/// JSON Schema generator for component validation
pub struct JsonSchemaGenerator;

impl CodeGenerator for JsonSchemaGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        let mut schema = serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "Canvas Layout",
            "description": "JSON Schema for a canvas component layout",
            "type": "array",
            "items": {
                "$ref": "#/definitions/CanvasComponent"
            },
            "definitions": {
                "CanvasComponent": {
                    "oneOf": [
                        { "$ref": "#/definitions/ButtonComponent" },
                        { "$ref": "#/definitions/TextComponent" },
                        { "$ref": "#/definitions/InputComponent" },
                        { "$ref": "#/definitions/ContainerComponent" },
                        { "$ref": "#/definitions/ImageComponent" },
                        { "$ref": "#/definitions/CardComponent" },
                        { "$ref": "#/definitions/SelectComponent" },
                        { "$ref": "#/definitions/CustomComponent" },
                        { "$ref": "#/definitions/LinkComponent" }
                    ]
                },
                "LinkComponent": {
                    "type": "object",
                    "required": ["Link"],
                    "properties": {
                        "Link": {
                            "type": "object",
                            "required": ["id", "text", "href"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "text": { "type": "string" },
                                "href": { "type": "string" },
                                "style": { "type": "object" },
                                "animation": { "type": ["object", "null"] },
                                "bindings": { "type": "object" }
                            }
                        }
                    }
                },
                "SelectComponent": {
                    "type": "object",
                    "required": ["Select"],
                    "properties": {
                        "Select": {
                            "type": "object",
                            "required": ["id", "options", "placeholder", "disabled"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "options": { "type": "string" },
                                "placeholder": { "type": "string" },
                                "disabled": { "type": "boolean" }
                            }
                        }
                    }
                },
                "ButtonComponent": {
                    "type": "object",
                    "required": ["Button"],
                    "properties": {
                        "Button": {
                            "type": "object",
                            "required": ["id", "label", "variant", "size", "disabled"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "label": { "type": "string" },
                                "variant": {
                                    "type": "string",
                                    "enum": ["Primary", "Secondary", "Outline", "Ghost"]
                                },
                                "size": {
                                    "type": "string",
                                    "enum": ["Small", "Medium", "Large"]
                                },
                                "disabled": { "type": "boolean" },
                                "on_click": { "type": ["string", "null"] }
                            }
                        }
                    }
                },
                "TextComponent": {
                    "type": "object",
                    "required": ["Text"],
                    "properties": {
                        "Text": {
                            "type": "object",
                            "required": ["id", "content", "style", "tag"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "content": { "type": "string" },
                                "style": {
                                    "type": "string",
                                    "enum": ["Heading1", "Heading2", "Heading3", "Body", "Caption"]
                                },
                                "tag": {
                                    "type": "string",
                                    "enum": ["H1", "H2", "H3", "P", "Span"]
                                }
                            }
                        }
                    }
                },
                "InputComponent": {
                    "type": "object",
                    "required": ["Input"],
                    "properties": {
                        "Input": {
                            "type": "object",
                            "required": ["id", "placeholder", "input_type", "required", "disabled"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "placeholder": { "type": "string" },
                                "input_type": {
                                    "type": "string",
                                    "enum": ["Text", "Password", "Email", "Number", "Tel"]
                                },
                                "required": { "type": "boolean" },
                                "disabled": { "type": "boolean" }
                            }
                        }
                    }
                },
                "ContainerComponent": {
                    "type": "object",
                    "required": ["Container"],
                    "properties": {
                        "Container": {
                            "type": "object",
                            "required": ["id", "children", "layout", "gap", "padding"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "children": {
                                    "type": "array",
                                    "items": { "$ref": "#/definitions/CanvasComponent" }
                                },
                                "layout": { "$ref": "#/definitions/LayoutType" },
                                "gap": { "type": "integer", "minimum": 0 },
                                "padding": { "$ref": "#/definitions/Spacing" }
                            }
                        }
                    }
                },
                "ImageComponent": {
                    "type": "object",
                    "required": ["Image"],
                    "properties": {
                        "Image": {
                            "type": "object",
                            "required": ["id", "src", "alt"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "src": { "type": "string" },
                                "alt": { "type": "string" },
                                "width": { "type": ["string", "null"] },
                                "height": { "type": ["string", "null"] }
                            }
                        }
                    }
                },
                "CardComponent": {
                    "type": "object",
                    "required": ["Card"],
                    "properties": {
                        "Card": {
                            "type": "object",
                            "required": ["id", "children", "padding", "shadow", "border", "border_radius"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "children": {
                                    "type": "array",
                                    "items": { "$ref": "#/definitions/CanvasComponent" }
                                },
                                "padding": { "type": "integer", "minimum": 0 },
                                "shadow": { "type": "boolean" },
                                "border": { "type": "boolean" },
                                "border_radius": { "type": "integer", "minimum": 0 }
                            }
                        }
                    }
                },
                "CustomComponent": {
                    "type": "object",
                    "required": ["Custom"],
                    "properties": {
                        "Custom": {
                            "type": "object",
                            "required": ["id", "name", "template", "props"],
                            "properties": {
                                "id": { "type": "string", "format": "uuid" },
                                "name": { "type": "string" },
                                "template": { "type": "string" },
                                "props": { "type": "object" }
                            }
                        }
                    }
                },
                "LayoutType": {
                    "oneOf": [
                        {
                            "type": "object",
                            "properties": {
                                "Flex": {
                                    "type": "object",
                                    "properties": {
                                        "direction": { "enum": ["Row", "Column"] },
                                        "wrap": { "type": "boolean" }
                                    }
                                }
                            }
                        },
                        {
                            "type": "object",
                            "properties": {
                                "Grid": {
                                    "type": "object",
                                    "properties": {
                                        "columns": { "type": "integer" },
                                        "rows": { "type": "integer" }
                                    }
                                }
                            }
                        },
                        { "const": "Stack" }
                    ]
                },
                "Spacing": {
                    "type": "object",
                    "required": ["top", "right", "bottom", "left"],
                    "properties": {
                        "top": { "type": "integer", "minimum": 0 },
                        "right": { "type": "integer", "minimum": 0 },
                        "bottom": { "type": "integer", "minimum": 0 },
                        "left": { "type": "integer", "minimum": 0 }
                    }
                }
            },
            "examples": [serde_json::to_value(components).unwrap_or_default()]
        });

        // JSON has no comments, so an attribution cannot be written as text. JSON
        // Schema (draft-07) reserves `$comment` for exactly this, and validators
        // ignore it, so the note survives without invalidating the document.
        if let Some(text) = &config.attribution {
            schema["$comment"] = serde_json::Value::String(text.clone());
        }

        serde_json::to_string_pretty(&schema)
            .map_err(|e| AppError::Export(format!("Failed to generate JSON Schema: {}", e)))
    }

    fn file_extension(&self) -> &str {
        "schema.json"
    }
}
