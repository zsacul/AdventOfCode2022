
struct Forest{
    field : Vec<Vec<u8>>,
    dx    : usize,
    dy    : usize,
}

impl Forest {
    fn new(data:&[String])->Self
    {
        let dx = data[0].len();
        let dy = data.len();

        let mut res = Self {
            field : vec![vec![0;dx];dy],
            dx,
            dy
        };

        for (y_pos,y) in data.iter().enumerate()
        {
            for x in 0..dx
            {
                let c = y.chars().nth(x).unwrap() as u8 - b'0';
                res.field[y_pos][x] = c;   
            }                
        }
        res
    }

    fn pos_ok(&self,x:i32,y:i32)->bool
    {
        x>=0 && y>=0 && x<self.dx as i32 && y<self.dy as i32
    }

    fn get_val(&self,x:i32,y:i32)->i32
    {
        if self.pos_ok(x,y) 
        {
            return self.field[y as usize][x as usize] as i32;
        }
        i32::MAX
    }

    fn all_visible(&self,x:i32,y:i32,move_x:i32,move_y:i32)->bool
    {
        let mut x = x;
        let mut y = y;
        let v = self.get_val(x,y);
        while self.pos_ok(x,y)
        {
            x+=move_x;
            y+=move_y;
            if self.pos_ok(x,y) && self.get_val(x,y)>=v { return false; }
        }
        true
    }

    fn all_visible_n(&self,x:i32,y:i32,move_x:i32,move_y:i32)->i32
    {
        let mut x = x;
        let mut y = y;
        let v = self.get_val(x,y);
        let mut res = 0;

        while self.pos_ok(x,y)
        {
            x+=move_x;
            y+=move_y;
            res+=1;
            if self.pos_ok(x,y) && self.get_val(x,y)>=v { return res; }
        }
        res-1
    }

    fn visible(&self,x:i32,y:i32)->bool
    {
        return self.all_visible(x,y, 0, 1) ||
               self.all_visible(x,y, 0,-1) ||
               self.all_visible(x,y, 1, 0) ||
               self.all_visible(x,y,-1, 0)
    }

    fn visible_n(&self,x:i32,y:i32)->i32
    {
        return self.all_visible_n(x,y, 0, 1) *
               self.all_visible_n(x,y, 0,-1) *
               self.all_visible_n(x,y, 1, 0) *
               self.all_visible_n(x,y,-1, 0)
    }


    #[allow(unused)]
    fn print(&self)
    {
         println!("{:?}",self.field);

        //for x in 0..self.n 
        //{
          //  println!("{}:{:?}",x,self.field[x]);
        //}
    }

    fn count_visible(&self)->usize 
    {
        let mut res = 0;
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                if self.visible(x as i32,y as i32) { res+=1; }
            }
        }

        res    
    }

    fn count_visible_n(&self)->usize 
    {
        let mut res = 0;
        let mut m = 0;

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                if self.visible(x as i32,y as i32)
                {
                    let c = self.visible_n(x as i32,y as i32);
                    if c>m { m=c;}                
    
                }
            }
        }

        m as usize
    }

    //422331


}


pub fn part1(data:&[String])->usize
{
    let f = Forest::new(data);
//    f.print();
    f.count_visible()
}

pub fn part2(data:&[String])->usize
{
    let f = Forest::new(data);
    //f.print();
    f.count_visible_n()

    

}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day8");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "30373".to_string(),
        "25512".to_string(),
        "65332".to_string(),
        "33549".to_string(),
        "35390".to_string(),
    ];
    assert_eq!(part1(&v),21);
}

#[test]
fn test2()
{
    let v = vec![
        "30373".to_string(),
        "25512".to_string(),
        "65332".to_string(),
        "33549".to_string(),
        "35390".to_string(),
    ];
    assert_eq!(part2(&v),8);
}
