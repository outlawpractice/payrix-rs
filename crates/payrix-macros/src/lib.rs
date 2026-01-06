//! Procedural macros for Payrix SDK entity types.
//!
//! This crate provides the `PayrixEntity` derive macro which generates
//! Create and Update request types from a single entity definition.

use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Ident, Type};

/// Field-level attributes for the PayrixEntity derive macro.
#[derive(Debug, FromField)]
#[darling(attributes(payrix))]
struct PayrixField {
    ident: Option<Ident>,
    ty: Type,

    /// Field is read-only (id, created, modified, creator, modifier).
    /// Excluded from both Create and Update types.
    #[darling(default)]
    readonly: bool,

    /// Field is only settable at creation time (merchant, customer, forlogin).
    /// Included in Create type, excluded from Update type.
    #[darling(default)]
    create_only: bool,

    /// Field is mutable after creation (name, description, inactive).
    /// Included in both Create and Update types.
    #[darling(default)]
    mutable: bool,

    /// Field is required in create type (not wrapped in Option).
    /// Use this for fields that must be provided when creating a resource.
    #[darling(default)]
    create_required: bool,

    /// Override the type for the create request.
    /// Use this when the input type differs from the response type.
    /// Example: `#[payrix(create_only, create_type = "PaymentInfo")]`
    #[darling(default)]
    create_type: Option<String>,
}

/// Struct-level attributes for the PayrixEntity derive macro.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(payrix), supports(struct_named))]
struct PayrixEntityArgs {
    ident: Ident,
    data: Data<(), PayrixField>,

    /// Name for the generated Create type (e.g., CreateAlert).
    #[darling(default)]
    create: Option<Ident>,

    /// Name for the generated Update type (e.g., UpdateAlert).
    #[darling(default)]
    update: Option<Ident>,
}

/// Extract the serde rename attribute value from field attributes.
fn get_serde_rename(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            if let Ok(nested) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            ) {
                for meta in nested {
                    if let syn::Meta::NameValue(nv) = meta {
                        if nv.path.is_ident("rename") {
                            if let syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(s),
                                ..
                            }) = nv.value
                            {
                                return Some(s.value());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Check if a type is Option<T>.
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Check if a type is Vec<T>.
fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// Check if a type is bool.
fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "bool";
        }
    }
    false
}

