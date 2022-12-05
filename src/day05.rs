//mod tools;

#[allow(unused)]
pub fn get_between(str:&str,from:&str,to:&str)->String
{
    if from.is_empty()
    {
        let e =          str.find(to).unwrap();
        return str[..e].to_string();
    }

    if to.is_empty()
    {
        let s =          str.find(from).unwrap();
        return str[s+from.len()..].to_string();
    }

        let s =          str.find(from).unwrap() + from.len();
        let e = s + str[s..].find(to  ).unwrap();
        
    str[s..e].to_string()
}

#[allow(unused)]
pub fn usize_get_between(str:&str,from:&str,to:&str)->usize
{
    get_between(str, from, to).parse::<usize>().unwrap()
}

struct Stacks{
    stacks : Vec<Vec<char>>,
    n:usize,
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
        if self.stacks[i].is_empty() {
            return ' ';
        }
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
        for _ in 0..n 
        {
            self.move_crate(a,b);
            //st.print();
        }
    }

    fn move_crates2(&mut self,n:usize,a:usize,b:usize)
    {
        let mut temp = vec![];
        for _ in 0..n 
        {
            let v = self.pop(a);
            temp.push(v);
        }
        
        //temp.reverse();

        for _ in 0..n 
        {
            let v = temp.pop().unwrap();
            self.push(b, v);
        }
    }

    fn fill_stack(&mut self,s:&Vec<&str>)
    {
        let dy = s.len()-1;

        for x in 0..self.n 
        {
            for i in (0..dy).rev() {
                let v = s[i].chars().nth(x*4+1).unwrap_or(' ');

                if v!= ' '
                {
                    self.push(x, v);
                }                
            }
        }
    }

    fn get_top(&mut self)->String
    {
        let mut res = vec![];
        for x in 0..self.n
        {
            let v = self.peek(x).to_string();
            res.push(v);
        }
        res.join("").trim().to_string()
    }

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
    let mut first = true;

    let mut fp= vec![];
    let mut st = Stacks::new(9);

    for s in data.iter() {
        
        if !first
        {
            let n = usize_get_between(s,"move "," from ");
            let f = usize_get_between(s," from "," to ");
            let t = usize_get_between(s," to ","");
            //println!("{}->{} {}",n,f,t);

            st.move_crates(n,f-1,t-1)
        }
        else
        if s==""
        {            
            st.fill_stack(&fp);
            st.print();
            first = false;
        }
        else {
            fp.push(s)
        }
        
    }
    
    st.get_top()
}

pub fn part2(data:&[String])->String
{
    let mut first = true;

    let mut fp= vec![];
    let mut st = Stacks::new(9);

    for s in data.iter() {
        
        if !first
        {
            let n = usize_get_between(s,"move "," from ");
            let f = usize_get_between(s," from "," to ");
            let t = usize_get_between(s," to ","");
            //println!("{}->{} {}",n,f,t);

            st.move_crates2(n,f-1,t-1)
            //for _ in 0..n 
            //{
            //    st.move_crate(f-1,t-1);
            //    st.print();
            //}
        }
        else
        if s==""
        {            
            //"move 1 from 2 to 1".to_string(),
                        
            st.fill_stack(&fp);
            st.print();
            first = false;
        }
        else {
            fp.push(s)
        }
        
    }
    
    st.get_top()    
 


    
    //data.iter().map(|s| Range::new(s) )
      //         .filter(|r| r.overlap())
        //       .count() as i32
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
