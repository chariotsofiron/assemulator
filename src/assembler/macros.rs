#[derive(Debug, Clone)]
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

    pub fn expand(&self, values: &[&str]) -> Vec<String> {
        let mut instructions = Vec::new();
        for instruction in &self.instructions {
            let mut instruction = instruction.clone();
            for (from, to) in self.args.iter().zip(values) {
                instruction = instruction.replace(&format!("${from}"), to);
            }
            instructions.push(instruction);
        }
        instructions
    }
}