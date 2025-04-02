use crate::fwikibot::util::http::Http;
use std::collections::HashMap;

pub struct Page {
	http_client: Http,
	api: String,
	name: String,
	text: String,
	loaded: bool,
}
impl Page {
	pub fn new( http_client: Http, api: String, name: String ) -> Page {
		return Page {
			http_client: http_client,
			api: api,
			name: name,
			text: String::new(),
			loaded: false
		};
	}

	pub fn get_name( &self ) -> String {
		return self.name.clone();
	}

	pub fn get_text( &mut self ) -> String {
		if !self.loaded {
			let mut params = HashMap::new();
			params.insert( "action", "query" );
			params.insert( "prop","revisions" );
			params.insert( "titles", self.name.as_str() );
			params.insert( "rvslots", "*" );
			params.insert( "rvprop", "content" );
			params.insert( "formatversion", "2" );
			let json = self.http_client.get( self.api.clone(), params );
			self.text = json[ "query" ][ "pages" ][ 0 ][ "revisions" ][ 0 ][ "slots" ][ "main" ][ "content" ].as_str().unwrap().to_string();
			self.loaded = true;
		}
		return self.text.clone();
	}

	pub fn set_text( &mut self, text: String ) {
		self.text = text;
	}

	pub fn save( &mut self, summary: String ) {
		let mut token_params = HashMap::new();
		token_params.insert( "action", "query" );
		token_params.insert( "meta", "tokens" );
		let token_json = self.http_client.get( self.api.clone(), token_params );
		let edit_token = token_json[ "query" ][ "tokens" ][ "csrftoken" ].clone();

		let mut edit_params = HashMap::new();
		edit_params.insert( "action", "edit" );
		edit_params.insert( "title", self.name.as_str() );
		edit_params.insert( "text", self.text.as_str() );
		edit_params.insert( "summary", summary.as_str() );
		edit_params.insert( "bot", "true" );
		edit_params.insert( "recreate", "true" );
		edit_params.insert( "token", edit_token.as_str().unwrap() );
		let edit_json = self.http_client.post( self.api.clone(), edit_params );
		if edit_json[ "edit" ][ "result" ].as_str().unwrap() == "Success" {
			println!( "...success" );
		} else {
			println!( "...failed" );
		}
	}
}
