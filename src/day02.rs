#[derive(PartialEq)]
enum Result {
    Win,
    Loose,
    Draw
}

enum Hand {
    Rock,
    Paper,
    Scissors
}

fn get_points(h:&Hand)->i32
{
    match h {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn get_hand(c: char)->Hand
{
    match c {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scissors,
        _         => panic!("wrong code")
    }
}

fn game(l:&Hand,r:&Hand)->Result
{
    match (l,r) {
        (Hand::Rock    ,Hand::Paper   ) => Result::Win,
        (Hand::Paper   ,Hand::Rock    ) => Result::Loose,
        (Hand::Rock    ,Hand::Scissors) => Result::Loose,
        (Hand::Scissors,Hand::Rock    ) => Result::Win,
        (Hand::Paper   ,Hand::Scissors) => Result::Win,
        (Hand::Scissors,Hand::Paper   ) => Result::Loose,
        _                               => Result::Draw
    }
}

fn wanted_result(c:char)->Result{
    match c {
        'X' => Result::Loose,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _   => panic!("wrong code")
    }    
}

fn get_result_points(result:&Result)->i32
{
    match result {
        Result::Loose => 0,
        Result::Draw  => 3,
        Result::Win   => 6,
    }
}

pub fn part1(data:&[String])->i32
{
    data.iter().map(|s|
        {
            let (l,r) = (s.chars().next().unwrap(),s.chars().nth(2).unwrap());
            let lh = get_hand(l);
            let rh = get_hand(r);
            get_points(&rh) + get_result_points(&game(&lh,&rh)) 
        }
    ).sum()
}

pub fn part2(data:&[String])->i32
{
    data.iter().map(|s|
        {
            let (l,r) = (s.chars().next().unwrap(),s.chars().nth(2).unwrap());
            let lh = get_hand(l);
            
            let mut rh = Hand::Rock;
            let mut result = game(&lh,&rh);

            if wanted_result(r)!=result
            {
                rh = Hand::Paper;
                result = game(&lh,&rh);         

                if wanted_result(r)!=result
                {
                    rh = Hand::Scissors;
                    result = game(&lh,&rh);         
                }
            }

            get_points(&rh) + get_result_points(&result) 
        }
    ).sum()   
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day2");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "A Y".to_string(),
        "B X".to_string(),
        "C Z".to_string(),
        ];
    assert_eq!(part1(&v),15);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "A Y".to_string(),
        "B X".to_string(),
        "C Z".to_string(),
        ];
    assert_eq!(part2(&v),12);
}