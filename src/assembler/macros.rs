pub struct Macro {
    /// List of arguments.
    pub args: Vec<String>,
    /// List of instructions.
    pub instructions: Vec<String>,
}


impl Macro {

    pub fn new(args: Vec<String>, instructions: Vec<String>) -> Self {
        Self {
            args,
            instructions,
        }
    }

    pub fn expand(&self) -> Vec<String> {
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