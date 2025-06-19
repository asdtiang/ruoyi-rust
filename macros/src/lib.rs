use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::Data::Struct;
use syn::{parse_macro_input, DataStruct, DeriveInput, Field, FnArg, ItemFn, LitFloat, LitInt, LitStr, Meta};

#[proc_macro_attribute]
pub fn pre_authorize(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
                                                   //  println!("{:?}",func);
    let func_vis = &func.vis; // pub
    let func_block = &func.block; //.stmts.iter().map(|r|r.to_token_stream().to_string()).collect::<Vec<_>>().join("\n"); // 函数主体实现部分{}

    let func_decl = &func.sig; // 函数申明

    let func_attrs = &func.attrs;
    let func_name = &func_decl.ident; // 函数名
    let func_asyncness = &func_decl.asyncness; // 函数名
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    // // 查看是否有HeaderMap
    // let mut header_map_ident = None;
    // func_inputs.iter().for_each(|i| {
    //     match i {
    //         // 提取形参的pattern
    //         // https://docs.rs/syn/1.0.1/syn/struct.PatType.html
    //         FnArg::Typed(ref val) => {
    //             let ty = (&val).ty.to_token_stream().to_string();
    //             if ty.ends_with("HeaderMap") {
    //                 header_map_ident = Some((&val).pat.to_token_stream());
    //             }
    //         } // pat没有办法移出val，只能借用，或者val.pat.clone()
    //         _ => unreachable!("it's not gonna happen."),
    //     }
    // });
    let permit_str = parse_macro_input!(attr as LitStr);
    let expanded = quote! { // 重新构建函数执行
        #(#func_attrs)*
        #func_vis #func_asyncness fn #func_name #func_generics(header_map_in_permit: axum::http::HeaderMap,#func_inputs) #func_output{
            match crate::token_auth::check_permit(header_map_in_permit, #permit_str).await {//fixme 判断参数中是否存在HeaderMap.302
                // ，以后再说
                None =>  #func_block
                Some(res) => { res.into_response() }
            }
        }
    };
    // } else {
    //     let header_map_ident=header_map_ident.unwrap();
    //     quote! { // 重新构建函数执行
    //         #(#func_attrs)*
    //         #func_vis #func_asyncness fn #func_name #func_generics(#func_inputs) #func_output{
    //             match crate::token_auth::check_permit(#header_map_ident, #permit_str).await {//fixme 判断参数中是否存在Request，以后再说
    //                 None =>  #func_block
    //                 Some(res) => { return res.into_response(); }
    //             }
    //         }
    //     }
    // };
    expanded.into()
}

// #[proc_macro_attribute]
// pub fn to_log(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
//     let func_vis = &func.vis; // pub
//     let func_block = &func.block; //.stmts.iter().map(|r|r.to_token_stream().to_string()).collect::<Vec<_>>().join("\n"); // 函数主体实现部分{}
//     let func_attrs = &func.attrs; //.stmts.iter().map(|r|r.to_token_stream().to_string()).collect::<Vec<_>>().join("\n"); // 函数主体实现部分{}
//
//     let func_decl = &func.sig; // 函数申明
//
//     let func_name = &func_decl.ident; // 函数名
//     let func_asyncness = &func_decl.asyncness; // 函数名
//     let func_generics = &func_decl.generics; // 函数泛型
//     let func_inputs = &func_decl.inputs; // 函数输入参数
//     let func_output = &func_decl.output; // 函数返回
//
//     // for Field {
//     //     // 该字段的标识符
//     //     ident,
//     //     // 该字段的可见性
//     //     vis,
//     //     // 分隔符 `:`
//     //     colon_token,
//     //     // 该字段的类型
//     //     ty,
//     //     ..
//     // } in func_inputs.iter(){
//     //     println!("{}",ty.to_token_stream());
//     // }
//     let s = attr.to_string();
//     let expanded = quote! { // 重新构建函数执行
//         #(#func_attrs)*
//         #func_vis #func_asyncness fn #func_name #func_generics(header_map_in_permit:axum::http::HeaderMap,#func_inputs) #func_output{
//             match crate::token_auth::check_permit(header_map_in_permit, #s).await {//fixme 判断参数中是否存在httpRequest，以后再说
//                  None =>  #func_block
//              Some(res) => { return res.resp_json(); }
//             }
//
//         }
//     };
//     expanded.into()
// }

