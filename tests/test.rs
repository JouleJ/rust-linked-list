use linkedlist::*;

#[test]
fn test_empty() {
    let empty_list = List::<i32>::new();
    assert_eq!(empty_list.len(), 0);
    assert_eq!(empty_list.get(0), None);
}

#[test]
fn test_push_front() {
    let empty_list = List::<i32>::new();
    let singulat_list = empty_list.push_front(123);
    assert_eq!(singulat_list.len(), 1);
    assert_eq!(singulat_list[0], 123);
}

#[test]
fn test_from_slice() {
    let my_list = List::<i32>::from_slice(&[1, 2, 3]);
    assert_eq!(my_list.len(), 3);
    assert_eq!(my_list[0], 1);
    assert_eq!(my_list[1], 2);
    assert_eq!(my_list[2], 3);
    assert_eq!(my_list.get(3), None);
}

#[test]
fn test_from_iter() {
    let my_list = List::<i32>::from_iter(vec![1, 2, 3].into_iter());
    assert_eq!(my_list.len(), 3);
    assert_eq!(my_list[0], 1);
    assert_eq!(my_list[1], 2);
    assert_eq!(my_list[2], 3);
    assert_eq!(my_list.get(3), None);
}

#[test]
fn test_iter() {
    let list1 = List::<i32>::from_slice(&[1, 2, 3, 4]);
    let list2: List::<i32> = list1.into_iter().map(|x| x * x).collect();
    assert_eq!(list2.len(), 4);
    assert_eq!(list2[0], 1);
    assert_eq!(list2[1], 4);
    assert_eq!(list2[2], 9);
    assert_eq!(list2[3], 16);
}

#[test]
fn test_for() {
    let list1 = List::<i32>::from_slice(&[5, 6, 7]);
    let mut vec = Vec::<i32>::new();
    for elem in &list1 {
        vec.push(*elem);
    }
    assert_eq!(vec, vec![5, 6, 7]);
}

#[test]
fn test_overlapping() {
    let base = List::<i32>::from_slice(&[1, 2, 3]);
    let extented1 = base.push_front(4);
    let extented2 = base.push_front(5);

    assert_eq!(extented1.len(), 4);
    assert_eq!(extented1[0], 4);
    assert_eq!(extented1[1], 1);
    assert_eq!(extented1[2], 2);
    assert_eq!(extented1[3], 3);

    assert_eq!(extented2.len(), 4);
    assert_eq!(extented2[0], 5);
    assert_eq!(extented2[1], 1);
    assert_eq!(extented2[2], 2);
    assert_eq!(extented2[3], 3);
}

#[test]
fn test_display() {
    let list = List::<i32>::from_slice(&[1, 2, 3]);
    assert_eq!(list.to_string(), "[1, 2, 3]");
}

#[test]
fn test_concat() {
    let list1 = List::<i32>::from_slice(&[1, 2, 3]);
    let list2 = List::<i32>::from_slice(&[4, 5, 6]);
    let sum_list = list1.concat(&list2);
    assert_eq!(sum_list.to_string(), "[1, 2, 3, 4, 5, 6]");
}

#[test]
fn test_concat_to_itself() {
    let list = List::<i32>::from_slice(&[1, 2]);
    let sum_list = list.concat(&list);
    assert_eq!(sum_list.to_string(), "[1, 2, 1, 2]");
}

#[test]
fn test_reverse() {
    let list = List::<i32>::from_slice(&[1, 2, 3]);
    assert_eq!(list.reverse().to_string(), "[3, 2, 1]");
}

#[test]
fn test_flat_map() {
    let list = List::<i32>::from_slice(&[1, 2, 3]);
    let list2 = list.flat_map(|x| List::<i32>::from_slice(&[-*x, *x]));
    assert_eq!(list2.to_string(), "[-1, 1, -2, 2, -3, 3]");
}
