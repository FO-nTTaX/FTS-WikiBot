mod fwikibot;

use crate::fwikibot::entities::wiki::Wiki;
use crate::fwikibot::util::http::Http;
use crate::fwikibot::util::secrets::Secrets;
use std::collections::HashMap;
use std::env;

fn main() {
	let mut args = HashMap::new();
	for raw_argument in env::args() {
		let argument = raw_argument.to_owned();
		if argument.starts_with( "--" ) {
			let ( _, val ) = argument.split_at( 2 );
			let arg: Vec<_> = val.splitn(2, ":").collect();
			args.insert( arg[ 0 ].to_string(), arg[ 1 ].to_string() );
		}
	}

	let http_client = Http::new( String::from( "FO-nTTaX Bot" ) );
	let secrets = Secrets::new();
	let mut wiki = Wiki::new(
		http_client,
		String::from( "https://liquipedia.net/".to_owned() + args[ "wiki" ].as_str() + "/api.php" )
	);
	wiki.login( secrets );
	let mut page = wiki.get_page( args[ "page" ].clone() );
	let pagename = page.get_name();
	let pagetext = page.get_text();
	println!( "Editing page \"{}\"", pagename );
	page.set_text( pagetext + &String::from( " test" ) );
	page.save( String::from( "Test edit" ) );
}