// extern crate syn;
// #[macro_use] extern crate quote;
// extern crate proc_macro;
// extern crate proc_macro2;
//
// use syn::parse::{Parse, ParseStream};
// use proc_macro2::{Ident, Span};
//
// struct MacroInput {
//     pub field_type: syn::Type,
//     pub field_name: String,
//     pub field_count: u64
// }
// impl Parse for MacroInput {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let field_type = input.parse::<syn::Type>()?;
//         let _comma = input.parse::<syn::token::Comma>()?;
//         let field_name = input.parse::<syn::LitStr>()?;
//         let _comma = input.parse::<syn::token::Comma>()?;
//         let count = input.parse::<syn::LitInt>()?;
//         Ok(MacroInput {
//             field_type: field_type,
//             field_name: field_name.value(),
//             field_count: count.base10_parse().unwrap()
//         })
//     }
// }
//
// #[proc_macro_attribute]
// pub fn derivefields(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(attr as MacroInput);
//     let mut found_struct = false;
//     item.into_iter().map(|r| {
//         match &r {
//             &proc_macro::TokenTree::Ident(ref ident) if ident.to_string() == "struct" => {
//                 found_struct = true;
//                 r
//             },
//             &proc_macro::TokenTree::Group(ref group) if group.delimiter() == proc_macro::Delimiter::Brace && found_struct == true => {
//                 let mut stream = proc_macro::TokenStream::new();
//                 stream.extend((0..input.field_count).fold(vec![], |mut state:Vec<proc_macro::TokenStream>, i| {
//                     let field_name_str = format!("{}_{}", input.field_name, i);
//                     let field_name = Ident::new(&field_name_str, Span::call_site());
//                     let field_type = input.field_type.clone();
//                     state.push(quote!(pub #field_name: #field_type,
//                     ).into());
//                     state
//                 }).into_iter());
//                 stream.extend(group.stream());
//                 proc_macro::TokenTree::Group(
//                     proc_macro::Group::new(
//                         proc_macro::Delimiter::Brace,
//                         stream
//                     )
//                 )
//             }
//             _ => r
//         }
//     }).collect()
// }

// #[proc_macro_attribute]
// pub fn log_calls(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as syn::ItemFn);
//     let func_name = &input.sig.ident;
//     let func_block = &input.block;
//
//     let expanded = quote! {
//         fn #func_name() {
//             println!("Calling function: {}", stringify!(#func_name));
//             #func_block
//         }
//     };
//
//     TokenStream::from(expanded)
// }

