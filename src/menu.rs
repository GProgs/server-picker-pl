use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
pub struct Menu {
    addrs: Vec<String>,
    names: Vec<String>,
    default_ind: usize,
}

impl Menu {

    pub fn new(addrs: Vec<String>, names: Vec<String>, default_ind: usize) -> Self {
        Self { addrs, names, default_ind }
    }

    pub fn show(self) -> Option<String> {
        // Problem is that deserialization can build "incorrect" Menu
        // with arrays having different lengths
        assert_eq!(self.addrs.len(),self.names.len(),"Array lengths inconsistent");
        assert!(self.addrs.len() > 0, "Array lengths zero");

        println!("Choose 0..{}",self.addrs.len());
        for (i,label) in self.names.iter().enumerate() {
            println!("{} - {}",i,label);
        }

        let mut sel_raw: String = String::new();
        let read_res = io::stdin().read_line(&mut sel_raw);
        if let Err(e) = read_res {
            println!("Cannot read stdin. {}",e);
            Menu::pause();
            return None;
        }
        let sel = sel_raw.trim();
        let maybe_sel = sel.parse::<usize>();

        // None w/out anything on stdout means that the input value was out of index
        let sel_addr: Option<&String> = match (sel,maybe_sel) {
            (s,_) if s.len() == 0 => self.addrs.get(self.default_ind), // Assume Enter to be the default choice
            (_,Ok(ind)) => self.addrs.get(ind), // Some/None
            (_,Err(e)) => { // Parsing failed
                println!("Invalid input. {}",e);
                None
            },
        };

        return sel_addr.cloned();
    }

    pub fn pause() -> () {
        println!("Press ENTER to continue");
        let mut _str = String::new();
        let _ = std::io::stdin().read_line(&mut _str);
    }

}
