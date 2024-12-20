use common::Answer;

fn get_vecs(s: &str) -> impl Iterator<Item = Vec<Vec<i32>>> + '_ {
    s.lines().map(|l| {
        l.split(',')
            .map(|x| {
                x.split('-')
                    .map(|x| x.parse::<i32>().expect("Expected integer"))
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    })
}

pub fn step1(s: &str) -> Answer {
    get_vecs(s)
        .filter(|pair| {
            (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[1][0] >= pair[0][0] && pair[1][1] <= pair[0][1])
        })
        .count()
        .into()
}

pub fn step2(s: &str) -> Answer {
    get_vecs(s)
        .filter(|pair| {
            (pair[0][0] >= pair[1][0] && pair[0][0] <= pair[1][1])
                || (pair[0][1] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[1][0] >= pair[0][0] && pair[1][0] <= pair[0][1])
                || (pair[1][1] >= pair[0][0] && pair[1][1] <= pair[0][1])
        })
        .count()
        .into()
}