//为查询DTO增加page_no和page_size，并提供impl From<&#ident> for rbatis::PageRequest
//根据参数增加params
//建议放在struct首位
#[proc_macro_attribute]
pub fn page_request(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut is_params = false;
    let mut is_data_scope = false;
    let mut no_page = false;

    //https://docs.rs/syn/latest/syn/macro.parse_macro_input.html#usage-with-parser
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("params") {
            is_params = true;
            Ok(())
        } else if meta.path.is_ident("dataScope") {
            is_data_scope = true;
            Ok(())
        } else if meta.path.is_ident("noPage") {
            no_page = true;
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });
    parse_macro_input!(attr with parser);
    let mut params = quote! {};
    if is_params {
        params = quote! {
            pub params:Option<std::collections::HashMap<String, String>>,
        };
    }

    // 将输入的 token 流解析为 `DeriveInput`
    let original_struct = parse_macro_input!(input as DeriveInput);

    // 从输入中解构出 data 和 ident 字段
    let DeriveInput {
        data,
        ident,
        attrs: atrrs_s,
        ..
    } = original_struct.clone();

    if let Struct(data_struct) = data {
        // 从这个数据结构中提取字段
        let DataStruct { fields, .. } = data_struct;
        // 创建用于作为输出的变量 new_fields
        let mut new_fields = quote!();
        for Field {
            // 该字段的标识符
            ident,
            // 该字段的属性
            attrs,
            // 该字段的可见性
            vis,
            // 分隔符 `:`
            colon_token,
            // 该字段的类型
            ty,
            ..
        } in fields
        {
            new_fields.extend(quote! {
                #(#attrs)*
                #vis #ident #colon_token #ty,
            });
        }
        let mut data_scope_impl = quote! {};
        if is_data_scope {
            data_scope_impl = quote! {
              impl crate::DataScopeTrait for #ident {
                 fn clear_data_scope_params(&mut self) {
                     let mut params =self.params.clone().unwrap_or_default();
                     params.remove("dataScope");
                     self.params=params.into();
                 }
                 fn set_data_scope_params(&mut self,value:&str) {
                     let mut params =self.params.clone().unwrap_or_default();
                     params.insert("dataScope".to_string(),value.to_string());
                     self.params=params.into();
                 }
             }
            }
        }

        let struct_page_extend = if no_page {
            quote! {}
        } else {
            quote! {
                #[serde(rename(deserialize = "pageNum"))]
                #[validate(range(min = 1))]
                pub page_no: Option<u64>,
                #[validate(range(min = 1, max = 50))]
                pub page_size: Option<u64>,
            }
        };
        let page_impl_extend = if no_page {
            quote! {}
        } else {
            quote! {
               impl From<&#ident> for rbatis::PageRequest {
                fn from(arg: &#ident) -> Self {
                rbatis::PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
                }
            }

            }
        };
        let res = quote! {
        #(#atrrs_s)*
        #[derive(validator::Validate)]
        pub struct #ident {
            #struct_page_extend
            #new_fields
            #params
        }
            #page_impl_extend
         #data_scope_impl
            };
        //println!("{}",res.to_string());
        res.into()
    } else {
        // 如果目标不是命名结构，则触发 panic 错误
        panic!("DeriveCustomModel 只能用于命名结构")
    }
}

