use crate::common::util::*;

#[test]
fn test_output_is_random_permutation() {
    let input_seq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = input_seq
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    let result = new_ucmd!()
        .pipe_in(input.as_bytes())
        .succeeds()
        .no_stderr()
        .stdout
        .clone();

    let mut result_seq: Vec<i32> = result
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    result_seq.sort();
    assert_ne!(result, input, "Output is not randomised");
    assert_eq!(result_seq, input_seq, "Output is not a permutation");
}

#[test]
fn test_zero_termination() {
    new_ucmd!().arg("-z").pipe_in("1\n2\n3\n4\n5").succeeds().no_stderr().stdout_is("1\02\03\04\05");
}
