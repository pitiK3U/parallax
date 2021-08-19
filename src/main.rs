// FIX: Objects are seethrough
use std::io::Write;
use std::convert::TryInto;
use rand::Rng;

const trail_space: i8 = 2;

// DRAWING OBJECTS ==================

const TREE: [&str; 4] = [r"      /\      
     /\*\     
    /\O\*\    
   /*/\/\/\   
  /\O\/\*\/\  
 /\*\/\*\/\/\ 
/\O\/\/*/\/O/\
      ||      
      ||      
      ||      ",
r"   __   _   
 _(  )_( )_ 
(_   _    _)
  (_) (__)  ",
r"   .(   
  /%/\  
 (%(%)) 
.-'..`-.
`-'.'`-'",
"
  ,d88b.d88b, 
  88888888888 
  `Y8888888Y' 
    `Y888Y'  
      `Y'    "
];


const LINEBLOCK: &str = r" ---------
/        /
--------- ";

// =================================

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i8,
    y: i8,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Object {
    string: &'static str,
    origin_pos: Point,
    size: Point,
    cycles: u8,
    move_cycles: u8,
}

impl Object {
    fn new(string: &'static str) -> Self {
        Object {
            string,
            origin_pos: Point {
                x: -10, y: 0 },
            size: Point {
                x: string.lines().into_iter().nth(0).unwrap().len() as i8,
                y: string.lines().count() as i8,
            },
            cycles: 0,
            move_cycles: 1,
        }
    }

    fn x(mut self, x: i8) -> Self {
        self.origin_pos.x = x;
        self
    }

    fn y(mut self, y: i8) -> Self {
        self.origin_pos.y = y;
        self
    }

    fn cycles_to_move(mut self, c: u8) -> Self {
        self.move_cycles = c;
        self
    }
}

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn clear() {
    // \x1B == \e (Escape char)
    print!("\x1Bc");
    //std::process::Command::new("clear").status().unwrap();
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut cols: i8 = 0;
    let mut lines: i8 = 0;
    init(&mut cols, &mut lines);

    let view_lines: i8 = lines - 6;
    let mut screen = String::new();

    let mut vecview: Vec<Object> = Vec::new();
    let mut vecroad: Vec<Object> = Vec::new();

    for _ in 0..=rng.gen_range(6, 14) {
        let y = rng.gen_range(10, view_lines);
        vecview.push(Object::new(TREE[rng.gen_range(0, TREE.len())]).x(rng.gen_range(0, cols)).y(y).cycles_to_move((view_lines - y) as u8));
    }
    
    let mut st = String::new();
    for _ in 0..=lines {
        st.push('\n');
    }
    print!("{}", st);

    loop {
        // Clear terminal screen
        screen.clear();
        clear();

        // Draw objects to string
        screen.push_str(&view(cols, view_lines, &vecview));
        screen.push_str(&road(cols, &vecroad));

        // Draw objects to terminal screen 
        print!("{}", screen);
        std::io::stdout().flush().unwrap();

        // Wait for next frame
        //sleep(300);
        sleep(1000/60);

        // Update program's objects
        update_road(cols, &mut vecroad);
        update_view(cols, view_lines, &mut vecview);
    }
}

fn init(cols: &mut i8, lines: &mut i8) {
    let utf8 = std::process::Command::new("tput").arg("cols").output().unwrap().stdout;
    let string: &str = std::str::from_utf8(&utf8).unwrap();
    *cols = string.trim().parse::<i8>().unwrap();

    let utf8 = std::process::Command::new("tput").arg("lines").output().unwrap().stdout;
    let string: &str = std::str::from_utf8(&utf8).unwrap();
    *lines = string.trim().parse::<i8>().unwrap();
}

fn view(cols: i8, lines: i8, vecview: &Vec<Object>) -> String {
    let mut view = String::new();

    for line in 0..lines {
        for col in 0..cols {
            let mut draw = false;
            let mut cvecview = vecview.iter().filter(|&obj| (line <= obj.origin_pos.y && line >= obj.origin_pos.y - obj.size.y + 1) && (obj.origin_pos.x <= col && col <= obj.origin_pos.x + obj.size.x - 1)).collect::<Vec<&Object>>().clone();
            if let Some(mut obj) = cvecview.iter().max_by_key(|&a| a.origin_pos.y) {
             loop {
                let str_x = col - obj.origin_pos.x; // obj.origin_pos.x + obj.size.x - 1 - col;
                let str_y = - obj.origin_pos.y + obj.size.y - 1 + line;
                //eprintln!("obj: {:#?}", obj);
                //eprintln!("str_x: {}\nstr_y: {}", str_x, str_y);
                let cur_char = obj.string.lines().nth(str_y.try_into().unwrap()).unwrap().chars().nth(str_x.try_into().unwrap()).unwrap();

                if cur_char == ' ' {
                    cvecview.remove(cvecview.clone().iter().position(|a| a==obj).unwrap());
                    if cvecview.iter().max_by_key(|&a| a.origin_pos.y).is_some() {
                        obj = cvecview.iter().max_by_key(|&a| a.origin_pos.y).unwrap();
                        continue;
                    }
                }
                //eprintln!("cur_char: {:#?}", cur_char);
                view.push(cur_char);
                draw = true;
                break;
            }
            }
        
            if !draw {
                view.push(' ');
            }
        }
        view.push('\n');
    }
    
    view
}

fn update_view(cols: i8, lines: i8, vecview: &mut Vec<Object>) {
    let mut rng = rand::thread_rng();

    for (i, obj) in vecview.clone().iter().enumerate().filter(|(_, obj)| obj.origin_pos.x > cols) {
        vecview.remove(i);
        let y = rng.gen_range(10, lines);
        vecview.push(Object::new(TREE[rng.gen_range(0, TREE.len())]).y(y).cycles_to_move((lines - y) as u8 /2));
    }

    for obj in vecview.iter_mut() {
        if obj.cycles == obj.move_cycles {
            obj.origin_pos.x += 1;
            obj.cycles = 0;
        } else {
            obj.cycles += 1;
        }
    }

    if vecview.iter().filter(|obj| obj.origin_pos.x <= trail_space - 1).next().is_none() {
        // spawn new 
    }
}

fn update_road(cols: i8, vecroad: &mut Vec<Object>) {
    for (i, obj) in vecroad.clone().iter().enumerate().filter(|(_, obj)| obj.origin_pos.x > cols) {
        vecroad.remove(i);
    }

    for obj in vecroad.iter_mut() {
            obj.origin_pos.x += 1;
    }

    if vecroad.iter().filter(|obj| obj.origin_pos.x <= trail_space - 1).next().is_none() {
        vecroad.push(Object::new(LINEBLOCK));
    }
    //DEBUG: println!("{:#?}", vecroad);
}

fn road(cols: i8, vecroad: &Vec<Object>) -> String {
    let mut road = String::new();
    road.push_str(&char_line('-', cols));
    road.push('\n'); //road.push_str(&char_line(' ', cols));
    for i in 0..=2 {
        let mut col = 0;
        while col < cols {
            for obj in vecroad {
                if obj.origin_pos.x == col || (col == 0 && obj.origin_pos.x < 0 && obj.origin_pos.x + obj.size.x > 0) {
                    let start: usize = std::cmp::max(0, -obj.origin_pos.x).try_into().unwrap();
                    let end: usize = std::cmp::min(obj.size.x, cols - col).try_into().unwrap();
                    if end > start {
                        road.push_str(&obj.string.lines().into_iter().nth(i).unwrap()[start..end]);
                        col += (end - start) as i8;
                        break;
                    }
                }
            }
            if col < cols {
                road.push(' ');
                col += 1;
            }
        }
        road.push('\n');
    }
    road
}

fn char_line(ch: char, cols: i8) -> String {
    let mut result = String::new();
    for _ in 0..cols {
        result.push(ch);
    }
    result.push('\n');
    result
}