//不打算使用，实际情况太复杂
// 根据注释情况生成DTO
#[proc_macro_attribute]
pub fn gen_dto(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut add_additional: Option<LitStr> = None;
    let mut update_additional: Option<LitStr> = None;

    //https://docs.rs/syn/latest/syn/macro.parse_macro_input.html#usage-with-parser
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("addAdditional") {
            add_additional = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("updateAdditional") {
            update_additional = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });

    parse_macro_input!(attr with parser);

    // 将输入的 token 流解析为 `DeriveInput`
    let original_struct = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        data,
        ident,
        attrs: atrrs_s,
        ..
    } = original_struct.clone();
    let struct_name = &ident;
    //重新生成源的struct
    let mut struct_expand = quote! {};
    let add_dto_name = quote::format_ident!("{}AddDTO", struct_name);
    let update_dto_name = quote::format_ident!("{}UpdateDTO", struct_name);
    let vo_name = quote::format_ident!("{}VO", struct_name);
    let mut add_dto_expand = quote! {};
    let mut update_dto_expand = quote! {};
    let mut vo_expand = quote! {};
    let mut add_from_expand = quote! {};
    let mut update_from_expand = quote! {};
    let mut vo_from_expand = quote! {};

    if let Struct(data_struct) = data {
        // 从这个数据结构中提取字段
        let DataStruct { fields, .. } = data_struct;
        // 创建用于作为输出的变量 new_fields
        // let mut new_fields = quote!();
        for Field {
            // 该字段的标识符
            ident,
            // 该字段的属性
            attrs,
            // 该字段的可见性
            vis,
            // 分隔符 `:`
            // colon_token,
            // 该字段的类型
            ty,
            ..
        } in fields
        {
            let mut validate_expand = quote! {};
            let mut serde_attr_expand = quote! {};
            let mut has_add = false;
            let mut has_update = false;
            let mut has_vo = false;
            for attr in attrs {
                if attr.path().is_ident("validate") {
                    validate_expand.extend(quote! {#attr});
                } else if attr.path().is_ident("serde") {
                    serde_attr_expand.extend(quote! {#attr});
                } else if attr.path().is_ident("dto") {
                    match attr.meta {
                        Meta::List(e) => {
                            //fixme 更改
                            let s = e.tokens.to_string();
                            println!("{s}");
                            let ss = s.split(",").map(|s| s.trim()).collect::<Vec<_>>();
                            if ss.contains(&"add") {
                                has_add = true;
                            }
                            if ss.contains(&"update") {
                                has_update = true;
                            }
                            if ss.contains(&"vo") {
                                has_vo = true;
                            }
                        }
                        _ => {}
                    }
                } else {
                    struct_expand.extend(quote! {
                        #attr
                    })
                }
            }
            struct_expand.extend(quote! {
                #vis #ident: #ty,
            });
            if has_add {
                add_dto_expand.extend(quote! {
                    #validate_expand
                    #vis #ident: #ty,
                });
                add_from_expand.extend(quote! {
                    #ident:arg.#ident,
                })
            } else {
                add_from_expand.extend(quote! {
                    #ident:None,
                })
            }
            if has_update {
                update_dto_expand.extend(quote! {
                    #validate_expand
                    #vis #ident: #ty,
                });
                update_from_expand.extend(quote! {
                    #ident:arg.#ident,
                })
            } else {
                update_from_expand.extend(quote! {
                    #ident:None,
                })
            }
            if has_vo {
                vo_expand.extend(quote! {
                    #serde_attr_expand
                    #vis #ident: #ty,
                });
                vo_from_expand.extend(quote! {
                    #ident:arg.#ident,
                })
            }
        }

        if add_additional.is_some() {
            let a = add_additional.unwrap().to_token_stream().to_string();
            let a = a[1..a.len() - 1].to_string();
            let a = a.parse::<proc_macro2::TokenStream>().unwrap();
            add_dto_expand.extend(a);
        }

        let expanded = quote! {
             #(#atrrs_s)*
             pub struct #struct_name {
                #struct_expand
            }
            #[derive(serde::Serialize, serde::Deserialize, validator::Validate, Clone, Debug)]
            #[serde(rename_all = "camelCase")]
            pub struct #add_dto_name {
                #add_dto_expand
            }
            impl From<#add_dto_name> for #struct_name {
                fn from(arg: #add_dto_name) -> Self {
                    #struct_name {
                        #add_from_expand
                    }
                }
            }
            #[derive(serde::Serialize, serde::Deserialize, validator::Validate, Clone, Debug)]
            #[serde(rename_all = "camelCase")]
            pub struct #update_dto_name {
                #update_dto_expand
            }
            impl From<#update_dto_name> for #struct_name {
                fn from(arg: #update_dto_name) -> Self {
                    #struct_name {
                        #update_from_expand
                    }
                }
            }
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #vo_name {
                #vo_expand
            }

            impl From<#struct_name> for #vo_name {
                fn from(arg: #struct_name) -> Self {
                    Self {
                        #vo_from_expand
                    }
                }
            }
        };
        println!("{}", expanded.to_string());
        //         println!("{:?}", add_fields);
        expanded.into()
    } else {
        // 如果目标不是命名结构，则触发 panic 错误
        panic!("DeriveCustomModel 只能用于命名结构")
    }
}

