#![allow(unused_assignments)]
extern crate proc_macro;

use std::collections::HashMap;

use syn::{parse_macro_input, AttributeArgs, ItemFn};

use crate::proc_macro::TokenStream;
use crate::macros::crud_table_impl::{impl_crud_driver, impl_crud};
use crate::macros::sql_impl::impl_macro_sql;
use crate::macros::py_sql_impl::{impl_macro_py_sql, impl_macro_html_sql};

mod macros;
mod util;

#[proc_macro_derive(CRUDTable)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let stream = impl_crud_driver(&ast, "", "", &HashMap::new());
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen impl CRUDTable:\n {}", stream);
        println!("............gen impl CRUDTable end............");
    }

    stream
}

/// auto create sql macro,this macro use RB.fetch_prepare and RB.exec_prepare
/// for example:
///     #[sql(RB, "select * from biz_activity where id = ?")]
///     async fn select(name: &str) -> BizActivity {}
#[proc_macro_attribute]
pub fn sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro sql:\n {}", stream);
        println!("............gen macro sql end............");
    }

    stream
}

/// py sql create macro,this macro use RB.py_fetch and RB.py_exec
/// for example:
///
///#[rb_py("
///     SELECT * FROM biz_activity
///     if  name != null:
///       AND delete_flag = #{del}
///       AND version = 1
///       if  age!=1:
///         AND version = 1
///       AND version = 1
///     AND a = 0
///       AND version = 1
///     and id in (
///     trim ',': for item in ids:
///       #{item},
///     )
///     and id in (
///     trim ',': for index,item in ids:
///       #{item},
///     )
///     trim 'AND':
///       AND delete_flag = #{del2}
///     choose:
///         when age==27:
///           AND age = 27
///         otherwise:
///           AND age = 0
///     WHERE id  = '2'")]
/// pub async fn py_select_rb(rbatis: &Rbatis, name: &str) -> Option<BizActivity> {}
#[proc_macro_attribute]
pub fn py_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_py_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen macro py_sql :\n {}", stream);
        println!("............gen macro py_sql end............");
    }
    stream
}

/// html sql create macro,this macro use RB.py_fetch and RB.py_exec
/// for example:
///
///  also,you can use arg  context_id:&str,RB:&Rbatis
/// #[py_sql(RB,"example/example.html")]
/// pub async fn py_select_rb(rbatis: &Rbatis, name: &str) -> Option<BizActivity> {}
#[proc_macro_attribute]
pub fn html_sql(args: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = impl_macro_html_sql(&target_fn, &args);
    #[cfg(feature = "debug_mode")]
        {
            println!("............gen macro html_sql :\n {}", stream);
            println!("............gen macro html_sql end............");
        }
    stream
}


/// CRUD table,You can define functionality using the following properties
/// #[crud_table]
/// #[crud_table(table_name:"biz_activity")]
/// #[crud_table(table_name:"biz_activity" | table_columns:"id,name,version,delete_flag" | formats_pg:"id:{}::uuid,name:{}::string")]
/// pub struct BizActivity {
///   pub id: Option<String>,
///   pub name: Option<String>,
///   pub version: Option<i32>,
///   pub delete_flag: Option<i32>,
/// }
#[proc_macro_attribute]
pub fn crud_table(args: TokenStream, input: TokenStream) -> TokenStream {
    let stream = impl_crud(args, input);
    #[cfg(feature = "debug_mode")]
    {
        println!("............gen impl CRUDTable:\n {}", stream);
        println!("............gen impl CRUDTable end............");
    }

    return stream;
}
