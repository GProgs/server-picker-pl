use peliluola_picker::{Config,Menu};
use countdown::Countdown;
use enigo::{Enigo,Key,
	Key::{Unicode,Return},
	Direction::Click,
	InputError, Keyboard, Settings,
};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{io::Cursor,thread::sleep,time::Duration};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

fn main() -> Result<(),InputError> {
    // Servers here
    fn open_server(serv: String) -> Result<(),std::io::Error> {
        //println!("{}",format!("steam://connect/{}",serv));
        open::that(format!("steam://connect/{}",serv))
    }

    // I CBA to find out what the correct error type should be.
    fn open_audio() -> Result<(OutputStream,OutputStreamHandle,Sink),Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        Ok((stream,stream_handle,sink))
    }

    // Make keyboard go tak-tak-tak.
    fn chat(enigo: &mut Enigo, key_chat: &Key, text: &str, dt: &Duration) -> Result<(),InputError> {
        enigo.key(*key_chat,Click)?;
        sleep(*dt);
        enigo.text(text)?;
	    sleep(*dt);
	    enigo.key(Return,Click)?;
	    sleep(*dt);
        Ok(())
    }

    // Open the config file.
    let maybe_conf = Config::read_file(String::from("peliluola.toml"));
    if let Err(e) = maybe_conf {
        println!("Ei voitu lukea konfiguraatiota. {}",e);
        Menu::pause();
        return Ok(());
    };
    let conf = maybe_conf.unwrap();

    //let dt_launch = Duration::from_secs(30); // +5.8s from sound
    //let dt = Duration::from_millis(250);
    //let keys_mirella: Vec<u8> = vec!(3,1); // what numbers you want to press
	//let key_chat = Unicode('y'); // your chat bind

    // Get input from the user
    // If it's None, tell the guy he messed up
    let Some(addr) = conf.menu.show() else {
        println!("Homma kusi, koita uusiksi!!");
        Menu::pause();
        return Ok(());
    };

    // Print command in the event that joining fails
    println!("Liittymiskomento: \"connect {}\" konsoliin.",&addr);

    // Handle IO errors
    if let Err(e) = open_server(addr.clone()) {
        println!("IO error while opening. {}",e);
        Menu::pause();
        return Ok(());
    };

    { // Audio & system scope

        // Create audio output stream
        let maybe_audio: Result<_,_> = open_audio();
        if let Err(e) = maybe_audio { // guard
            println!("Audio error. {}",e);
            Menu::pause();
            return Ok(());
        };
        let (_stream,_stream_handle,sink) = maybe_audio.unwrap();

        // Load sound into the sink
        let audio_bytes = include_bytes!("faceit.ogg");
        let audio_cursor = Cursor::new(audio_bytes); // add Read and Seek impls
        let maybe_source = Decoder::new(audio_cursor);
        if let Err(e) = maybe_source { // guard
            println!("Creating source failed. {}",e);
            Menu::pause();
            return Ok(());
        }
        let source = maybe_source.unwrap();

        // Get launch delay by seeing whether cs2 launched
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );
        let mut procs = system.processes_by_name("cs2".as_ref()); // get Iterator of processes
        let dt_launch: Duration = match procs.next() { // get launch time
            Some(_) => conf.dt_launch,
            None => conf.dt_launch_nocs2
        };
        // Convert Duration to usize seconds
        let dt_launch_secs = match usize::try_from(dt_launch.as_secs()) {
            Ok(i) => i,
            Err(e) => {
                println!("Could not convert dt_launch to usize. {}",e);
                Menu::pause();
                return Ok(());
            }
        };
        
        // Tell them to join the server
        println!("Liity servulle {} sekunnissa...",dt_launch_secs);
        Countdown::new(dt_launch_secs).start();
        //sleep(dt_launch);

        // Play the sound and block current thread
        sink.append(source);
        sink.sleep_until_end();

    }

    // Start typing out
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let key_chat = Unicode(conf.key_chat);

    chat(&mut enigo, &key_chat, "!mvp", &conf.dt)?;

    /*
    enigo.key(key_chat,Click)?;
	sleep(conf.dt);
	enigo.text("!mvp")?;
	sleep(conf.dt);
	enigo.key(Return,Click)?;
	sleep(conf.dt);
    */

	// Then go thru each key and submit in turn
	for number in conf.keys_mvp.chars() {
        chat(&mut enigo, &key_chat, &(format!("!{}",number)), &conf.dt)?;
        /*
		enigo.key(key_chat,Click)?;
		sleep(conf.dt);
		enigo.text(&(format!("!{}",number)))?;
		sleep(conf.dt);
		enigo.key(Return,Click)?;
		sleep(conf.dt);
        */
	}

    // Finally, enter !1 for Yes.
    chat(&mut enigo, &key_chat, "!1", &conf.dt)?;

    println!("Done!");
    Ok(())
}