///同ruoyi DataScope
#[proc_macro_attribute]
pub fn data_scope(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
                                                   //  println!("{:?}",func);
    let func_vis = &func.vis; // pub

    //去年大括号
    let mut stmts_expanded = quote! {};
    func.block
        .stmts
        .iter()
        .for_each(|r| stmts_expanded.extend(r.to_token_stream()));
    let func_decl = &func.sig; // 函数申明

    let func_attrs = &func.attrs;
    let func_name = &func_decl.ident; // 函数名
    let func_asyncness = &func_decl.asyncness; // 函数名
    let func_generics = &func_decl.generics; // 函数泛型
    let func_inputs = &func_decl.inputs; // 函数输入参数
    let func_output = &func_decl.output; // 函数返回

    let mut dept_alias: Option<LitStr> = None;
    let mut user_alias: Option<LitStr> = None;

    //https://docs.rs/syn/latest/syn/macro.parse_macro_input.html#usage-with-parser
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("deptAlias") {
            dept_alias = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("userAlias") {
            user_alias = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });
    parse_macro_input!(attr with parser);
    let mut dto_ident = None;
    func_inputs.iter().for_each(|i| {
        match i {
            // 提取形参的pattern
            // https://docs.rs/syn/1.0.1/syn/struct.PatType.html
            FnArg::Typed(ref val) => {
                let ty = (&val).ty.to_token_stream().to_string();
                if ty.ends_with("DTO") {
                    dto_ident = Some((&val).pat.to_token_stream());
                }
            } // pat没有办法移出val，只能借用，或者val.pat.clone()
            _ => {}
        }
    });
    let dept_alias = dept_alias.map(|a| a.to_token_stream()).unwrap_or_default();
    let expanded = quote! { // 重新构建函数执行
        #(#func_attrs)*
        #func_vis #func_asyncness fn #func_name #func_generics(#func_inputs) #func_output{
            let mut #dto_ident = #dto_ident.clone();
            crate::system::service::data_scope::build_data_scope(&mut #dto_ident, #dept_alias, #user_alias).await?;
            #stmts_expanded

        }
    };
    //  println!("{}", expanded.to_string());
    expanded.into()
}

