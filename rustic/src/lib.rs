use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(HelloWorld)] 
pub fn hello_world_derive(input: TokenStream) -> TokenStream { 
    let input = parse_macro_input!(input as DeriveInput); 
    let name = input.ident; 
    
    let expanded = quote! { 
        impl HelloWorld for #name { 
            fn hello_world() -> String { 
                format!("Hello World! from {}", stringify!(#name)) 
            } 
        } 
    }; 
    expanded.into() 
}

#[proc_macro_derive(SqlModel)]
pub fn sql_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let table_name = name.to_string().to_lowercase();

    let fields = match input.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields_named) => fields_named.named.iter().collect::<Vec<_>>(),
            _ => panic!("SqlModel only supports named fields."),
        },
        _ => panic!("SqlModel can only be derived for structs."),
    };

    let create_columns: Vec<_> = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap().to_string();
        let sql_type = match &f.ty {
            syn::Type::Path(type_path) => {
                let type_ident = type_path.path.segments.last().unwrap().ident.to_string();
                match type_ident.as_str() {
                    "String" => "TEXT",
                    "str" => "TEXT",
                    "i8" | "i16" | "i32" | "i64" | "isize" => "INTEGER",
                    "u8" | "u16" | "u32" | "u64" | "usize" => "INTEGER",
                    "f32" | "f64" => "REAL",
                    "bool" => "BOOLEAN",
                    _ => "TEXT"
                }
            },
            _ => "TEXT"
        };
        quote! {
            format!("{} {}", #ident, #sql_type)
        }
    }).collect();

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_idents: Vec<_> = field_names.clone();
    let field_names_str: Vec<_> = field_names.iter().map(|f| f.to_string()).collect();

    let gen = quote! {
        impl #name {
            pub fn create_table_sql() -> String {
                format!(
                    "CREATE TABLE IF NOT EXISTS {} ({});",
                    #table_name,
                    vec![#(#create_columns),*].join(", ")
                )
            }

            pub fn insert_sql(&self) -> String {
                let values = vec![
                    #(
                        format!("'{}'", &self.#field_idents)
                    ),*
                ].join(", ");
            
                format!(
                    "INSERT INTO {} ({}) VALUES ({});",
                    #table_name,
                    vec![#(#field_names_str),*].join(", "),
                    values
                )
            }

            pub fn select_all_sql() -> String {
                format!("SELECT * FROM {};", #table_name)
            }

            pub fn select_by_id_sql(id_field: &str, id_value: &str) -> String {
                format!("SELECT * FROM {} WHERE {} = '{}';", #table_name, id_field, id_value)
            }

            pub fn update_sql(&self, id_field: &str, id_value: &str) -> String {
                let updates = vec![
                    #(
                        format!("{} = '{}'", #field_names_str, &self.#field_idents)
                    ),*
                ].join(", ");
            
                format!(
                    "UPDATE {} SET {} WHERE {} = '{}';",
                    #table_name,
                    updates,
                    id_field,
                    id_value
                )
            }

            pub fn delete_sql(id_field: &str, id_value: &str) -> String {
                format!("DELETE FROM {} WHERE {} = '{}';", #table_name, id_field, id_value)
            }
        }
    };

    gen.into()
}