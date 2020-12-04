#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(irrefutable_let_patterns)]
// #![allow(unused_mut)]
// #![allow(unused_assignments)]




// Import Modules Here




// Use Statements Here
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
// use std::io::{stdin, stdout, Write};
use std::env;




#[derive(Debug)]
struct Runtime_Environment{
    Functions: HashMap<String, Environments::Function>,
    Variables: HashMap<String, Environments::Variable>
}

impl Runtime_Environment{

    pub fn new() -> Self{
        return Runtime_Environment{
            Functions: HashMap::new(),
            Variables: HashMap::new()
        }
    }
}

impl Runtime_Environment{

    pub fn Add_Function(&mut self, _Function: Environments::Function) -> Option<Environments::Function>{
        return self.Functions.insert(_Function.Name.clone(), _Function);
    }

    pub fn Add_Variable(&mut self, _Variable: Environments::Variable) -> Option<Environments::Variable>{
        return self.Variables.insert(_Variable.Name.clone(), _Variable);
    }
}




mod Environments{

    use std::collections::HashMap;
    use std::io::{stdin, stdout, Write};



    #[derive(Debug, PartialEq, Clone)]
    pub enum SPECIAL_STOP{
        CONTINUE,
        BREAK,
        RETURN(Option<String>)
    }



    #[derive(Debug, PartialEq, Clone)]
    pub enum TYPE{
        VOID,
        BOOL,
        INT,
        DOUBLE,
        CHAR,
        STRING
    }


    pub fn Get_Type(Type: &String) -> TYPE{

        if Type.eq("VOID"){
            return TYPE::VOID
        }

        else if Type.eq("BOOL"){
            return TYPE::BOOL
        }

        else if Type.eq("INT"){
            return TYPE::INT
        }

        else if Type.eq("DOUBLE"){
            return TYPE::DOUBLE
        }

        else if Type.eq("CHAR"){
            return TYPE::CHAR
        }

        else{
            return TYPE::STRING
        }
    }


    

    #[derive(Debug)]
    pub struct Function{
        pub Name: String,
        pub Definitions: HashMap<String, Definition>
    }

    impl Function{

        pub fn new() -> Self{
            return Function{
                Name: String::new(),
                Definitions: HashMap::new()
            }
        }
    }




    #[derive(Debug)]
    pub struct Definition{
        pub Name: String,
        pub Return_Type: TYPE,
        pub Params: Vec<Param>,
        pub Statements: Vec<Vec<String>>
    }

    impl Definition{

        pub fn new() -> Self{
            return Definition{
                Name: String::new(),
                Return_Type: TYPE::VOID,
                Params: Vec::new(),
                Statements: Vec::new()
            }
        }
    }




    #[derive(Debug)]
    pub struct Param{
        pub Name: String,
        pub Type: TYPE
    }

    impl Param{

        pub fn new() -> Self{
            return Param{
                Name: String::new(),
                Type: TYPE::VOID
            }
        }
    }




    #[derive(Debug, PartialEq, Clone)]
    pub struct Variable{
        pub Name: String,
        pub Type: TYPE,
        pub Value: String,
        pub Number_Value: f32
    }

    impl Variable{

        pub fn new(Type: TYPE, Name: String, Value: String) -> Self{
            return Variable{
                Number_Value: if Type == TYPE::BOOL || Type == TYPE::INT || Type == TYPE::DOUBLE{
                    Value.parse::<f32>().unwrap()
                } else {0.0},
                Value,
                Name,
                Type
            }
        }
    }

    impl Variable{

