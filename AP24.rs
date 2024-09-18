//TODO: How tf do I make static classes and attributes?
//TODO: File output
struct Output {}
impl Output {
    pub fn error(err:&str) {
        println!("{}", err);
        std::process::exit(0);
    }

    pub fn info(info:&str) {
        println!("{}", info);
    }
}

const TEST_INPUT:&str = "
//**********************************************
//Example 1
//**********************************************
Dimension 3,3,2
A 1,2,1
B 2,0,1
C 2,1,0
D 2,0,0
E 2,2,1
F 0,1,2
";

#[derive(Clone, PartialEq, Debug)]
enum Encoding {
    Hole,
    Up,
    Down,
    UpDown,
    Empty,
}

#[derive(Clone, Debug)]
struct Piece {
    encoding: Vec<Encoding>,
    label: char,
    permutations: Option<Vec<Piece>>
}

impl Piece {
    pub fn new(enc: Vec<Encoding>, lb: char) -> Self {
        Piece {
            encoding: enc,
            label: lb,
            permutations: None
        }
    }

    pub fn get_label(&self) -> char {
        self.label
    }

    pub fn get_encoding(&self) -> &Vec<Encoding> {
        &self.encoding
    }

    pub fn get_permutations(&mut self) -> Vec<Piece> {
        if self.permutations.is_none() {
            let mut permutations = vec![self.clone()];
            let u = self.turn_u();
            let r = self.turn_r();
            let ru = self.turn_ru();
            let mut should_push_u = false;
            let mut should_push_r = false;
            let mut should_push_ru = false;
            if !self.is_same(&u) {
                should_push_u = true;
            }
            if !self.is_same(&r) && !u.is_same(&r) {
                should_push_r = true;
            }
            if !self.is_same(&ru) && !r.is_same(&ru) && !u.is_same(&ru) {
                should_push_ru = true;
            }
            if should_push_u {
                permutations.push(u);
            }
            if should_push_r {
                permutations.push(r);
            }
            if should_push_ru {
                permutations.push(ru);
            }
            self.permutations = Some(permutations);
        }
        self.permutations.clone().unwrap()
    }

    fn is_same(&self, piece: &Piece) -> bool {
        if piece.label != self.label {
            return false;
        }
        if piece.encoding.len() != self.encoding.len() {
            return false;
        }
        for i in 0..self.encoding.len() {
            if piece.encoding[i] != self.encoding[i] {
                return false;
            }
        }
        true
    }

    fn turn_ru(&self) -> Piece {
        self.turn_u().turn_r()
    }

    fn turn_r(&self) -> Piece {
        let mut code = self.encoding.clone();
        code.reverse();
        Piece::new(code, self.label)
    }

    fn turn_u(&self) -> Piece {
        let mut code = self.encoding.clone();
        for i in 0..code.len() {
            code[i] = match self.encoding[i] {
                Encoding::Up => Encoding::Down,
                Encoding::Down => Encoding::Up,
                _ => self.encoding[i].clone()
            };
        }
        Piece::new(code, self.label)
    }
}

#[derive(Clone, Debug)]
struct File {
    height:i32,
    width:i32,
    comments:Vec<String>,
    pieces:Vec<Piece>
}

