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
        _ => Result::Draw
    }
}

pub fn part1(data:&[String])->i32
{
    data.iter().map(|s|
        {
            let cmd : Vec<&str> = s.split(' ').collect();
            let lh = get_hand(cmd[0].chars().next().unwrap());
            let rh = get_hand(cmd[1].chars().next().unwrap());

            let result = game(&lh,&rh);
            
            let cnt = match result
            {
                Result::Win   => 6,
                Result::Loose => 3,
                Result::Draw  => 0,
            };

            get_points(&rh)+cnt
        }
    ).sum()
}

pub fn part2(data:&[String])->i32
{
    data.iter().map(|s|
        {
            let cmd : Vec<&str> = s.split(' ').collect();
            let (mut l,mut r) = (cmd[0],cmd[1]);


            println!("[{}]",r);
            if r=="Y"
            {
                let ns = match l {
                    "A" => "X",//rock
                    "B" => "Y",//paper
                    "C" => "Z",//scisors
                    _   => "",
                };
    
                r = ns;
            }
            else if r=="X"
            {
                let ns = match l {
                    "A" => "Z",//rock
                    "B" => "X",//paper
                    "C" => "Y",//scisors
                    _   => "",
                };
    
                r = ns;
            }
            else
            {
                let ns = match l {
                    "A" => "Y",//rock
                    "B" => "Z",//paper
                    "C" => "X",//scisors
                    _   => "",
                };
    
                r = ns;
            }
            

            let lp = match l {
                "A" => 1,//rock
                "B" => 2,//paper
                "C" => 3,//scisors
                _   => 0,
            };
            let rp = match r {
                "X" => 1,//rock
                "Y" => 2,//paper
                "Z" => 3,//scisors
                _   => 0,
            };
            let ss = format!("{}{}",l,r);

            let sss = &ss[..2];
//            let cnt = match sss
//            {
//                "X"=> 0,
//                "Y"=> 3,
//                "Z"=> 6,
//                _   => 0,
//            };
//
            let cnt = match sss
            {
                "AX"=> 3,
                "AY"=> 6,
                "AZ"=> 0,
                
                "BX"=> 0,
                "BY"=> 3,
                "BZ"=> 6,

                "CX"=> 6,
                "CY"=> 0,
                "CZ"=> 3,
                _   => 0,
            };            

            //let mut cnt=0;
            //if lp>rp { cnt = 1; }
            //if lp<rp { cnt =-1; }
            
            println!("*{}* {} {}",sss,rp,cnt);
            rp+cnt
        }
    ).sum()
    
    //let mut tab = get_table(data);
    //tab.sort();
    //tab.iter().rev().take(3).sum()    
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