        pub fn Print(&self) -> String{

            if self.Type == TYPE::BOOL{
                if self.Number_Value == 0.0{
                    return String::from("False");
                }
                else{
                    return String::from("True");
                }
            }

            else if self.Type == TYPE::INT{
                return (self.Number_Value as i32).to_string()
            }

            else if self.Type == TYPE::DOUBLE{
                return self.Number_Value.to_string()
            }

            else if self.Type == TYPE::CHAR{
                return self.Value.clone()
            }

            else{
                
                let mut string = String::new();
                let mut Special_Character = false;

                for Char in self.Value.chars(){
                    
                    if Special_Character{

                        if Char == 'n'{
                            string.push_str("\n");
                        }
                        else{
                            string.push(Char);
                        }
                        Special_Character = false;
                    }
                    else{
                        if Char == '\\'{
                            Special_Character = true;
                        }
                        else{
                            string.push(Char);
                        }
                    }
                }

                return string;
            }
        }

        pub fn Input(&mut self){

            match stdout().flush(){
                _ => ()
            }

            if self.Type == TYPE::BOOL{
                let mut Line = String::new();
    
                loop{
                    stdin().read_line(&mut Line).expect("Can't Read Input Line");
    
                    if Line.eq("\r\n"){
                        Line.clear();
                        continue
                    }
    
                    else if Line.eq("0\r\n"){
                        self.Value = String::from("0");
                        self.Number_Value = 0.0;
                        break;
                    }
    
                    else if Line.eq("1\r\n"){
                        self.Value = String::from("1");
                        self.Number_Value = 1.0;
                        break;
                    }
    
                    else{
                        panic!(format!("Input Value `{}` Of Not Type Of Bool {{0, 1}}", Line));
                    }
                }
            }

            else if self.Type == TYPE::INT || self.Type == TYPE::DOUBLE{
                let mut Line = String::new();
    
                loop{
                    stdin().read_line(&mut Line).expect("Can't Read Input Line");
    
                    if Line.eq("\r\n"){
                        Line.clear();
                        continue
                    }
    
                    let Line = String::from(Line.split_at(Line.len()-2).0);
    
                    match Line.parse::<f32>(){
                        
                        Err(Error) => panic!("{}", Error),
    
                        Ok(Number) => {
                            self.Value = Line.clone();
                            self.Number_Value = Number;
                        }
                    }
                    break;
                }
            }

            else if self.Type == TYPE::CHAR{
                let mut Line = String::new();
    
                loop{
                    stdin().read_line(&mut Line).expect("Can't Read Input Line");
    
                    if Line.eq("\r\n"){
                        Line.clear();
                        continue
                    }
    
                    let Line = String::from(Line.split_at(Line.len()-2).0);
    
                    let Value: u8 = Line.as_bytes()[0];
    
                    if Value > 127{
                        panic!(format!("Input Value `{}` Of Not Type Of CHAR", Line));
                    }
    
                    self.Value = String::from(Value as char);
    
                    break;
                }
            }
    
            else if self.Type == TYPE::STRING{
                let mut Line = String::new();
    
                loop{
                    stdin().read_line(&mut Line).expect("Can't Read Input Line");
    
                    if Line.eq("\r\n"){
                        Line.clear();
                        continue
                    }
    
                    let Line = String::from(Line.split_at(Line.len()-2).0);
    
                    self.Value = Line;
    
                    break;
                }
            }
        }

        pub fn Change_Value(&mut self, Value: String){
            self.Value = Value;

            self.Number_Value = if self.Type == TYPE::BOOL || self.Type == TYPE::INT || self.Type == TYPE::DOUBLE{
                self.Value.parse::<f32>().unwrap()
            } else { self.Number_Value };
        }

        pub fn Number(&self) -> f32{

            if self.Type == TYPE::BOOL{
                return self.Number_Value.floor();
            }

            else if self.Type == TYPE::INT{
                return self.Number_Value.floor();
            }

            else if self.Type == TYPE::DOUBLE{
                return self.Number_Value
            }

            else{
                panic!("Get Number Of String");
            }
        }
    }
}