//TODO: Actually read file
impl File {
    pub fn new(input:String) -> Self {
        let mut f = File {
            height: 0,
            width: 0,
            comments: Vec::new(),
            pieces: Vec::new()
        };
        let mut lines = Vec::new();
        let mut num_pieces = 0;

        for temp in input.split("\n") {
            if temp != "" {
                lines.push(temp);
            }
        }

        let mut line_number = 0;
        let mut found_dimension = false;
        for line in lines {
            line_number += 1;
            
            if line.starts_with("//") {
                f.comments.push(line.to_string());
                continue;
            }

            if line.starts_with("Dimension ") {
                let dim = f.read_dimension(line, line_number);
                f.width = dim[0];
                f.height = dim[1];
                found_dimension = true;
                continue;
            }

            f.pieces.push(f.read_piece(line, line_number));
            num_pieces += 1;
        }

        if !found_dimension {
            //TODO: Throw exception instead of killing the program
            Output::error("No dimension declared");
        }

        if num_pieces > f.width * f.height {
            Output::error("Too many puzzle pieces");
        }

        if num_pieces <= 0 {
            Output::error("No puzzle pieces");
        }

        f
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_comments(&self) -> &Vec<String> {
        &self.comments
    }

    pub fn get_pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    fn read_dimension(&self, line:&str, line_number:i32) -> Vec<i32> {
        let mut result:Vec<i32> = Vec::new();
        let strline = line.to_string();
        let dim:Vec<&str> = strline[10..].split(",").collect();
        result.push(dim[0].to_string().parse().unwrap());
        let width:i32 = dim[1].to_string().parse().unwrap();
        if result[0] != width {
            Output::error(format!("Width {} and length {} doesn't match in line {}", result[0], width, line_number).as_str());
        }
        result.push(dim[2].to_string().parse().unwrap());
        result
    }

    fn read_piece(&self, line:&str, line_number:i32) -> Piece {
        let strline = line.to_string();
        let data:Vec<&str> = strline.split(" ").collect();
        let label:char = data[0].chars().nth(0).unwrap();
        if !label.is_ascii_uppercase() || !label.is_ascii_alphabetic() {
            Output::error(format!("Label {} is not valid in line {}", label, line_number).as_str());
        }
        let temp = data[1].to_string();
        let encodings:Vec<&str> = temp.split(",").collect();
        if encodings.len() as i32 != self.width && self.width != 0 {
            Output::error(format!("Invalid number of encodings in line {}", line_number).as_str());
        }
        let mut codes:Vec<Encoding> = Vec::new();
        for code in &encodings {
            let enc = code.to_string().parse().unwrap();
            if enc < 0 || enc > 4 {
                Output::error(format!("Encoding {} in line {} is invalid", enc, line_number).as_str());
            }
            //TODO: Can I cast this?
            codes.push(match enc {
                0 => Encoding::Hole,
                1 => Encoding::Up,
                2 => Encoding::Down,
                3 => Encoding::UpDown,
                _ => Encoding::Empty
            });
        }
        Piece::new(codes, label)
    }
}

#[derive(Clone, Debug)]
struct Layer {
    number:i32,
    data:Vec<Vec<Encoding>>,
    pieces:Vec<char>
}

impl Layer {
    pub fn new(width:i32, num:i32) -> Self {
        let mut l = Layer {
            number: num,
            data: Vec::new(),
            pieces: Vec::new()
        };
        //TODO: Can I create a n x n matrix in a more elegant way?
        for i in 0..width {
            l.pieces.push('\0');
            l.data.push(Vec::new());
            for _j in 0..width {
                l.data[i as usize].push(Encoding::Hole);
            }
        }
        l
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }

    pub fn is_turned(&self) -> bool {
        self.number % 2 == 0
    }

    pub fn get_pos(&self, x:usize, y:usize) -> &Encoding {
        &self.data[x][y]
    }

    pub fn insert(&mut self, piece:Piece, pos:usize) {
        let encoding = piece.get_encoding();
        for i in 0..encoding.len() {
            if self.is_turned() {
                self.data[pos][i] = encoding[i].clone();
            }
            else {
                self.data[i][pos] = encoding[i].clone();
            }
        }
        self.pieces[pos] = piece.get_label();
    }

    pub fn get_piece(&self, pos:usize) -> char {
        self.pieces[pos]
    }
}

#[derive(Clone, Debug)]
struct Box {
    layers:Vec<Layer>,
    width:i32
}

impl Box {
    pub fn new(w:i32, h:i32) -> Self {
        let mut b = Box {
            layers: Vec::new(),
            width: w
        };
        for i in 0..h {
            b.layers.push(Layer::new(w, i + 1));
        }
        b
    }

    pub fn is_allowed(&self, piece:Piece, layer:usize, pos:usize) -> bool {
        if layer == 0 {
            return true;
        }
        let encoding = piece.get_encoding();
        let turned = self.layers[layer].is_turned();
        for i in 0..encoding.len() {
            let enc = &encoding[i];
            let mut enclower = self.layers[layer - 1].get_pos(i, pos);
            if turned {
                enclower = self.layers[layer - 1].get_pos(pos, i);
            }
            let mut enclowest = &Encoding::Hole;
            if layer != 1 {
                enclowest = self.layers[layer - 2].get_pos(i, pos);
                if turned {
                    enclowest = self.layers[layer - 2].get_pos(pos, i);
                }
            }
            if !self.can_be_placed(&enc, enclower, enclowest) {
                return false;
            }
        }
        true
    }

    pub fn insert(&mut self, piece:Piece, layer:usize, pos:usize) {
        self.layers[layer].insert(piece, pos);
    }

