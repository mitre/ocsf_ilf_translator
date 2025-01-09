/**
 * Copyright 2025 The MITRE Corporation

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, DeriveInput, FieldsNamed};

/// This macro is used to generate a dynamic dispatch parsing function from a tagged enum. This lets you describe a
/// resulting data type, ID for parsing choice, parser, and internal data representation in one Enum definition.
/// Example:
/// ```
/// // This input code:
/// #[derive(PartialEq, Eq, Debug, ParseFromUid)]
/// #[Parser(self::my_parser)]
/// enum MyEnum {
///     #[Uid(1)]
///     TypeA(A),
///     #[Uid(2)]
///     TypeB(B),
/// }
/// // Generates the following code:
/// impl MyEnum {
///     pub fn parse_from_id(
///         id: usize,
///         data: &str,
///     ) -> Result<MyEnum, Box<dyn std::error::Error>> {
///         match id {
///             1 => self::my_parser::<A>(data).map(|a| MyEnum::TypeA(a)),
///             2 => self::my_parser::<B>(data).map(|a| MyEnum::TypeB(a)),
///             _ => Err(String::from("Not a valid ID").into()),
///         }
///     }
/// }
/// ```

#[proc_macro_derive(ParseFromUid, attributes(Parser, Uid))]
pub fn parse_from_uid_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Could not parse input");
    impl_parse_from_uid(&ast)
}

fn impl_parse_from_uid(ast: &syn::DeriveInput) -> TokenStream {
    // Name of the enum the ParseFromUid Tag is on
    let name = &ast.ident;
    let syn::Data::Enum(data) = &ast.data else {
        panic!("ParseFromUid can only be run on an Enum")
    };

    // Locate the parser attribute, extract the parse method.
    let Some(parser_attr) = ast.attrs.iter().find(|a| a.path.is_ident("Parser")) else {
        panic!("Parser Attribute required")
    };
    let parse_method = parser_attr
        // TODO: could I use a more specific parser?
        .parse_args_with(syn::Expr::parse)
        .expect("Expression");

    // Find all variants marked with a tag
    let mut valid_variants: Vec<(syn::LitInt, syn::Ident, syn::Type)> = vec![];
    for variant in &data.variants {
        let Some(uid_attr) = variant.attrs.iter().find(|a| a.path.is_ident("Uid")) else {
            panic!("All variants must be marked with a tag")
        };

        // Get the uid from the helper attribute
        let uid = uid_attr
            .parse_args_with(syn::LitInt::parse)
            .expect("Exactly one integer");

        // Make sure the enum variant has exactly 1 field, and that it's unnamed
        let syn::Fields::Unnamed(syn::FieldsUnnamed {
            unnamed: fields, ..
        }) = &variant.fields
        else {
            panic!("Uid can only be defined on enum variants with exactly 1 unnamed field")
        };
        if fields.len() != 1 {
            panic!("Uid can only be defined on enum variants with exactly 1 unnamed field")
        }
        let field = fields
            .first()
            .expect("Enum Variant should have exactly 1 unnamed field");

        // Keep track of the number, name, and unnamed field type of each variant
        valid_variants.push((uid, variant.ident.clone(), field.ty.clone()))
    }

    if valid_variants.is_empty() {
        panic!("No valid attributes are annotated with #[try_into_inner]")
    }

    // Generate the interior of the match statement
    let mut generated_code = vec![];
    for (uid, variant_name, wrapped_type) in valid_variants.into_iter() {
        let generated_match_arm =
            quote! {#uid => #parse_method::<#wrapped_type>(data).map(|a| #name::#variant_name(a)),};
        generated_code.push(generated_match_arm);
    }

    // Return the final generated function, as a member of the main Enum
    quote! {
        impl #name {
            pub fn parse_from_id(id: usize, data: &str) -> Result<#name, Box<dyn std::error::Error>> {
                match id {
                    #(#generated_code)*
                    _ => Err(String::from("Not a valid ID").into())
                }
            }
        }
    }
    .into()
}

/// This macro is used to generate an IntoILFAttributes trait for a given struct, which flattens all of it's values into
///  a Vec of ILF-compliant Values and their "Path" through the object as a string.
/// Example:
/// ```
/// // This input code:
///
/// #[derive(SerializeILF)]
/// struct MyStruct {
///     a_field: i32,
///     optional_field: Option<String>,
///     vec_field: Vec<bool>,
///     nested_field: MyOtherStruct,
/// }
/// #[derive(SerializeILF)]
/// struct MyOtherStruct {
///     float: f64,
/// }
///
/// // Generates the following code:
/// struct MyStruct {
///     a_field: i32,
///     optional_field: Option<String>,
///     vec_field: Vec<bool>,
///     nested_field: MyOtherStruct,
/// }
/// impl crate::struct_fields::IntoILFAttributes for MyStruct {
///     fn into_ilf_attributes<'b>(
///         &self,
///         path: String,
///         store: &'b mut Vec<(String, seal_lib::Value)>,
///     ) {
///         let dotted_path = if path.is_empty() {
///             path.clone()
///         } else {
///             path.clone() + "."
///         };
///         self.a_field.into_ilf_attributes(dotted_path.clone() + "a_field", store);
///         self.optional_field
///             .into_ilf_attributes(dotted_path.clone() + "optional_field", store);
///         self.vec_field.into_ilf_attributes(dotted_path.clone() + "vec_field", store);
///         self.nested_field
///             .into_ilf_attributes(dotted_path.clone() + "nested_field", store);
///     }
/// }
/// struct MyOtherStruct {
///     float: f64,
/// }
/// impl crate::struct_fields::IntoILFAttributes for MyOtherStruct {
///     fn into_ilf_attributes<'b>(
///         &self,
///         path: String,
///         store: &'b mut Vec<(String, seal_lib::Value)>,
///     ) {
///         let dotted_path = if path.is_empty() {
///             path.clone()
///         } else {
///             path.clone() + "."
///         };
///         self.float.into_ilf_attributes(dotted_path.clone() + "float", store);
///     }
/// }
/// ```
#[proc_macro_derive(SerializeILF, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    // Get name, impl details for struct
    let name = input.ident;
    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    // Get all named struct fields
    let struct_fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(FieldsNamed { named, .. }, ..),
        ..
    }) = input.data
    {
        // Convert field name and type to strings and store them
        named
            .into_iter()
            .map(|field| {
                // "unwrap" should be safe, we've already confirmed this is a set of named fields
                field.ident.unwrap().clone()
            })
            .collect::<Vec<_>>()
    } else {
        panic!("Derive(SerializeILF) only applicable to named structs");
    };

    // Get the field names as strings so we can add them to the "path" variable as we recurse down
    let fields_as_str = struct_fields
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>();

    let a: TokenStream = TokenStream::from(quote! {
        impl #impl_generics crate::struct_fields::IntoILFAttributes for #name #type_generics #where_clause {
            fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, seal_lib::Value)>) {
                let dotted_path = if path.is_empty() {path.clone()} else {path.clone() + "."};
                #(self.#struct_fields.into_ilf_attributes(dotted_path.clone() + #fields_as_str, store);)*
            }
        }
    }).into();

    a
}
