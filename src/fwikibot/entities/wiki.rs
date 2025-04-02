use crate::fwikibot::entities::page::Page;
use crate::fwikibot::util::http::Http;
use crate::fwikibot::util::secrets::Secrets;
use std::collections::HashMap;

pub struct Wiki {
	http_client: Http,
	api: String,
	loggedin: bool,
}
impl Wiki {
	pub fn new( http_client: Http, api: String ) -> Wiki {
		return Wiki {
			http_client: http_client,
			api: api,
			loggedin: false
		};
	}
	pub fn login( &mut self, secrets: Secrets ) {
		let user = secrets.get( String::from( "user" ) );
		let password = secrets.get( String::from( "password" ) );
		if !self.loggedin {
			let mut token_params = HashMap::new();
			token_params.insert( "action", "query" );
			token_params.insert( "meta", "tokens" );
			token_params.insert( "type", "login" );
			let token_json = self.http_client.get( self.api.clone(), token_params );
			let login_token = token_json[ "query" ][ "tokens" ][ "logintoken" ].clone();

			let mut login_params = HashMap::new();
			login_params.insert( "action", "login" );
			login_params.insert( "lgname", user.as_str() );
			login_params.insert( "lgpassword", password.as_str() );
			login_params.insert( "lgtoken", login_token.as_str().unwrap() );
			self.http_client.post( self.api.clone(), login_params );

			self.loggedin = true;
		}
	}

	pub fn get_page( &mut self, page_name: String ) -> Page {
		let page = Page::new( self.http_client.clone(), self.get_api(), page_name );
		return page;
	}

	pub fn get_api( &self ) -> String {
		return self.api.clone();
	}
}
