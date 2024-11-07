use common::Answer;

fn get_vecs(s: &str) -> Vec<Vec<Vec<i32>>> {
    s.lines()
        .map(|l| {
            l.split(',')
                .map(|x| {
                    x.split('-')
                        .map(|x| x.parse::<i32>().expect("Expected integer"))
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<_>>()
}

pub fn step1(s: &str) -> Answer {
    get_vecs(s)
        .iter()
        .filter(|pair| {
            (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[1][0] >= pair[0][0] && pair[1][1] <= pair[0][1])
        })
        .count()
        .into()
}

pub fn step2(s: &str) -> Answer {
    get_vecs(s)
        .iter()
        .filter(|pair| {
            (pair[0][0] >= pair[1][0] && pair[0][0] <= pair[1][1])
                || (pair[0][1] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[1][0] >= pair[0][0] && pair[1][0] <= pair[0][1])
                || (pair[1][1] >= pair[0][0] && pair[1][1] <= pair[0][1])
        })
        .count()
        .into()
}
