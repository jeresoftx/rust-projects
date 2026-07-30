//! Máquina de estados para un contenedor educativo.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Created,
    Running,
    Stopped,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container {
    pub image: String,
    pub id: u64,
    pub state: State,
}
impl Container {
    pub fn new(image: &str, id: u64) -> Self {
        Self {
            image: image.into(),
            id,
            state: State::Created,
        }
    }
    pub fn start(&mut self) -> Result<(), String> {
        if self.state != State::Created {
            return Err("transición de inicio inválida".into());
        }
        self.state = State::Running;
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), String> {
        if self.state != State::Running {
            return Err("transición de detención inválida".into());
        }
        self.state = State::Stopped;
        Ok(())
    }
}