fn main(){

    /*let File_Path = String::from(
        "E:/My Projects/Engine Programming Language/Engine Compiler/compiler_in_rust/init.vir"
    );*/

    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        panic!("Please Enter File Path Only");
    }
    
    let mut _File = match File::open(&args[1]){

        Err(_) => panic!("Error in Opening The File"),

        Ok(Opened_File) => Opened_File
    };


    let mut Reader = BufReader::new(_File);

    let mut _Environment = Runtime_Environment::new();

    loop{
        let Line = Get_Line(&mut Reader);

        if Line == None{
            break
        }

        let Line = Line.unwrap();

        if Line[0] == "fn"{

            let mut _Function = Environments::Function::new();
            _Function.Name = Line[1].clone();

            for _i in 0..Line[2].parse::<i32>().unwrap(){

                let _Definition_Line = Get_Line(&mut Reader).unwrap();
                
                let mut _Definition = Environments::Definition::new();
                _Definition.Name = _Definition_Line[1].clone();
                _Definition.Return_Type = Environments::Get_Type( &_Definition_Line[2] );

                for _j in 0.._Definition_Line[3].parse::<i32>().unwrap(){

                    let _Param_Line = Get_Line(&mut Reader).unwrap();
                    
                    let mut _Param = Environments::Param::new();
                    _Param.Name = _Param_Line[1].clone();
                    _Param.Type = Environments::Get_Type(&_Param_Line[2]);

                    _Definition.Params.push(_Param);
                }

                Get_Line(&mut Reader); // Start Definition Statements
                
                loop{

                    let De_Statements_Line = Get_Line(&mut Reader).unwrap();
                    
                    if De_Statements_Line[0] == "end_de_stmts"{ // End Definition Statements
                        break;
                    }

                    _Definition.Statements.push(De_Statements_Line);
                }

                _Function.Definitions.insert(_Definition.Name.clone(), _Definition);
            }

            _Environment.Add_Function(_Function);
        }

        else{
            Execute_Command(&mut _Environment, &Line, &mut Reader);
        }
    }

    //println!("{:#?}", _Environment.Variables);
}


fn Get_Line(Reader: &mut BufReader<File>) -> Option<Vec<String>>{

    match Reader.lines().next(){

        None => None,

        Some(Line) => {
            
            match Line{

                Err(_) => panic!("Error in Reading Line"),

                Ok(_Line) => {
                    let _Line: Vec<&str> = _Line.split_terminator(':').collect::<Vec<&str>>();
                    let _Line: Vec<String> = _Line.iter().map(|S| S.to_string()).collect();
                    return Some( _Line );
                }
            }
        }
    }
}


