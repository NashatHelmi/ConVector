use color_eyre::eyre::Result;

pub struct Repl<'a> {
    on_init: &'a dyn Fn() -> Result<()>,
    on_update: &'a dyn Fn(&mut Self, String) -> Result<()>,
    _is_running: bool,
}

impl<'a> Repl<'a> {
    pub fn new(
        on_init: &'a impl Fn() -> Result<()>,
        on_update: &'a impl Fn(&mut Self, String) -> Result<()>,
    ) -> Self {
        Repl {
            on_init,
            on_update,
            _is_running: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self._on_init()?;

        while self._is_running {
            self._on_update()?;
        }

        Ok(())
    }

    fn _on_init(&mut self) -> Result<()> {
        (self.on_init)()?;
        self._is_running = true;
        Ok(())
    }

    fn _on_update(&mut self) -> Result<()> {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // _on_exit ()
        if input.trim() == "exit" {
            println!("Goodbye!");
            self._is_running = false;
            return Ok(());
        }

        (self.on_update)(self, input)?;
        Ok(())
    }
}
