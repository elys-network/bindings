use crate::types::PageRequest;

#[test]
fn pagination_filter_test() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut pagination: PageRequest = PageRequest {
        key: None,
        offset: None,
        limit: 2,
        count_total: false,
        reverse: false,
    };

    let (res, p_res) = pagination.filter(vec.clone()).unwrap();
    let (first, rest) = vec.split_at(2);

    assert_eq!(res, first);
    assert!(p_res.next_key.is_some());

    pagination.update(p_res.next_key);
    let (res, p_res) = pagination.filter(vec.clone()).unwrap();
    let (second, last) = rest.split_at(2);

    assert_eq!(res, second);
    assert!(p_res.next_key.is_some());

    pagination.update(p_res.next_key);
    let (res, p_res) = pagination.filter(vec.clone()).unwrap();

    assert_eq!(res, last);
    assert!(p_res.next_key.is_none());
}
