use reqwest;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;
use serde_json::Value;

#[derive(Clone)]
pub struct Http {
	client: Client
}
impl Http {
	pub fn new( user_agent: String ) -> Http {
		return Http {
			client: Http::get_client( user_agent.clone() )
		};
	}

	fn get_client( user_agent: String ) -> Client {
		return reqwest::blocking::ClientBuilder::new().user_agent( user_agent ).gzip( true ).cookie_store( true ).build().unwrap();
	}

	pub fn get( &self, url: String, mut params: HashMap<&str, &str> ) -> Value {
		params.insert( "format", "json" );

		let request = self.client.get( url ).query( &params ).timeout( Duration::new( 5, 0 ) );
		let response = request.send().unwrap();
		if response.status().is_success() {
			return response.json::<Value>().unwrap();
		}
		return Value::String( String::new() );
	}

	pub fn post( &self, url: String, mut params: HashMap<&str, &str> ) -> Value {
		params.insert( "format", "json" );

		let request = self.client.post( url ).form( &params ).timeout( Duration::new( 5, 0 ) );
		let response = request.send().unwrap();
		if response.status().is_success() {
			return response.json::<Value>().unwrap();
		}
		return Value::String( String::new() );
	}
}
