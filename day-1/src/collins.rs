pub fn expense_report_brute_force(exp: &Vec<u32>) -> Option<u32> {
    for n in exp.iter() {
        for i in exp.iter() {
            if n + i == 2020 {
                return Some(n * i);
            }
        }
    }
    None
}

fn main() {
    let expenses = vec![1721, 979, 366, 299, 675, 1456];
    let res = expense_report_brute_force(&expenses);
    println!("Solution 1: {}", res.unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expense_report() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];
        let res = expense_report_brute_force(&expenses).unwrap();
        assert_eq!(res, 514579);
    }
}