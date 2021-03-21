use trigout::get_socket_name;

#[test]
fn test_get_socket_name() {
    let with_name_args = vec!["".to_owned(), "a".to_owned()];
    let with_no_name_args = vec!["".to_owned()];
    let with_space_args = vec!["".to_owned(), "a s".to_owned()];

    assert_eq!("a", get_socket_name(&with_name_args));
    assert_eq!("0", get_socket_name(&with_no_name_args));
    assert_eq!("a s", get_socket_name(&with_space_args));
}
