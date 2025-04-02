use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct Secrets {
	secrets: HashMap<String, String>
}
impl Secrets {
	pub fn new() -> Secrets {
		return Secrets {
			secrets: Secrets::get_secrets()
		};
	}

	fn get_secrets() -> HashMap<String, String> {
		let secrets_json = Secrets::read_secrets_file().unwrap();

		let mut secrets = HashMap::new();
		secrets.insert( String::from( "user" ), String::from( secrets_json[ "user" ].as_str().unwrap() ) );
		secrets.insert( String::from( "password" ), String::from( secrets_json[ "password" ].as_str().unwrap() ) );
		return secrets;
	}
	
	fn read_secrets_file() -> Result<serde_json::Value, Box<dyn Error>> {
		let file = File::open( "./secrets.json" )?;
		let reader = BufReader::new( file );
		let secrets_json: serde_json::Value = serde_json::from_reader( reader )?;
		return Ok( secrets_json );
	}

	pub fn get( &self, key: String ) -> String {
		return self.secrets.get( &key ).unwrap().clone();
	}
}
