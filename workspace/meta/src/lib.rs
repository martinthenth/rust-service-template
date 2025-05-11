use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn data_case(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let expanded = quote! {
        #[tokio::test]
        async fn #fn_name() {
            let url = std::env::var("DATABASE_URL").unwrap();
            let pool = sqlx::postgres::PgPoolOptions::new().max_connections(1).connect(&url).await.unwrap();
            let mut conn = pool.begin().await.unwrap();

            async #fn_block.await;

            conn.rollback().await.unwrap();
            pool.close().await;
        }
    };

    TokenStream::from(expanded)
}