    pub fn get_layers(&mut self) -> &mut Vec<Layer> {
        &mut self.layers
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    fn can_be_placed(&self, upper:&Encoding, lower:&Encoding, lowest:&Encoding) -> bool {
        match upper {
            //TODO: Use same code for same cases?
            Encoding::Up => {
                return *lower != Encoding::Up && *lower != Encoding::UpDown;
            },
            Encoding::Empty => {
                return *lower != Encoding::Up && *lower != Encoding::UpDown;
            },
            Encoding::Down => {
                return *lower == Encoding::Hole && *lowest != Encoding::Up && *lowest != Encoding::UpDown;
            },
            Encoding::UpDown => {
                return *lower == Encoding::Hole && *lowest != Encoding::Up && *lowest != Encoding::UpDown;
            },
            _ => { return true; }
        }
    }
}

#[derive(Clone, Debug)]
struct Solver {
    pieces:Vec<Piece>,
    fullcontainer:Option<Box>
}

impl Solver {
    pub fn new(b:&mut Box, p:&Vec<Piece>) -> Self {
        let mut s = Solver {
            pieces: p.clone(),
            fullcontainer: None
        };
        if !s.solve(0, b) {
            Output::error("There is no valid soluation for the puzzle");
        }
        s.fullcontainer = Some(b.clone());
        s
    }

    fn solve(&mut self, piece:usize, container:&mut Box) -> bool {
        let mut copy = self.clone();
        if piece > 0 {
            copy.remove_piece(piece - 1, container);
        }
        if copy.pieces.len() == 0 {
            return true;
        }
        for i in 0..copy.pieces.len() {
            let permutations = copy.pieces[i].get_permutations();
            for j in 0..permutations.len() {
                let layer = self.get_layer(piece, container);
                let pos = self.get_pos(piece, container);
                if container.is_allowed(permutations[j].clone(), layer, pos) {
                    container.insert(permutations[j].clone(), layer, pos);
                    if copy.solve(piece + 1, container) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_layer(&self, piece:usize, container:&Box) -> usize {
        (piece as i32 / container.get_width()) as usize
    }

    fn get_pos(&self, piece:usize, container:&Box) -> usize {
        (piece as i32 % container.get_width()) as usize
    }

    fn remove_piece(&mut self, piece:usize, container:&mut Box) {
        let layer = self.get_layer(piece, container);
        let pos = self.get_pos(piece, container);
        let label = container.get_layers()[layer].get_piece(pos);
        for i in 0..self.pieces.len() {
            if self.pieces[i].get_label() == label {
                self.pieces.remove(i);
                break;
            }
        }
    }

    pub fn print_solution(&mut self) {
        let mut container = match &self.fullcontainer {
            Some(cont) => cont.clone(),
            None => { return; }
        };
        let width = container.get_width();
        let height = container.get_layers().len();
        Output::info(format!("Dimension {},{},{}", width, width, height).as_str());
        Output::info("Piece placement");
        let layers:&mut Vec<Layer> = container.get_layers();
        layers.reverse();
        for e in layers {
            Output::info(format!("Layer {}", e.get_number()).as_str());
            if e.is_turned() {
                let mut line;
                for i in 0..width {
                    line = "".to_string();
                    for j in 0..width {
                        //TODO: Can I cast an Encoding entry to an integer?
                        line = line + " " + match e.get_pos(i as usize, j as usize) {
                            Encoding::Up => "1",
                            Encoding::Down => "2",
                            Encoding::UpDown => "3",
                            Encoding::Empty => "4",
                            _ => "0"
                        };
                    }
                    line = line + " " + format!("{}", e.get_piece(i as usize)).as_str();
                    Output::info(&line[1..]);
                }
            }
            else {
                let mut line;
                for i in 0..width {
                    line = "".to_string();
                    for j in 0..width {
                        //TODO: Can I cast an Encoding entry to an integer?
                        line = line + " " + match e.get_pos(i as usize, j as usize) {
                            Encoding::Up => "1",
                            Encoding::Down => "2",
                            Encoding::UpDown => "3",
                            Encoding::Empty => "4",
                            _ => "0"
                        };
                    }
                    Output::info(&line[1..]);
                }
                line = "".to_string();
                for i in 0..width {
                    line = line + " " + format!("{}", e.get_piece(i as usize)).as_str();
                }
                Output::info(&line[1..]);
            }
            if e.get_number() != 1 {
                Output::info("");
            }
        }
    }
}

fn main() {
    let input = File::new(TEST_INPUT.to_string());
    let mut container = Box::new(input.get_width(), input.get_height());
    let mut solver = Solver::new(&mut container, input.get_pieces());
    for c in input.get_comments() {
        Output::info(c.as_str());
    }
    solver.print_solution();
}
