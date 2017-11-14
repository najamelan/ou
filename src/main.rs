extern crate pgs_files;
extern crate pwhash   ;
extern crate privdrop ;
extern crate users    ;

use std::process::exit;
use std::process::Command;
use std::os::unix::process::CommandExt;

use pgs_files::shadow;

use pwhash   ::unix;
use privdrop ::PrivDrop;
use users    ::get_current_uid;


fn main()
{
	let user = "test";
	let pass = askpass( user );

	// Get shadow entry
	//
	let option = shadow::get_entry_by_name( user );
	let entry  = option.expect( "User does not exist" );

	// Compare pass with shadow entry
	//
	if ! unix::verify( &pass, &entry.passwd )
	{
		print!( "Wrong password" );
		exit( 2 );
	}


	// We're good, user authenticated.
	//
	// Change uid
	//
	let dropped = PrivDrop::default().user( user ).apply();

	print!("{:?}\n", dropped);

	// Run command
	//
	let output = Command::new( "id" ).output()

		.unwrap_or_else( |e| { panic!( "failed to execute id: {}", e ) } )
	;

	let stdout = std::str::from_utf8( &output.stdout ).unwrap();

	print!( "{:?}", stdout );
}



fn askpass( user: &str ) -> String
{
	// Run zenity
	//
	let output = Command::new( "zenity" )

		.args( &[ "--password", "--title", &format!( "'Password for user {:}'", user ) ] )
		.uid( get_current_uid() )
		.output()
		.unwrap_or_else( |e| { panic!( "Failed to execute zenity: {}", e ) } )
	;


	// Remove trailing newline
	//
	std::str::from_utf8( &output.stdout[ ..&output.stdout.len()-1 ] )

		.expect( "Password from returned from zenity contains invalid utf." )
		.to_string()
}