fn Execute_Command(_Environment: &mut Runtime_Environment, Command: &Vec<String>, Reader: &mut BufReader<File>) -> Option<Environments::SPECIAL_STOP>{

    let mut _Environment = _Environment;
    let mut Reader = Reader;

    if Command[0] == "var"{

        if Command[1] == "BOOL"{
            _Environment.Add_Variable(Environments::Variable::new(
                Environments::Get_Type(&Command[1]), Command[2].clone(), Command[3].clone()
            ));
        }

        else if Command[1] == "INT"{
            _Environment.Add_Variable(Environments::Variable::new(
                Environments::Get_Type(&Command[1]), Command[2].clone(), Command[3].clone()
            ));
        }

        else if Command[1] == "DOUBLE"{
            _Environment.Add_Variable(Environments::Variable::new(
                Environments::Get_Type(&Command[1]), Command[2].clone(), Command[3].clone()
            ));
        }

        else if Command[1] == "CHAR"{
            let Value = String::from(&Command[3].as_str()[1..Command[3].len()-1]);
            _Environment.Add_Variable(Environments::Variable::new(
                Environments::Get_Type(&Command[1]), Command[2].clone(), Value
            ));
        }

        else if Command[1] == "STRING"{
            let Value = String::from(&Command[3].as_str()[1..Command[3].len()-1]);
            _Environment.Add_Variable(Environments::Variable::new(
                Environments::Get_Type(&Command[1]), Command[2].clone(), Value
            ));
        }

        return None
    }

    else if Command[0] == "add_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = First_Number + Second_Number;
        Result_Var.Value = Result_Var.Number_Value.to_string();

        return None
    }

    else if Command[0] == "mul_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = First_Number * Command[3].parse::<f32>().unwrap();
        Result_Var.Value = Result_Var.Number_Value.to_string();

        return None
    }

    else if Command[0] == "sub_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = First_Number - Second_Number;
        Result_Var.Value = Result_Var.Number_Value.to_string();

        return None
    }

    else if Command[0] == "mul_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = First_Number * Second_Number;
        Result_Var.Value = Result_Var.Number_Value.to_string();

        return None
    }

    else if Command[0] == "div_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = First_Number / Second_Number;
        Result_Var.Value = Result_Var.Number_Value.to_string();

        return None
    }

    else if Command[0] == "mod_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = First_Number % Second_Number;
        Result_Var.Value = Result_Var.Number_Value.to_string();

        return None
    }

    else if Command[0] == "add_s_v"{
        let First_String = _Environment.Variables.get(&Command[2]).unwrap().Value.clone();
        let Value = String::from(&Command[3].as_str()[1..Command[3].len()-1]);

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Value = First_String + &Value;

        return None
    }

    else if Command[0] == "add_s_v_v"{
        let First_String = _Environment.Variables.get(&Command[2]).unwrap().Value.clone();
        let Second_String = _Environment.Variables.get(&Command[3]).unwrap().Value.clone();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Value = First_String + &Second_String;

        return None
    }

    else if Command[0] == "gt_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number > Second_Number{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "gteq_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number >= Second_Number{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "lt_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number < Second_Number{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "lteq_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number <= Second_Number{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "eq_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number == Second_Number{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "neq_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number != Second_Number{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "and_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number != 0.0 && Second_Number != 0.0{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "or_v_v"{
        let First_Number = _Environment.Variables.get(&Command[2]).unwrap().Number();
        let Second_Number = _Environment.Variables.get(&Command[3]).unwrap().Number();

        let mut Result_Var = _Environment.Variables.get_mut(&Command[1]).unwrap();
        Result_Var.Number_Value = if First_Number != 0.0 || Second_Number != 0.0{1.0} else {0.0};
        Result_Var.Value = Result_Var.Number_Value.to_string();
    }

    else if Command[0] == "call_v"{

        // call_v : result : fun : def : count : ...params
        
        let mut _Old_Variables: Vec<Option<Environments::Variable>> = Vec::new();

        for i in 0..Command[4].parse::<usize>().unwrap(){

            let _Variable_Value = _Environment.Variables.get(&Command[5 + i]).unwrap().Value.clone();

            let _Variable_Type = _Environment.Variables.get(&Command[5 + i]).unwrap().Type.clone();

            let _Variable_Name: String
                = _Environment.Functions.get(&Command[2]).unwrap()
                    .Definitions.get(&Command[3]).unwrap()
                    .Params[i].Name.clone();

            _Old_Variables.push( _Environment.Add_Variable(Environments::Variable::new(
                _Variable_Type,
                _Variable_Name,
                _Variable_Value
            )) );
        }

        let _Statements = _Environment.Functions.get(&Command[2]).unwrap().Definitions.get(&Command[3]).unwrap().Statements.clone();
        let mut _Return_Variable = None;

        // Execute Function Definition Statements
        for Statement in _Statements.iter(){
            match Execute_Command(&mut _Environment, Statement, &mut Reader){
                Some(Environments::SPECIAL_STOP::CONTINUE)|
                Some(Environments::SPECIAL_STOP::BREAK) => (),

                None => (),

                Some(Environments::SPECIAL_STOP::RETURN(Variable)) => {
                    if Variable == None{
                        _Return_Variable = None
                    }
                    else{
                        _Return_Variable = Variable;
                    }
                    break
                }
            }
        }

        if _Return_Variable == None{
            panic!(format!("Return None On Function `{}` On Definition `{}`", Command[2], Command[3]));
        }

        let Value: String = _Environment.Variables.get_mut(&_Return_Variable.unwrap()).unwrap().Value.clone();
        _Environment.Variables.get_mut(&Command[1]).unwrap().Change_Value(Value);

        if _Old_Variables.len() != 0 && _Old_Variables[0] != None{

            for _Variable in _Old_Variables.iter(){
                let _Variable = _Variable.as_ref().unwrap().clone();
                _Environment.Variables.insert(_Variable.Name.clone(), _Variable);
            }
        }

        return None
    }

    else if Command[0] == "call"{
        
        let mut _Old_Variables: Vec<Option<Environments::Variable>> = Vec::new();

        for i in 0..Command[3].parse::<usize>().unwrap(){

            let _Variable_Value = _Environment.Variables.get(&Command[4 + i]).unwrap().Value.clone();

            let _Variable_Type = _Environment.Variables.get(&Command[4 + i]).unwrap().Type.clone();

            let _Variable_Name: String
                = _Environment.Functions.get(&Command[1]).unwrap()
                    .Definitions.get(&Command[2]).unwrap()
                    .Params[i].Name.clone();

            _Old_Variables.push( _Environment.Add_Variable(Environments::Variable::new(
                _Variable_Type,
                _Variable_Name,
                _Variable_Value
            )) );
        }

        let _Statements = _Environment.Functions.get(&Command[1]).unwrap().Definitions.get(&Command[2]).unwrap().Statements.clone();
        let mut _Return_Variable = None;

        // Execute Function Definition Statements
        for Statement in _Statements.iter(){
            match Execute_Command(&mut _Environment, Statement, &mut Reader){
                Some(Environments::SPECIAL_STOP::CONTINUE)|
                Some(Environments::SPECIAL_STOP::BREAK) => (),

                None => (),

                Some(Environments::SPECIAL_STOP::RETURN(Variable)) => {
                    if Variable == None{
                        _Return_Variable = None
                    }
                    else{
                        _Return_Variable = Variable;
                    }
                    break
                }
            }
        }

        if _Old_Variables.len() != 0 && _Old_Variables[0] != None{

            for _Variable in _Old_Variables.iter(){
                let _Variable = _Variable.as_ref().unwrap().clone();
                _Environment.Variables.insert(_Variable.Name.clone(), _Variable);
            }
        }

        return None
    }

    else if Command[0] == "if"{

        //println!("{}", Command[1]);

        let mut Entered_if = false;
        let If_Name: String = Command[1].clone();

        loop{
            let Line = Get_Line(&mut Reader).unwrap();

            //println!("Main:{}:{:?}", Entered_if, Line);

            if Line[0] == "end_if" && Line[1] == If_Name{
                break
            }

            else if !Entered_if{
                
                if Line[0] == "if_exp"{
                    Entered_if = true;

                    let Line = Get_Line(&mut Reader).unwrap();
                    let If_Definition_Name = Line[1].clone();

                    loop{
                        
                        let Line = Get_Line(&mut Reader).unwrap();
                        
                        if Line[0] == "end_if_stmts" && Line[1] == If_Definition_Name{
                            break
                        }

                        // Execute if Statements
                        if Entered_if{
                            match Execute_Command(&mut _Environment, &Line, &mut Reader){
                                Some(SPECIAL_STOP) => return Some(SPECIAL_STOP),
                                None => ()
                            }
                        }
                    }
                }

                else{
                    Execute_Command(&mut _Environment, &Line, &mut Reader);

                    if Line[0] == "var"{
                        //println!("{:?}", _Environment.Variables.get(&Line[2]).unwrap());
                    }

                    loop{
                        let Line = Get_Line(&mut Reader).unwrap();

                        //println!("Def:{}:{:?}", Entered_if, Line);
                        
                        if Line[0] == "if_exp"{

                            //println!("{:?}", _Environment.Variables.get(&Line[1]).unwrap());
    
                            if Line[1] == "true"{
                                Entered_if = true;
                            }
    
                            else{
                                if _Environment.Variables.get(&Line[1]).unwrap().Number() == 1.0{
                                    Entered_if = true;
                                }
                            }
    
                            let Line = Get_Line(&mut Reader).unwrap();
                            //println!("Exe:{}:{:?}", Entered_if, Line);
                            //println!("");
                            let If_Definition_Name = Line[1].clone();
    
                            loop{
                                
                                let Line = Get_Line(&mut Reader).unwrap();
                                
                                if Line[0] == "end_if_stmts" && Line[1] == If_Definition_Name{ // Here
                                    break
                                }
                                
                                // Execute if Statements
                                if Entered_if{
                                    match Execute_Command(&mut _Environment, &Line, &mut Reader){
                                        Some(SPECIAL_STOP) => return Some(SPECIAL_STOP),
                                        None => ()
                                    }
                                }
                            }
    
                            break;
                        }
                        else{
                            Execute_Command(&mut _Environment, &Line, &mut Reader); // Here

                            /*if Line[0] == "var"{
                                println!("{:?}", _Environment.Variables.get(&Line[2]).unwrap());
                            }

                            else if Line[0] == "add_v_v"{
                                println!("{:?}", _Environment.Variables.get(&Line[3]).unwrap());
                                println!("{:?}", _Environment.Variables.get(&Line[2]).unwrap());
                                println!("{:?}", _Environment.Variables.get(&Line[1]).unwrap());
                            }

                            else if Line[0] == "lt_v_v"{
                                println!("{:?}", _Environment.Variables.get(&Line[3]).unwrap());
                                println!("{:?}", _Environment.Variables.get(&Line[2]).unwrap());
                                println!("{:?}", _Environment.Variables.get(&Line[1]).unwrap());
                            }*/
                        }
                    }
                }
            }
        }

        return None
    }

    else if Command[0] == "sw"{

        let Variable_Name: String = Command[1].clone();
        let _Variable_Value = _Environment.Variables.get(&Variable_Name).unwrap().Value.clone();

        let Switch_Name = Command[2].clone();
        
        let mut Entered_Switch = false;

        loop{

            let Line = Get_Line(&mut Reader).unwrap();

            if Line[0] == "end_sw" && Line[1] == Switch_Name{
                break
            }

            else if !Entered_Switch{
                
                if Line[0] == "sw_i"{

                    if Line[1] == _Variable_Value{
                        Entered_Switch = true;
                    }
                }

                else if Line[0] == "sw_c" || Line[0] == "sw_s"{

                    let Value = String::from(&Line[1].as_str()[1..Line[1].len()-1]);
                    if Value == _Variable_Value{
                        Entered_Switch = true;
                    }
                }

                else if Line[0] == "sw_d"{
                    Entered_Switch = true;
                }

                let Line = Get_Line(&mut Reader).unwrap();
                let Switch_Definition_Name = Line[1].clone();

                loop{

                    let Line = Get_Line(&mut Reader).unwrap();
                    if Line[0] == "end_sw_stmts" && Line[1] == Switch_Definition_Name{
                        break
                    }

                    // Execute Switch Statements
                    if Entered_Switch{
                        match Execute_Command(&mut _Environment, &Line, &mut Reader){
                            Some(SPECIAL_STOP) => return Some(SPECIAL_STOP),
                            None => ()
                        }
                    }
                }
            }
        }

        return None
    }

    else if Command[0] == "loop"{

        let Loop_Name = Command[1].clone();
        let mut Loop_init_Variable = String::new();
        
        loop{
            let Line = Get_Line(&mut Reader).unwrap();

            if Line[0] == "loop_init"{

                if Line[1] != "true"{

                    Loop_init_Variable = Line[1].clone();
                
                    let Number_Value = _Environment.Variables.get(&Line[2]).unwrap().Number();
                    let Result_Var = _Environment.Variables.get_mut(&Loop_init_Variable).unwrap();
                    Result_Var.Number_Value = Number_Value;
                    Result_Var.Value = Result_Var.Number_Value.to_string();
                }
                break
            }
            Execute_Command(&mut _Environment, &Line, &mut Reader);
        }

        let mut Loop_Expression: Vec<Vec<String>> = Vec::new();
        let Loop_Expression_Variable: String;
        let mut is_Compare_To_init_Name = false;

        loop{
            let Line = Get_Line(&mut Reader).unwrap();

            if Line[0] == "loop_exp"{
                if Line[1] != "true"{
                    is_Compare_To_init_Name = true;
                }
                Loop_Expression_Variable = Line[2].clone();
                break
            }

            Loop_Expression.push(Line);
        }

        let mut Loop_Step: Vec<Vec<String>> = Vec::new();
        let Loop_Step_Variable: String;

        loop{
            let Line = Get_Line(&mut Reader).unwrap();

            if Line[0] == "loop_step"{
                Loop_Step_Variable = Line[1].clone();
                break
            }

            Loop_Step.push(Line);
        }


        let mut Loop_Statements: Vec<Vec<String>> = Vec::new();
        Get_Line(&mut Reader).unwrap();
        loop{
            let Line = Get_Line(&mut Reader).unwrap();
            if Line[0] == "end_loop_stmts" && Line[1] == Loop_Name{
                break
            }
            Loop_Statements.push(Line);
        }


        loop{
            
            // Compare Expression
            for Expression in Loop_Expression.iter(){
                Execute_Command(&mut _Environment, Expression, &mut Reader);
            }
            
            if Loop_Expression_Variable != "true"{
                let Exp_Value = _Environment.Variables.get(&Loop_Expression_Variable).unwrap().Number();
                if is_Compare_To_init_Name{
                    let Number_Value = _Environment.Variables.get(&Loop_init_Variable).unwrap().Number();
                    if Number_Value == Exp_Value{
                        break
                    }
                }
                else{
                    if Exp_Value < 1.0{
                        break
                    }
                }
            }

            
            // Execute Statements
            for Statement in Loop_Statements.iter(){
                match Execute_Command(&mut _Environment, Statement, &mut Reader){
                    Some(Environments::SPECIAL_STOP::CONTINUE) => break,
                    Some(Environments::SPECIAL_STOP::BREAK) => return None,
                    Some(Environments::SPECIAL_STOP::RETURN(Variable)) => return Some(Environments::SPECIAL_STOP::RETURN(Variable)),

                    None => ()
                }
            }


            // Step Statement
            for Expression in Loop_Step.iter(){
                Execute_Command(&mut _Environment, Expression, &mut Reader);
            }

            if Loop_Step_Variable != "true"{
                let Step_Value = _Environment.Variables.get(&Loop_Step_Variable).unwrap().Number();
                let mut Result_Var = _Environment.Variables.get_mut(&Loop_init_Variable).unwrap();
                Result_Var.Number_Value += Step_Value;
                Result_Var.Value = Result_Var.Number_Value.to_string();
            }
        }

        return None
    }

    else if Command[0] == "con"{
        return Some(Environments::SPECIAL_STOP::CONTINUE);
    }

    else if Command[0] == "br"{
        return Some(Environments::SPECIAL_STOP::BREAK);
    }

    else if Command[0] == "pr"{
        print!("{}", _Environment.Variables.get(&Command[1]).unwrap().Print());
    }

    else if Command[0] == "in"{
        _Environment.Variables.get_mut(&Command[1]).unwrap().Input();
    }

    else if Command[0] == "re"{
        if Command[1] == "none"{
            return Some(Environments::SPECIAL_STOP::RETURN(None))
        }
        return Some(Environments::SPECIAL_STOP::RETURN(Some(Command[1].clone())));
    }

    return None
}
