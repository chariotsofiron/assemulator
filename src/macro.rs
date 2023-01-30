

pub struct Macro {
    /// Name of the macro.
    pub name: String,
    /// List of arguments.
    pub args: Vec<String>,
    /// List of instructions.
    pub instructions: Vec<String>,

    pub length: usize,
}


impl Macro {

    pub fn new(name: String, args: Vec<String>, instructions: Vec<String>) -> Self {
        Self {
            name,
            args,
            instructions,
            0,
        }
    }

    pub fn substitute(&self) -> Vec<String> {
        let mut instructions = Vec::new();
        for instruction in &self.instructions {
            let mut instruction = instruction.clone();
            for (i, arg) in self.args.iter().enumerate() {
                instruction = instruction.replace(&format!("${}", i), arg);
            }
            instructions.push(instruction);
        }
        instructions
    }
}