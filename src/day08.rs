
struct Forest 
{
    dx    : usize,
    dy    : usize,
    field : Vec<Vec<u8>>,
}

impl Forest 
{
    fn new(data:&[String])->Self
    {
        let dx = data[0].len();
        let dy = data.len();

        let mut res = Self {
            dx,
            dy,
            field : vec![vec![0;dx];dy],
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

    fn get_val(&self,x:i32,y:i32)->u8
    {
        if self.pos_ok(x,y) 
        {
            return self.field[y as usize][x as usize];
        }
        u8::MAX
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

    fn get_num_visible(&self,x:i32,y:i32,move_x:i32,move_y:i32)->i32
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

    fn any_visible(&self,x:i32,y:i32)->bool
    {
        return self.all_visible(x,y, 0, 1) ||
               self.all_visible(x,y, 0,-1) ||
               self.all_visible(x,y, 1, 0) ||
               self.all_visible(x,y,-1, 0)
    }

    fn visible_n(&self,x:i32,y:i32)->i32
    {
        return self.get_num_visible(x,y, 0, 1) *
               self.get_num_visible(x,y, 0,-1) *
               self.get_num_visible(x,y, 1, 0) *
               self.get_num_visible(x,y,-1, 0)
    }


    #[allow(unused)]
    fn print(&self)
    {
         println!("{:?}",self.field);
    }

    fn count_visible(&self)->usize 
    {
        (0..self.dy).flat_map(move |a| (0..self.dx).map(move |b| (a as i32, b as i32)))
                    .filter(|(x,y)| self.any_visible(*x,*y))
                    .count()
    }

    fn count_visible_n(&self)->usize 
    {
        (0..self.dy).flat_map(move |y| (0..self.dx).map(move |x| self.visible_n(x as i32,y as i32)))
                    .max()
                    .unwrap_or(0) as usize
    }
}

pub fn part1(data:&[String])->usize
{
    Forest::new(data).count_visible()    
}

pub fn part2(data:&[String])->usize
{
    Forest::new(data).count_visible_n()
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