/// Transform a type to Option<T> for request types.
/// - Option<T> stays as Option<T>
/// - bool becomes Option<bool>
/// - T becomes Option<T>
fn wrap_in_option(ty: &Type) -> TokenStream2 {
    if is_option_type(ty) {
        quote! { #ty }
    } else if is_bool_type(ty) {
        quote! { Option<bool> }
    } else {
        quote! { Option<#ty> }
    }
}

/// Information about a field to include in a request type.
struct RequestField {
    name: Ident,
    ty: Type,
    rename: Option<String>,
    /// If true, the field is required (not wrapped in Option).
    required: bool,
    /// Override type for create requests (parsed from string).
    override_type: Option<String>,
}

/// Generate a request type (Create or Update) with the specified fields.
fn generate_request_type(
    type_name: &Ident,
    fields: &[RequestField],
    is_create: bool,
    source_name: &Ident,
) -> TokenStream2 {
    let field_defs: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let rename_attr = field.rename.as_ref().map(|r| {
                quote! { #[serde(rename = #r)] }
            });
            let field_doc = format!("See [`{}`] for field documentation.", source_name);

            // Determine the field type
            let (field_ty, skip_attr) = if field.required {
                // Required field: use override type if specified, otherwise original type
                if let Some(ref override_type) = field.override_type {
                    let ty: Type = syn::parse_str(override_type)
                        .expect("Invalid create_type value");
                    (quote! { #ty }, quote! {})
                } else {
                    // For required fields, extract inner type if it's Option<T>
                    let ty = &field.ty;
                    if is_option_type(ty) {
                        // Extract inner type from Option<T>
                        if let Type::Path(type_path) = ty {
                            if let Some(segment) = type_path.path.segments.last() {
                                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                                        return quote! {
                                            #[doc = #field_doc]
                                            #rename_attr
                                            pub #name: #inner
                                        };
                                    }
                                }
                            }
                        }
                    }
                    (quote! { #ty }, quote! {})
                }
            } else {
                // Optional field: use override type wrapped in Option, or wrap original
                if let Some(ref override_type) = field.override_type {
                    let ty: Type = syn::parse_str(override_type)
                        .expect("Invalid create_type value");
                    (quote! { Option<#ty> }, quote! { #[serde(skip_serializing_if = "Option::is_none")] })
                } else {
                    let wrapped_ty = wrap_in_option(&field.ty);
                    (wrapped_ty, quote! { #[serde(skip_serializing_if = "Option::is_none")] })
                }
            };

            quote! {
                #[doc = #field_doc]
                #rename_attr
                #skip_attr
                pub #name: #field_ty
            }
        })
        .collect();

    let type_doc = if is_create {
        format!("Request body for creating a new [`{}`].", source_name)
    } else {
        format!("Request body for updating an existing [`{}`].", source_name)
    };

    // Don't derive Default if there are required fields
    let has_required = fields.iter().any(|f| f.required);
    let derives = if has_required {
        quote! { #[derive(Debug, Clone, serde::Serialize)] }
    } else {
        quote! { #[derive(Debug, Clone, Default, serde::Serialize)] }
    };

    quote! {
        #[doc = #type_doc]
        #derives
        #[serde(rename_all = "camelCase")]
        pub struct #type_name {
            #(#field_defs),*
        }
    }
}

/// Derive macro for generating Create and Update types from a Payrix entity.
///
/// # Attributes
///
/// ## Struct-level
/// - `#[payrix(create = CreateTypeName)]` - Name for the Create type
/// - `#[payrix(update = UpdateTypeName)]` - Name for the Update type
///
/// ## Field-level
/// - `#[payrix(readonly)]` - Field is read-only, excluded from request types
/// - `#[payrix(create_only)]` - Field only in Create type (e.g., merchant, customer)
/// - `#[payrix(mutable)]` - Field in both Create and Update types
/// - `#[payrix(create_required)]` - Field is required in Create type (not wrapped in Option)
/// - `#[payrix(create_type = "SomeType")]` - Use a different type for Create requests
///
/// Fields without any payrix attribute are excluded from request types.
///
/// # Example
///
/// ```ignore
/// #[derive(PayrixEntity)]
/// #[payrix(create = CreateAlert, update = UpdateAlert)]
/// pub struct Alert {
///     #[payrix(readonly)]
///     pub id: PayrixId,
///
///     #[payrix(readonly)]
///     pub created: Option<String>,
///
///     #[payrix(create_only)]
///     pub forlogin: Option<PayrixId>,
///
///     #[payrix(mutable)]
///     pub name: Option<String>,
/// }
///
/// // For types with required create fields:
/// #[derive(PayrixEntity)]
/// #[payrix(create = CreateToken)]
/// pub struct Token {
///     #[payrix(readonly)]
///     pub id: PayrixId,
///
///     // Required for creation, uses a different input type
///     #[payrix(create_only, create_required, create_type = "PaymentInfo")]
///     pub payment: Option<PaymentMethod>,
///
///     // Required for creation
///     #[payrix(create_only, create_required)]
///     pub customer: Option<PayrixId>,
/// }
/// ```
#[proc_macro_derive(PayrixEntity, attributes(payrix))]
pub fn derive_payrix_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get original attributes to check for serde renames on fields
    let original_fields: Vec<_> = if let syn::Data::Struct(data) = &input.data {
        if let syn::Fields::Named(fields) = &data.fields {
            fields.named.iter().collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let args = match PayrixEntityArgs::from_derive_input(&input) {
        Ok(args) => args,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let struct_name = &args.ident;

    // Default type names if not specified
    let create_name = args
        .create
        .unwrap_or_else(|| format_ident!("Create{}", struct_name));
    let update_name = args
        .update
        .unwrap_or_else(|| format_ident!("Update{}", struct_name));

    // Collect fields for each request type
    let mut create_fields: Vec<RequestField> = Vec::new();
    let mut update_fields: Vec<RequestField> = Vec::new();

    let fields = match args.data {
        Data::Struct(ref fields) => fields,
        _ => panic!("PayrixEntity only supports structs"),
    };

    for (idx, field) in fields.iter().enumerate() {
        let field_name = match &field.ident {
            Some(name) => name.clone(),
            None => continue,
        };

        // Skip Vec<T> fields (nested relations)
        if is_vec_type(&field.ty) {
            continue;
        }

        // Get serde rename from original field
        let serde_rename = original_fields
            .get(idx)
            .and_then(|f| get_serde_rename(&f.attrs));

        // Determine which types this field should be included in
        if field.readonly {
            // Readonly fields are excluded from both types
            continue;
        } else if field.create_only {
            // Create-only fields go in Create type only
            create_fields.push(RequestField {
                name: field_name,
                ty: field.ty.clone(),
                rename: serde_rename,
                required: field.create_required,
                override_type: field.create_type.clone(),
            });
        } else if field.mutable {
            // Mutable fields go in both types
            create_fields.push(RequestField {
                name: field_name.clone(),
                ty: field.ty.clone(),
                rename: serde_rename.clone(),
                required: field.create_required,
                override_type: field.create_type.clone(),
            });
            // Update fields don't use create_required or create_type
            update_fields.push(RequestField {
                name: field_name,
                ty: field.ty.clone(),
                rename: serde_rename,
                required: false,
                override_type: None,
            });
        }
        // Fields without any payrix attribute are excluded
    }

    // Generate the types
    let create_type = generate_request_type(&create_name, &create_fields, true, struct_name);
    let update_type = generate_request_type(&update_name, &update_fields, false, struct_name);

    let expanded = quote! {
        #create_type

        #update_type
    };

    TokenStream::from(expanded)
}
