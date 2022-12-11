struct Operation
{
    left : i64,
    oper : char,
    right : i64,
    acc : i64,
}

impl Operation 
{
    fn new(l:&str)->Self
    {
        let ll = l.replace("old","-1");
        
        let ll : Vec<&str> = ll.split(' ').collect(); 
        Self 
        {
            left  : ll[0].parse().unwrap(), 
            oper  : ll[1].chars().next().unwrap(),
            right : ll[2].parse().unwrap(), 
            acc   : 0
        }
    }

    fn des(&self)->String
    {
        format!("{} {} {}",self.left,self.oper,self.right)
    }

    fn calc(&mut self)->i64
    {
        let mut a = self.left;
        let mut b = self.right;

        if a==-1 { a = self.acc; }
        if b==-1 { b = self.acc; }

        match self.oper {
            '+' => { self.acc = self.left + self.right },
            '-' => { self.acc = self.left - self.right },
            '*' => { self.acc = self.left * self.right },
            '/' => { self.acc = self.left / self.right },
            _   => panic!("rr"),
        }
        self.acc
    }
}

struct Monkey{
    id: usize,
    items : Vec<i64>,
    operation: Operation,
    test_div : i64,
    test_t: usize,
    test_f: usize,
}

impl Monkey 
{
    fn new(id:usize,items:&str,operation:&str,test:&str,test_true:&str,test_false:&str)->Self
    {
        let i    =      items.split("  Starting items: ").last().unwrap(); 
        let o    =      operation.split("Operation: new = ").last().unwrap(); 
        let t1   =      test.split("Test: divisible by ").last().unwrap(); 
        let t2   =      test_true.split("If true: throw to monkey ").last().unwrap(); 
        let t3   =      test_false.split("If false: throw to monkey ").last().unwrap(); 
        //let left  : Vec<&str> = tab[0].split('-').collect(); 
        //let right : Vec<&str> = tab[1].split('-').collect(); 

        println!("_{}",i);
        let it: Vec<i64> = i.split(", ")
                            .map(|i| i.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>();

        Self 
        {
            id,
            items: it,
            operation: Operation::new(o),
            test_div: t1.parse::<i64>().unwrap(),
            test_t:   t2.parse::<usize>().unwrap(),
            test_f:   t3.parse::<usize>().unwrap(),
        }
    }

    fn go(&self)->usize
    {
        if self.operation.acc % self.test_div==0
        {
            self.test_t
        }
        else {
            self.test_f
        }
    }

    fn print(&self)
    {
        println!();
        println!("    Monkey {}:",self.id);
        println!("  Starting items: {:?}",self.items);
        println!("  Operation: {}"      ,self.operation.des());
        println!("  Test: divisible by {}"         ,self.test_div );
        println!("    If true: throw to monkey {}" ,self.test_t);
        println!("    If false: throw to monkey {}",self.test_f);
    }
}

pub fn part1(data:&[String])->i32
{
//    let monkeys = data.split(|s| s.is_empty())
                                   //.collect::<Vec<[String]>>();

    let monkeys = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();


    let mm = monkeys.iter()
                   .map(|&m| Monkey::new(0,
                                        &m[1][..],
                                        &m[2][..],
                                        &m[3][..],
                                        &m[4][..],
                                        &m[5][..])).collect::<Vec<Monkey>>();

                                        for m in mm
                                        {
                                            m.print();
                                        }
     0
           //.count() as i32
}

pub fn part2(data:&[String])->i32
{
    0
    //data.iter().map(|s| Range::new(s) )
      //         .filter(|r| r.overlap())
        //       .count() as i32
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day 11");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
"        Monkey 0:".to_string(),
"        Starting items: 79, 98".to_string(),
"        Operation: new = old * 19".to_string(),
"        Test: divisible by 23".to_string(),
"          If true: throw to monkey 2".to_string(),
"          If false: throw to monkey 3".to_string(),
"".to_string(),
"      Monkey 1:".to_string(),
"        Starting items: 54, 65, 75, 74".to_string(),
"        Operation: new = old + 6".to_string(),
"        Test: divisible by 19".to_string(),
"          If true: throw to monkey 2".to_string(),
"          If false: throw to monkey 0".to_string(),
"".to_string(),
"      Monkey 2:".to_string(),
"        Starting items: 79, 60, 97".to_string(),
"        Operation: new = old * old".to_string(),
"        Test: divisible by 13".to_string(),
"          If true: throw to monkey 1".to_string(),
"          If false: throw to monkey 3".to_string(),
"".to_string(),
"      Monkey 3:".to_string(),
"        Starting items: 74".to_string(),
"        Operation: new = old + 3".to_string(),
"        Test: divisible by 17".to_string(),
"          If true: throw to monkey 0".to_string(),
"          If false: throw to monkey 1".to_string(),
    ];
    assert_eq!(part1(&v),2);
}

#[test]
fn test2()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),4);
}
