struct Command{
    cycle: i64,
    val  : i64,
}

impl Command {
    fn new(cycle:i64,val:i64)->Self {
        Self { cycle,val }
    }
}

pub fn part1(data:&[String])->i64
{
    let mut cycle=0;
    let mut commads = vec![];

    for s in data.iter() 
    {
        let tab : Vec<&str> = s.split(' ').collect(); 
        let command   = tab[0];
        let mut val = 0;

        if command=="noop"
        {
            cycle+=1;
        }
        else
        if command=="addx"
        {
            val = tab[1].parse::<i64>().unwrap();
            cycle+=2;
            commads.push(Command::new(cycle,val))
        }
    }

    cycle=0;
    let mut sum=0i64;
    let mut signal=1i64;
    let mut screen = vec![vec!['#';40];6];
    let mut id=0;

    while id<commads.len()
    {
        let c = commads[id];

        let signal_strength = cycle*signal;
        if (cycle-20)%40==0 && cycle>0
        {
            
            sum+=signal_strength;            
            println!("sum {}",sum);
        }

        if cycle==c.cycle
        {
            signal+= c.val;
            id+=1;
        }        

        
        let yy = ((cycle)/40)%6;
        let xx =  (cycle)%40;
        let x_pos = (signal+40000)%40;
        cycle+=1;
        
        if (xx-x_pos).abs()<=1
        {
            screen[yy as usize][xx as usize] = '#';
        }
        else
        {
            screen[yy as usize][xx as usize] = '.';
        }
    }
    
    for line in screen
    {
        for c in line 
        {
            print!("{}",c);
        }
        println!("");
    }

   
    sum

}



pub fn part2(data:&[String])->usize
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 10");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1_0()
{
    let v = vec![
        "addx 15".to_string(),
        "addx -11".to_string(),
        "addx 6".to_string(),
        "addx -3".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx -8".to_string(),
        "addx 13".to_string(),
        "addx 4".to_string(),
        "noop".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx 5".to_string(),
        "addx -1".to_string(),
        "addx -35".to_string(),
        "addx 1".to_string(),
        "addx 24".to_string(),
        "addx -19".to_string(),
        "addx 1".to_string(),
        "addx 16".to_string(),
        "addx -11".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 21".to_string(),
        "addx -15".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -3".to_string(),
        "addx 9".to_string(),
        "addx 1".to_string(),
        "addx -3".to_string(),
        "addx 8".to_string(),
        "addx 1".to_string(),
        "addx 5".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -36".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "addx 7".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 2".to_string(),
        "addx 6".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 7".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx -13".to_string(),
        "addx 13".to_string(),
        "addx 7".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "addx -33".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 2".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 8".to_string(),
        "noop".to_string(),
        "addx -1".to_string(),
        "addx 2".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx 17".to_string(),
        "addx -9".to_string(),
        "addx 1".to_string(),
        "addx 1".to_string(),
        "addx -3".to_string(),
        "addx 11".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -13".to_string(),
        "addx -19".to_string(),
        "addx 1".to_string(),
        "addx 3".to_string(),
        "addx 26".to_string(),
        "addx -30".to_string(),
        "addx 12".to_string(),
        "addx -1".to_string(),
        "addx 3".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -9".to_string(),
        "addx 18".to_string(),
        "addx 1".to_string(),
        "addx 2".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 9".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx -1".to_string(),
        "addx 2".to_string(),
        "addx -37".to_string(),
        "addx 1".to_string(),
        "addx 3".to_string(),
        "noop".to_string(),
        "addx 15".to_string(),
        "addx -21".to_string(),
        "addx 22".to_string(),
        "addx -6".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx 2".to_string(),
        "addx 1".to_string(),
        "noop".to_string(),
        "addx -10".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "addx 20".to_string(),
        "addx 1".to_string(),
        "addx 2".to_string(),
        "addx 2".to_string(),
        "addx -6".to_string(),
        "addx -11".to_string(),
        "noop".to_string(),
        "noop".to_string(),
        "noop".to_string(),
    ];
    assert_eq!(part1(&v),13140_0);
}


#[test]
fn test1_1()
{
    let v = vec![
        "noop".to_string(),
        "addx 3".to_string(),
        "addx -5".to_string(),
    ];
    assert_eq!(part1(&v),13140);
}



#[test]
fn test2()
{
    let v = vec![

    ];
    assert_eq!(part2(&v),36);
}
