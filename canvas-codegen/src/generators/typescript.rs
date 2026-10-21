//! TypeScriptGenerator: typescript output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;
use crate::utils::helpers::attribution;

/// TypeScript types generator
pub struct TypeScriptGenerator;

impl CodeGenerator for TypeScriptGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        _components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        let types = r#"/**
 * TypeScript type definitions for a canvas layout
 * Generated from a canvas component layout
 */

// Component ID type
export type ComponentId = string;

// Button variants
export type ButtonVariant = 'Primary' | 'Secondary' | 'Outline' | 'Ghost';

// Button sizes
export type ButtonSize = 'Small' | 'Medium' | 'Large';

// Text styles
export type TextStyle = 'Heading1' | 'Heading2' | 'Heading3' | 'Body' | 'Caption';

// HTML tags for text
export type TextTag = 'H1' | 'H2' | 'H3' | 'P' | 'Span';

// Input types
export type InputType = 'Text' | 'Password' | 'Email' | 'Number' | 'Tel';

// Flex direction
export type FlexDirection = 'Row' | 'Column';

// Layout types
export type LayoutType = 
  | { Flex: { direction: FlexDirection; wrap: boolean } }
  | { Grid: { columns: number; rows: number } }
  | 'Stack';

// Spacing
export interface Spacing {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

// Visual style settings a component can carry. Mirrors the frontend's
// ComponentStyle; every field is optional.
export interface ComponentStyle {
  padding?: string;
  margin?: string;
  width?: string;
  height?: string;
  color?: string;
  background_color?: string;
  border_color?: string;
  border_width?: number;
  border_radius?: number;
  font_size?: number;
  font_weight?: string;
  text_align?: string;
  display?: string;
  flex_direction?: string;
  gap?: string;
  custom_css?: string;
}

// Animation presets
export type AnimationType =
  | 'None'
  | 'FadeIn'
  | 'SlideInUp'
  | 'SlideInDown'
  | 'SlideInLeft'
  | 'SlideInRight'
  | 'Bounce'
  | 'ZoomIn'
  | 'Pulse';

export interface Animation {
  animation_type: AnimationType;
  duration: number;
  delay: number;
  infinite: boolean;
}

// Button component
export interface ButtonComponent {
  id: ComponentId;
  label: string;
  variant: ButtonVariant;
  size: ButtonSize;
  disabled: boolean;
  on_click?: string | null;
}

// Text component
export interface TextComponent {
  id: ComponentId;
  content: string;
  style: TextStyle;
  tag: TextTag;
}

// Input component
export interface InputComponent {
  id: ComponentId;
  placeholder: string;
  input_type: InputType;
  required: boolean;
  disabled: boolean;
}

// Select component
export interface SelectComponent {
  id: ComponentId;
  options: string;
  placeholder: string;
  disabled: boolean;
}

// Container component
export interface ContainerComponent {
  id: ComponentId;
  children: CanvasComponent[];
  layout: LayoutType;
  gap: number;
  padding: Spacing;
}

// Custom component
export interface CustomComponent {
  id: ComponentId;
  name: string;
  template: string;
  props: Record<string, PropValue>;
}

// Property value types
export type PropValue = 
  | { String: string }
  | { Number: number }
  | { Boolean: boolean }
  | 'Null';

// Image component
export interface ImageComponent {
  id: ComponentId;
  src: string;
  alt: string;
  width?: string | null;
  height?: string | null;
}

// Card component
export interface CardComponent {
  id: ComponentId;
  children: CanvasComponent[];
  padding: number;
  shadow: boolean;
  border: boolean;
  border_radius: number;
}

// Link component
export interface LinkComponent {
  id: ComponentId;
  text: string;
  href: string;
  style?: ComponentStyle;
  animation?: Animation | null;
  bindings?: Record<string, string>;
}

// Canvas component union type
export type CanvasComponent = 
  | { Button: ButtonComponent }
  | { Text: TextComponent }
  | { Input: InputComponent }
  | { Container: ContainerComponent }
  | { Image: ImageComponent }
  | { Card: CardComponent }
  | { Select: SelectComponent }
  | { Custom: CustomComponent }
  | { Link: LinkComponent };

// Layout type (array of components)
export type Layout = CanvasComponent[];

// Project export type
export interface Project {
  name: string;
  description?: string;
  layout: Layout;
  theme: string;
  created_at: string;
  updated_at: string;
}

// Helper type guards
export function isButton(c: CanvasComponent): c is { Button: ButtonComponent } {
  return 'Button' in c;
}

export function isText(c: CanvasComponent): c is { Text: TextComponent } {
  return 'Text' in c;
}

export function isInput(c: CanvasComponent): c is { Input: InputComponent } {
  return 'Input' in c;
}

export function isContainer(c: CanvasComponent): c is { Container: ContainerComponent } {
  return 'Container' in c;
}

export function isImage(c: CanvasComponent): c is { Image: ImageComponent } {
  return 'Image' in c;
}

export function isCard(c: CanvasComponent): c is { Card: CardComponent } {
  return 'Card' in c;
}

export function isSelect(c: CanvasComponent): c is { Select: SelectComponent } {
  return 'Select' in c;
}

export function isCustom(c: CanvasComponent): c is { Custom: CustomComponent } {
  return 'Custom' in c;
}
"#;

        Ok(attribution(config, "// ", "") + types)
    }

    fn file_extension(&self) -> &str {
        "d.ts"
    }
}
