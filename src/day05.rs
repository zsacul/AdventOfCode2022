use super::tools;

struct Stacks{
    stacks : Vec<Vec<char>>,
    n : usize,
}

impl Stacks {
    fn new(n:usize)->Self
    {
        Self {
            stacks : vec![vec![];n],
            n
        } 
    }

    fn push(&mut self,i:usize,v:char)
    {
        self.stacks[i].push(v);
    }

    fn pop(&mut self,i:usize)->char
    {
        self.stacks[i].pop().unwrap()
    }

    fn peek(&mut self,i:usize)->char
    {
        if self.stacks[i].is_empty() { return ' '; }
        let v = self.pop(i);
        self.push(i,v);
        v
    }

    fn move_crate(&mut self,a:usize,b:usize)
    {
        let v = self.pop(a);
        self.push(b, v);
    }

    fn move_crates(&mut self,n:usize,a:usize,b:usize)
    {
        for _ in 0..n { self.move_crate(a,b); }
    }

    fn move_crates2(&mut self,n:usize,a:usize,b:usize)
    {
        let mut temp = vec![];
        for _ in 0..n { temp.push(self.pop(a));            }
        for _ in 0..n { self.push(b, temp.pop().unwrap()); }
    }

    fn fill_stack(&mut self,s:&[String])
    {
        for x in 0..self.n 
        {
            for i in (0..s.len()-1).rev() 
            {
                let v = s[i].chars().nth(x*4+1).unwrap_or(' ');
                if v!= ' ' { self.push(x, v); }                
            }
        }
    }

    fn get_top(&mut self)->String
    {
        let mut res = vec![];
        for x in 0..self.n
        {
            res.push(self.peek(x).to_string());
        }
        res.join("").trim().to_string()
    }

    #[allow(unused)]
    fn print(&self)
    {
        for x in 0..self.n 
        {
            println!("{}:{:?}",x,self.stacks[x]);
        }
    }
}

pub fn part1(data:&[String])->String
{
    let d = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
    let mut stacks = Stacks::new(9);
    
    stacks.fill_stack(d[0]);

    for s in d[1].iter()
    {
        let id   = tools::usize_get_between(s,"move " ," from ");
        let from = tools::usize_get_between(s," from "," to ");
        let to   = tools::usize_get_between(s," to "  ,"");
        stacks.move_crates(id,from-1,to-1);
    }

    stacks.get_top()
}

pub fn part2(data:&[String])->String
{
    let d = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
    let mut stacks = Stacks::new(9);
    
    stacks.fill_stack(d[0]);

    for s in d[1].iter()
    {
        let id   = tools::usize_get_between(s,"move " ," from ");
        let from = tools::usize_get_between(s," from "," to ");
        let to   = tools::usize_get_between(s," to "  ,"");
        stacks.move_crates2(id,from-1,to-1);
    }

    stacks.get_top()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day5");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "    [D]    ".to_string(),
        "[N] [C]    ".to_string(),
        "[Z] [M] [P]".to_string(),
        " 1   2   3 ".to_string(),
        "".to_string(),
        "move 1 from 2 to 1".to_string(),
        "move 3 from 1 to 3".to_string(),
        "move 2 from 2 to 1".to_string(),
        "move 1 from 1 to 2".to_string(),
        ];
    assert_eq!(part1(&v),"CMZ".to_string());
}

#[test]
fn test2()
{
    let v = vec![
        "    [D]    ".to_string(),
        "[N] [C]    ".to_string(),
        "[Z] [M] [P]".to_string(),
        " 1   2   3 ".to_string(),
        "".to_string(),
        "move 1 from 2 to 1".to_string(),
        "move 3 from 1 to 3".to_string(),
        "move 2 from 2 to 1".to_string(),
        "move 1 from 1 to 2".to_string(),
    ];
    assert_eq!(part2(&v),"MCD".to_string());
}
