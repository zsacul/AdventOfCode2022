#[derive(Copy,Clone)]
struct Command 
{
    cycle : i64,
    value : i64,
}

impl Command 
{
    fn new(cycle:i64,value:i64)->Self 
    {
        Self { cycle,value }
    }
}

pub fn part1(data:&[String],print_screen:bool)->i64
{
    let mut cycle    = 0;

    let commands = data.iter()
                        .map(
                                |line|
                                {
                                    let tab : Vec<&str> = line.split(' ').collect(); 

                                    if tab[0]=="addx"
                                    {
                                        cycle+=2;
                                        Command::new(cycle,tab[1].parse::<i64>().unwrap())
                                    }
                                    else 
                                    {
                                        cycle+=1;
                                        Command::new(cycle,0)                                       
                                    }
                                }
                        ).collect::<Vec<Command>>();

    let mut sum    = 0i64;
    let mut signal = 1i64;
    let mut screen = vec![vec!['?';40];6];
    let mut id = 0;
    cycle = 0;

    while id<commands.len()
    {
        let command         = commands[id];
        let signal_strength = cycle*signal;

        if (cycle-20)%40==0
        {
            sum+=signal_strength;            
        }

        if cycle==command.cycle
        {
            signal+=command.value;
            id+=1;
        }        
        
        if print_screen
        {
            let yy = (((cycle)/40)%6) as usize;
            let xx = ( (cycle)%40   ) as usize;
            
            screen[yy][xx] = if (xx as i64 - signal).abs()<=1 {'#'} else {'.'}   
        }

        cycle+=1;
    }
    
    if print_screen
    {
        for line in screen
        {
            println!("{}",line.into_iter().collect::<String>());
        }   
    }
   
    sum
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 10");
    println!("part1: {}",part1(data,false));
    println!("part2: "); part1(data,true);
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
    assert_eq!(part1(&v,true),13140);
}

#[test]
fn test1_1()
{
    let v = vec![
        "noop".to_string(),
        "addx 3".to_string(),
        "addx -5".to_string(),
    ];
    assert_eq!(part1(&v,false),0);
}