#[proc_macro_attribute]
pub fn transactional(ident: TokenStream, item: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(ident as Ident);
    let func = parse_macro_input!(item as ItemFn); // 我们传入的是一个函数，所以要用到ItemFn
    let func_vis = &func.vis; // pub
    let func_block = &func.block; //.stmts.iter().map(|r|r.to_token_stream().to_string()).collect::<Vec<_>>().join("\n"); // 函数主体实现部分{}

    let func_decl = &func.sig; // 函数申明

    let func_attrs = &func.attrs;
    let expanded = quote! { // 重新构建函数执行
        #(#func_attrs)*
        #func_vis #func_decl{
            let #ident = crate::pool!().acquire_begin().await?;
            let res=#func_block;
            match res {
                Ok(_)=>{#ident.commit().await?;}
                Err(_)=>{
                     println!("error ");
                    #ident.rollback().await?;}
            }
            res
        }
    };
    expanded.into()
}
#[derive(Debug)]
struct ExcelAttribute {
    name: Option<LitStr>,
    dict_type: Option<LitStr>,
    default_value: Option<LitStr>,
    read_converter_exp:Option<LitStr>,
    num_format: Option<LitStr>,
    width:Option<LitFloat>,
}
macro_rules! to_token_string {
    ($self_:ident,$name:ident,$tokens:ident) => {
        match &$self_.$name {
            None => $tokens.extend(quote! {
                $name:None,
            }),
            Some(t) => $tokens.extend(quote! {
                $name:Some(#t.to_string()),
            }),
        }
    };
}
macro_rules! to_token_int {
    ($self_:ident,$name:ident,$tokens:ident) => {
        match &$self_.$name {
            None => $tokens.extend(quote! {
                $name:None,
            }),
            Some(t) => $tokens.extend(quote! {
                $name:Some(#t),
            }),
        }
    };
}
impl ToTokens for ExcelAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.name {
            None => tokens.extend(quote! {
                name:"".to_string(),
            }),
            Some(t) => tokens.extend(quote! {
                name:#t.to_string(),
            }),
        }
        to_token_string! (self,dict_type,tokens);
        to_token_string! (self,default_value,tokens);
        to_token_string! (self,read_converter_exp,tokens);
        to_token_string! (self,num_format,tokens);
        to_token_int! (self,width,tokens);
    }
}
#[proc_macro_derive(Export, attributes(excel))]
pub fn export(item: TokenStream) -> TokenStream {
    // 将输入的 token 流解析为 `DeriveInput`
    let original_struct = syn::parse_macro_input!(item as syn::DeriveInput);
    let DeriveInput {
        data,
        ident,
        attrs: atrrs_s,
        ..
    } = original_struct.clone();
    let mut expand = quote! {};

    let parser = |input: syn::parse::ParseStream| {
        let mut excel_attr = ExcelAttribute {
            name: None,
            dict_type: None,
            default_value: None,
            read_converter_exp:None,
            num_format: None,
            width:None
        };
        // 解析位置参数（字符串字面量）
        if input.peek(syn::LitStr) {
            excel_attr.name = Some(input.parse::<syn::LitStr>()?);
        }

        // 解析逗号分隔的参数
        while !input.is_empty() {
            input.parse::<syn::Token![,]>()?;

            // 解析命名参数
            if input.peek(syn::Ident) && input.peek2(syn::Token![=]) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<syn::Token![=]>()?;

                match ident.to_string().as_str() {
                    "name" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.name = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "dictType" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.dict_type = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "defaultValue" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.default_value = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "readConverterExp" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.read_converter_exp = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "numFormat" => {
                        if input.peek(syn::LitStr) {
                            excel_attr.num_format = Some(input.parse::<syn::LitStr>()?);
                        }
                    }
                    "width" => {
                        if input.peek(syn::LitFloat) {
                            excel_attr.width = Some(input.parse::<syn::LitFloat>()?);
                        }
                    }
                    _ => return Err(input.error("Unknown parameter, expected `path`")),
                }
            } else if excel_attr.name.is_none() && input.peek(syn::LitStr) {
                // 处理没有前置逗号的位置参数
                excel_attr.name = Some(input.parse::<syn::LitStr>()?);
            } else {
                return Err(input.error("Expected named parameter or string literal"));
            }
        }
        Ok(excel_attr)
    };

    if let Struct(data_struct) = data {
        // 从这个数据结构中提取字段
        let DataStruct { fields, .. } = data_struct;
        // 创建用于作为输出的变量 new_fields
        // let mut new_fields = quote!();
        for Field {
            // 该字段的标识符
            ident,
            // 该字段的属性
            attrs,
            // 该字段的可见性
            vis,
            // 分隔符 `:`
            // colon_token,
            // 该字段的类型
            ty,
            ..
        } in fields
        {
            for attr in attrs {
                if attr.path().is_ident("excel") {
                    let dto_attr = attr.parse_args_with(parser).unwrap();
                    let ident_camel_case =
                        to_camel_case(&(ident.clone().map(|s| s.to_string()).unwrap_or_default()));
                    expand.extend(quote! {
                        excel_gen_attr.push(crate::ExcelGenAttr{
                                camel_case_indent: #ident_camel_case.to_string(),
                               #dto_attr
                            read_converter_map:None
                        });
                    })
                }
            }
        }
    }
    let res = quote! {
       impl #ident {
         pub fn get_excel_attr()->Vec<crate::ExcelGenAttr> {
                let mut excel_gen_attr=vec!();
                #expand
               excel_gen_attr
            }
        }
    };
    println!("result: {}", res);
    res.into()
}

use regex::Regex;

fn to_camel_case(text: &str) -> String {
    Regex::new(r"[_-]")
        .unwrap()
        .split(text)
        .enumerate()
        .map(|(i, x)| {
            if i != 0usize {
                let mut s = x.to_string();
                if let Some(r) = s.get_mut(0..1) {
                    r.make_ascii_uppercase()
                }
                s
            } else {
                x.to_string()
            }
        })
        .collect()
}
