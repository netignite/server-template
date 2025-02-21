use lib_api_data::export;

#[test]
fn test_type_export() {
    let types = export::get_typescript_definitions();
    assert_eq!(
        types
            .split("\n")
            .filter(|line| { !line.is_empty() })
            .collect::<Vec<&str>>(),
        [
            "export type Response<T> = { data: T, };",
            "export type ErrorResponse<T> = { message: string, code: number | null, data: T | null, };",
            "export type ResponseType<T> = { \"type\": \"success\" } & Response<T> | { \"type\": \"fail\" } & Response<T> | { \"type\": \"error\" } & ErrorResponse<T>;"
        ]
    )
}
